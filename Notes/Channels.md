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