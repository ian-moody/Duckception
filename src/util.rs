use hyper::header::{AsHeaderName, HeaderMap, HeaderValue};
use regex::Regex;
use tokio::fs;
use uuid::Uuid;

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
    info!("Getting file: {}", file_loc);
    fs::read(file_loc).await
}

pub fn get_session(a: &str) -> (&str, String) {
    lazy_static! {
        static ref USER_ID_REGEX: Regex = Regex::new(r"session=[^;]+").unwrap();
    }
    match USER_ID_REGEX.find(a) {
        Some(mat) => {
            let parts = &a[mat.start()..mat.end()].split(":").collect::<Vec<&str>>();
            // TODO Handle case where cookie is not a valid UUID
            (
                &parts[0][8..],
                Uuid::parse_str(parts[1]).unwrap().to_string(),
            )
        }
        None => ("", "".to_string()),
    }
}

pub fn header_match<S: AsHeaderName>(
    headers: &HeaderMap<HeaderValue>,
    name: S,
    value: &str,
) -> bool {
    headers
        .get(name)
        .and_then(|v| v.to_str().ok())
        .map(|v| v.to_lowercase() == value)
        .unwrap_or(false)
}
