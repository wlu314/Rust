~~~
struct Point<T, U> {
	
	x: T,
	y: U,

}

fn main() {

	let coord = Point {x: 5.0, y: 5.0};
	let coord2 = Point {x: "x", y: 5.0};

}
~~~
- Generics are placeholder until you fill in a concrete type
- They are power in the sense it makes the code more modular and handles different modular types
