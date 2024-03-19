```
pub trait Into<T>: Sized {
    // Required method
    fn into(self) -> T;
}
```
- Opposite of [[From]]
- A value-to-value conversion that consumes the input value