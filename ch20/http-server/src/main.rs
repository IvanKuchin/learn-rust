use std::io::BufReader;
use std::io::{BufRead, Write};
use std::net::TcpStream;
use std::net::TcpListener;
use std::thread;

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader
        .lines()
        .next()
        .expect("Could not read from stream")
        .expect("Could not read line from stream");

    let status_line = match request_line.as_str() {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "root"),
        "GET /another HTTP/1.1" => ("HTTP/1.1 200 OK", "another"),
        _ => ("HTTP/1.1 404 NOT FOUND", "error"),
    };

    let response = format!("{}\r\nContent-Length: {}\r\n\r\n{}", status_line.0, status_line.1.len(), status_line.1);
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();

}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        thread::spawn( || {
            handle_connection(stream);
        });
    }
}
