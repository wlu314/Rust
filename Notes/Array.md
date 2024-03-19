# Array
Hold the same data type and accessing brackets
~~~
let array = [1,2,3];
let i = array[0]; i = 1;
~~~

Another Initialization Technique
~~~
let array2: [i32; 3] = [4,5,6]; //unsigned 32 bit with 3 elements
~~~

Must use [[Variables and Mutability]] in order to make the array mutable.
~~~
let array2: [i32; 3] = [4,5,6]; //unsigned 32 bit with 3 elements
println!("{}", array2[1]);

array2[1] = 10;
println!('{}', array2[1]); //error because it's not mutable
~~~

~~~
let mut array2: [i32; 3] = [4,5,6]; //unsigned 32 bit with 3 elements
println!("{}", array2[1]);

array2[1] = 10;
println!('{}', array2[1]); //this will work
~~~