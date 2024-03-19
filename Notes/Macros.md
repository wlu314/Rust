**What is the difference between a macro and function?**
Macros are a way to write code that writes other code. This is also called meta-programming. The `derive` attribute which generates an implementation of various traits. There are macros such as `vec!` and `println!` that are commonly used. Meta-programming reduces the code you have to write and maintain. Macros have functionalities such that functions don't. 

A function signature must declare the number and type of parameters the function has. Macros can take a variable number of parameters. Macros are expanded before compile time. Macro can implement [[Traits]] on a given type, while a [[Function]] cannot. This is because it gets called a t runtime and a trait must be implemented at compile time.
**To define custom function macro:**
```
marco_rules! macro_name {
	($param1: expr, $param2: expr) => {
		//logic
		//returns omething
	};
}
```
- to call this macro use `macro_name!` 


## Procedural Macros
- used to generate code at compile time
- acts more like a function, it accepts some code as input, operates on that code and produces some code as an output rather than matching patterns and then replacing the cdoe with other code.
- 3 types: attribute macros (add metadata to items), function macro (use to define functions that take in function that generates a block of code included in program), derive macro (automatically implement traits on struct or enums)

**Defining a procedural macro**
```
use proc_macro;

#[some_attribute]
pub fn some_name(input: TokenStream) -> TokenStream {

}
```
*This function takes in `TokenStream` as an input and produces a `TokenStream` as an output. *

[[Writing a Custom Derive Macro]]

