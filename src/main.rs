use rand::Rng;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::io;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::path::Path;
use std::sync::{Arc, Mutex};
use warp::{http::Response, Filter};

#[cfg(feature = "include_site")]
fn abc() {
  println!("Import site ");
}

#[cfg(not(feature = "include_site"))]
fn abc() {
  println!("get from disk every time");
}

// mod ws;
// mod handler;

pub struct Client {
  pub user_id: usize,
  pub topics: Vec<String>,
  // pub sender : Option<mpsc::UnboundedSender<std::result::Result<Message, warp::Error>>>
}

pub struct RegisterRequest {
  user_id: usize,
}

pub struct RegisterResponse {
  url: String,
}

pub struct Event {
  topic: String,
  user_id: Option<usize>,
  message: String,
}

pub struct TopicRequst {
  topics: Vec<String>,
}

// type Result<T> = std::result::Result<T, Rejection>;
type Clients = Arc<Mutex<HashMap<String, Client>>>;

#[tokio::main]
async fn main() {
  
  abc();

  static BODY: &str = r#"
<html>
  <head>
      <title>HTML with warp!</title>
  </head>
  <body>
      <h1>404</h1>
  </body>
</html>
"#;

  static BODY2: &str = r#"
<html>
  <head>
      <title>HTML with warp!</title>
  </head>
  <body>
      <h1>test index.html</h1>
  </body>
</html>
"#;

  let clients: Clients = Arc::new(Mutex::new(HashMap::new()));

  let not_found = warp::any().map(|| {
    let number: u32 = rand::thread_rng().gen_range(1, 101);
    println!("Hello {}", number);
    return warp::reply::with_header(
      warp::reply::html(BODY),
      "Set-Cookie",
      format!("rust_warp_cookie=val_{}; SameSite=Strict", number),
    );
  });

  let get_game = |a: String| {
    let number: u32 = rand::thread_rng().gen_range(1, 101);
    println!("Hello {} {}", number, a);
    if a != "index.html" {
      println!("HI")
    }
    return warp::reply::with_header(
      warp::reply::html(BODY2),
      "Set-Cookie",
      format!("rust_warp_cookie=val_{}; SameSite=Strict", number),
    );
  };

  let index = warp::path!(String).map(get_game);
  let www = warp::fs::dir("dist");
  let valid_dist = Path::new("dist/index.html").exists();
  println!("valid_dist {}", valid_dist);
  // let tt = warp::path!(String).map(|a: String| format!("Hello {}", a));

  // let routes = index.or(www).or(not_found);
  let routes = www;

  warp::serve(routes).run(([127, 0, 0, 1], 7878)).await;
}
