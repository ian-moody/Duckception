use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use std::convert::Infallible;

async fn hey_world(_: Request<Body>) -> Result<Response<Body>, Infallible> {
  Ok(Response::new(Body::from("hello__world!!")))
}

// async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {

#[tokio::main]
async fn main() {
  println!("Starting");
  let service_maker = make_service_fn(|_conn| async { Ok::<_, Infallible>(service_fn(hey_world)) });
  let socket_address = ([127, 0, 0, 1], 7878).into();
  let server = Server::bind(&socket_address).serve(service_maker);
  println!("Started");
  if let Err(e) = server.await {
    eprintln!("server error: {}", e);
  }
}
