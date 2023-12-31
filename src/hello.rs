use ferris_says::say;
use std::io::{stdout, BufWriter};

pub fn hello() {
    println!("Hello, world!");

    let stdout = stdout();
    let message = String::from("Hello fellow citizens!");
    let width = message.chars().count() + 10;

    // mut: mutable variable, pointer or sth like that?...
    let mut writer = BufWriter::new(stdout.lock());
    say(message.as_bytes(), width + 20, &mut writer).unwrap();
}
