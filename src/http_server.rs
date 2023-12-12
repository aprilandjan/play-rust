use std::fs;
use std::{
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

use crate::thread_pool::ThreadPool;

pub fn serve_and_shut_down() {
    println!("");
    println!("start http server...");

    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    let pool = ThreadPool::new(4);

    // `take` makes the iterator yield at specific count
    for stream in listener.incoming().take(2) {
        let stream = stream.unwrap();
        println!("incoming connection established");
        pool.execute(|| {
            handle_stream(stream);
        });
    }

    println!("http server shut down.");
}

fn handle_stream(mut stream: TcpStream) {
    let buffer_reader = BufReader::new(&stream);
    let req: Vec<_> = buffer_reader
        .lines()
        .map(|r| r.unwrap()) // extract correct utf8 char line?...
        .take_while(|item| !item.is_empty()) // filter empty lines
        .collect(); // lazy execute the iterator

    // ':#?' means pretty-print the `Debug` formatting, with line-breaks and indentation
    // see https://doc.rust-lang.org/std/fmt/index.html#sign0
    // println!("Request: {:#?}", req);
    println!("Request path: {:?}", req[0]);

    // if req[0] == "GET / HTTP/1.1" {
    //   let status_line = "HTTP/1.1 200 OK";
    //   let contents = "<div>hello, world</div>";
    //   let size = contents.len();
    //   let resp_data = format!("{status_line}\r\nContent-Length: {size}\r\n\r\n{contents}");

    //   stream.write_all(resp_data.as_bytes()).unwrap();
    // } else {
    //   let status_line = "HTTP/1.1 200 OK";
    //   let contents = "<div>404 Not found</div>";
    //   let size = contents.len();
    //   let resp_data = format!("{status_line}\r\nContent-Length: {size}\r\n\r\n{contents}");

    //   stream.write_all(resp_data.as_bytes()).unwrap();
    // }

    // another route mode...
    let (status_line, filename) = match &req[0][..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "index.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(10));
            ("HTTP/1.1 200 OK", "sleep.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
    };

    let contents = fs::read_to_string("static/".to_owned() + filename).unwrap();
    let size = contents.len();
    let resp_data = format!("{status_line}\r\nContent-Length: {size}\r\n\r\n{contents}");
    stream.write_all(resp_data.as_bytes()).unwrap();
}
