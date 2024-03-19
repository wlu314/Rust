Types who values can be duplicated simply by copying bits
~~~
pub trait Copy: Clone {...}
~~~

## Implementing Copy
**Deriving it**
~~~
#[derive(Copy, Clone)]
struct MyStruct;
~~~
**Manually**
~~~
struct MyStruct;
impl Copy for MyStruct {}
impl Clone for MyStruct {
	fn clone (&self) -> MyStruct {
		*self
	}
}
~~~
- making a type Copy place restriction on the type
- [[Copy]] is not overloadable, it's a simpler bit-wise copy
- [[Clone]] is an explicit action, using ```x.clone()```
- ```Clone``` is a super trait of a ```Copy``` thus everything which is Copy must implement a Clone

### When is a type a Copy 
A type can implement copy if all it's components implement copy.

```
struct PointList {
    points: Vec<Point>,
}
```
This cannot impl Copy because Vec<T> is not Copy