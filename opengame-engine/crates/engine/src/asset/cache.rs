use std::collections::HashMap;

pub struct AssetCache<T> {
    assets: HashMap<String, T>,
}

impl<T> AssetCache<T> {
    pub fn new() -> Self {
        Self {
            assets: HashMap::new(),
        }
    }

    pub fn insert(&mut self, key: &str, asset: T) {
        self.assets.insert(key.to_string(), asset);
    }

    pub fn get(&self, key: &str) -> Option<&T> {
        self.assets.get(key)
    }

    pub fn get_mut(&mut self, key: &str) -> Option<&mut T> {
        self.assets.get_mut(key)
    }

    pub fn contains(&self, key: &str) -> bool {
        self.assets.contains_key(key)
    }

    pub fn remove(&mut self, key: &str) -> Option<T> {
        self.assets.remove(key)
    }

    pub fn len(&self) -> usize {
        self.assets.len()
    }

    pub fn is_empty(&self) -> bool {
        self.assets.is_empty()
    }

    pub fn clear(&mut self) {
        self.assets.clear();
    }
}

impl<T> Default for AssetCache<T> {
    fn default() -> Self {
        Self::new()
    }
}
