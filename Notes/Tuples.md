can store different [[Rust/Notes/Datatypes|Datatypes]] within a tuple
~~~
let tup = (500, "hi", true);
~~~

To access variables within a tuple, you would use the following operator. Starts from 0.
~~~
println!("{}", tup.0)

let tup = (500, "hi", true);
let (x,y,z) = tup;
println!("{}", x); //prints 500
println!("{}", y); //prints hi
println!("{}", z); //prints true
~~~
