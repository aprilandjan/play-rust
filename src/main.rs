use ferris_says::say;
use std::io::{stdout, BufWriter};

// "import * as lifetimes from './lifetimes'" equivalent in rust
mod lifetimes;
mod closure;
mod smart_pointers;

fn main() {
    println!("Hello, world!");

    let stdout = stdout();
    let message = String::from("Hello fellow citizens!");
    let width = message.chars().count() + 10;

    println!("width: {}", width);

    // mut: mutable variable, pointer or sth like that?...
    let mut writer = BufWriter::new(stdout.lock());
    say(message.as_bytes(), width + 20, &mut writer).unwrap();

    lifetimes::test_lifetimes();

    closure::create_node_tree();

    smart_pointers::box_value_in_heap();
    smart_pointers::drop_automatically_when_leave();
    smart_pointers::drop_manually();
}