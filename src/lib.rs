// Copyright (c) 2018 Henrik Patjens (hpatjens@gmail.com)
// 
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
// 
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
// 
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

use std::borrow::Borrow;
use std::iter::Iterator;
use std::mem;
use std::vec;
use std::slice;

mod tests;

/// LinearMap is a map that is implemented using arrays. The elements are stored unsorted
/// which has the consequence that every operation takes at least O(n) time. Therefore, 
/// this map is only suited for small numbers of entries when frequent inserts,
/// lookups and removes are required. When the number of entries is kept small, LinearMap
/// performs better than [`BTreeMap`] and [`HashMap`] from the standard library in the average 
/// cases. When is comes to iterating over the entries of the map, the performance
/// advantage of LinearMap compared to [`BTreeMap`] and HashMap rises with the number of
/// entries.
///
/// To provide good interchangeability between maps, LinearMap provides the most important
/// subset of methods which are also provided by [`BTreeMap`] and [`HashMap`]. Parts of the API
/// requiring ordering are excluded, like `range` and `range_mut` from [`BTreeMap`]. 
/// The `Entry API` is also excluded, however might be implemented later.
///
/// [`BTreeMap`]: https://doc.rust-lang.org/std/collections/struct.BTreeMap.html
/// [`HashMap`]: https://doc.rust-lang.org/std/collections/struct.HashMap.html
///
/// # Examples
///
/// ```
/// use linear_map::LinearMap;
///
/// let mut movie_reviews = LinearMap::new();
/// 
/// // review some movies.
/// movie_reviews.insert("Office Space",       "Deals with real issues in the workplace.");
/// movie_reviews.insert("Pulp Fiction",       "Masterpiece.");
/// movie_reviews.insert("The Godfather",      "Very enjoyable.");
/// movie_reviews.insert("The Blues Brothers", "Eye lyked it alot.");
/// 
/// // check for a specific one.
/// if !movie_reviews.contains_key("Les Misérables") {
///     println!("We've got {} reviews, but Les Misérables ain't one.",
///              movie_reviews.len());
/// }
///  
/// // oops, this review has a lot of spelling mistakes, let's delete it.
/// movie_reviews.remove("The Blues Brothers");
///  
/// // look up the values associated with some keys.
/// let to_find = ["Up!", "Office Space"];
/// for book in &to_find {
///     match movie_reviews.get(book) {
///        Some(review) => println!("{}: {}", book, review),
///        None => println!("{} is unreviewed.", book)
///     }
/// }
///  
/// // iterate over everything.
/// for (movie, review) in &movie_reviews {
///     println!("{}: \"{}\"", movie, review);
/// }
///
/// ```
pub struct LinearMap<K, V> 
    where K: PartialEq
{
    keys: Vec<K>,
    values: Vec<V>,
}

