use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::io::{self, BufRead};
use std::collections::HashMap;

pub fn start_server() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    for stream in listener.incoming(){
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}

struct Request {
    endpoint: String,
    reqtype: RequestType,
    body: String    
}

impl Request {
    fn new() -> Request {
        return Request {endpoint: String::from("/"), reqtype: RequestType::NONE, body: String::from("") }
    }
}

enum RequestType {
   GET, POST, PUT, DELETE, NONE
}

struct Response {
    body : String
}

impl Response {
    fn new() -> Response {
        return Response { body: String::from("") };
    }

    fn new_failed() -> Response {
        return Response { body: String::from("Your request is not valid!") }
    }
}

impl RequestType {
    fn to_string(&self) -> String {
        match &self {
            RequestType::GET => return String::from("GET"),
            RequestType::POST => return String::from("POST"),
            RequestType::PUT => return String::from("PUT"),
            RequestType::DELETE => return String::from("DELETE"),
            RequestType::NONE => return String::from("NONE")
        }
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    let data = String::from_utf8_lossy(&buffer[..]);
    let mut request = Request::new();
    for line in data.lines() {

        if line.contains("HTTP/1.1") {
            if line.contains("GET") {
                request.reqtype = RequestType::GET
            }
            else if line.contains("POST") {
                request.reqtype = RequestType::POST
            }
            else if line.contains("PUT") {
                request.reqtype = RequestType::PUT
            }       
            else if line.contains("DELETE") {
                request.reqtype = RequestType::DELETE
            }

            if request.reqtype.to_string() != "NONE" {
               
                let line = &line.replace(&request.reqtype.to_string(), "").replace("HTTP/1.1", "").replace(" ", "");
                request.endpoint = line.to_string();
            } 
            break;
        }
    }

    println!("{} request received on {}", request.reqtype.to_string(), request.endpoint);
    stream.write(generate_response(&request).body.as_bytes());
}


fn generate_response(request: &Request) -> Response {
    let mut urlmap = HashMap::new();
    urlmap.insert("/hello".to_string(), "Hello world!".to_string());
    urlmap.insert("/burakturker".to_string(), "Hi my name is Burak Turker!".to_string());
    let mut response = Response::new();
    let rbody ;
    match urlmap.get(&request.endpoint) {
        Some(s) => rbody = s,
        None => {
            println!("{} => Url not found.", request.endpoint);
            response.body = format!(
                "HTTP/1.1 404 NOT FOUND\r\nContent-Length: {}\r\n\r\n{}",
                "Page not found bro".len(),
                "Page not found bro"
            );
            return response;
        }
    }

    response.body = format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
        rbody.len(),
        rbody
    );

    return response;
}
