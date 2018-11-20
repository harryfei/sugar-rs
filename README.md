# Sugar

Syntax sugar to make your Rust life more sweet.

## Usage
```rust
use sugar::*;
```
## Overview

```rust
use sugar::*;

// vec of boxed value
let vb1 = vec_box![1, 2, 3];

// vec/list comprehension
let vb2 = v![Box::new(i), for i in 1..4];

// hashmap construction
let hm1 = hashmap!{
    1 => 2,
    2 => 3,
    3 => 4,
};

// hashmap comprehension
let hm2 = m!{i => i + 1, for i in 1..4};

let _ = max!(1, 2, 3);
let _ = min!(1, 2, 3);

if cmp!(1, < num， < 3) {
    println!("hello world");
}

```

More detail in sugar's [documentation](https://docs.rs/sugar).
