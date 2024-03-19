Structure let you name and package related value into a single value so you can deal with it in a single unit. 


**Name Field Struct**
~~~
struct User {
	active: bool,
	username: String,
	sign_in_count: u32,
}

fn main() {
	let user1 = User{active: true, username: String::from("Tyler"), sign_in_count: 0};
	println!("{}", user1.username); //prints out Tyler
	let user2 = build_user(String::from("Wesley"));
	println!("{}", user2.username); //Wesley
}

fn build_user(username: String) -> User {

	User {
		username, //user name doesnt need to initial to itself
		active: true,
		sign_in_count: 1,
	}
}
~~~

**Vector Struct**
~~~
struct Coordinates(i32, i32, i32);
~~~
- recognized by the order they are placed in

**Unit - Like Struct**
~~~
struct UnitStruct;
~~~

## default()
~~~
struct Car {
	mpg: u32,
	color: String,
	top_speed: u32,
}

impl Car {

fn default() -> Self {
	Car {
		mpg: 0,
		color: String::new(),
		top_speed: 0,
	}
}

	fn set_mpg(&mut self, new_mpg: u32) {
		self.mpg = new_mpg
	}
	
	fn set_color(&mut self, new_color: String) {
		self.color = new_color
	}

	fn set_top_speed(&mut self, new_top_speed: u32) {
		self.top_speed = new_top_speed
	}

  

fn main() {
	let mut ferrari = Car::default(); //default
	ferrari.set_mpg(300);
	ferrari.set_color(String::from("Red"));
	ferrari.set_top_speed(150);
	println!("{}", ferrari.mpg);
	println!("{}", ferrari.color);
	println!("{}", ferrari.top_speed);
}
~~~
## [[Methods - impl]]
- the implement must have the same name as the structure
- all methods must start with &self 
- to access element inside of the struct, must use ==self.attribute==

## [[Variables and Mutability]] - Mutable Structs
- add the ==mut== keyword when initializing a *struct* 
- if it's not mutable, then it cannot be set using a setter in [[Methods - impl]]
~~~
impl Square {
	fn set_width(&mut self, new_width: u32) {
		self.width = new_width
	}
}
fn main() {
	let mut sq = Square {width: 5, height: 5};
	sq.set_width(6); //sets the sq instance to a wdith of 6 instead of 5 
}
~~~