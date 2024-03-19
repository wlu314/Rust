- Collection of <Key, Pair> values
- Query using key
- B-Tree Map and HashMap are the two options

## HashMap
`use std::collections::HashMap;`

**Initialization**
`let mut hm = HashMap::new()`

**Methods**
`.insert(k: K, v: V)`
- cannot have multiple have multiple values mapped to a key
- if you insert the same key, then it updates the value, also returns old value
*Example*
```
let mut hm = HashMap::new();

hm.insert(1, 1);
hm.insert(2, 2);
hm.insert(3, 4);

let old = hm.insert(3, 3);

// old contains Some(4) 
```

`.contains_key(&5)` 
- checks if there was a `mut` key

`.get(&key)`
- this return Some(value)
- return Option< T >, if they key doesn't None is returned

`.remove(&key)`
- removes the key value pair
- return Option< T > => None or Some(T) of the value

`.remove_entry(&key)`
- removes the key value pair 
- return Option< T > => None or Some(K, V) of the key value pair

`.clear()` 
- clears the HashMap

# BTreeMap
`use std::collections::BTreeMap`
An order map based on a Binary Tree. Similar methods to a HashMap. Every element is stored in a heap allocated node. This means that every insertion triggers a heap-allocation.
