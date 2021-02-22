use futures::prelude::*;
use headers::{self, HeaderMapExt};
use hyper::{
    header::{self, HeaderValue},
    service::Service,
    upgrade::Upgraded,
    Body, Method, Request, Response, Server, StatusCode,
};
use std::{
    env,
    pin::Pin,
    task::{Context, Poll},
};
#[macro_use]
extern crate lazy_static;
use mime_guess;
use tokio_tungstenite::{tungstenite::protocol, WebSocketStream};
mod game;
mod util;

#[macro_use]
extern crate log;

struct GameService {
    games: game::SharedGames,
}

impl Service<Request<Body>> for GameService {
    type Response = Response<Body>;
    type Error = hyper::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, _: &mut Context) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: Request<Body>) -> Self::Future {
        Box::pin(controller(req, self.games.clone()))
    }
}

struct GameServiceMaker {
    games: game::SharedGames,
}

impl<T> Service<T> for GameServiceMaker {
    type Response = GameService;
    type Error = hyper::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, _: &mut Context) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, _: T) -> Self::Future {
        let games = self.games.clone();
        Box::pin(async move { Ok(GameService { games: games }) })
    }
}

fn upgrade_connection(
    req: Request<Body>,
) -> Result<
    (
        Response<Body>,
        impl Future<Output = Result<WebSocketStream<Upgraded>, ()>> + Send,
    ),
    Response<Body>,
> {
    info!("Websocket handshake started!");
    let mut res = Response::new(Body::empty());
    let headers = req.headers();
    let key = headers.typed_get::<headers::SecWebsocketKey>();
    let connection_is_upgrade = match headers.typed_get::<headers::Connection>() {
        Some(x) => x.contains(header::UPGRADE),
        None => false,
    };

    if !key.is_none()
        && util::header_match(headers, header::UPGRADE, "websocket")
        && util::header_match(headers, header::SEC_WEBSOCKET_VERSION, "13")
        && connection_is_upgrade
    {
        let h = res.headers_mut();
        h.typed_insert(headers::Upgrade::websocket());
        h.typed_insert(headers::SecWebsocketAccept::from(key.unwrap()));
        h.typed_insert(headers::Connection::upgrade());
        *res.status_mut() = StatusCode::SWITCHING_PROTOCOLS;

        let upgraded = req
            .into_body()
            .on_upgrade()
            .map_err(|err| error!("Cannot create websocket: {} ", err))
            .and_then(|upgraded| async {
                info!("Connection upgraded to websocket");
                let r =
                    WebSocketStream::from_raw_socket(upgraded, protocol::Role::Server, None).await;
                Ok(r)
            });

        Ok((res, upgraded))
    } else {
        *res.status_mut() = StatusCode::BAD_REQUEST;
        Err(res)
    }
}

fn handle_ws_connection(req: Request<Body>) -> Result<Response<Body>, std::io::Error> {
    let res = match upgrade_connection(req) {
        Err(res) => res,
        Ok((res, ws)) => {
            let ws_task = async {
                match ws.await {
                    Ok(ws) => {
                        info!("Spawning WS");
                        let mut counter = 0;
                        let (tx, rc) = ws.split();
                        let rc = rc.try_filter_map(|m| {
                            info!("Got Message {:?}", m);
                            future::ok(match m {
                                protocol::Message::Text(t) => {
                                    counter += 1;
                                    Some(protocol::Message::text(format!(
                                        "Response {}: {}",
                                        counter, t
                                    )))
                                }
                                _ => None,
                            })
                        });

                        match rc.forward(tx).await {
                            Err(e) => error!("ws Error! {:?}", e),
                            Ok(_) => info!("WEBSCOKET ENDED"),
                        }
                    }
                    Err(e) => error!("ws Error! {:?}", e),
                }
            };

            tokio::spawn(ws_task);
            res
        }
    };
    Ok(res)
}

fn error_response(err: String) -> Response<Body> {
    Response::builder()
        .status(StatusCode::INTERNAL_SERVER_ERROR)
        .body(err.into())
        .unwrap()
}

