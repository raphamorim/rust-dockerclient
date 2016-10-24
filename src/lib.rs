#![crate_type = "lib"]
#![crate_name = "docker"]

extern crate hyper;
extern crate rustc_serialize;

use std::io::Read;
use rustc_serialize::json::{self, Json};
use std::env::{self, VarError};
use hyper::{Client, Url};

pub struct Docker {
    endpoint: &'static str
}

impl Docker {
    pub fn new(endpoint : &'static str) -> Docker {
        Docker { endpoint: endpoint }
    }

    pub fn list_images(&self) {
        let api: &str = "/images/json";
        let url = &format!("{}{}", self.endpoint, api);
        docker_list_images(url)
    }
}

fn docker_list_images(url : &str) {
    let client = Client::new();
    println!("{}", url);
    let mut response = match client.get(url).send() {
        Ok(response) => response,
        Err(_) => panic!("Whoops."),
    };
    let mut buf = String::new();
    match response.read_to_string(&mut buf) {
        Ok(_) => (),
        Err(_) => panic!("I give up."),
    };
    // println!("{}", buf);
    let json = Json::from_str(&buf).unwrap();
    println!("{}", json.find_path(&["Id"]).unwrap());
}

// #[test]
// fn main() {
    // let endpoint = "http://127.0.0.1:5000";
    // let client = Docker::new(endpoint);
    // let images = client.list_images();
// }
