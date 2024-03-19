Stored as a vector of strings. Strings are always guarantee UTF sequence. String is allocated on the heap, not null terminated. 

**Initialization**
~~~
let name = String::from("Tyler");
let course = "Rust".to_string();
~~~

**String Literals**
- use when you don't want UTF-8
~~~
let rust = "\x52\x75\x73\x54"; //this is Rust
~~~

**Modification**
old_string.replace("old string", "new string")
~~~
let new_name = name.replace("Tyler", "ty")
~~~

**String [[Slices]]**  
&str
contains the address and data of the length. Cannot modify string slice
~~~
let str1 = "hello"; //&str
let str2 = str1.to_string(); //this turns str1 into a string
let str3 = &str2;
~~~

**Methods**
Equal ==
Not equal !=

