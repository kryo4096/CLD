extern crate regex;

mod status;
mod headers;


use regex::Regex;

use std::net::*;
use std::io::*;
use std::fs::*;
use headers::HTTPHeaders;
use status::HTTPStatus;






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

    pub fn send(&mut self, s : &str){
        write!(self.stream, "{}", s);
    }

    pub fn sep(&mut self) {
        write!(self.stream, "\n\n");
    }

    pub fn sendln(&mut self, s : &str){
        self.send(s);
        self.send("\n");
    }

    pub fn send_response(status: HTTPStatus, header: HTTPHeaders, content: &str) {
        //TODO
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




    match File::open(format!("www/{}",path)) {
        Ok(mut f) => {
            let mut content = String::new();

            f.read_to_string(&mut content).unwrap();

            stream.sendln("HTTP/1.1 200 OK");
            stream.sendln(format!("Content-Type: {}",parse_file_ext(&path)).as_str());
            stream.sep();
            stream.send(&content);
        }

        Err(_) => {
            stream.sendln("HTTP/1.1 404 Not Found");
            stream.sendln("Content-Type: text/html");
            stream.sep();
            stream.send(include_str!("404.html"));
        }
    }

}

fn parse_file_ext(name: &str) -> &str {

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