async fn controller(
    req: Request<Body>,
    game_arc: game::SharedGames,
) -> Result<Response<Body>, hyper::Error> {
    match req.method() {
        &Method::GET => {
            let path_parts: Vec<&str> = req.uri().path().split("/").collect();
            let res = match path_parts[1] {
                "ws" => {
                    info!("Websocket handshake started!");
                    let headers = req.headers();
                    debug!("We got these headers: {:?}", headers);
                    handle_ws_connection(req).unwrap_or_else(|e| error_response(e.to_string()))
                }
                "room" => {
                    let mut response = Response::new(Body::empty());
                    if path_parts.len() == 3 {
                        let mut set_session = false;
                        let room = path_parts[2];
                        let cookie = match req.headers().get("Cookie") {
                            Some(c) => c.to_str().unwrap_or(""),
                            None => "",
                        };

                        let (session_room, mut user_id) = util::get_session(cookie);
                        info!(
                            "session cookie values room: {} user_id: {}",
                            session_room, user_id
                        );
                        if user_id.is_empty() {
                            user_id = game::make_user_id();
                            set_session = true;
                        } else if session_room != room {
                            set_session = true;
                        }

                        if set_session {
                            let cookie_value = format!(
                                "session={}:{}; SameSite=Strict; HttpOnly=true; Path=/;",
                                room, user_id
                            );
                            response.headers_mut().insert(
                                header::SET_COOKIE,
                                HeaderValue::from_str(&cookie_value).unwrap(),
                            );
                        }

                        game::join_game(game_arc, user_id, room);

                        let index_file: String = "index.html".to_string();
                        *response.body_mut() = match util::get_web_file(&index_file).await {
                            Ok(file) => Body::from(file),
                            Err(_) => Body::empty(),
                        };
                    }
                    response
                }
                "web" => {
                    let mut response = Response::new(Body::empty());
                    if path_parts.len() >= 2 {
                        let requested_file = path_parts[2..].join("/");
                        let mut problem = true;
                        // TODO brotti compression if supported
                        // https://crates.io/crates/async-compression
                        if !requested_file.ends_with(".html") {
                            match util::get_web_file(&requested_file).await {
                                Ok(file) => {
                                    problem = false;
                                    let media_type = mime_guess::from_path(&requested_file)
                                        .first_or_octet_stream()
                                        .to_string();
                                    info!("Media type guess {}", media_type);
                                    response.headers_mut().insert(
                                        header::CONTENT_TYPE,
                                        HeaderValue::from_str(&media_type).unwrap(),
                                    );
                                    *response.body_mut() = Body::from(file);
                                }
                                Err(_) => {}
                            }
                        }
                        if problem {
                            *response.status_mut() = StatusCode::NOT_FOUND;
                        }
                    }
                    response
                }
                "action" => match path_parts[2] {
                    "thing" => {
                        info!("thing endpoint hit!");
                        Response::new(Body::from("thing endpoint hit!"))
                    }
                    _ => Response::new(Body::empty()),
                },
                _ => {
                    let a = "404.html".to_string();
                    Response::new(match util::get_web_file(&a).await {
                        Ok(not_found_file) => Body::from(not_found_file),
                        Err(_) => Body::empty(),
                    })
                }
            };

            Ok(res)
        }
        _ => {
            let mut response = Response::default();
            *response.status_mut() = StatusCode::METHOD_NOT_ALLOWED;
            response
                .headers_mut()
                .insert(header::ALLOW, HeaderValue::from_str("GET, POST").unwrap());
            Ok(response)
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    env_logger::init();
    let args = env::args().collect::<Vec<String>>();
    let port: u16 = match args.get(1) {
        Some(val) => val.parse().unwrap_or(7878),
        None => 7878,
    };
    let games = game::new_game();

    // TODO implement TLS
    let socket_address = ([0, 0, 0, 0], port).into();
    let server = Server::bind(&socket_address).serve(GameServiceMaker { games: games });
    info!("Duckception started at {}", socket_address);
    server.await?;

    Ok(())
}
