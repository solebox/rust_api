extern crate hyper;
extern crate rustc_serialize;
use rustc_serialize::json;
use std::io::{Read, Write};
use hyper::server::{Server, Request, Response};
use hyper::status::StatusCode;

#[derive(RustcDecodable, RustcEncodable)]
struct TestStruct {
    token: String,
    pass: String
}
fn main() {

    Server::http("0.0.0.0:8001").unwrap().handle(|mut req: Request, mut res: Response| {
        match req.method {
            hyper::Post => {
                let mut result = "".to_string();
                for header in req.headers.iter() {
                    let my_headers = format!("{}:{}\n", 
                                        header.name(), 
                                        &header.value_string().to_string());
                    result.push_str(&my_headers);
                }
                let my_version = format!("{}\n", req.version);
                result.push_str(&my_version);
                let my_remote = format!("{}\n", req.remote_addr);
                result.push_str(&my_remote);
                
                // body handling
                let mut my_body = String::new();
                let len = req.read_to_string(&mut my_body);

                let decoded_body: TestStruct = json::decode(&my_body).unwrap();
                result.push_str(&my_body);
                let my_decoded_body = format!("\ntoken:{}", decoded_body.token);
                result.push_str(&my_decoded_body);

                write!(&mut res.start().unwrap(),"{}",&result).unwrap();
                //io::copy(&mut req, &mut res.start().unwrap()).unwrap();
            },
            _ => *res.status_mut() = StatusCode::MethodNotAllowed
        }
    }).unwrap();
}
