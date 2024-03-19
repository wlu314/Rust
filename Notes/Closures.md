Closures are function like construct to store in a variable. 
- anonymous functions you can save in a variable or pass as argument to other functions 
- captures values from the scope they are defined in
- doesn't need to specify parameter or annotations. However if you don't annotate the type, you must use call it with the parameter of the type you want

*Before*
```
fn sort_population(city_vec: &mut Vec<City>) {
	city_vec.sort_by_key(population_helper)
}

fn population_helper(pop: &City) -> u64 {
	pop.population
}
```

*After*
```
fn sort_pop_closure(cities: &mut Vec<City>) {
	cities.sort_by_key(|city| city.population);
}
```

- `| |` are anonymous function that allows you to define an inline function without a name
- they are short, one-off function passed as arguments

### **Syntax**
`|args1, args2, ...| expression`

```
let x = 4; 
let equal_to_x = |z| z == x; 
println!("{}", equal_to_x(4)); // prints "true"
```

***No Annotations***
```
let add = |x i32| -> i32 {x+1};
let add2 = |x| x+1; 
add_v2(1); //must call the function to infer the type
```

### Copying Closures
```
let y = 5;
let add_y = |x| x+y;
let copy = add_y;
println!("{}", add_y(copy(10))); //20
```
This is because copy is first called. 10 is added onto 5. Then 15 is passed as a parameter into add_y(). This adds 15 on to 5 = 20. [[Variables and Mutability]] values don't implement copy or clone
# Fn, FnMut, FnOnce
Fn 
- closures that are called multiple times, borrows values immutably
- `|args| v.contains(args)` can check many times
FnMut
- closures that can be called multiple times, if the closure is declare mutable and borrows values mutably
- `|args| v.push(args)` v must be mut to push
FnOnce
- closures that can only be called once
- `| | drops(v)` can only drop once

