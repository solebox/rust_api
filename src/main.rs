extern crate hyper;
extern crate rustc_serialize;
extern crate hyperz;

use hyperz::mein_libs::cryptoz::{enc, dec};
use rustc_serialize::json;
use std::io::{Read, Write};
use hyper::server::{Server, Request, Response};
use hyper::status::StatusCode;

#[derive(RustcDecodable, RustcEncodable)]
struct TestStruct {
    token: String,
    pass: String
}

#[derive(RustcDecodable, RustcEncodable)]
struct DecMessage {
    service: String
}

fn main() {
    //test_enc();

    Server::http("0.0.0.0:8001").unwrap().handle(|mut req: Request, mut res: Response| {
        let key = [149, 251, 204, 100, 110, 129, 252, 206, 71, 66, 193, 99, 43, 218, 49, 35, 199, 112, 22, 154, 126, 9, 226, 228, 49, 162, 243, 50, 1, 174, 207, 254];
        match req.method {
            hyper::Post => {
                let mut result = "".to_string();

                let my_remote = format!("{}\n", req.remote_addr);
                //result.push_str(&my_remote);

                let my_version = format!("{}\n", req.version);
                result.push_str(&my_version);


                for header in req.headers.iter() {
                    let my_headers = format!("{}:{}\n", 
                                        header.name(), 
                                        &header.value_string().to_string());
                    result.push_str(&my_headers);
                }


                
                // body handling
                let mut my_body = String::new();
                let len = req.read_to_string(&mut my_body);

                let decoded_body: TestStruct = json::decode(&my_body).unwrap();
                result.push_str(&my_body);
                let my_decoded_body = format!("\ntoken:{}\n", decoded_body.token);

//                result.push_str(&my_decoded_body);

                let decrypted = dec(&decoded_body.token, &key);
                let my_decrypted_token = format!("decrypted: {}", decrypted);
                //result.push_str(&my_decrypted_token);

                insert_to_db(&decrypted, &req.remote_addr, &result);

                let response = "wrong creds";
                write!(&mut res.start().unwrap(),"{}",&response).unwrap();
                //io::copy(&mut req, &mut res.start().unwrap()).unwrap();
            },
            _ => *res.status_mut() = StatusCode::MethodNotAllowed
        }
    }).unwrap();
}

fn insert_to_db(decrypted_token: &String, remote_ip: &std::net::SocketAddr, request_data: &String) {
    //implement insertion to mongodb
    let decrypted_json_msg: DecMessage = json::decode(decrypted_token).unwrap();
    println!("service: {}", decrypted_json_msg.service);
    println!("remote_ip: {}", remote_ip);
    println!("ver_headers_body: {}", request_data);
}

fn test_enc(){
    let message = "{\"service\": \"storm proxies\"}";

   // let mut key: [u8; 32] = [0; 32];



    // In a real program, the key and iv may be determined
    // using some other mechanism. If a password is to be used
    // as a key, an algorithm like PBKDF2, Bcrypt, or Scrypt (all
    // supported by Rust-Crypto!) would be a good choice to derive
    // a password. For the purposes of this example, the key and
    // iv are just random values.

    // cheating here a bit for science
    let key = [149, 251, 204, 100, 110, 129, 252, 206, 71, 66, 193, 99, 43, 218, 49, 35, 199, 112, 22, 154, 126, 9, 226, 228, 49, 162, 243, 50, 1, 174, 207, 254];


    let encryptush: String = enc(&message.to_string(), &key);
    let res = dec(&encryptush.to_string(), &key);
    println!("{}", &encryptush);
    println!("{}", &res);
    assert!(message.as_bytes() == res.as_bytes());
}

