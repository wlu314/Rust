Concurrent programming is where different parts of a program executes independently and parallel programming occurs where different parts of a program executes at the same time. Fearless concurrency allows you to write code that is free of subtle bugs and [[Refactoring]] is easier. High Level languages promise benefits from giving up control to gain abstraction. Low level languages are expected to provide solution with best performance in any given situation and have fewer abstractions over the hardware.

# [[Threads]]
Threads are used to run code simultaneously. In operating systems, an executed program's code is run in a process and the operating system manages multiple processes at once. In a program, you can have independent parts that run simultaneously. The feature that allows are called threads. Web server could have multiple threads that respond to more than one request at the same time. Splitting computation into multiple threads for multiple task can improve performance but adds complexity. You don't know which will run first that can lead to issues. 
- Race conditions -> accessing data in an inconsistent order 
- Deadlocks -> two threads waiting for each other preventing it to continue 
- Bug that are hard to reproduce and fix

Rust uses a 1:1 model of thread implementation where a program uses one OS thread per one language thread.

```
use std::thread;
use std::time::Duration;

fn main() {
    thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
}
```
- when main thread of a Rust program completes, all spawn threads are shut down
```
hi number 1 from the main thread!
hi number 1 from the spawned thread!
hi number 2 from the main thread!
hi number 2 from the spawned thread!
hi number 3 from the main thread!
hi number 3 from the spawned thread!
hi number 4 from the main thread!
hi number 4 from the spawned thread!
hi number 5 from the spawned thread!
```
The main thread is printed first. When though the logic said it will print until 9, it stopped after the last main thread had finished

# Join 
To fix spawn code that stopped prematurely due to main thread ending, you can save the return value of `thread::spawn` in a variable. The return type is `JoinHandle`. 
```
use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
    handle.join().unwrap();
}
```
Output should look similar
```
hi number 1 from the main thread!
hi number 2 from the main thread!
hi number 1 from the spawned thread!
hi number 3 from the main thread!
hi number 2 from the spawned thread!
hi number 4 from the main thread!
hi number 3 from the spawned thread!
hi number 4 from the spawned thread!
hi number 5 from the spawned thread!
hi number 6 from the spawned thread!
hi number 7 from the spawned thread!
hi number 8 from the spawned thread!
hi number 9 from the spawned thread!
```
- The two thread are alternating but the main thread waits because of the call to handle.join() and doesn't end until the spawn thread finishes

**Placement**
- if join is placed in between the for loops the output will allow the spread thread to execute first
```
use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    handle.join().unwrap();

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
}

output: 
hi number 1 from the spawned thread!
hi number 2 from the spawned thread!
hi number 3 from the spawned thread!
hi number 4 from the spawned thread!
hi number 5 from the spawned thread!
hi number 6 from the spawned thread!
hi number 7 from the spawned thread!
hi number 8 from the spawned thread!
hi number 9 from the spawned thread!
hi number 1 from the main thread!
hi number 2 from the main thread!
hi number 3 from the main thread!
hi number 4 from the main thread!
```

## Move [[Closures]] with Thread
We use move keyword with closures passed to `thread::spawn` because closures take ownership of the values it uses from the environment thus transferring [[Rust/Notes/Ownership|Ownership]] of those values from one thread to another.

```
use std::thread;

fn main() {
	let v = vec![1, 2, 3];
	let handle = thread::spawn(move || {
		println!("Vector: {:?}", v);
	});
	handle.join().unwrap();
}
```

the move keyword forces the `thread` to take ownership. If a thread takes ownership within the scope, you cannot call that variables outside of the thread. 

## [[Channels]]
A channels transfers data from one thread to another. There is a transmitter and receiver. A channel is closed if either transmitter or receiver is dropped.

```
use std::sync::mpsc;

fn main() {
	let (tx, rx) = mpsc::channel();
}
```
- `mpsc` stands for multiple producer, single consumer. Rust standard library implements channels means a channel can send multiple sending ends that produce values but only one receiving that takes the values
- transmitter has a `send()` method that takes the value we want to send
- `send()` method returns a `Result<T ,E>` type, if a receiver has been dropped and there's nowhere to send the value the send will return an error
- **receiver**  has two methods `recv` and `try_recv`
	- recv: block the main thread's execution and wait for a value to sent down the channel, once sent it will return a `Return<T, E>`
	- try_recv: doesn't block and will return `Result<T, E>` immediately

### Ownership
- once the value has been sent to another thread, the receiving thread can modify or drop it before using value

## Multiple Producers By Cloning Transmitter 
- by using the `clone()` method, it will give us a new transmitter we can pass to the first spawned thread

## [[Shared-State (Mutex)]] Concurrency
Shared memory concurrency is like multiple ownership. Multiple threads can access the same memory location at the same time. `mutex` short for mutual exclusion allows for only one thread to gain access to some data at any given time. To gain access data in a mutex, a thread must signal using a mutex's lock. Lock is a data structure that keeps track of who has access to the data.