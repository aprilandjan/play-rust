use std::{net::{TcpListener, TcpStream}, io::{BufReader, BufRead}};

pub fn serve() {
  println!("start http server...");

  let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

  for stream in listener.incoming() {
    let stream = stream.unwrap();
    println!("incoming connection established");
    handle_stream(stream);
  }
}

fn handle_stream(mut stream: TcpStream) {
  let buffer_reader = BufReader::new(&stream);
  let req: Vec<_> = buffer_reader
    .lines()
    .map(|r|{r.unwrap()}) // extract correct utf8 char line?...
    .take_while(|item|{!item.is_empty()}) // filter empty lines
    .collect(); // lazy execute the iterator

  // ':#?' means pretty-print the `Debug` formatting, with line-breaks and indentation
  // see https://doc.rust-lang.org/std/fmt/index.html#sign0
  println!("Request: {:#?}", req);
}