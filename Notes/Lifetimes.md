- helps dangling reference
- ' is the keyword
- guarantees memory safety such that a reference always points to a valid memory 
~~~
// &i32 //normal reference to 32-bit integer
// &'a i32 //lifetime reference
// &/a mut 32 //mutatble lifetime reference
~~~

Lifetime in [[Rust/Notes/Structs|Structs]]
~~~
struct myString<'a> {
	text: &'a str,
}
fn main() {
	let str = String::from("This is my String");
	let x = myString(text: str.as_str());
}
~~~

**Static Lifetime**
~~~
let s: &'static string = "This is a static lifetime";
~~~
- this will live on forever