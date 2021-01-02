use hyper::{ header::{ AsHeaderName, HeaderMap, HeaderValue} };
use tokio::fs;
use regex::Regex;

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

pub async fn get_web_file(file_name: &String) -> Result<Vec<u8>, std::io::Error> {
  let file_loc = format!("dist/{}", file_name);
  println!("get_file Name: {} Location: {}", file_name, file_loc);
  fs::read(file_loc).await
}

pub fn get_user_id_cookie(cookies: &HeaderValue) -> &str {
  lazy_static! {
    static ref USER_ID_REGEX: Regex = Regex::new(r"user_id=[^;]+").unwrap();
  }
  let a = cookies.to_str().unwrap();
  match USER_ID_REGEX.find(a) {
    Some(mat) => &a[mat.start() + 8..mat.end()],
    None => "",
  }
}

pub fn header_match<S: AsHeaderName>(headers: &HeaderMap<HeaderValue>, name: S, value: &str) -> bool {
  headers
    .get(name)
    .and_then(|v| v.to_str().ok())
    .map(|v| v.to_lowercase() == value)
    .unwrap_or(false)
}