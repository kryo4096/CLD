use std::collections::HashMap;
use std::fmt::{Display,Formatter,Result};




#[derive(Clone, Copy)]
pub struct HTTPStatus{
    version: &'static str,
    code: u16,
    name: &'static str,
}

impl HTTPStatus {
    pub fn from_code(code: u16) -> Option<HTTPStatus>{



            Some(HTTPStatus{
                version: "1.1",
                code:   code,
                name:   Self::get_message(code),
            })

    }

    pub fn get_message(status: u16) -> &'static str {
        let mut status_codes: HashMap<u16, &'static str> = [
            (100,"Continue"),
            (101,"Switching Protocols"),
            (200,"OK"),
            (201,"Created"),
            (202,"Accepted"),
            (203,"Non-Authoritative Information"),
            (204,"No Content"),
            (205,"Reset Content"),
            (206,"Partial Content"),
            (300,"Multiple Choices"),
            (301,"Moved Permanently"),
            (302,"Found"),
            (303,"See Other"),
            (304,"Not Modified"),
            (305,"Use Proxy"),
            (307,"Temporary Redirect"),
            (400,"Bad Request"),
            (401,"Unauthorized"),
            (402,"Payment Required"),
            (403,"Forbidden"),
            (404,"Not Found"),
            (405,"Method Not Allowed"),
            (406,"Not Acceptable"),
            (407,"Proxy Authentication Required"),
            (408,"Request Time-out"),
            (409,"Conflict"),
            (410,"Gone"),
            (411,"Length Required"),
            (412,"Precondition Failed"),
            (413,"Request Entity Too Large"),
            (414,"Request-URI Too Large"),
            (415,"Unsupported Media Type"),
            (416,"Requested range not satisfiable"),
            (417,"Expectation Failed"),
            (500,"Internal Server Error"),
            (501,"Not Implemented"),
            (502,"Bad Gateway"),
            (503,"Service Unavailable"),
            (504,"Gateway Time-out"),
            (505,"HTTP Version not supported")
        ].iter().cloned().collect();

        status_codes.entry(status).or_insert("Invalid Status Code")
    }
}

impl Display for HTTPStatus {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "HTTP/{} {} {}\n", self.version, self.code, self.name)
    }
}
