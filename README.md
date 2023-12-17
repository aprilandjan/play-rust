# play-rust

A beginner's rust-lang exploration :)

## add dependencies

```bash
$ cargo add ferris-says
```

## build & run

```bash
$ cargo build
$ cargo run
```

## format & prettify

```bash
$ cargo fmt
$ cargo fix
```

## Thoughts

1. No need to write parentheses in `if` `for` control flow statement
2. Can still use `return something;` if we want
3. Seems that there's no builtin random generator, had to use crate `rand` to do that([ref](https://stackoverflow.com/questions/19671845/how-can-i-generate-a-random-number-within-a-range-in-rust))
4. Seems that is is very hard to create a recursive closure?([ref](https://stackoverflow.com/questions/16946888/is-it-possible-to-make-a-recursive-closure-in-rust))
5. The naming conver of a `.rs` file in rust project is unclear, `kebab-case` or `snake_case`?([ref](https://stackoverflow.com/questions/74103439/how-to-use-rust-files-with-kebab-case/74103745#74103745))
6. Need to **declare** a module first before using, maybe something like `#ifndef & #define` in c++([ref](https://www.reddit.com/r/learnrust/comments/ms4nz2/rust_module_importing/)):

    ```rs
    /// define a module in some file
    mod utils;
    /// use it in some other file
    use utils;
    use crate::utils;   // from 'project root directory' to find the module
    use super::utils;   // from 'parent directory' to find the module
    ```

7. When I want to use a **un-exported** or **indirect** type from a dependency, for example, a return type from a `new`, it is painful. There's no such thing like typescript `ReturnType<fn>`, had to find where the exact typings are from, and `cargo add` that exact version of dep([ref](https://stackoverflow.com/questions/65897257/how-to-use-a-type-from-dependences-dependence)).

## References

- <https://www.rust-lang.org/learn/get-started>
