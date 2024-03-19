- Set only holds keys
- No duplicates
- Two Types of Sets: BTreeSet and HashSet

# HashSet
`use std::collections:HashSet;`

```
let mut hs = HashSet::new();
```

**Methods**
`.insert()` 
- inserts keys into a set 

`.remove(&k)`
- 

## Iterator
You can use an iter() to iterate through the HashSet 
```
for x in hs.iter() {
	println!("{}", x);
}
```

## Union, Intersection, Difference
```
let intersection = &set1 & &set2;
//or 
hs.intersection(&set2);
```
- intersection gets the values that both set1 and set2 have in common

```
let union = &set1 | &set2;
```
- contains in value in either two of the HashSet (no duplicates)

```
//set1 contains {1,2,3,4}
//set2 contains {1,3,5,7}
let diff = &set1 - &set2;
//diff contains {2,4}
```