- Operators can be overloaded via traits
- Some operators can be used to do different tasks based on the inputted arguments
- Example: The + operator calls the add method. The add method is part of the Add Trait

~~~
use std::ops;

struct Foo;
struct Bar;

#[derive(Debug)]
strcut FooBar;

#[derive(Debug)]
strcut BarFoo;

// The "std::ops::Add" trait is used to specify the fucntionality of +
// Implements Foo + Bar = FooBar (Foo.add(Bar)) 
impl ops::Add<Bar> for Foo {
	type Output = FooBar;

	fn add(self, _rhs: Bar) -> FooBar {
		println!(" Foo.add(Bar) is called using + operator");
		FooBar 
	}

}

impl ops::Add<Foo> for Bar {
    type Output = BarFoo;

    fn add(self, _rhs: Foo) -> BarFoo {
        println!("> Bar.add(Foo) was called");

        BarFoo
    }
}

~~~
