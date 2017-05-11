use headers::HTTPHeaders;
use status::HTTPStatus;
use std::fmt::Display;
//use std::os::Process;

pub struct HTTPResponse {
    pub status: HTTPStatus,
    pub headers: HTTPHeaders,
    pub content: String,
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

}


impl Display for HTTPResponse {
    pub fn fmt(&mut self, response: HTTPResponse) {
        write!(self.stream, "{}", response.status);
        write!(self.stream, "{}", response.headers);
        write!(self.stream, "{}", response.content);
    }
}
