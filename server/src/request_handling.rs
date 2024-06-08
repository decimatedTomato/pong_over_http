use std::error::Error;
use serde::{Deserialize, Serialize};
use crate::{game::{GameState, Vec2}, ErrorAlias};

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
    Neutral = 0,
    Up = 1,
    Down = 2,
}

impl ToString for RequestType {
    fn to_string(&self) -> String {
         match self {
            RequestType::Up => String::from("Up"),
            RequestType::Down => String::from("Down"),
            RequestType::Neutral => String::from("Neutral"),
        }
    }
}

pub struct GameRequest {
    player_num: usize,
    request_type: RequestType,
}
#[derive(Serialize, Deserialize)]
struct ParsedRequest {
    request_type: RequestType,
}
impl GameRequest {
    pub fn from_request_body(player_num: usize, request_body: &String) -> Result<Self, ErrorAlias> {
        println!("{request_body}");
        let parsed_request: ParsedRequest = serde_json::from_str(request_body)?;
        Ok(Self {
            player_num,
            request_type: parsed_request.request_type,
        })
    } 
}
impl ToString for GameRequest {
    fn to_string(&self) -> String {
        return format!("Player Number: {}, Request Type: {}", self.player_num, self.request_type.to_string());
    }
}

#[derive(Serialize, Deserialize)]
pub struct GameResponse {
    p1_pos: Vec2,
    p2_pos: Vec2,
    ball_pos: Vec2,
    current_score: [u32; 2],
}
impl GameResponse {
    pub fn new(game_state: &GameState) -> Self {
        Self {
            p1_pos: game_state.p1_pos.clone(),
            p2_pos: game_state.p2_pos.clone(),
            ball_pos: game_state.ball_pos.clone(),
            current_score: game_state.current_score.into(),
        }
    }
}
