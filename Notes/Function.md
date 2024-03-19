Create function using fn keyword

*Example Function of GCD*
~~~
fn (mut a: u64, mut b: u64) -> u64 { //u64 is the return type
	while a != 0 {
		if a < b {
			let c = a;
			a = b;
			b = c; 
		}
		a = a % b;
	}
	b //return by not putting a semi-colon
} 

~~~