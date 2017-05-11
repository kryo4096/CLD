extern crate regex;

mod status;
mod headers;
mod response;


use regex::Regex;

use std::net::*;
use std::io::*;
use std::fs::*;
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


    pub fn send_response(&mut self, f: &mut Formatter) {
        write!(f, "{}", self.status);
        write!(f, "{}", self.headers);
        write!(f, "{}", self.content);
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
                http_stream.send_response(answer(&request));

            }
            Err(e)=>{panic!(e)}
        }
    }
}

fn parse_mime(path: &str) -> &'static str {

    let ext_opt = path.split('.').last();

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

fn answer(request: &str) -> HTTPResponse {
    println!("{}",&request);
    let re = Regex::new(r"GET\s+/?(\S*)\s+HTTP").unwrap();

    let caps = &re.captures(&request).expect("Invalid HTTP Request")[1];

    let path;


    match caps {
        "" => path = "index.htmlt",
        s => path = s,
    }

    match File::open(format!("templates/{}",path)) {
        Ok(mut f) => {
            let mut content = String::new();

            f.read_to_string(&mut content).unwrap();

            let mut res = HTTPResponse::new(200,content);

            res.headers.content_type(parse_mime(&path));
            println!(res)
            return res

        }

        Err(_) => {
            return HTTPResponse::error(404)
        }
    }
}
