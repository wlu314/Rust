```
pub trait From<T>: Sized {
    // Required method
    fn from(value: T) -> Self;
}
```
- used to do value-to-value conversions while consuming the input value
- opposite of [[Into]]
- Prefer implementing From over Into because From automatically provides one with the implementation of Into 

