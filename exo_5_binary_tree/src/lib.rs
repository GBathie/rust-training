/// Maps keys of type `K` to values of type `V`.
pub struct BinaryTreeMap<K, V> {
    // TODO: add struct members,
    // remove _marker below
    _marker: (K, V),
}

/// Hint: you may need to define additional structs/enums,
/// or add trait bounds, etc.
impl<K, V> BinaryTreeMap<K, V> {
    pub fn new() -> Self {
        todo!("Create a new empty map")
    }

    pub fn len(&self) -> usize {
        todo!("Return the number of elements in the map")
    }

    pub fn is_empty(&self) -> bool {
        todo!("Whether the map is empty")
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        todo!("Insert the key value pair in the map, return the previous value if there was one")
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        todo!("Retrieve the value associated to a key if it exists")
    }

    pub fn contains(&self, key: &K) -> bool {
        todo!("Whether the map contains the given key")
    }

    pub fn remove(&mut self, key: &K) -> Option<V> {
        todo!("If there is a value with the given key, remove it from the map and return it")
    }
}

/// Create an iterator over the (key, value) pairs of the map,
/// ordered by key.
///
/// Fix this definition !
/// It should consume the map and return owned pairs, not references!
impl<K, V> IntoIterator for BinaryTreeMap<K, V> {
    type Item = (K, V);

    type IntoIter = BinaryTreeMapIntoIterator<K, V>;

    fn into_iter(self) -> Self::IntoIter {
        todo!()
    }
}

pub struct BinaryTreeMapIntoIterator<K, V> {
    _marker: (K, V),
}

impl<K, V> Iterator for BinaryTreeMapIntoIterator<K, V> {
    type Item = (K, V);

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn map_insert_contains() {
        let mut map = BinaryTreeMap::new();

        map.insert(1, (10, 12));
        map.insert(4, (23, 17));
        map.insert(2, (1, 1));

        assert_eq!(map.get(&0), None);
        assert_eq!(map.get(&1), Some(&(10, 12)));
        assert_eq!(map.get(&2), Some(&(1, 1)));
        assert_eq!(map.get(&3), None);
        assert_eq!(map.get(&4), Some(&(23, 17)));

        assert_eq!(map.insert(2, (15, 0)), Some((1, 1)));
    }

    #[test]
    fn map_remove() {
        let mut map = BinaryTreeMap::new();

        map.insert(1, (10, 12));
        map.insert(4, (23, 17));
        map.insert(2, (1, 1));
        map.insert(5, (1, 3));
        assert_eq!(map.len(), 4);

        assert_eq!(map.get(&4), Some(&(23, 17)));
        assert_eq!(map.remove(&4), Some((23, 17)));
        assert_eq!(map.get(&4), None);
        assert_eq!(map.insert(4, (8, 9)), None);
        map.remove(&1);
        map.remove(&2);
        assert_eq!(map.len(), 2);
    }

    #[test]
    fn map_into_iter() {
        let mut map = BinaryTreeMap::new();

        map.insert(1, "Hello");
        map.insert(4, "are");
        map.insert(2, "how");
        map.insert(5, "you?");

        let mut iter = map.into_iter();
        assert_eq!(iter.next(), Some((1, "Hello")));
        assert_eq!(iter.next(), Some((2, "how")));
        assert_eq!(iter.next(), Some((4, "are")));
        assert_eq!(iter.next(), Some((5, "you?")));
        assert_eq!(iter.next(), None);
    }
}
