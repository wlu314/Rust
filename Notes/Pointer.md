Pointers are variables that contain an address in memory. The address points to some other data. In Rust, a reference is the easiest and most common pointer. Smart pointers are data structure that act like a pointer, but have additional metadata and capabilities. Smart pointer can own the data they point at in many cases. A `Vec<T>` and `String` are both examples of smart pointers as they let you look at memory and manipulate it.

Smart pointer are implemented using struct where they implement the `Deref` and `Drop` traits. The `Deref` trait allows an instance of the smart pointer to behave like a reference.
## Box< T > 
- allocates data on heap while the pointer to the heap data remain on the stack
- the pointers however remain on the stack
- initialization `let b = Box::new()`

## Dereference
- `*variable` gives us the value at that pointer instead of a reference
- implementing this traits allow programmer to customize the behavior for the operator 

## Rc and Arc
`use std::rc::Rc;` 

Rc - reference counting
- allocated on the heap
- keeps the number of reference to see if it's still in use
- not thread safe
Arc - Atomic reference counting 
- share sharing between threads
- only use when sharing thread 
- thread safe

## RefCell
`use std::cell:RefCell;`
- allows internal mutable pattern
- this is an unsafe code
- enforced at runtime
- allows access to borrow -> Ref< T >
- allows access to borrow_mut -> RefMut< T >
- cannot have multiple borrow out unless you have Rc