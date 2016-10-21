#![crate_type = "lib"]
#![crate_name = "docker"]

extern crate hyper;
use std::io::Read;

struct Docker {
    endpoint: &'static str
}

static mut Client : Docker = Docker {
    endpoint: "/var/run/docker.sock"
};

pub fn new_client(end : &'static str) {
    unsafe {
        Client = Docker {
            endpoint: end
        };
    }
}

pub fn list_images() {
    let client = hyper::Client::new();
    let api: &str = "/images/json";
    let url = format!("{}{}", unsafe { Client.endpoint }, api);
    println!("{}", url);
    let mut response = match client.get(&url).send() {
        Ok(response) => response,
        Err(_) => panic!("Whoops."),
    };
    let mut buf = String::new();
    match response.read_to_string(&mut buf) {
        Ok(_) => (),
        Err(_) => panic!("I give up."),
    };
    println!("buf: {}", buf);
}

