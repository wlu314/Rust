```
pub trait Iterator {
	type Item;
	fn next(&mut self) -> Option<Self::Item>;
	//default methods
}
```
- implement your own iterator
***Example***
```
impl Iterator for Range {
	type Item = u32;
	fn next(&mut self) -> Option<Self::Item> {
		if self.start >= self.end {
			return None;
		}
		let result = Some(self.start);
		self.start += 1;
		result
		}
}
fn main() {
	let mut range = Range {start: 0, end: 10};
	for r in range{
		println!("{}", r); //prints out from 0 - 9
	}
}
```

```
let vec_num = vec![1,2,3];
	for val in vec_num.iter() {
	println!("{}", val);
}
```
- any type that implements the iterator trait
- Iterable is any type that implements into iterator

