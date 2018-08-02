use std::borrow::Borrow;
use std::cmp::{Ord, Ordering};
use std::iter::Iterator;
use std::mem;
use std::slice;

mod tests;

pub struct LinearMap<K, V> 
    where K: Ord
{
    keys: Vec<K>,
    values: Vec<V>,
}

impl<K, V> LinearMap<K, V> 
    where K: Ord
{
    pub fn new() -> Self {
        LinearMap {
            keys: Vec::new(),
            values: Vec::new(),
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        LinearMap {
            keys: Vec::with_capacity(capacity),
            values: Vec::with_capacity(capacity),
        }
    }

    pub fn clear(&mut self) {
        self.keys.clear();
        self.values.clear();
    }

    pub fn get<Q>(&self, key: &Q) -> Option<&V> 
        where
            K: Borrow<Q>,
            Q: Ord + ?Sized,
    {
        if let Some(i) = self.find(key) {
            Some(&self.values[i])
        } else {
            None
        }
    }

    pub fn get_mut<Q>(&mut self, key: &Q) -> Option<&mut V> 
        where
            K: Borrow<Q>,
            Q: Ord + ?Sized,
    {
        if let Some(i) = self.find(key) {
            Some(&mut self.values[i])
        } else {
            None
        }
    }

    pub fn get_key_value<Q>(&self, key: &Q) -> Option<(&K, &V)> 
        where
            K: Borrow<Q>,
            Q: Ord + ?Sized,
    {
        if let Some(i) = self.find(key) {
            Some((&self.keys[i], &self.values[i]))
        } else {
            None
        }
    }

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

    pub fn capacity(&self) -> usize {
        self.keys.capacity()
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        if let Some(i) = self.find(&key) {
            self.keys[i] = key;
            Some(mem::replace(&mut self.values[i], value))
        } else {
            self.keys.push(key);
            self.values.push(value);
            None
        }
    }

    pub fn len(&self) -> usize {
        self.keys.len()
    }

    pub fn is_empty(&self) -> bool {
        self.keys.is_empty()
    }

    pub fn remove<Q>(&mut self, key: &Q) -> Option<V> 
        where
            K: Borrow<Q>,
            Q: Ord + ?Sized, 
    {
        if let Some(i) = self.find(key) {
            self.keys.swap_remove(i);
            Some(self.values.swap_remove(i))
        } else {
            None
        }
    }

    pub fn contains_key<Q>(&self, key: &Q) -> bool 
        where
            K: Borrow<Q>,
            Q: Ord + ?Sized,
    {
        self.find(key) != None
    }

    pub fn keys(&self) -> slice::Iter<K> {
        self.keys.iter()
    }

    pub fn values(&self) -> slice::Iter<V> {
        self.values.iter()
    }

    pub fn values_mut(&mut self) -> slice::IterMut<V> {
        self.values.iter_mut()
    }

    pub fn iter(&self) -> Iter<K, V> {
        Iter {
            key: self.keys.iter(),
            value: self.values.iter(),
        }
    }

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
            Q: Ord + ?Sized
    {
        for (i, k) in self.keys.iter().enumerate() {
            if key.cmp(k.borrow()) == Ordering::Equal {
                return Some(i);
            }
        }
        None
    }
}

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