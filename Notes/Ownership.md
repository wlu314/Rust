Rust uses its ownership module. It doesn't have a garbage collector necessarily. 

**Stack**
- Last in, First Out
- All data must have a known fixed sized. Local Variables are stored, then popped when not used.
- One owner at a time, free when out of scope

**Heap**
- Data without a known size or may change. 
- Allocating to the heap is slower
- Search is slower. 

## Move
~~~
// This is invalid 
let x = vec!["tyler".to_string()];
let y = x;
println!("{:?}", x); // y has taken ownership, you can no longer call the x variable
~~~

~~~
fn main {
	let s = String::from("takes"); // create a variable with a string
	takes_ownership(s); // gives ownership to the function
}

fn takes_ownership(s: String) {
	let strin = s;
	println!("{}", strin);
}
~~~
- i32 has the copy trait
- a function can give ownership by returning a value from the stack (i.e. String)
- a function outside the scope can give and take ownership
- 
## Clone
- very expensive operation
~~~
let x = vec!["tyler".to_string()];
let y = x.clone();
let z = y.clone();
println!("{:?}", x); // copies correctly
println!("{:?}", y); // copies correctly
println!("{:?}", z); // copies correctly
~~~

## Copy
- implemented on type on the stack: boolean, Char, Tuple (only if every element implements copy), integer
- When you pass a variable that doesn't implement copy trait, into a function (not by reference) the variable is taken out of scope.

## Referencing 
- References allows us to make a reference without taking ownership. Borrowing the value. To make a reference use the ==&== character
- Shared reference lets you read not modify. Can have as many as possible
- Mutable reference allows you to read and write. Unlike shared, you cannot have any other reference active. 
~~~
main()
let mut s String::from("Hello");
change_String(&mut s);
println!("{}",s);

fn change_string (some_string: &String) {
	some_string.push_str(", world!");
}
~~~