Shared memory concurrency is like multiple ownership. Multiple threads can access the same memory location at the same time. `mutex` short for mutual exclusion allows for only one thread to gain access to some data at any given time. To gain access data in a mutex, a thread must signal using a mutex's lock. Lock is a data structure that keeps track of who has access to the data.
```
use std::sync::Mutex;

fn main() {
	let m = Mutex::new(5);
	{
		let mut num = m.lock().unwrap();
		*num = 6;
	}
}
```
- must call `lock` method on `mutex` 
- `Mutex<T>` is a smart [[Pointer|Pointer]]
- multiple ownership with multiple threads by using `Rc<T>` to create a reference counted
```
use std::sync::Mutex;
use std::thread;

fn main() {
	let counter = Mutex::new(0);
	let mut handles = vec![];
	for _ in 0..10 {
		let handle = thread::spawn(move || {
			let mut num = counter.lock().unwrap();
			*num += 1;
	});
	handles.push(handle);
	}
	for handle in handles {
		handle.join().unwrap();
	}
	println!("Result: {}", *counter.lock().unwrap());
}
```
- this code will not run as the ownership of counter has been transferred. After the thread, it is dropped
- we can fix this using a smart [[Pointer]] `Rc<T>` to create a reference counted value
- `join` method is used in [[thread]] to ensure all spawn threads finish executing before continues
``
```
fn main() {
	let counter = Rc::new(Mutex::new(0));
	let mut handles = vec![];
	
	for _ in 0..10 {
		let counter = Rc::clone(&counter);
```
- however a new issue occurs because `Rc<T>` is not safe to share across threads, we can use Atomic Reference Counting `Arc<T>` 
- [[Arc]] is used in concurrent situations

```
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
```

## [[Mutex Poisoning]]
When there is an error within the `thread::spawn` the data inside is lost and the mutex could be poisoned. 