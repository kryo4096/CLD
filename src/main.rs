extern crate regex;

mod status;
mod headers;
mod response;


use regex::Regex;

use std::net::*;
use std::io::*;
use std::fs::*;
use headers::HTTPHeaders;
use status::HTTPStatus;
use response::HTTPResponse;






struct HTTPStream {
    stream: TcpStream,
}

impl HTTPStream {

    pub fn from_tcp_stream(stream: TcpStream) -> HTTPStream {
        HTTPStream{stream: stream}
    }

    pub fn get_request(&self, buf: &mut String){

        let reader = BufReader::new(&self.stream);

        for line in reader.lines().by_ref(){

            let line = line.unwrap();
            match &*line{
                "" => break,
                & _ => {
                    buf.push_str(&*line.to_string());
                    buf.push_str("\n");
                }

            }

        }

    }


    pub fn send_response(&mut self, response: HTTPResponse) {
        write!(self.stream, "{}", response.status);
        write!(self.stream, "{}", response.headers);
        write!(self.stream, "{}", response.content);
    }


}



fn main() {

    let listener = TcpListener::bind("127.0.0.1:10101").unwrap();

    for stream_res in listener.incoming() {
        match stream_res {
            Ok(stream) => {
                let mut http_stream = HTTPStream::from_tcp_stream(stream);
                let mut request = String::new();
                http_stream.get_request(&mut request);
                answer(&mut http_stream, &request);

            }
            Err(e)=>{panic!(e)}
        }
    }
}

fn answer(stream: &mut HTTPStream, request: &str){
    let re = Regex::new(r"GET\s+/?(\S*)\s+HTTP").unwrap();

    let caps = &re.captures(&request).expect("Invalid HTTP Request")[1];

    let path;


    match caps {
        "" => path = "index.html",
        s => path = s,
    }


    stream.send_response(HTTPResponse::from_file(path));

}
