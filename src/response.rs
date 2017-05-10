use headers::HTTPHeaders;
use status::HTTPStatus;

use std::fs::File;
use std::io::Read;

pub struct HTTPResponse {
    pub status: HTTPStatus,
    pub headers: HTTPHeaders,
    pub content: String,
}

fn parse_file_ext(name: &str) -> &'static str {

    let ext_opt = name.split('.').last();

    let ext;

    match ext_opt {
        None => return "application/octet-stream",
        Some(e) => ext = e ,
    };

    match ext {
        "html" | "htm" => "text/html",
        "css" => "text/css",
        "txt" => "text/plain",
        _ => "application/octet-stream",
    }
}

impl HTTPResponse {
    pub fn new(status : u16, content: String) -> Self{
        Self {
            status: HTTPStatus::from_code(status).expect("Invalid status code."),
            headers: HTTPHeaders::new(),
            content: content,
        }
    }

    pub fn error(status :u16) -> Self{
        let mut res = Self::new(status,format!(include_str!("error.htmlt"),status,status,HTTPStatus::get_message(status)));
        res.headers.content_type("text/html");
        res
    }

    pub fn from_file(path : &str) -> Self{
        match File::open(format!("www/{}",path)) {
            Ok(mut f) => {
                let mut content = String::new();

                f.read_to_string(&mut content).unwrap();

                let mut res = HTTPResponse::new(200,content);

                res.headers.content_type(parse_file_ext(&path));

                res

            }

            Err(_) => {
                Self::error(404)
            }
        }
    }
}
