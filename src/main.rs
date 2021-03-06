extern crate hyper;
extern crate rustc_serialize;
extern crate hyperz;
extern crate mongodb;
extern crate bson;

use hyperz::mein_libs::cryptoz::{enc, dec};
use hyperz::mein_libs::mongo::{insert_to_db, get_connection};
use rustc_serialize::json;
use std::io::{Read, Write};
use hyper::server::{Server, Request, Response};
use hyper::status::StatusCode;
use mongodb::{Client, ThreadedClient};

use bson::Bson;

use mongodb::db::ThreadedDatabase;

#[derive(RustcDecodable, RustcEncodable)]
struct TestStruct {
    token: String,
    pass: String
}


fn main() {
    test_enc();
    let client = get_connection("localhost".to_string(), 27017);
    Server::http("0.0.0.0:8001").unwrap().handle(|mut req: Request, mut res: Response| {
        let key = [104, 101, 175, 217, 217, 34, 43, 150, 203, 235, 61, 161, 114, 41, 143, 106, 254, 248, 248, 77, 171, 127, 103, 194, 158, 98, 143, 144, 138, 157, 209, 173];
        match req.method {
            hyper::Post => {

                let mut result = "".to_string();

                //let my_remote = format!("{}\n", req.remote_addr);
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
//                let my_decoded_body = format!("\ntoken:{}\n", decoded_body.token);

//                result.push_str(&my_decoded_body);

                let decrypted = dec(&decoded_body.token, &key);
//                let my_decrypted_token = format!("decrypted: {}", decrypted);
                //result.push_str(&my_decrypted_token);

                insert_to_db(&client, &decrypted, &req.remote_addr, &result);

                let response = "wrong creds";
                write!(&mut res.start().unwrap(),"{}",&response).unwrap();
                //io::copy(&mut req, &mut res.start().unwrap()).unwrap();
            },
            _ => *res.status_mut() = StatusCode::MethodNotAllowed
        }
    }).unwrap();
}


fn test_enc(){
    let message = "{\"service\": \"dora the explorer\"}";

   // let mut key: [u8; 32] = [0; 32];



    // In a real program, the key and iv may be determined
    // using some other mechanism. If a password is to be used
    // as a key, an algorithm like PBKDF2, Bcrypt, or Scrypt (all
    // supported by Rust-Crypto!) would be a good choice to derive
    // a password. For the purposes of this example, the key and
    // iv are just random values.

    // cheating here a bit for science
    let key = [104, 101, 175, 217, 217, 34, 43, 150, 203, 235, 61, 161, 114, 41, 143, 106, 254, 248, 248, 77, 171, 127, 103, 194, 158, 98, 143, 144, 138, 157, 209, 173];


    let encryptush: String = enc(&message.to_string(), &key);
    let res = dec(&encryptush.to_string(), &key);
    println!("{}", &encryptush);
    println!("{}", &res);
    assert!(message.as_bytes() == res.as_bytes());
}