impl<K, V> LinearMap<K, V> 
    where K: PartialEq
{
    /// Creates an empty `LinearMap`.
    ///
    /// The map is initially created with a capacity of 0, so it will not allocate until it
    /// is first inserted into.
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate linear_map;
    /// use linear_map::LinearMap;
    /// 
    /// let mut map: LinearMap<usize, &str> = LinearMap::new();
    /// ```
    pub fn new() -> Self {
        LinearMap {
            keys: Vec::new(),
            values: Vec::new(),
        }
    }

    /// Creates an empty `LinearMap` with the specified capacity.
    ///
    /// The map will be able to hold at least `capacity` elements without reallocating. 
    /// If `capacity` is 0, the hash map will not allocate.
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate linear_map;
    /// use linear_map::LinearMap;
    /// 
    /// let mut map: LinearMap<usize, &str> = LinearMap::with_capacity(100);
    /// ```
    pub fn with_capacity(capacity: usize) -> Self {
        LinearMap {
            keys: Vec::with_capacity(capacity),
            values: Vec::with_capacity(capacity),
        }
    }

    /// Clears the map, removing all key-value pairs. Keeps the allocated memory for reuse.
    ///
    /// # Time Complexity
    ///
    /// O(1)
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate linear_map;
    /// use linear_map::LinearMap;
    /// 
    /// let mut map = LinearMap::new();
    /// map.insert(0, "a");
    /// map.clear();
    /// assert!(map.is_empty());
    ///
    /// ```
    pub fn clear(&mut self) {
        self.keys.clear();
        self.values.clear();
    }

    /// Returns a reference to the requested value when available.
    /// 
    /// # Time Complexity
    ///
    /// O(n) where n is the number of elements in the map.
    ///
    /// # Examples
    /// 
    /// ```
    /// extern crate linear_map;
    /// use linear_map::LinearMap;
    /// 
    /// let mut map = LinearMap::new();
    /// map.insert(0, "a");
    /// map.insert(1, "b");
    /// map.insert(2, "c");
    /// assert_eq!(map.get(&1), Some(&"b"));
    /// ```
    pub fn get<Q>(&self, key: &Q) -> Option<&V> 
        where
            K: Borrow<Q>,
            Q: PartialEq + ?Sized,
    {
        if let Some(i) = self.find(key) {
            Some(&self.values[i])
        } else {
            None
        }
    }

    /// Returns a mutable reference to the requested value when available.
    /// 
    /// # Time Complexity
    ///
    /// O(n) where n is the number of elements in the map.
    ///
    /// # Examples
    /// 
    /// ```
    /// extern crate linear_map;
    /// use linear_map::LinearMap;
    /// 
    /// let mut map = LinearMap::new();
    /// map.insert(0, "a");
    /// map.insert(1, "b");
    /// map.insert(2, "c");
    /// assert_eq!(map.get_mut(&1), Some(&mut "b"));
    /// ```
    pub fn get_mut<Q>(&mut self, key: &Q) -> Option<&mut V> 
        where
            K: Borrow<Q>,
            Q: PartialEq + ?Sized,
    {
        if let Some(i) = self.find(key) {
            Some(&mut self.values[i])
        } else {
            None
        }
    }

    /// Returns a tuple with references to the requested key and value when available.
    /// 
    /// # Time Complexity
    ///
    /// O(n) where n is the number of elements in the map.
    ///
    /// # Examples
    /// 
    /// ```
    /// extern crate linear_map;
    /// use linear_map::LinearMap;
    /// 
    /// let mut map = LinearMap::new();
    /// map.insert(0, "a");
    /// map.insert(1, "b");
    /// map.insert(2, "c");
    /// assert_eq!(map.get_key_value(&1), Some((&1, &"b")));
    /// ```
    pub fn get_key_value<Q>(&self, key: &Q) -> Option<(&K, &V)> 
        where
            K: Borrow<Q>,
            Q: PartialEq + ?Sized,
    {
        if let Some(i) = self.find(key) {
            Some((&self.keys[i], &self.values[i]))
        } else {
            None
        }
    }

    /// Moves all values from `other` into `self`.
    /// 
    /// # Time Complexity
    ///
    /// O(n^2) where n is the number of elements in the map.
    ///
    /// # Examples
    /// 
    /// ```
    /// extern crate linear_map;
    /// use linear_map::LinearMap;
    /// 
    /// let mut map1 = LinearMap::new();
    /// map1.insert(0, "a");
    /// map1.insert(1, "b");
    ///
    /// let mut map2 = LinearMap::new();
    /// map2.insert(1, "c"); // Replaces the entry (1, "b") in map1
    /// map2.insert(2, "d");
    ///
    /// map1.append(&mut map2);
    ///
    /// assert_eq!(map1.get(&0), Some(&"a"));
    /// assert_eq!(map1.get(&1), Some(&"c")); // Value from map2 survived
    /// assert_eq!(map1.get(&2), Some(&"d"));
    /// ```
    pub fn append(&mut self, other: &mut LinearMap<K, V>) {
        for i in 0..other.len() {
            unsafe {
                let mut key = mem::uninitialized();
                let mut value = mem::uninitialized();
                mem::swap(&mut key, &mut other.keys[i]);
                mem::swap(&mut value, &mut other.values[i]);
                self.insert(key, value);
            }
        }
        other.clear();
    }

    /// Returns the number of elements the map can hold without reallocating.
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate linear_map;
    /// use linear_map::LinearMap; 
    ///
    /// assert_eq!(LinearMap::<i32, &str>::new().capacity(), 0);
    /// assert_eq!(LinearMap::<i32, &str>::with_capacity(100).capacity(), 100);
    /// ```
    pub fn capacity(&self) -> usize {
        self.keys.capacity()
    }

    /// Inserts a key-value pair into the map.
    /// 
    /// If the map did not have this key present, `None` is returned.
    /// 
    /// If the map did have this key present, the value is updated, and the old value 
    /// is returned. The key is not updated, though; matters for types that can be == 
    /// without being identical.
    ///
    /// # Time Complexity
    ///
    /// O(n) where n is the number of elements in the map.
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate linear_map;
    /// use linear_map::LinearMap; 
    ///
    /// let mut map = LinearMap::new();
    /// assert_eq!(map.insert(0, "a"), None);
    /// assert_eq!(map.insert(1, "b"), None);
    /// assert_eq!(map.insert(1, "c"), Some("b"));
    ///
    /// ```
    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        if let Some(i) = self.find(&key) {
            Some(mem::replace(&mut self.values[i], value))
        } else {
            self.keys.push(key);
            self.values.push(value);
            None
        }
    }

    /// Returns the number of elements in the map.
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate linear_map;
    /// use linear_map::LinearMap; 
    ///
    /// let mut map = LinearMap::new();
    /// map.insert(0, "a");
    /// map.insert(1, "b");
    /// assert_eq!(map.len(), 2);
    /// ```
    pub fn len(&self) -> usize {
        self.keys.len()
    }

    /// Returns if the map is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate linear_map;
    /// use linear_map::LinearMap; 
    ///
    /// let mut map = LinearMap::new();
    /// assert!(map.is_empty());
    /// map.insert(0, "a");
    /// assert!(!map.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.keys.is_empty()
    }

    /// Removes the entry from the map.
    ///
    /// # Time Complexity
    ///
    /// O(n) where n is the number of elements in the map.
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate linear_map;
    /// use linear_map::LinearMap; 
    ///
    /// let mut map = LinearMap::new();
    /// map.insert(0, "a");
    /// map.insert(1, "b");
    /// map.remove(&0);
    /// assert_eq!(map.len(), 1);
    /// ```
    pub fn remove<Q>(&mut self, key: &Q) -> Option<V> 
        where
            K: Borrow<Q>,
            Q: PartialEq + ?Sized, 
    {
        if let Some(i) = self.find(key) {
            self.keys.swap_remove(i);
            Some(self.values.swap_remove(i))
        } else {
            None
        }
    }

    /// Returns `true` if the map contains a value for the specified key.
    ///
    /// # Time Complexity
    ///
    /// O(n) where n is the number of elements in the map.
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate linear_map;
    /// use linear_map::LinearMap; 
    ///
    /// let mut map = LinearMap::new();
    /// map.insert(0, "a");
    /// assert!(map.contains_key(&0));
    /// assert!(!map.contains_key(&1));
    /// ```
    pub fn contains_key<Q>(&self, key: &Q) -> bool 
        where
            K: Borrow<Q>,
            Q: PartialEq + ?Sized,
    {
        self.find(key) != None
    }

    /// Gets an iterator over the keys of the map, unsorted.
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate linear_map;
    /// use linear_map::LinearMap; 
    ///
    /// let mut map = LinearMap::new();
    /// map.insert(1, "a");
    /// map.insert(2, "b");
    /// map.insert(3, "c");
    /// 
    /// for key in map.keys() {
    ///     println!("{}", key);
    /// }
    /// ```
    pub fn keys(&self) -> slice::Iter<K> {
        self.keys.iter()
    }

    /// Gets an iterator over the values of the map, unsorted.
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate linear_map;
    /// use linear_map::LinearMap; 
    ///
    /// let mut map = LinearMap::new();
    /// map.insert(1, "a");
    /// map.insert(2, "b");
    /// map.insert(3, "c");
    /// 
    /// for value in map.values() {
    ///     println!("{}", value);
    /// }
    /// ```
    pub fn values(&self) -> slice::Iter<V> {
        self.values.iter()
    }

    /// Gets a mutable iterator over the values of the map, unsorted.
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate linear_map;
    /// use linear_map::LinearMap; 
    ///
    /// let mut map = LinearMap::new();
    /// map.insert(1, "a");
    /// map.insert(2, "b");
    /// map.insert(3, "c");
    /// 
    /// for value in map.values_mut() {
    ///     *value = "d";
    /// }
    ///
    /// assert!(map.values().all(|v| *v == "d"));
    /// ```
    pub fn values_mut(&mut self) -> slice::IterMut<V> {
        self.values.iter_mut()
    }

    /// Gets an iterator over the entries of the map, unsorted.
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate linear_map;
    /// use linear_map::LinearMap; 
    ///
    /// let mut map = LinearMap::new();
    /// map.insert(1, "a");
    /// map.insert(2, "b");
    /// map.insert(3, "c");
    /// 
    /// for (key, value) in map.iter() {
    ///     println!("{}: {}", key, value);
    /// }
    /// ```
    pub fn iter(&self) -> Iter<K, V> {
        Iter {
            key: self.keys.iter(),
            value: self.values.iter(),
        }
    }

    /// Gets a mutable iterator over the entries of the map, unsorted.
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate linear_map;
    /// use linear_map::LinearMap; 
    ///
    /// let mut map = LinearMap::new();
    /// map.insert(1, "a");
    /// map.insert(2, "b");
    /// map.insert(3, "c");
    /// 
    /// for (_key, value) in map.iter_mut() {
    ///     *value = "d";
    /// }
    ///
    /// assert!(map.values().all(|v| *v == "d"));
    /// ```
    pub fn iter_mut(&mut self) -> IterMut<K, V> {
        IterMut {
            key: self.keys.iter(),
            value: self.values.iter_mut(),
        }
    }
    
    #[inline]
    fn find<Q>(&self, key: &Q) -> Option<usize>
        where
            K: Borrow<Q>,
            Q: PartialEq + ?Sized
    {
        for (i, k) in self.keys.iter().enumerate() {
            if key.eq(k.borrow()) {
                return Some(i);
            }
        }
        None
    }
}

