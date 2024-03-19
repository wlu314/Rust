~~~
enum Pet {
	dog, cat, fish
}
impl Pet {
	fn what_am_I(&self) -> &'static str {
		match self {
			Pet::dog => "I am a dog",
			Pet::cat => "I am a fish"
			Pet::fish => "I am a fish",
		}
	}
}
fn main() {
	let dog = Pet::dog;
	println!("{}", dog.what_am_I());
}
~~~
- methods can be implemented on enums
- enums can be used as a type/param

~~~
enum IpAddrKind {
	V4(String),
}

fn main() {
	let home = IpAddrKind::V4(String::from("127.0.0.1"));
}
~~~

## Options 
- Rust doesn't use null
- Used to handle an absence of a vlaue
- None => no value and Some(value) means some value exists
- used to avoid null pointer errors that are common in other languages
~~~
let some_number = Some(5);
let some_string = Some("A string");
let nothing: Option<i8> = None;

//Options, T is a generic type
enum Option<T> {
	None,
	Some(T),
}
~~~

~~~
let x: i32 = 5;
let y: Option<i32> = Some(5);
// this means that y can either be some value of i32 or None
~~~

## Match
- match is used to "match" a parameter to a specified output
*Example*
~~~
let five = Some(5);
let six = plus_one(five);
let none = plus_one(None);
println!("{:?}", six); //Returns Some(6)

fn plus_one(x: Option<i32>) -> Option<i32> {
	match x {
		None => None,
		Some(i) => Some(i + 1),
	}
}
~~~

*Another Example*
~~~
fn what_pet(input: &str) {
	match input { 
		"Dog" => println!("This is a dog"),
		_ => println!("This can be anything"),
	}
}
~~~
- the underscore handle anything that isn't caught earlier. Kind of like a default in a try and catch statement in Java.

## Let Statements
~~~
let dog2 = Some(Pet::dog);
if let Some(Pet::dog) = dog2 {
	println!("This animal is indeed a dog!");
} else {
	println!("Not a dog");
}


// it says while there is some number being popped off, then print that number
let mut stack = Vec::new();
stack.push(1);
stack.push(2);
stack.push(3);
while let Some(top) = stack.pop() {
	println!("{}", top)
}
~~~