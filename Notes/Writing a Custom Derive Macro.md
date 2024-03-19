I will create a crate `hello_macro` that defines a trait name `HelloMacro` with the function named `hello_macro`. We will provide a procedural [[Macros]] so the user can annotate their type with `#[derive(HelloMacro)]` to get the default implementation of the `hello_macro` function. This will print out a general statement.

*main.rs*
```
use hello_macro::HelloMacro;

struct Pancakes;

impl HelloMacro for Pancakes {
    fn hello_macro() {
        println!("Hello, Macro! My name is Pancakes!");
    }
}

fn main() {
    Pancakes::hello_macro();
}
```
*this will print a statement*

`cargo new hello_macro --lib` First make a new lib crate

*lib.rs*
```
pub trait HelloMacro {
    fn hello_macro();
}
```
https://doc.rust-lang.org/book/ch19-06-macros.html