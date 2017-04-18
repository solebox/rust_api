

use bson::Bson;
use mongodb::{Client, ThreadedClient};
use mongodb::db::ThreadedDatabase;
use std;
use rustc_serialize::json;

#[derive(RustcDecodable, RustcEncodable)]
struct DecMessage {
    service: String
}

pub fn insert_to_db(decrypted_token: &String, remote_ip: &std::net::SocketAddr, request_data: &String) {
    //implement insertion to mongodb
    let decrypted_json_msg: DecMessage = json::decode(decrypted_token).unwrap();
    println!("service: {}", decrypted_json_msg.service);
    println!("remote_ip: {}", remote_ip);
    println!("ver_headers_body: {}", request_data);

    let client = Client::connect("localhost", 27017)
        .expect("Failed to initialize standalone client.");

    let coll = client.db("test").collection("services");

    let doc = doc! { "service" => (decrypted_json_msg.service.to_string()),
                      "remote_ip" => (remote_ip.to_string()),
                        "request_data" => (request_data.to_string()) };

    // Insert document into 'test.movies' collection
    coll.insert_one(doc.clone(), None)
        .ok().expect("Failed to insert document.");

    // Find the document and receive a cursor
    let mut cursor = coll.find(Some(doc.clone()), None)
        .ok().expect("Failed to execute find.");

    let item = cursor.next();

    // cursor.next() returns an Option<Result<Document>>
    match item {
        Some(Ok(doc)) => match doc.get("service") {
            Some(&Bson::String(ref service)) => println!("{}", service),
            _ => panic!("Expected service to be a string!"),
        },
        Some(Err(_)) => panic!("Failed to get next from server!"),
        None => panic!("Server returned no results!"),
    }
}
