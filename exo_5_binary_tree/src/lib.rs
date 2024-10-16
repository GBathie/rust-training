use std::{cmp::Ordering, mem};

/// Maps keys of type `K` to values of type `V`.
pub struct BinaryTreeMap<K, V> {
    root: BinaryTreeNode<K, V>,
    size: usize,
}

pub enum BinaryTreeNode<K, V> {
    Leaf,
    Node {
        key: K,
        value: V,
        left: Box<BinaryTreeNode<K, V>>,
        right: Box<BinaryTreeNode<K, V>>,
    },
}

impl<K, V> BinaryTreeNode<K, V>
where
    K: Ord,
{
    fn is_leaf(&self) -> bool {
        matches!(self, BinaryTreeNode::Leaf)
    }

    pub fn insert(&mut self, key: K, mut value: V) -> Option<V> {
        match self {
            BinaryTreeNode::Leaf => {
                *self = Self::Node {
                    key,
                    value,
                    left: Box::from(BinaryTreeNode::Leaf),
                    right: Box::from(BinaryTreeNode::Leaf),
                };
                None
            }
            BinaryTreeNode::Node {
                key: node_key,
                value: node_value,
                left,
                right,
            } => match key.cmp(node_key) {
                Ordering::Less => left.insert(key, value),
                Ordering::Equal => {
                    mem::swap(&mut value, node_value);
                    Some(value)
                }
                Ordering::Greater => right.insert(key, value),
            },
        }
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        match self {
            BinaryTreeNode::Leaf => None,
            BinaryTreeNode::Node {
                key: node_key,
                value,
                left,
                right,
            } => match key.cmp(node_key) {
                Ordering::Less => left.get(key),
                Ordering::Equal => Some(value),
                Ordering::Greater => right.get(key),
            },
        }
    }

    /// Replace self with the right child, and return the old (key, value)
    /// pair (or do nothing and return None if the node is a leaf).
    ///
    /// Used for compliance with the borrow checker in the [`remove`] function.
    fn replace_with_right(&mut self) -> Option<(K, V)> {
        if let BinaryTreeNode::Node {
            key, value, right, ..
        } = mem::replace(self, Self::Leaf)
        {
            let _ = mem::replace(self, *right);
            Some((key, value))
        } else {
            None
        }
    }

    /// Replace self with the left child, and return the old (key, value)
    /// pair (or do nothing and return None if the node is a leaf).
    ///
    /// Used for compliance with the borrow checker in the [`remove`] function.
    fn replace_with_left(&mut self) -> Option<(K, V)> {
        if let BinaryTreeNode::Node {
            key, value, left, ..
        } = mem::replace(self, Self::Leaf)
        {
            let _ = mem::replace(self, *left);
            Some((key, value))
        } else {
            None
        }
    }

    fn pop_smallest(&mut self) -> Option<(K, V)> {
        match self {
            BinaryTreeNode::Leaf => None,
            BinaryTreeNode::Node { left, .. } => {
                if left.is_leaf() {
                    self.replace_with_right()
                } else {
                    left.pop_smallest()
                }
            }
        }
    }

    pub fn remove(&mut self, key: &K) -> Option<V> {
        match self {
            BinaryTreeNode::Leaf => None,
            BinaryTreeNode::Node {
                key: node_key,
                value,
                left,
                right,
            } => match key.cmp(node_key) {
                Ordering::Less => left.remove(key),
                Ordering::Equal => {
                    if let Some((k, v)) = right.pop_smallest() {
                        let _ = mem::replace(node_key, k);
                        Some(mem::replace(value, v))
                    } else {
                        // `right` is a leaf.
                        // SAFETY: we know that self is BinaryTreeNode::Node, so this returns `Some((k, v))`.
                        let (_, v) = self.replace_with_left().unwrap();
                        Some(v)
                    }
                }
                Ordering::Greater => right.remove(key),
            },
        }
    }
}

impl<K, V> Default for BinaryTreeMap<K, V>
where
    K: Ord,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<K, V> BinaryTreeMap<K, V>
where
    K: Ord,
{
    pub fn new() -> Self {
        Self {
            root: BinaryTreeNode::Leaf,
            size: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.size
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        let res = self.root.insert(key, value);
        if res.is_none() {
            self.size += 1;
        }
        res
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        self.root.get(key)
    }

    pub fn contains(&self, key: &K) -> bool {
        self.get(key).is_some()
    }

    pub fn remove(&mut self, key: &K) -> Option<V> {
        let res = self.root.remove(key);
        if res.is_some() {
            self.size -= 1;
        }
        res
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
        BinaryTreeMapIntoIterator {
            stack: vec![InOrderNode::Begin(self.root)],
        }
    }
}

enum InOrderNode<K, V> {
    /// First time we see a node
    Begin(BinaryTreeNode<K, V>),
    /// We have already gone throught the left branch of the node
    Middle {
        kv: (K, V),
        right: BinaryTreeNode<K, V>,
    },
}

pub struct BinaryTreeMapIntoIterator<K, V> {
    stack: Vec<InOrderNode<K, V>>,
}

impl<K, V> Iterator for BinaryTreeMapIntoIterator<K, V> {
    type Item = (K, V);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let cur = self.stack.pop()?;

            match cur {
                InOrderNode::Begin(BinaryTreeNode::Node {
                    key,
                    value,
                    left,
                    right,
                }) => {
                    self.stack.push(InOrderNode::Middle {
                        kv: (key, value),
                        right: *right,
                    });
                    self.stack.push(InOrderNode::Begin(*left));
                }
                InOrderNode::Middle { kv, right } => {
                    self.stack.push(InOrderNode::Begin(right));
                    break Some(kv);
                }
                _ => (), // Ignore leaf nodes
            }
        }
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
