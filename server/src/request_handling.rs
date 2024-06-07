use std::error::Error;
use serde::{Deserialize, Serialize};
use crate::ErrorAlias;

#[derive(Debug)]
pub struct HttpRequest {
    pub request: String,
    pub header: String,
    pub body: String,
}

impl HttpRequest {
    pub fn parse_request(request: &str, req_size: usize) -> Result<Self, Box<dyn Error + 'static>> {
        let mut request_iter = request.lines().into_iter().map(|line| line.to_string()).into_iter();

        let request = request_iter.next().unwrap();
        let mut header = String::new();
        let mut body = String::new();

        let mut found_body = false;
        let mut read_size = 0;

        for line in request_iter {
            read_size += line.len();
            if read_size > req_size {
                let mut split = line.split("\0");
                body += split.next().unwrap();
                break;
            }

            if line.is_empty() {
                found_body = true;
                continue;
            }
            if !found_body {
                header += &line;
                header += " "
            }
            else {
                body += &line;
                body += " "
            }
        }

        let parsed_req = HttpRequest {
            request,
            header,
            body
        };

        return Ok(parsed_req);
    }
}

#[derive(Serialize, Deserialize)]
enum RequestType {
    Up,
    Down,
}

impl ToString for RequestType {
    fn to_string(&self) -> String {
         match self {
            RequestType::Up => String::from("Up"),
            RequestType::Down => String::from("Down"),
        }
    }
}

pub struct GameRequest {
    player_num: usize,
    request_type: RequestType,
    multiplier: f32,
}

#[derive(Serialize, Deserialize)]
struct ParsedRequest {
    request_type: RequestType,
    multiplier: f32,
}

impl GameRequest {
    pub fn from_request_body(player_num: usize, request_body: &String) -> Result<Self, ErrorAlias> {
        println!("{request_body}");
        let parsed_request: ParsedRequest = serde_json::from_str(request_body)?;
        Ok(Self {
            player_num,
            request_type: parsed_request.request_type,
            multiplier: parsed_request.multiplier,
        })
    } 
}

impl ToString for GameRequest {
    fn to_string(&self) -> String {
        return format!("Player Number: {}, Request Type: {}, Multiplier: {}", self.player_num, self.request_type.to_string(), self.multiplier);
    }
}
