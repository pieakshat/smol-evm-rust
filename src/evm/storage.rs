use primitive_types::U256; 
use std::collections::HashMap;

pub struct Storage {
    data: HashMap<U256, U256>, 
}

pub enum StorageError {
    StorageAccessError, 
}


impl Storage {

    pub fn new() -> Self {
        Storage {
            data: HashMap::new(), 
        }
    }

    pub fn load_storage(&self, storage: &Storage, slot: U256) -> Result<U256, StorageError> {
        storage.load(slot)
    }

    pub fn store_storage(&mut self, storage: &mut Storage, slot: U256, value: U256) -> Result<(), StorageError> {
        storage.store(slot, value) 
    }

    pub fn store(&mut self, key: U256, value: U256) -> Result<(), StorageError> {
        self.data.insert(key, value);
        Ok(())
    }

    pub fn load(&self, key: U256) -> Result<U256, StorageError> {
        Ok(*self.data.get(&key).unwrap_or(&U256::zero()))
    }

    pub fn contains(&self, key: U256) -> bool {
        self.data.contains_key(&key)
    }
}
