// use futures::TryStreamExt as _;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Method, Request, Response, Server, StatusCode};
use std::convert::Infallible;
use tokio::fs;

// COMPILE TIME FILE INCLUDES

// static INDEDX_HTML: &str = include_str!("../dist/index.html");
static NOT_FOUND_HTML: &str = include_str!("../dist/404.html");
// static files: HashMap<&str, &str> = [("index.html", INDEDX_HTML)].iter().cloned().collect();
// fn get_file(file_name: &str) -> &str {
//   match file_name {
//     "index.html" => INDEDX_HTML,
//     _ => NOT_FOUND_HTML,
//   }
// }

async fn hey_world(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
  let path_parts: Vec<&str> = req.uri().path().split("/").collect();
  let part = path_parts[1];
  let is_valid_path = path_parts.len() == 2 && !part.is_empty();
  println!("method? {} path_parts {:?}", req.method(), path_parts);

  match req.method() {
    &Method::GET => {
      let mut response = Response::new(Body::empty());
      let html_body = if is_valid_path {
        let file_loc = format!("dist/{}", part);
        println!("hey? {} {}", part, file_loc);
        let content = match fs::read(file_loc).await {
          Ok(file) => file,
          Err(_) => br#"e{"ddie"}"#.to_vec(),
        };
        Body::from(content)
      } else {
        Body::from(NOT_FOUND_HTML)
      };
      *response.body_mut() = html_body;
      Ok(response)
    }
    &Method::POST => {
      println!("Just POSTED!!!");
      let full_body = hyper::body::to_bytes(req.into_body()).await?;

      let reversed = full_body
        .iter()
        .rev()
        .cloned()
        .map(|byte| byte.to_ascii_uppercase())
        .collect::<Vec<u8>>();

      // let mapping = req.into_body().map_ok(|chunk| {
      //   chunk
      //     .iter()
      //     .map(|byte| byte.to_ascii_uppercase())
      //     .collect::<Vec<u8>>()
      // });
      // Ok(Response::new(Body::wrap_stream(mapping))
      Ok(Response::new(Body::from(reversed)))
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
  let service_maker = make_service_fn(|_conn| async { Ok::<_, Infallible>(service_fn(hey_world)) });
  let socket_address = ([127, 0, 0, 1], 7878).into();
  let server = Server::bind(&socket_address).serve(service_maker);
  println!("Started {}", socket_address);
  server.await?;
  Ok(())
}
