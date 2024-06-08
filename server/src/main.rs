mod request_handling;
pub mod game;
use std::{
    error::Error,
    io::Read,
    net::{TcpListener, TcpStream},
    thread::{self, JoinHandle},
};

pub type ErrorAlias = Box<dyn Error + 'static>;

fn main() -> Result<(), ErrorAlias> {
    let listener = TcpListener::bind("127.0.0.1:42069")?;
    let mut handles: Vec<JoinHandle<()>> = Vec::new();

    for stream in listener.incoming() {
        let stream = match stream {
            Ok(r) => r,
            Err(e) => {
                eprintln!("{}", e);
                continue;
            }
        };

        let thread = thread::spawn(move || handle_stream(stream));
        handles.push(thread);
    }

    handles
        .into_iter()
        .for_each(|handle| handle.join().unwrap());
    Ok(())
}

fn handle_stream(mut stream: TcpStream) {
    let mut buf = [0; 2048];
    loop {
        let size = stream.read(&mut buf).unwrap();
        let request = std::str::from_utf8(&buf).unwrap();

        let parsed_request = request_handling::HttpRequest::parse_request(request, size).unwrap();
        if parsed_request.body.is_empty() {
            //todo: fix this shit
            continue;
        }

        println!("{:?}", parsed_request);
        let game_req = request_handling::GameRequest::from_request_body(0, &parsed_request.body).unwrap();
        println!("{}", game_req.to_string());
    }
}
