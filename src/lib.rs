// Copyright 2019 Octavian Oncescu

use hashbrown::HashMap;
use std::borrow::{Borrow, Cow, ToOwned};
use std::hash::Hash;

#[derive(Clone, Copy, Debug, PartialEq)]
/// The form of the entry in the map. Can be either
/// `Borrowed` or `Owned`.
pub enum Form {
    /// The map entry is borrowed.
    Borrowed,

    /// The map entry is owned.
    Owned,
}

pub struct CowHashMap<'a, K, V> 
    where K: Hash + ?Sized + PartialEq + Eq + ToOwned,
          V: ToOwned + ?Sized,
{
    inner: HashMap<Cow<'a, K>, Cow<'a, V>>
}

impl<'a, K, V> CowHashMap<'a, K, V> 
    where K: Hash + ?Sized + PartialEq + Eq + ToOwned,
          V: ToOwned + ?Sized,
{
    /// Creates a new `CowHashMap`.
    /// 
    /// ## Example
    /// ```rust
    /// use hashcow::CowHashMap;
    /// 
    /// let hm: CowHashMap<str, String> = CowHashMap::new();
    /// ```
    #[inline]
    pub fn new() -> Self {
        CowHashMap {
            inner: HashMap::new()
        }
    }

    /// Creates a new `CowHashMap` with the specified capacity.
    /// 
    /// ## Example
    /// ```rust
    /// use hashcow::CowHashMap;
    /// 
    /// let hm: CowHashMap<str, String> = CowHashMap::with_capacity(5);
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
    /// ## Example
    /// ```rust
    /// use hashcow::CowHashMap;
    /// 
    /// let hm: CowHashMap<str, [u8]> = CowHashMap::new();
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

    /// Shrinks the map as much as possible while retaining the number of elements.
    #[inline]
    pub fn shrink_to_fit(&mut self) {
        self.inner.shrink_to_fit();
    }

    /// Returns true if the map contains no elements.
    ///
    /// ## Example
    /// ```rust
    /// use hashcow::CowHashMap;
    /// 
    /// let mut hm: CowHashMap<str, [u8]> = CowHashMap::new();
    /// assert!(hm.is_empty());
    /// 
    /// hm.insert_owned("key".to_owned(), vec![1, 2, 3]);
    /// assert!(!hm.is_empty());
    /// ```
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Inserts a new key/value pair into the map with the value
    /// being in the owned form.
    /// 
    /// This function returns `None` if there was no value previously 
    /// associated with the given key. If the key is replaced, this
    /// function returns the previous value. If the previous value
    /// is borrowed, it will be cloned and then returned.
    /// 
    /// ## Example
    /// ```rust
    /// use hashcow::CowHashMap;
    /// 
    /// let mut hm: CowHashMap<str, [u8]> = CowHashMap::new();
    /// hm.insert_owned("key".to_owned(), vec![1, 2, 3]);
    ///
    /// assert_eq!(hm.len(), 1);
    /// ```
    #[inline]
    pub fn insert_owned(&mut self, key: <K as ToOwned>::Owned, value: <V as ToOwned>::Owned) -> Option<<V as ToOwned>::Owned> {
        self.inner.insert(Cow::Owned(key), Cow::Owned(value)).map(|x| x.into_owned())
    }

    /// Inserts a new key/value pair into the map with the value
    /// being in the owned form and the key in borrowed form.
    /// 
    /// This function returns `None` if there was no value previously 
    /// associated with the given key. If the key is replaced, this
    /// function returns the previous value. If the previous value
    /// is borrowed, it will be cloned and then returned.
    /// 
    /// ## Example
    /// ```rust
    /// use hashcow::CowHashMap;
    /// 
    /// let mut hm: CowHashMap<str, [u8]> = CowHashMap::new();
    /// hm.insert_owned_borrowed_key("key", vec![1, 2, 3]);
    ///
    /// assert_eq!(hm.len(), 1);
    /// ```
    #[inline]
    pub fn insert_owned_borrowed_key(&mut self, key: &'a K, value: <V as ToOwned>::Owned) -> Option<<V as ToOwned>::Owned> {
        self.inner.insert(Cow::Borrowed(key), Cow::Owned(value)).map(|x| x.into_owned())
    }

    /// Inserts a new key/value pair in to the map with the value
    /// being in borrowed form.
    /// 
    /// This function returns `None` if there was no value previously 
    /// associated with the given key. If the key is replaced, this
    /// function returns the previous value. If the previous value
    /// is borrowed, it will be cloned and then returned.
    /// 
    /// ## Example
    /// ```rust
    /// use hashcow::CowHashMap;
    /// 
    /// let mut hm: CowHashMap<str, [u8]> = CowHashMap::new();
    /// hm.insert_borrowed("key", &[1, 2, 3]);
    ///
    /// assert_eq!(hm.len(), 1);
    /// ```
    #[inline]
    pub fn insert_borrowed(&mut self, key: &'a K, value: &'a V) -> Option<<V as ToOwned>::Owned> {
        self.inner.insert(Cow::Borrowed(key), Cow::Borrowed(value)).map(|x| x.into_owned())
    }

    /// Inserts a new key/value pair in to the map with the value
    /// being in borrowed form and the key in owned form.
    /// 
    /// This function returns `None` if there was no value previously 
    /// associated with the given key. If the key is replaced, this
    /// function returns the previous value. If the previous value
    /// is borrowed, it will be cloned and then returned.
    /// 
    /// ## Example
    /// ```rust
    /// use hashcow::CowHashMap;
    /// 
    /// let mut hm: CowHashMap<str, [u8]> = CowHashMap::new();
    /// hm.insert_borrowed_owned_key("key".to_owned(), &[1, 2, 3]);
    ///
    /// assert_eq!(hm.len(), 1);
    /// ```
    #[inline]
    pub fn insert_borrowed_owned_key(&mut self, key: <K as ToOwned>::Owned, value: &'a V) -> Option<<V as ToOwned>::Owned> {
        self.inner.insert(Cow::Owned(key), Cow::Borrowed(value)).map(|x| x.into_owned())
    }

    /// Attempts to retrieve a reference to an item stored in the map.
    /// 
    /// ## Example
    /// ```rust
    /// use hashcow::CowHashMap;
    /// 
    /// let mut hm: CowHashMap<str, [u8]> = CowHashMap::new();
    /// hm.insert_borrowed("key1", &[1, 2, 3]);
    /// hm.insert_owned("key2".to_owned(), vec![4, 5, 6]);
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
    /// ## Example
    /// ```rust
    /// use hashcow::{Form, CowHashMap};
    /// 
    /// let mut hm: CowHashMap<str, [u8]> = CowHashMap::new();
    /// hm.insert_borrowed("key1", &[1, 2, 3]);
    /// 
    /// assert_eq!(hm.entry_form(&"key1").unwrap(), Form::Borrowed);
    /// 
    /// {
    ///     // This will clone the entry stored at this key
    ///     let entry = hm.get_mut(&"key1").unwrap();
    ///     assert_eq!(entry, &mut vec![1, 2, 3]);
    ///     
    ///     *entry = vec![4, 5, 6];
    /// }
    ///
    /// assert_eq!(hm.entry_form(&"key1").unwrap(), Form::Owned);
    /// assert_eq!(hm.get(&"key1").unwrap(), &[4, 5, 6]);
    /// ```
    #[inline]
    pub fn get_mut(&mut self, key: &K) -> Option<&mut <V as ToOwned>::Owned> {
        self.inner.get_mut(key).map(|v| v.to_mut())
    }

    #[inline]
    /// Returns an iterator over the keys of the map.
    /// 
    /// ## Example
    /// ```rust
    /// # #[macro_use] extern crate hashcow; fn main() {
    /// # use std::collections::HashSet;
    /// use hashcow::CowHashMap;
    /// 
    /// let mut hm: CowHashMap<str, [u8]> = CowHashMap::new();
    /// hm.insert_borrowed("key1", &[1, 2, 3]);
    /// hm.insert_owned("key2".to_owned(), vec![4, 5, 6]);
    /// 
    /// let keys: HashSet<&str> = hm.keys().collect();
    /// assert_eq!(keys, set!["key1", "key2"]);
    /// # }
    /// ```
    #[inline]
    pub fn keys(&self) -> impl Iterator<Item = &K> {
        self.inner.keys().map(|k| k.borrow())
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
    /// Can be either `Form::Borrowed` or `Form::Owned`.
    #[inline]
    pub fn entry_form(&self, key: &K) -> Option<Form> {
        let val = self.inner.get(key)?;

        match val {
            Cow::Borrowed(_) => Some(Form::Borrowed),
            Cow::Owned(_) => Some(Form::Owned),
        }
    }

    /// Returns a cloned version of the map but with
    /// the entries in borrowed form.
    /// 
    /// ## Example
    /// ```rust
    /// use hashcow::{Form, CowHashMap};
    /// 
    /// let mut hm: CowHashMap<str, [u8]> = CowHashMap::new();
    /// hm.insert_owned("key".to_owned(), vec![1, 2, 3]);
    /// 
    /// assert_eq!(hm.entry_form(&"key").unwrap(), Form::Owned);
    /// 
    /// let hm_clone = hm.borrow_fields();
    /// assert_eq!(hm_clone.entry_form(&"key").unwrap(), Form::Borrowed);
    /// ```
    #[inline]
    pub fn borrow_fields(&'a self) -> Self {
        let collection: HashMap<Cow<'a, K>, Cow<'a, V>> = self.inner
            .iter()
            .map(|(k, v)| {
                match (k, v) {
                    (Cow::Owned(key), Cow::Owned(val)) => {
                        (Cow::Borrowed((*key).borrow()), Cow::Borrowed((*val).borrow()))
                    }

                    (Cow::Borrowed(key), Cow::Owned(val)) => {
                        (Cow::Borrowed(*key), Cow::Borrowed((*val).borrow()))
                    }

                    (Cow::Owned(key), Cow::Borrowed(val)) => {
                        (Cow::Borrowed((*key).borrow()), Cow::Borrowed(*val))
                    }

                    (Cow::Borrowed(key), Cow::Borrowed(val)) => {
                        (Cow::Borrowed(*key), Cow::Borrowed(*val))
                    }
                }
                
            })
            .collect();

        CowHashMap { inner: collection }
    }
}

#[macro_use]
mod macros;

#[cfg(test)]
mod tests {
    use super::*;
}
