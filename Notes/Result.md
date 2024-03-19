- Result is a enum with `Ok(T)` and `Err(T)`
```
enum Result<T, E> {
	Ok(T),
	Err(E),
}
```

*Example*: Catching Errors
```
use std::fs::File;

fn main() {
	let file = File::opn("err.txt");
	let file = mmatch file {
		Ok(file) => file,
		Err(error) => match error.kind() {
			ErrorKind::NotFound => match File::create("err.txt") {
				Ok(file_created) => file_created,
				Err(err) => panic!("Cannot create the file"),
			}
			_ => panic!("Unknown Error"),
		}	
	};
}
```
- method generate a file err.txt because there wasn't one found due to catching methods

`let file = File::open("Error.txt").expect("Error opening the file!");`
- this generates a panic and returns a message that allows us to debug the code