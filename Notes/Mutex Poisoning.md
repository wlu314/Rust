```
let mut guard = match lock.lock() {
	Ok(guard) => guard,
	Err(poisoned) => poisoned.into_inner(),
};
```
- use a match statement to retrieve the data 