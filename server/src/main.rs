use std::{
    error::Error,
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    thread::{self, JoinHandle},
};

fn main() -> Result<(), Box<dyn Error + 'static>> {
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
    let mut buf = [0; 1000];
    loop {
        stream.read(&mut buf).unwrap();

        let request = String::from_utf8_lossy(&buf);

        println!("client request: {}", request);
        stream.write("Hello World".as_bytes()).unwrap();
    }
}
