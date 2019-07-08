# HashCow
[![Build Status]][travis] [![Discord Badge]][Discord] [![Latest Version]][crates.io] [![Documentation]][docs.rs] 

HashCow is a HashMap implementation with copy-on-write keys and values.

---

Originally built for optimizing the [Purple Protocol](https://github.com/purpleprotocol/purple), this library provides a way to link HashMaps in memory that have duplicate entries. Instead of the duplicate data, it is instead borrowed and it is only cloned when mutation is needed.

### Using HashCow
```rust
use hashcow::{Form, CowHashMap};

let mut hm: CowHashMap<str, [u8]> = CowHashMap::new();

// We insert an owned value in the map
hm.insert_owned("key".to_owned(), vec![1, 2, 3]);
assert_eq!(hm.entry_form(&"key").unwrap(), Form::Owned);

// We now create a clone with borrowed fields
let mut hm_clone = hm.borrow_fields();
assert_eq!(hm_clone.entry_form(&"key").unwrap(), Form::Borrowed);

// On mutation, the borrowed entry is cloned
let entry = hm_clone.get_mut(&"key").unwrap();

// We now mutate the cloned value
*entry = vec![4, 5, 6];
assert_eq!(hm_clone.entry_form(&"key").unwrap(), Form::Owned);

// The two maps now have different entries for the same key
assert_eq!(hm.get(&"key").unwrap(), &[1, 2, 3]);
assert_eq!(hm_clone.get(&"key").unwrap(), &[4, 5, 6]);
```

### Contributing
We welcome anyone wishing to contribute to HashCow! Check out the [issues section][issues] of the repository before starting out.

### License

HashCow is licensed under the MIT license.

[Build Status]: https://travis-ci.org/purpleprotocol/hashcow.svg?branch=master
[Discord Badge]: https://img.shields.io/discord/435827644915777536.svg
[Discord]: https://discord.gg/eGBzyaA
[travis]: https://travis-ci.org/purpleprotocol/hashcow
[crates.io]: https://crates.io/crates/hashcow
[Latest Version]: https://img.shields.io/crates/v/hashcow.svg
[Documentation]: https://docs.rs/hashcow/badge.svg
[docs.rs]: https://docs.rs/hashcow
[issues]: https://github.com/purpleprotocol/hashcow/issues
