extern crate regex;

mod status;
mod headers;
mod response;


use regex::*;

use std::net::*;
use std::io::*;
use std::fs::*;
use std::process::Command;
use response::HTTPResponse;

use std::str;

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


    pub fn send_response(&mut self, res: HTTPResponse) {
        write!(self.stream, "{}", res).unwrap();

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
        "htmlt" | "html" | "htm" => "text/html",
        "css" => "text/css",
        "txt" => "text/plain",
        _ => "application/octet-stream",
    }
}

fn command_replacer(caps :&Captures) -> String {
    let out = Command::new(caps.name("command").expect("This should never happen!").as_str()).output()
        .expect("Failed to execute command").stdout;

    let mut s = String::from_utf8_lossy(&out).into_owned();
    s = str::replace(&s,"\t","&emsp;");

    str::replace(&s.to_string(),"\n","<br>")
}

fn answer(request_str: &str) -> HTTPResponse {

    let request_re = Regex::new(r"GET\s+/?(\S*)\s+HTTP").unwrap();


    let request = &request_re.captures(&request_str);

    let caps;

    match request {
        &Some(ref r) => caps = &r[1],
        &None => return HTTPResponse::error(400),
    }
    println!("Request:\n{}",request_str);
    let path = match caps {
        "" => "index.htmlt",
        s => s,
    };

    let mut content = String::new();

    match File::open(format!("templates/{}",path)) {
        Ok(mut f) => {

            f.read_to_string(&mut content).unwrap();

        }

        Err(_) => {
            return HTTPResponse::error(404);
        }
    }


    let cont = Regex::new(r"<!(?P<command>\S*)>").unwrap().replace_all(&content, command_replacer);

    let mut res = HTTPResponse::new(200,cont.to_string());
    res.headers.content_type(parse_mime(&path),"utf-8");
    println!("Response:\n{}",res);
    return res;
}
