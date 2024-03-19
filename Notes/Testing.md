```
#[cfg(test)]
mod tests {
	use super::*; //includes all the methods outside of mod tests
	#[test]
	fn it_works() {
		let result = 2 + 2;
		assert_eq!(result, 4); //assert_ne! is not equals
	}
	#[test]
	//random method
	fn call_simple_add() {
		assert!(simple_add())
	}
}

fn simple_add() -> bool {
	//method
}
```
`#[ignore]` will ignore functions
`#[should_panic]` will run okay if the function is suppose to [[Panic]]

## Terminal Commands
`cargo test` to test the library 
`cargo test -- --test-threads=1` test custom amount of threads
`cargo test function_name` will test specific function 
