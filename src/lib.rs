// Copyright 2019 Octavian Oncescu

use hashbrown::HashMap;
use std::borrow::{Borrow, Cow, ToOwned};
use std::hash::Hash;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EntryForm {
    /// The map entry is borrowed.
    Borrowed,

    /// The map entry is owned.
    Owned,
}

pub struct CowHashMap<'a, K, V> 
    where K: Hash + Sized + PartialEq + Eq + Clone,
          V: ToOwned + ?Sized,
{
    inner: HashMap<K, Cow<'a, V>>
}

impl<'a, K, V> CowHashMap<'a, K, V> 
    where K: Hash + Sized + PartialEq + Eq + Clone,
          V: ToOwned + ?Sized,
{
    /// Creates a new `CowHashMap`.
    /// 
    /// ```rust
    /// use hashcow::CowHashMap;
    /// 
    /// let hm: CowHashMap<&str, String> = CowHashMap::new();
    /// ```
    #[inline]
    pub fn new() -> Self {
        CowHashMap {
            inner: HashMap::new()
        }
    }

    /// Creates a new `CowHashMap` with the specified capacity.
    /// 
    /// ```rust
    /// use hashcow::CowHashMap;
    /// 
    /// let hm: CowHashMap<&str, String> = CowHashMap::with_capacity(5);
    /// assert!(hm.capacity() >= 5);
    /// ```
    #[inline]
    pub fn with_capacity(capacity: usize) -> Self {
        CowHashMap {
            inner: HashMap::with_capacity(capacity)
        }
    }

    /// Returns the number of elements the map can hold without reallocating.
    /// 
    /// This number is a lower bound; the map might be able to hold more elements, but is guaranteed to be able to hold at least this many elements.
    /// 
    /// ```rust
    /// use hashcow::CowHashMap;
    /// 
    /// let hm: CowHashMap<&str, [u8]> = CowHashMap::new();
    /// assert_eq!(hm.capacity(), 0);
    /// ```
    #[inline]
    pub fn capacity(&self) -> usize {
        self.inner.capacity()
    }

    /// Reserves capacity for at least additional more elements to be inserted in the map. 
    /// The collection may reserve more space to avoid frequent reallocations.
    #[inline]
    pub fn reserve(&mut self, additional: usize) {
        self.inner.reserve(additional);
    }

    /// Inserts a new key/value pair into the map with the value
    /// being in the owned form.
    /// 
    /// This function returns `None` if there was no value previously 
    /// associated with the given key. If the key is replaced, this
    /// function returns the previous value. If the previous value
    /// is borrowed, it will be cloned and then returned.
    /// 
    /// ```rust
    /// use hashcow::CowHashMap;
    /// 
    /// let mut hm: CowHashMap<&str, [u8]> = CowHashMap::new();
    /// hm.insert_owned("key", vec![1, 2, 3]);
    ///
    /// assert_eq!(hm.len(), 1);
    /// ```
    #[inline]
    pub fn insert_owned(&mut self, key: K, value: <V as ToOwned>::Owned) -> Option<<V as ToOwned>::Owned> {
        self.inner.insert(key, Cow::Owned(value)).map(|x| x.into_owned())
    }

    /// Inserts a new key/value pair in to the map with the value
    /// being in borrowed form.
    /// 
    /// This function returns `None` if there was no value previously 
    /// associated with the given key. If the key is replaced, this
    /// function returns the previous value. If the previous value
    /// is borrowed, it will be cloned and then returned.
    /// 
    /// ```rust
    /// use hashcow::CowHashMap;
    /// 
    /// let mut hm: CowHashMap<&str, [u8]> = CowHashMap::new();
    /// hm.insert_borrowed("key", &[1, 2, 3]);
    ///
    /// assert_eq!(hm.len(), 1);
    /// ```
    #[inline]
    pub fn insert_borrowed(&mut self, key: K, value: &'a V) -> Option<<V as ToOwned>::Owned> {
        self.inner.insert(key, Cow::Borrowed(value)).map(|x| x.into_owned())
    }

    /// Attempts to retrieve a reference to an item stored in the map.
    /// 
    /// ```rust
    /// use hashcow::CowHashMap;
    /// 
    /// let mut hm: CowHashMap<&str, [u8]> = CowHashMap::new();
    /// hm.insert_borrowed("key1", &[1, 2, 3]);
    /// hm.insert_owned("key2", vec![4, 5, 6]);
    ///
    /// assert_eq!(hm.len(), 2);
    /// assert_eq!(hm.get(&"key1").unwrap(), &[1, 2, 3]);
    /// assert_eq!(hm.get(&"key2").unwrap(), &[4, 5, 6]);
    /// ```
    #[inline]
    pub fn get(&self, key: &K) -> Option<&V> {
        self.inner.get(key).map(|v| v.as_ref())
    }

    /// Attempts to retrieve a mutable reference to the owned
    /// form of an item stored in the map. 
    /// 
    /// If the stored entry is in the borrowed form, this function
    /// will clone the underlying data.
    /// 
    /// ```rust
    /// use hashcow::{EntryForm, CowHashMap};
    /// 
    /// let mut hm: CowHashMap<&str, [u8]> = CowHashMap::new();
    /// hm.insert_borrowed("key1", &[1, 2, 3]);
    /// 
    /// assert_eq!(hm.entry_form(&"key1").unwrap(), EntryForm::Borrowed);
    /// 
    /// {
    ///     // This will clone the entry stored at this key
    ///     let entry = hm.get_mut(&"key1").unwrap();
    ///     assert_eq!(entry, &mut vec![1, 2, 3]);
    ///     
    ///     *entry = vec![4, 5, 6];
    /// }
    ///
    /// assert_eq!(hm.entry_form(&"key1").unwrap(), EntryForm::Owned);
    /// assert_eq!(hm.get(&"key1").unwrap(), &[4, 5, 6]);
    /// ```
    #[inline]
    pub fn get_mut(&mut self, key: &K) -> Option<&mut <V as ToOwned>::Owned> {
        self.inner.get_mut(key).map(|v| v.to_mut())
    }

    /// Makes a specific value in the map owned, if it isn't so already.
    /// 
    /// This function does not do anything if the value is already in owned
    /// form.
    #[inline]
    pub fn make_owned(&mut self, key: &K) -> Option<&V> {
        let val = self.inner.get_mut(key)?;
        
        match val {
            Cow::Borrowed(v) => {
                *val = Cow::Owned(v.to_owned());
                self.inner.get(key).map(|v| v.as_ref())
            }

            Cow::Owned(_) => {
                self.inner.get(key).map(|v| v.as_ref())
            }
        }
    }

    /// Returns the number of elements that are currently in the map.
    #[inline]
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    /// If an entry with the given key exists, this function
    /// returns the underlying form in which it is stored in 
    /// the map.
    /// 
    /// Can be either `EntryForm::Borrowed` or `EntryForm::Owned`.
    #[inline]
    pub fn entry_form(&self, key: &K) -> Option<EntryForm> {
        let val = self.inner.get(key)?;

        match val {
            Cow::Borrowed(_) => Some(EntryForm::Borrowed),
            Cow::Owned(_) => Some(EntryForm::Owned),
        }
    }

    /// Returns a cloned version of the map but with
    /// the entries in borrowed form.
    /// 
    /// ```rust
    /// use hashcow::{EntryForm, CowHashMap};
    /// 
    /// let mut hm: CowHashMap<&str, [u8]> = CowHashMap::new();
    /// hm.insert_owned("key", vec![1, 2, 3]);
    /// 
    /// assert_eq!(hm.entry_form(&"key").unwrap(), EntryForm::Owned);
    /// 
    /// let hm_clone = hm.borrow_fields();
    /// assert_eq!(hm_clone.entry_form(&"key").unwrap(), EntryForm::Borrowed);
    /// ```
    #[inline]
    pub fn borrow_fields(&'a self) -> Self {
        let collection: HashMap<K, Cow<'a, V>> = self.inner
            .iter()
            .map(|(k, v)| {
                match v {
                    Cow::Owned(val) => {
                        (k.clone(), Cow::Borrowed((*val).borrow()))
                    }

                    Cow::Borrowed(val) => {
                        (k.clone(), Cow::Borrowed(*val))
                    }
                }
                
            })
            .collect();

        CowHashMap { inner: collection }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
