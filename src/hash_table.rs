type HashFunc = fn (&str) -> u64;

#[derive(PartialEq, Debug)]
pub enum HashTableError {
    Collision(String),
}
pub struct NaiveHashTable<T: Default + Copy> {
    arr: Vec<Option<T>>,
    hash_function: HashFunc,
}

impl<T: Default + Copy> NaiveHashTable<T> {
    pub fn new(hash_function: HashFunc) -> Self {
        Self {hash_function, arr: vec![]}
    }

    pub fn set(&mut self, key: &str, value: T) -> Result<(), HashTableError> {
        let hashed_index = (self.hash_function)(key);
        
        if hashed_index as usize + 1 > self.arr.len() {
            self.arr.resize(hashed_index as usize + 1, None);
        }

        if let Some(Some(v)) = self.arr.get(hashed_index as usize) {
            return Err(HashTableError::Collision("Collision occured".to_owned()));
        } else {
            self.arr[hashed_index as usize] = Some(value);
        }

        Ok(())
    }
    
    pub fn get(&self, key: &str) -> Option<T> {
        let hashed_index = (self.hash_function)(key);
        if let Some(Some(v)) = self.arr.get(hashed_index as usize) {
            return Some(*v);
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn simple_hash(key: &str) -> u64 {
        key.len() as u64
    }

    #[test]
    fn new_creates_empty_table() {
        let table: NaiveHashTable<i32> = NaiveHashTable::new(simple_hash);
        assert_eq!(table.arr.len(), 0);
    }

    #[test]
    fn set_inserts_value() {
        let mut table: NaiveHashTable<i32> = NaiveHashTable::new(simple_hash);
        assert_eq!(table.set("key1", 42), Ok(()));
        assert_eq!(table.arr.len(), 5); // "key1".len() == 4, but we need an extra slot for the value
        assert_eq!(table.arr[4], Some(42));
    }

    #[test]
    fn set_handles_collision() {
        let mut table: NaiveHashTable<i32> = NaiveHashTable::new(simple_hash);
        assert_eq!(table.set("key1", 42), Ok(()));
        assert_eq!(table.set("key1", 43), Err(HashTableError::Collision("Collision occured".to_owned())));
    }

    #[test]
    fn get_retrieves_value() {
        let mut table: NaiveHashTable<i32> = NaiveHashTable::new(simple_hash);
        assert_eq!(table.set("key1", 42), Ok(()));
        assert_eq!(table.set("key22", 43), Ok(()));
        assert_eq!(table.set("key333", 44), Ok(()));
        assert_eq!(table.get("key1"), Some(42));
        assert_eq!(table.get("key22"), Some(43));
        assert_eq!(table.get("key333"), Some(44));
    }

    #[test]
    fn get_returns_none_for_missing_key() {
        let table: NaiveHashTable<i32> = NaiveHashTable::new(simple_hash);
        assert_eq!(table.get("missing_key"), None);
    }
}
