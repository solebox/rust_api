extern crate hyper;
use hyper::buffer::BufReader;
use hyper::header::{Host, TransferEncoding, Encoding};
use hyper::net::NetworkStream;
use std::io::{self,Read, Write};
use hyper::server::{Server, Request, Response};
use hyper::status::StatusCode;

fn main() {

    Server::http("0.0.0.0:8000").unwrap().handle(|mut req: Request, mut res: Response| {
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
                let mut line = String::new();
                let len = req.read_to_string(&mut line);

                let my_body = format!("{}\n", line);
                result.push_str(&line);

                write!(&mut res.start().unwrap(),"{}",&result).unwrap();
                //io::copy(&mut req, &mut res.start().unwrap()).unwrap();
            },
            _ => *res.status_mut() = StatusCode::MethodNotAllowed
        }
    }).unwrap();
}
