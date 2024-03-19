**[[Panic]]**
- allows a program to terminate immediately and provide feedback to the program
- use `panic!()` to specify a string payload that is built using formatting syntax
- used to generate a panic and start unwinding its stack, the runtime will take care of freeing resources be calling the destructor of the objects

**[[Result]]**
- `panic!` and `Result` are similar in such that they are used to handle errors
- `panic!` is used to construct errors that represents a bug that has been detected in my program, it provides you with a message 
- `Result` is to wrap other type that represent successful results using `Ok(T)` or error types that is `Err(E)`

**`?` Operator**
- concise way of error propagation
- decrease match statements
- cannot use in main method, use expect in main thread
**Before Operator**
```
fn read_file_contents(path: &str) -> Result<String, std::io::Error> {
    let result = std::fs::read_to_string(path);
    match result {
        Ok(contents) => Ok(contents),
        Err(e) => Err(e),
    }
}
```

**After Operator**
```
fn read_file_contents(path: &str) -> Result<String, std::io::Error> {
    let contents = std::fs::read_to_string(path)?;
    Ok(contents)
}
```

this handles success and error cases. 