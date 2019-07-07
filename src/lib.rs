// Copyright 2019 Octavian Oncescu

use hashbrown::HashMap;
use std::borrow::ToOwned;
use std::hash::Hash;
use std::marker::PhantomData;

#[derive(Clone)]
pub struct CowHashMap<'a, K, V> 
    where K: Hash + Sized + PartialEq + Eq,
          V: 'a + ToOwned + 'a + Sized,
{
    inner: HashMap<K, V>,
    phantom: PhantomData<&'a V>
}

impl<'a, K, V> CowHashMap<'a, K, V> 
    where K: Hash + Sized + PartialEq + Eq,
          V: 'a + ToOwned + 'a + Sized,
{
    /// Creates a new `CowHashMap`.
    /// 
    /// ```rust
    /// use cow_hashmap::CowHashMap;
    /// 
    /// let hm: CowHashMap<String, String> = CowHashMap::new();
    /// ```
    pub fn new() -> Self {
        CowHashMap {
            inner: HashMap::new(),
            phantom: PhantomData
        }
    }

    /// Creates a new `CowHashMap` with the specified capacity.
    /// 
    /// ```rust
    /// use cow_hashmap::CowHashMap;
    /// 
    /// let hm: CowHashMap<String, String> = CowHashMap::with_capacity(5);
    /// assert!(hm.capacity() >= 5);
    /// ```
    pub fn with_capacity(capacity: usize) -> Self {
        CowHashMap {
            inner: HashMap::with_capacity(capacity),
            phantom: PhantomData
        }
    }

    /// Returns the number of elements the map can hold without reallocating.
    /// 
    /// This number is a lower bound; the map might be able to hold more elements, but is guaranteed to be able to hold at least this many elements.
    /// 
    /// ```rust
    /// use cow_hashmap::CowHashMap;
    /// 
    /// let hm: CowHashMap<String, String> = CowHashMap::new();
    /// assert_eq!(hm.capacity(), 0);
    /// ```
    pub fn capacity(&self) -> usize {
        self.inner.capacity()
    }

    /// Reserves capacity for at least additional more elements to be inserted in the map. 
    /// The collection may reserve more space to avoid frequent reallocations.
    pub fn reserve(&mut self, additional: usize) {
        self.inner.reserve(additional);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
