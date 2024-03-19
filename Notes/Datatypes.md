**Integer**
- can be 8, 16, 32, 64, 128 bit.
~~~
let x: i8 = 10; //i8 stand for 8 bit
~~~

- can be signed or unsigned
~~~
let y: u8 = 10; //creates unsigned 8 bit int
~~~

- integer literal
- underscores are just visual separators to enhance readability
~~~
let decimal = 02_55; //255
let hex = 0xff; //255
let octal = 0o377; //255
let binary = 0b1111_1111; //255
~~~

- ASCII value
~~~
let byte = b'A'; //value 65
~~~

**Float**
- f64 is default on modern CPU
- to change you would set this in initialization
~~~
let x = 2.0;
let y: f32 = 1.0;
~~~

**Boolean**
- the following example is both the same initialization this is because rust compiler is smart to under the first variable is a bool.
~~~
let t = true;
let f: bool = false;
~~~

**Character**
- straight forward
~~~
let c = 'c';
~~~

