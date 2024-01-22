// in rust, `.rs` filename will be treat as an mod
// and thus the filename will be an identifier
// which `0_hello.rs` will trigger `invalid identifier` error
// here we use `#[path]` attribute to bypass that
// https://stackoverflow.com/questions/69210771/why-cant-an-identifier-start-with-a-number

#[path ="./0_hello.rs"]
pub mod hello;
// instead, we can use below to save an export level depth
// pub use hello::*;

#[path ="./1_lifetimes.rs"]
pub mod lifetimes;

#[path ="./2_closure.rs"]
pub mod closure;

#[path ="./3_smart_pointers.rs"]
pub mod smart_pointers;

#[path ="./4_concurrency.rs"]
pub mod concurrency;

#[path ="./5_http_server.rs"]
pub mod http_server;

#[path ="./6_try_enum.rs"]
pub mod try_enum;

#[path ="./7_from_and_into.rs"]
pub mod from_and_into;

#[path ="./8_iterator.rs"]
pub mod iterator;

#[path ="./9_generic.rs"]
pub mod generic;
