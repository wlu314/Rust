## Max Heap
`use std::collections::BinaryHeap;`

**Properties**
- largest value is always in the front
- can be initialized with an array => `let heap = BinaryHeap::from([1,2,3]);`
`let mut bheap = BinaryHeap::new();`
`.push()` pushes the onto heap
`.pop()` removes largest value
`.peek()` returns largest value without removing


## Min Heap
`use std::collections::BinaryHeap;`
`use std::cmp::Reverse;`

- must wrap values in Reverse `heap.push(Reverse(1))`

## Time Complexity
Push: O(1)
Pop: O(log(n))
Peek/Peek_mut: O(1)