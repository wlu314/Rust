## Concurrent Situations
Atomic reference counting is safe to use in concurrent situations over a normal reference. counter. The reason why all Rc aren't just Arc by default is because thread safely comes with performance penalty.