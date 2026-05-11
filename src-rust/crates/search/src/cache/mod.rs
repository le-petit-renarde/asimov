//! Cache — LRU cache for hot paths

use std::collections::HashMap;
use std::sync::Arc;
use parking_lot::RwLock;

/// LRU Cache implementation
pub struct LruCache<K, V> {
    map: RwLock<HashMap<K, Arc<V>>>,
    order: RwLock<Vec<K>>,
    capacity: usize,
}

impl<K: Eq + std::hash::Hash + Clone, V: Clone> LruCache<K, V> {
    pub fn new(capacity: usize) -> Self {
        Self {
            map: RwLock::new(HashMap::new()),
            order: RwLock::new(Vec::new()),
            capacity,
        }
    }
    
    /// Get a value from cache
    pub fn get(&self, key: &K) -> Option<Arc<V>> {
        let map = self.map.read();
        let value = map.get(key).cloned()?;
        
        // Move to end (most recently used)
        drop(map);
        let mut order = self.order.write();
        if let Some(pos) = order.iter().position(|k| k == key) {
            order.rotate_left(pos);
            order.push(order.remove(0));
        }
        
        Some(value)
    }
    
    /// Insert a value into cache
    pub fn insert(&self, key: K, value: V) {
        let value = Arc::new(value);
        
        {
            let mut map = self.map.write();
            map.insert(key.clone(), value);
        }
        
        {
            let mut order = self.order.write();
            order.retain(|k| k != &key);
            order.push(key);
            
            // Evict if over capacity
            while order.len() > self.capacity {
                if let Some(oldest) = order.first().cloned() {
                    order.remove(0);
                    self.map.write().remove(&oldest);
                }
            }
        }
    }
    
    /// Clear the cache
    pub fn clear(&self) {
        self.map.write().clear();
        self.order.write().clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_lru_cache() {
        let cache = LruCache::new(3);
        
        cache.insert("a", 1);
        cache.insert("b", 2);
        cache.insert("c", 3);
        
        assert_eq!(cache.get(&"a").map(|v| v.as_ref().clone()), Some(1));
        assert_eq!(cache.get(&"b").map(|v| v.as_ref().clone()), Some(2));
        assert_eq!(cache.get(&"c").map(|v| v.as_ref().clone()), Some(3));
        
        // This should evict "b" since we accessed "a" last
        cache.insert("d", 4);
        
        assert!(cache.get(&"b").is_none());
        assert!(cache.get(&"d").is_some());
    }
}
