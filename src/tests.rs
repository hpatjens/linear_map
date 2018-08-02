#[cfg(test)]
mod tests {
    use LinearMap;

    #[test]
    fn new() {
        let map = LinearMap::<usize, usize>::new();
        assert!(map.is_empty());
        assert_eq!(map.len(), 0);
        assert_eq!(map.capacity(), 0);
    }

    #[test]
    fn with_capacity() {
        const N: usize = 10;
        let map = LinearMap::<usize, usize>::with_capacity(N);
        assert!(map.is_empty());
        assert_eq!(map.len(), 0);
        assert_eq!(map.capacity(), N);
    }

    #[test]
    fn insert() {
        let mut map = LinearMap::new();
        map.insert(0, "Hello");
        map.insert(1, "World!");
        assert_eq!(map.len(), 2);
    }

    #[test]
    fn contains_key() {
        let mut map = LinearMap::new();
        map.insert(0, "Hello");
        map.insert(1, "World!");
        assert!(map.contains_key(&0));
        assert!(map.contains_key(&1));
    }

    #[test]
    fn get() {
        let mut map = LinearMap::new();
        map.insert(0, "Hello");
        map.insert(1, "World!");
        assert_eq!(map.get(&0), Some(&"Hello"));
        assert_eq!(map.get(&1), Some(&"World!"));
        assert_eq!(map.get(&2), None);
    }

    #[test]
    fn get_mut() {
        let mut map = LinearMap::new();
        map.insert(0, "Hello");
        map.insert(1, "World!");

        *map.get_mut(&0).unwrap() = "ello";
        *map.get_mut(&1).unwrap() = "orld!";

        assert_eq!(map.get_mut(&0), Some(&mut "ello"));
        assert_eq!(map.get_mut(&1), Some(&mut "orld!"));
        assert_eq!(map.get_mut(&2), None);
    }

    #[test]
    fn get_key_value() {
        let mut map = LinearMap::new();
        map.insert(0, "Hello");
        map.insert(1, "World!");
        assert_eq!(map.get_key_value(&0), Some((&0, &"Hello")));
        assert_eq!(map.get_key_value(&1), Some((&1, &"World!")));
        assert_eq!(map.get_key_value(&2), None);
    }

    #[test]
    fn append() {
        let mut map1 = LinearMap::new();
        map1.insert(0, "Hello");
        map1.insert(1, "World!");

        let mut map2 = LinearMap::new();
        map2.insert(1, "foo");
        map2.insert(2, "bar");

        map1.append(&mut map2);

        assert!(map2.is_empty());
        assert_eq!(map1.len(), 3);
        assert_eq!(map1.get(&0), Some(&"Hello"));
        assert_eq!(map1.get(&1), Some(&"foo"));
        assert_eq!(map1.get(&2), Some(&"bar"));
        assert_eq!(map1.get(&3), None);
    }

    #[test]
    fn clear() {
        let mut map = LinearMap::new();
        map.insert(0, "Hello");
        map.insert(1, "World!");
        map.clear();
        assert!(map.is_empty());
        assert_eq!(map.len(), 0);
    }

    #[test]
    fn remove() {
        let mut map = LinearMap::new();
        map.insert(0, "Hello");
        map.insert(1, "World!");
        assert_eq!(map.remove(&0), Some("Hello"));
        assert_eq!(map.len(), 1);
    }

    #[test]
    fn keys() {
        let mut map = LinearMap::new();
        map.insert(0, "Hello");
        map.insert(1, "World!");
        let mut iter = map.keys();
        assert_eq!(iter.next(), Some(&0));
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn values() {
        let mut map = LinearMap::new();
        map.insert(0, "Hello");
        map.insert(1, "World!");
        let mut iter = map.values();
        assert_eq!(iter.next(), Some(&"Hello"));
        assert_eq!(iter.next(), Some(&"World!"));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn values_mut() {
        let mut map = LinearMap::new();
        map.insert(0, String::from("Hello"));
        map.insert(1, String::from("World!"));

        {
            let mut iter_mut = map.values_mut();
            iter_mut.next().unwrap().remove(0);
            iter_mut.next().unwrap().remove(0);
        }

        let mut iter = map.values();
        assert_eq!(iter.next(), Some(&String::from("ello")));
        assert_eq!(iter.next(), Some(&String::from("orld!")));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iter() {
        let mut map = LinearMap::new();
        map.insert(0, "Hello");
        map.insert(1, "World!");
        let mut iter = map.iter();
        assert_eq!(iter.next(), Some((&0, &"Hello")));
        assert_eq!(iter.next(), Some((&1, &"World!")));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iter_mut() {
        let mut map = LinearMap::new();
        map.insert(0, String::from("Hello"));
        map.insert(1, String::from("World!"));

        {
            let mut iter_mut = map.iter_mut();
            iter_mut.next().unwrap().1.remove(0);
            iter_mut.next().unwrap().1.remove(0);
        }

        let mut iter = map.iter_mut();
        assert_eq!(iter.next(), Some((&0, &mut String::from("ello"))));
        assert_eq!(iter.next(), Some((&1, &mut String::from("orld!"))));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn into_iter() {
        let mut map = LinearMap::new();
        map.insert(0, String::from("Hello"));
        map.insert(1, String::from("World!"));

        let mut iter = map.into_iter();
        assert_eq!(iter.next(), Some((0, String::from("Hello"))));
        assert_eq!(iter.next(), Some((1, String::from("World!"))));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn for_into_iter() {
        let mut map = LinearMap::new();
        map.insert(0, String::from("Hello"));
        map.insert(1, String::from("World!"));

        let mut vec = Vec::new();
        for (k, v) in map {
            vec.push((k, v));
        }

        let mut iter = vec.into_iter();
        assert_eq!(iter.next(), Some((0, String::from("Hello"))));
        assert_eq!(iter.next(), Some((1, String::from("World!"))));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn for_iter() {
        let mut map = LinearMap::new();
        map.insert(0, String::from("Hello"));
        map.insert(1, String::from("World!"));

        let mut vec = Vec::new();
        for (k, v) in &map {
            vec.push((k, v));
        }

        let mut iter = vec.into_iter();
        assert_eq!(iter.next(), Some((&0, &String::from("Hello"))));
        assert_eq!(iter.next(), Some((&1, &String::from("World!"))));
        assert_eq!(iter.next(), None);
    }


    #[test]
    fn for_iter_mut() {
        let mut map = LinearMap::new();
        map.insert(0, String::from("Hello"));
        map.insert(1, String::from("World!"));

        for (_, v) in &mut map {
            v.remove(0);
        }

        let mut iter = map.iter();
        assert_eq!(iter.next(), Some((&0, &String::from("ello"))));
        assert_eq!(iter.next(), Some((&1, &String::from("orld!"))));
        assert_eq!(iter.next(), None);
    }
}
