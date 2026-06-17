use std::hash::{DefaultHasher, Hash, Hasher};

#[derive(Debug)]
pub struct HashTable<K, V, const N: usize> {
    pub len: usize,
    pub table: [Vec<(K, V)>; N],
}

impl<K, V, const N: usize> Default for HashTable<K, V, N> {
    fn default() -> Self {
        Self {
            len: 0,
            table: std::array::from_fn(|_| Vec::new()),
        }
    }
}

impl<K: Hash + Eq, V, const N: usize> HashTable<K, V, N> {
    pub fn new() -> Self {
        Self::default()
    }

    fn hash(&self, key: &K) -> usize {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        let big_hash = hasher.finish();
        (big_hash as usize) % N
    }

    pub fn insert(&mut self, key: K, value: V) {
        let index = self.hash(&key);
        let bucket = &mut self.table[index];

        for (k, v) in bucket.iter_mut() {
            if *k == key {
                *v = value;
                return;
            }
        }

        bucket.push((key, value));
        self.len += 1;
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        let index = self.hash(key);
        let bucket = &self.table[index];

        for (k, v) in bucket.iter() {
            if k == key {
                return Some(v);
            }
        }
        None
    }

    pub fn remove(&mut self, key: &K) -> Option<V> {
        let index = self.hash(key);
        let bucket = &mut self.table[index];

        let pos = bucket.iter().position(|(k, _)| k == key)?;

        self.len -= 1;
        let value = bucket.swap_remove(pos);
        Some(value.1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_and_get() {
        let mut hash_table: HashTable<&str, i32, 11> = HashTable::new();
        assert!(hash_table.len == 0);
        hash_table.insert("foo", 42);
        assert!(hash_table.len == 1);

        assert_eq!(hash_table.get(&"foo"), Some(&42));
        assert_eq!(hash_table.get(&"bar"), None);
    }

    #[test]
    fn test_insert_and_remove() {
        let mut hash_table: HashTable<&str, i32, 11> = HashTable::new();
        hash_table.insert("foo", 42);
        assert!(hash_table.len == 1);

        assert_eq!(hash_table.remove(&"foo"), Some(42));
        assert!(hash_table.len == 0);
        assert_eq!(hash_table.get(&"foo"), None);
    }
}
