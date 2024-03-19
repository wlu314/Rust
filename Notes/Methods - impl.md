~~~
struct Square {
	width: u32,
	height: u32,
}
impl Square {
	fn (&self) -> u32 {
		self.width * self.height
	}
}

fn main() {
	let sq = Square {width: 5, height: 5};
	let area = sq.area();
}
~~~
- the implement must have the same name as the structure
- all methods must start with &self 
- to access element inside of the struct, must use ==self.attribute==