// // "import * as lifetimes from './lifetimes'" equivalent in rust
// // ...no it's not 'import', just a declaration(and should be declared somewhere once?)
// // see https://www.reddit.com/r/learnrust/comments/ms4nz2/rust_module_importing/
mod lessons;

fn main() {
    lessons::hello::hello();

    lessons::lifetimes::test_lifetimes();

    lessons::closure::create_node_tree();

    lessons::smart_pointers::box_value_in_heap();
    lessons::smart_pointers::drop_automatically_when_leave();
    lessons::smart_pointers::drop_manually();
    lessons::smart_pointers::multiple_references_over_one_value();

    lessons::concurrency::thread_do_not_guaranty_exec_order();
    lessons::concurrency::wait_for_thread_done();
    lessons::concurrency::lock_threads_using_mutex_or_not();

    // http_server::serve_and_shut_down();

    lessons::try_enum::output_random_enum_structured_enum_value();

    lessons::from_and_into::convert_address_to_location();

    lessons::iterator::filter_numeric_value_in_vec();

    lessons::generic::call_generic_function_for_num();

    lessons::structure_with_ref_members::a_person_with_a_cat();
}
