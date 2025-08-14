use std::collections::HashMap;
use std::time::{Duration, SystemTime};
use serde::{Deserialize, Serialize};

/// Caches audio data with LRU eviction and statistics.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioCache {
    cache: HashMap<String, CachedAudio>,
    max_size_bytes: usize,
    current_size_bytes: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct CachedAudio {
    data: super::AudioData,
    created_at: SystemTime,
    access_count: u32,
    last_accessed: SystemTime,
}


impl AudioCache {
    /// Create a new audio cache with a specified maximum size in megabytes.
    /// The cache will evict the least recently used entries when it exceeds this size.
    pub fn new(max_size_mb: usize) -> Self {
        Self {
            cache: HashMap::new(),
            max_size_bytes: max_size_mb * 1024 * 1024,
            current_size_bytes: 0,
        }
    }

    /// Get the audio data from the cache by key.
    /// If the key exists, it updates the access statistics and returns the audio data.
    pub fn get(&mut self, key: &str) -> Option<super::AudioData> {
        if let Some(cached) = self.cache.get_mut(key) {
            // Update access statistics
            cached.access_count += 1;
            cached.last_accessed = SystemTime::now();
            Some(cached.data.clone())
        } else {
            None
        }
    }

    /// Insert new audio data into the cache.
    /// If the key already exists, it replaces the existing entry.
    pub fn insert(&mut self, key: String, data: super::AudioData) {
        let data_size = data.size_bytes();

        // Remove existing entry if present
        if let Some(existing) = self.cache.remove(&key) {
            self.current_size_bytes -= existing.data.size_bytes();
        }

        // Ensure we have space for the new entry
        self.ensure_capacity(data_size);

        let cached_audio = CachedAudio {
            data,
            created_at: SystemTime::now(),
            access_count: 1,
            last_accessed: SystemTime::now(),
        };

        self.cache.insert(key, cached_audio);
        self.current_size_bytes += data_size;
    }

    /// Remove an entry from the cache by key.
    /// If the key exists, it removes the entry and returns the audio data.
    pub fn remove(&mut self, key: &str) -> Option<super::AudioData> {
        if let Some(cached) = self.cache.remove(key) {
            self.current_size_bytes -= cached.data.size_bytes();
            Some(cached.data)
        } else {
            None
        }
    }

    /// Check if the cache contains a key.
    /// Returns true if the key exists in the cache, false otherwise.
    pub fn contains_key(&self, key: &str) -> bool {
        self.cache.contains_key(key)
    }

    /// Clear the cache, removing all entries.
    /// This resets the cache size to zero.
    pub fn clear(&mut self) {
        self.cache.clear();
        self.current_size_bytes = 0;
    }

    /// Get the number of entries in the cache.
    /// Returns the count of cached audio entries.
    pub fn len(&self) -> usize {
        self.cache.len()
    }

    /// Check if the cache is empty.
    /// Returns true if there are no entries in the cache, false otherwise.
    pub fn is_empty(&self) -> bool {
        self.cache.is_empty()
    }

    /// Get the current size of the cache in bytes.
    /// Returns the total size of all cached audio data.
    pub fn current_size_bytes(&self) -> usize {
        self.current_size_bytes
    }

    /// Get the maximum size of the cache in bytes.
    /// Returns the configured maximum size of the cache.
    pub fn max_size_bytes(&self) -> usize {
        self.max_size_bytes
    }

    /// Get the percentage of cache usage.
    /// Returns the percentage of the cache that is currently in use.
    pub fn usage_percentage(&self) -> f64 {
        if self.max_size_bytes == 0 {
            0.0
        } else {
            (self.current_size_bytes as f64 / self.max_size_bytes as f64) * 100.0
        }
    }

    /// Cleanup expired entries based on the specified maximum age.
    /// This method removes entries that have not been accessed within the specified duration.
    pub fn cleanup_expired(&mut self, max_age: Duration) {
        let now = SystemTime::now();
        let mut keys_to_remove = Vec::new();

        for (key, cached) in &self.cache {
            if let Ok(age) = now.duration_since(cached.created_at) {
                if age > max_age {
                    keys_to_remove.push(key.clone());
                }
            }
        }

        for key in keys_to_remove {
            self.remove(&key);
        }
    }

    fn ensure_capacity(&mut self, needed_bytes: usize) {
        while self.current_size_bytes + needed_bytes > self.max_size_bytes && !self.cache.is_empty()
        {
            self.evict_lru();
        }
    }

    //evicts least recently used entry
    fn evict_lru(&mut self) {
        let mut oldest_key: Option<String> = None;
        let mut oldest_time = SystemTime::now();

        for (key, cached) in &self.cache {
            if cached.last_accessed < oldest_time {
                oldest_time = cached.last_accessed;
                oldest_key = Some(key.clone());
            }
        }

        if let Some(key) = oldest_key {
            self.remove(&key);
        }
    }

    /// Get statistics about the cache, including entry count, current size, max size, and usage percentage.
    /// Returns a `CacheStats` struct containing the statistics.
    pub fn stats(&self) -> CacheStats {
        let mut total_access_count = 0;
        let mut oldest_entry = SystemTime::now();
        let mut newest_entry = SystemTime::UNIX_EPOCH;

        for cached in self.cache.values() {
            total_access_count += cached.access_count;
            if cached.created_at < oldest_entry {
                oldest_entry = cached.created_at;
            }
            if cached.created_at > newest_entry {
                newest_entry = cached.created_at;
            }
        }

        CacheStats {
            entry_count: self.cache.len(),
            current_size_bytes: self.current_size_bytes,
            max_size_bytes: self.max_size_bytes,
            usage_percentage: self.usage_percentage(),
            total_access_count,
            oldest_entry: if self.cache.is_empty() {
                None
            } else {
                Some(oldest_entry)
            },
            newest_entry: if self.cache.is_empty() {
                None
            } else {
                Some(newest_entry)
            },
        }
    }
}

/// Statistics about the audio cache.
/// This struct contains information about the number of entries, current size, maximum size, usage percentage
/// Statistics about the audio cache.
pub struct CacheStats {
    /// The number of entries currently in the cache.
    pub entry_count: usize,
    /// The total size of all cached audio data in bytes.
    pub current_size_bytes: usize,
    /// The maximum allowed size of the cache in bytes.
    pub max_size_bytes: usize,
    /// The percentage of cache usage.
    pub usage_percentage: f64,
    /// The total number of accesses to cached entries.
    pub total_access_count: u32,
    /// The creation time of the oldest entry in the cache.
    pub oldest_entry: Option<SystemTime>,
    /// The creation time of the newest entry in the cache.
    pub newest_entry: Option<SystemTime>,
}

impl std::fmt::Display for CacheStats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Cache Stats: {} entries, {:.1}% full ({}/{} bytes), {} total accesses",
            self.entry_count,
            self.usage_percentage,
            self.current_size_bytes,
            self.max_size_bytes,
            self.total_access_count
        )
    }
}
