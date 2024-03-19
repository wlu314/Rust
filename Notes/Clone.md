```
pub trait Clone: Sized {
    // Required method
    fn clone(&self) -> Self;

    // Provided method
    fn clone_from(&mut self, source: &Self) { ... }
}
```
- differs from [[Copy]] such that Copy is implicit and inexpensive bit-wise copy. Implicit means that operation happen automatically without having to initiate the copy 
  *example*
~~~
let x = 5;
let y = x; // x is copy to y implicity
~~~
- Clone is always explicit and may/may not be expensive
~~~
Requires explicit action by the .clone() method 
~~~
- cannot reimplement [[Copy]], can reimplement [[Clone]], this makes Cloning more flexible providing custom logic