/// An iterator over the entries of a LinearMap.
///
/// This struct is created by the `iter` method on [`LinearMap`](struct.LinearMap.html). See its documentation for more.
pub struct Iter<'a, K: 'a, V: 'a> {
    key: slice::Iter<'a, K>,
    value: slice::Iter<'a, V>,
}

impl<'a, K: 'a, V: 'a> Iterator for Iter<'a, K, V> {
    type Item = (&'a K, &'a V);

    fn next(&mut self) -> Option<(&'a K, &'a V)> {
        if let Some(key) = self.key.next() {
            let value = self.value.next().unwrap();
            Some((key, value))
        } else {
            None
        }
    }
}

impl<'a, K, V> IntoIterator for &'a LinearMap<K, V> 
    where 
        K: PartialEq + 'a,
        V: 'a
{
    type Item = (&'a K, &'a V);
    type IntoIter = Iter<'a, K, V>;

    fn into_iter(self) -> Iter<'a, K, V> {
        self.iter()
    }
}

/// An iterator over the entries of a LinearMap.
///
/// This struct is created by the `iter_mut` method on [`LinearMap`](struct.LinearMap.html). See its documentation for more.
pub struct IterMut<'a, K: 'a, V: 'a> {
    key: slice::Iter<'a, K>,
    value: slice::IterMut<'a, V>,
}

