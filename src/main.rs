mod hello;
// "import * as lifetimes from './lifetimes'" equivalent in rust
mod lifetimes;
mod closure;


fn main() {
    hello::hello();

    lifetimes::test_lifetimes();

    closure::create_node_tree();
}