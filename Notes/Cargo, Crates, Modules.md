Packages: A Cargo feature that lets you build, test and share crates.
	Bundle of one or more crates that provides a set of functionalities. A package contains cargo.toml files that are command line tool that are used to build the code.
Crates: A tree of modules that produces a library or executable
	Crates can come in binary or a library.
Module: Allows you to control organization, scope and privacy of paths
Path: way of naming an item such as a struct, function, or module.


## Creating a package
We enter the command cargo new:
~~~
$ cargo new my-project
	Created binary (applcaition) 'my-project' package
$ ls my-project 
Cargo.toml //declare dependencies
src 
# ls my-project/src
main.rs

~~~


## Privacy and Control Scope
Declaring Modules: In the crate root file, you can declare new modules by using the command `mod module-name` 

Declaring submodules: In any file other than the crate root, you can declare submodules. 

**Public vs private**
Code within a module is private from its parent module by default. To make module public, declare it using `pub mod` instead of `mod`

**Use Keyword**
`use` creates shortcut to items that reduce repetition of long paths. 
~~~
crate::garden::vegetables::Asparagus,
//instead you can do
use crate::garden::vegetables::Asparagus; 
~~~
you will only need to write `Asparagus` to make use of that type in the scope.

***Before***: 
![[Use Keyword- Before.png]]
***After***
![[Use Keyword - After.png]]
# Module
to include a module within another, use the inline command `pub mod mod-name`. This tells the compiler to include the code in your current file. To declare a new library use the terminal command `cargo new ex_name --lib`. This creates a `lib.rs` file.

## Cargo Modules
~~~
cargo install cargo-modules //install different modules
cargo-modules structure
~~~
