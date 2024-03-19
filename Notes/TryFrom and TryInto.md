Unlike [[From]] and [[Into]], the TryFrom and TryInto traits are used for fallible conversion and return ==Return== 

~~~
use std::convert::TryFrom;
use std::convert::TryInto;

#derive(Debug, PartialEq)]
struct EvenNumber(i32);

impl TryFrom<i32> for EvenNumber {
	type Error = ();

	fn try_from(value: i32) -> Result<Self, Self::Error> {
		if value % 2 == 0 {
			Ok(EvenNumber(value))
		} else {
			Err(())
		}
	}
}

fn main() {
	//TryFrom
	assert_eq!(EvenNumber::try_from(8), O))

}


~~~

*Explanation*
`#[derive(...)]` is an attribute that implements certain traits for a [[Rust/Notes/Structs|Structs]] or [[Enums]] in Rust. Traits are a way to define functionality.

`Debug` is a trait that allows the struct or enum to be formatted using the `{:?}` formatter within the `println!`. Enables you to print the values of instances of your types in a way that is suitable for debugging. Usually printing a struct or enum will result in compile-time error

`PartialEq` allows for instances of the struct and enum to be compared for equality using `==` and `!=`