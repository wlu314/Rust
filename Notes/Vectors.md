dynamically resizable [[array]] on the heap. Must be [[Variables and Mutability]] in order to add nums to it.

**vec!**
~~~
let mut nums = vec![1,2,3];
nums.push(4);  //adds to the end
nums.pop(); //removes least value
~~~

**Vec::new()**
~~~
let mut vector = Vec::new(); //same as vec!
vec.push("test"); //as 1st element is added, the vec is declared type string
vec.push("String);
println!("{:?}",vec); //prints ["test","String"]
~~~
- [[Control Flow]] if you want to loop over a vector named vec, make sure to reference it using ==&==vec. 

**Iterator** 
~~~
let v: Vec<i32> = (0..5).collect();
//prints out values 0,1,2,3,4
~~~


**Methods**
~~~
`.last()` returns the last value of the vector 
`.first()` returns the first value of the vector
`.len()` checks the length
`.is_empty()` return bool
`.insert(index, element)` allows to insert at a specific index
`.remove(index` removes element at index 
`.sort()` sorts the vector 
`.reverse()` reverse the order of vector
~~~

**Sizing** 
vector.capacity()
~~~
let mut vec = Vec::<i32>::with_capacity(2);
println!("{}", vec.capacity()); // returns 2
~~~
The vector updates automatically my adding old vector in new vector when a greater capacity is needed.