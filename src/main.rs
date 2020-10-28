use hyper::service::Service;
use hyper::{header::HeaderValue, Body, Method, Request, Response, Server, StatusCode};
use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll};
use tokio::fs;
use uuid::Uuid;
#[macro_use]
extern crate lazy_static;
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

type SharedGames = Arc<Mutex<HashMap<String, game::GameRoom>>>;

struct GameService {
  games: SharedGames,
}

impl Service<Request<Body>> for GameService {
  type Response = Response<Body>;
  type Error = hyper::Error;
  type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

  fn poll_ready(&mut self, _: &mut Context) -> Poll<Result<(), Self::Error>> {
    Poll::Ready(Ok(()))
  }

  fn call(&mut self, req: Request<Body>) -> Self::Future {
    let b = self.games.clone();
    Box::pin(controller(req, b))
  }
}

struct GameServiceMaker {
  games: SharedGames,
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

async fn controller(
  req: Request<Body>,
  game_arc: SharedGames,
) -> Result<Response<Body>, hyper::Error> {
  match req.method() {
    &Method::GET => {
      let path_parts: Vec<&str> = req.uri().path().split("/").collect();

      let res = match path_parts[1] {
        "room" => {
          let mut response = Response::new(Body::empty());
          if path_parts.len() == 3 {
            let user_id = match req.headers().get("Cookie") {
              // TODO Handle case where cookie is not a valid uuid
              Some(c) => Uuid::parse_str(get_user_id_cookie(c)).unwrap().to_string(),
              None => {
                let uuid = Uuid::new_v4().to_string();
                let cookie_value = format!("user_id={}; SameSite=Strict; HttpOnly=true; Path=/;", uuid);
                response.headers_mut().insert(
                  hyper::header::SET_COOKIE,
                  HeaderValue::from_str(&cookie_value).unwrap(),
                );
                uuid
              }
            };

            let room = path_parts[2];
            println!("{} is joining Room: {}", user_id, room);

            {
              let mut games = game_arc.lock().unwrap();

              let current_games = games.keys();
              for (i, key) in current_games.enumerate() {
                println!("Current Games {} :: {}", i, key);
              }

              let room_key = room.to_string();
              let game = match games.get_mut(&room_key) {
                Some(val) => {
                  match val.players.iter().find(|p| p.id == user_id) {
                    Some(you) => {
                      println!("Found you! {}: {}", you.id, you.name);
                    }
                    None => {
                      println!("Didn't find you!");
                      val.players.push(game::Player::new(user_id));
                    }
                  };
                  val
                }
                None => {
                  println!("Adding new game {}", room_key);
                  let mut vec = Vec::with_capacity(20);
                  vec.push(game::Player::new(user_id));
                  games.insert(room_key.clone(), game::GameRoom { players: vec });
                  games.get_mut(&room_key).unwrap()
                }
              };

              println!("Player count: {}", game.players.len());
              for p in &game.players {
                println!("  {}: {}", p.id, p.name);
              }
              println!("\n");
            }

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
            if !requested_file.ends_with(".html") {
              match get_file(&requested_file).await {
                Ok(file) => {
                  problem = false;
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
      *response.status_mut() = StatusCode::NOT_FOUND;
      Ok(response)
    }
  }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
  let games = Arc::new(Mutex::new(HashMap::new()));
  let socket_address = ([127, 0, 0, 1], 7878).into();
  let server = Server::bind(&socket_address).serve(GameServiceMaker { games: games });
  println!("Serving game at {}", socket_address);
  server.await?;
  Ok(())
}
