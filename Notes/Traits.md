## Traits
- trait keyword
- kind of like an abstract class in java
- if the trait is not implemented for a struct, the original trait's method will be used instead. Kind of like overrides in java

*Trait initialization*
~~~
trait Overview {
	fn overview(&self) -> String {
		String::from("This is a rust course.")
	}
}

struct Course {
	headline: String,
	author: String,
}

impl Overview for Course {
	fn overview(&self) -> String {
		format!("{}, {}", self.author, self.headline)
	}
}

fn main() {
	let course1 = Course{headline: String::from("Headline"), author: String::from("Tyler"),};
	println!("{}", course1.overview());
}
~~~

### Trait as Parameter
~~~
// Simple cases
fn call_overview(item: &impl Overview) {
	println!("Overview: {}", item.overview())
}

// Expresses more complexity clearly
// Work extremely well with multiple objects of the same trait
fn call_overview<T: Overview>(item: &T) {
	println!("Overview: {}", item.overview())
}

fn call<T: trait1>(item1: &T, item2: &T) {
	//functionality
}
~~~

### Utility Traits - Drops
- free resources: out of scope, elements out of a vector
~~~
impl Drop for Course {
	fn drop(&mut self) {
		println!("Dropping: {}", self.author );
	}
}

~~~
- this automatically drops the self.author as leaving the main function goes out of scope

## [[Clone]]
a common trait for the ability to explicitly duplicate an object

## [[Copy]]
Types whose values can be duplicated simply by copying bits.

[[Into]]