impl<'a, K: 'a, V: 'a> Iterator for IterMut<'a, K, V> {
    type Item = (&'a K, &'a mut V);

    fn next(&mut self) -> Option<(&'a K, &'a mut V)> {
        if let Some(key) = self.key.next() {
            let value = self.value.next().unwrap();
            Some((key, value))
        } else {
            None
        }
    }
}

impl<'a, K, V> IntoIterator for &'a mut LinearMap<K, V>
    where 
        K: PartialEq + 'a,
        V: 'a
{
    type Item = (&'a K, &'a mut V);
    type IntoIter = IterMut<'a, K, V>;

    fn into_iter(self) -> IterMut<'a, K, V> {
        self.iter_mut()
    }
}

/// An iterator over the entries of a LinearMap.
///
/// This struct is created by the `into_iter` method on [`LinearMap`](struct.LinearMap.html). See its documentation for more.
pub struct IntoIter<K, V> {
    key: vec::IntoIter<K>,
    value: vec::IntoIter<V>,
}

impl<K, V> Iterator for IntoIter<K, V> {
    type Item = (K, V);

    fn next(&mut self) -> Option<(K, V)> {
        if let Some(key) = self.key.next() {
            let value = self.value.next().unwrap();
            Some((key, value))
        } else {
            None
        }
    }
}

impl<K: PartialEq, V> IntoIterator for LinearMap<K, V> {
    type Item = (K, V);
    type IntoIter = IntoIter<K, V>;

    fn into_iter(self) -> IntoIter<K, V> {
        IntoIter {
            key: self.keys.into_iter(),
            value: self.values.into_iter(),
        }
    }
}