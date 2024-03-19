[[From]] and [[Into]] traits are inherently linked

### From
For example we can easily convert a `str` into a `String`
~~~
let my_str = "hello";
let my_string = String::from(my_str);
~~~

**Defining a conversion for your own type**
~~~
use std::convert::From;

#[derive(Debug)]
struct Number {
	value: i32,
}

impl From<i32> for Number {
	fn from(item: i32) -> Self {
		Number { value: item }
	}
}

fn main() {
	let num = Number::from(30);
	println!("My number is {:?}", num);
}
~~~

### Into 
The into trait is a reciprocal of the From trait. Using the Into trait requires specification of the type to convert into as the compiler is not able to determine it most of the time

~~~
use std::convert::Into;

#[derive(Debug)]
struct Number {
	value: i32,
}

impl Into<Number> for i32 {
	fn into(self) -> Number {
		Number { value: self }
	}
}

fn main() {
	let int = 5; 
	let num: Number = int.into();
	println!("My number is {:?}", num);
}
~~~

Because some conversion are more fallible use [[TryFrom and TryInto]]

