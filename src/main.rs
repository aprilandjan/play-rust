mod hello;
// "import * as lifetimes from './lifetimes'" equivalent in rust
mod lifetimes;
mod closure;
mod smart_pointers;


fn main() {
    hello::hello();

    lifetimes::test_lifetimes();

    closure::create_node_tree();

    smart_pointers::box_value_in_heap();
    smart_pointers::drop_automatically_when_leave();
    smart_pointers::drop_manually();
    smart_pointers::multiple_references_over_one_value();
}