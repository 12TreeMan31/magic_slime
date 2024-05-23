use serde::{Deserialize, Serialize};
use std::str;

#[derive(Serialize, Deserialize, Debug)]
pub enum Request {
    CONNECT {
        username: String,
        ip: [u8; 4],
        port: u16,
    },
    POST {
        to: String,
        message: String,
    },
    Error,
}

impl Request {
    pub fn validate(buf: &[u8]) -> Option<Request> {
        let buf: &str = str::from_utf8(buf).unwrap();
        match serde_json::from_str(buf) {
            Ok(req) => Some(req),
            Err(_) => None,
        }
    }
    pub fn to_string(&self) -> String {
        match serde_json::to_string(self) {
            Ok(str) => str,
            Err(e) => e.to_string(),
        }
    }
}

pub fn example_calls() {
    println!("--------------EXAMPLE--------------");
    let e: Request = Request::CONNECT {
        username: "example".to_string(),
        ip: [127, 0, 0, 1],
        port: 8888,
    };
    let st: String = serde_json::to_string(&e).unwrap();
    println!("{st}");
    let ss: String = String::from("Hello World!");
    let e: Request = Request::POST {
        to: "example".to_string(),
        message: ss,
    };
    let st: String = serde_json::to_string(&e).unwrap();
    println!("{st}");
    println!("-----------------------------------");
}
