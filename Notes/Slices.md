Region of an array or vector of any length

**Initialization** 
~~~
let v: Vec<i32> = (0..5).collect();
println!("{:?}", v); //prints [0,1,2,3,4]

let slices: &[i32] = &v;
println!("{:?}", slices); //prints [0,1,2,3,4]

let slices: &[i32] = &v[2..4];
println!("{:?}", slices); //prints [2,3]
~~~

An ordinary reference (&) points to a single value. A slice points to a range on consecutive value. Both are non owning. 