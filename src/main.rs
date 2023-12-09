mod hello;
// "import * as lifetimes from './lifetimes'" equivalent in rust
// ...no it's not 'import', just a declaration(and should be declared somewhere once?)
// see https://www.reddit.com/r/learnrust/comments/ms4nz2/rust_module_importing/
mod lifetimes;
mod closure;
mod smart_pointers;
mod concurrency;
mod http_server;
mod thread_pool;

fn main() {
    hello::hello();

    lifetimes::test_lifetimes();

    closure::create_node_tree();

    smart_pointers::box_value_in_heap();
    smart_pointers::drop_automatically_when_leave();
    smart_pointers::drop_manually();
    smart_pointers::multiple_references_over_one_value();

    concurrency::thread_do_not_guaranty_exec_order();
    concurrency::wait_for_thread_done();
    concurrency::lock_threads_using_mutex_or_not();

    http_server::serve();
}