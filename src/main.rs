use hyper::service::Service;
use hyper::{
  header::{self, AsHeaderName, HeaderMap, HeaderValue},
  Body, Method, Request, Response, Server, StatusCode,
};
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use tokio::fs;
use uuid::Uuid;
#[macro_use]
extern crate lazy_static;
use mime_guess;
use regex::Regex;
mod game;

// COMPILE TIME FILE INCLUDES
// static INDEDX_HTML: &str = include_str!("../dist/index.html");
// static NOT_FOUND_HTML: &str = include_str!("../dist/404.html");
// static files: HashMap<&str, &str> = [("index.html", INDEDX_HTML)].iter().cloned().collect();
// fn get_file(file_name: &str) -> &str {
//   match file_name {
//     "index.html" => INDEDX_HTML,
//     _ => NOT_FOUND_HTML,
//   }
// }

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

async fn get_file(file_name: &String) -> Result<Vec<u8>, std::io::Error> {
  let file_loc = format!("dist/{}", file_name);
  println!("get_file Name: {} Location: {}", file_name, file_loc);
  fs::read(file_loc).await
}

fn get_user_id_cookie(cookies: &HeaderValue) -> &str {
  lazy_static! {
    static ref USER_ID_REGEX: Regex = Regex::new(r"user_id=[^;]+").unwrap();
  }
  let a = cookies.to_str().unwrap();
  match USER_ID_REGEX.find(a) {
    Some(mat) => &a[mat.start() + 8..mat.end()],
    None => "",
  }
}

fn header_match<S: AsHeaderName>(headers: &HeaderMap<HeaderValue>, name: S, value: &str) -> bool {
  headers
    .get(name)
    .and_then(|v| v.to_str().ok())
    .map(|v| v.to_lowercase() == value)
    .unwrap_or(false)
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
          println!("Websocket handshake started!");
          let mut res = Response::new(Body::empty());
          let headers = req.headers();
          println!("We got these headers: {:?}", headers);

          // TODO consider using typed haeders https://docs.rs/headers/0.3.2/headers/

          let has_websocket_key = headers
            .get(header::SEC_WEBSOCKET_KEY)
            .and_then(|v| v.to_str().ok())
            .map(|v| !v.is_empty())
            .unwrap_or(false);

          if has_websocket_key
            && header_match(headers, header::UPGRADE, "websocket")
            && header_match(headers, header::SEC_WEBSOCKET_VERSION, "13")
            && header_match(headers, header::CONNECTION, "upgrade")
          {
            println!("HAPPY DAYS!");
            *res.status_mut() = StatusCode::NOT_FOUND;

          // let n = headers.typed_get::<hyper::server::conn::Connection>();
          } else {
            *res.status_mut() = StatusCode::BAD_REQUEST;
          }
          res
        }
        "room" => {
          let mut response = Response::new(Body::empty());
          if path_parts.len() == 3 {
            let user_id = match req.headers().get("Cookie") {
              // TODO Handle case where cookie is not a valid UUID
              Some(c) => Uuid::parse_str(get_user_id_cookie(c)).unwrap().to_string(),
              None => {
                let uuid = Uuid::new_v4().to_string();
                let cookie_value =
                  format!("user_id={}; SameSite=Strict; HttpOnly=true; Path=/;", uuid);
                response.headers_mut().insert(
                  hyper::header::SET_COOKIE,
                  HeaderValue::from_str(&cookie_value).unwrap(),
                );
                uuid
              }
            };

            let room = path_parts[2];
            game::join_game(game_arc, user_id, room);

            let index_file: String = "index.html".to_string();
            *response.body_mut() = match get_file(&index_file).await {
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
              match get_file(&requested_file).await {
                Ok(file) => {
                  problem = false;
                  let media_type = mime_guess::from_path(&requested_file)
                    .first_or_octet_stream()
                    .to_string();
                  println!("Media type guess {}", media_type);
                  response.headers_mut().insert(
                    hyper::header::CONTENT_TYPE,
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
            println!("thing endpoint hit!");
            Response::new(Body::from("thing endpoint hit!"))
          }
          _ => Response::new(Body::empty()),
        },
        _ => {
          let a = "404.html".to_string();
          Response::new(match get_file(&a).await {
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
      response.headers_mut().insert(
        hyper::header::ALLOW,
        HeaderValue::from_str("GET, POST").unwrap(),
      );
      Ok(response)
    }
  }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
  let games = game::new_game();
  // TODO implement TLS
  let socket_address = ([127, 0, 0, 1], 7878).into();
  let server = Server::bind(&socket_address).serve(GameServiceMaker { games: games });
  println!("Serving game at {}", socket_address);
  server.await?;
  Ok(())
}
