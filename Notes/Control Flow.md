**If Statements**
~~~
if one > 0 {
	println!("True");
} else {
	println!("False");
}
~~~

**Loop Statements** 
~~~
loop {
	println!("Loop"); //runs indefinitely 
}

//counters in loops
'counter: loop {
	//statements
}
~~~
- nested loops can be used
- break if used to break out of infinite loop

**While Loops**
~~~
let mut num = 0;
	while num < 5 {
	println!("Num: {}", num);
	num +=1;
}
~~~
- works like java while loop

**For Loops**
~~~
let vec: Vec<i8> = (0..8).collect(); //collect turns anything iterable and into a collection. 

let vec: Vec<i8> = (0..8).collect();
for element in vec {
	println!("{}", element); //print numbers 0 - 8
}
~~~