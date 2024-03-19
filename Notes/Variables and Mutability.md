For a variable to be mutable (meaning reassignment of a variable) use the `mut` key word. Mutable don't implement [[Clone]] or [[Copy]]
~~~
let x = 5;
x = 6; //this will run into an error 

let mut x = 5;
x = 6; //this will change x to 6
~~~

# Constants
constants is a [[Datatypes]] that can never be mutable regardless of using `mut`
~~~
const SECOND:i8 = 60;
~~~