//! ===============================================================================
//! FILE: hash_table_demo.rs | LEARN/learn_core/src/demos/hash_table_demo.rs
//! PURPOSE: Hash table visualization with collision handling animations
//! MODIFIED: 2026-01-07
//! LAYER: LEARN -> learn_core -> demos
//! ===============================================================================

use crate::{Demo, ParamMeta, Rng};
use super::pseudocode::{Pseudocode, hash_table as pc_hash};

/// A bucket entry in the hash table
#[derive(Clone, Debug)]
pub struct HashEntry {
    pub key: String,
    pub value: i32,
}

/// Collision resolution strategy
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CollisionStrategy {
    Chaining,       // Linked list per bucket
    LinearProbing,  // Check next slot
}

/// Animation state for hash table operations
#[derive(Clone, Debug, PartialEq)]
pub enum HashAnimation {
    Idle,
    Hashing { key: String, progress: f32, hash_value: usize },
    Inserting { bucket: usize, progress: f32 },
    Searching { key: String, bucket: usize, probe: usize, progress: f32 },
    Collision { bucket: usize, probe: usize, progress: f32 },
}

/// Hash table visualization demo
#[derive(Clone)]
pub struct HashTableDemo {
    /// Buckets (chaining: each bucket is a Vec)
    pub buckets: Vec<Vec<HashEntry>>,
    /// Number of buckets
    pub num_buckets: usize,
    /// Total entries
    pub size: usize,
    /// Collision strategy
    pub strategy: CollisionStrategy,
    /// Animation state
    pub animation: HashAnimation,
    /// Animation speed
    pub speed: f32,
    /// Currently highlighted bucket
    pub highlight_bucket: Option<usize>,
    /// Highlight chain position
    pub highlight_chain: Option<usize>,
    /// Number of collisions in current operation
    pub collisions: usize,
    /// Status message
    pub message: String,
    /// Pseudocode state
    pub pseudocode: Pseudocode,
    /// RNG
    rng: Rng,
}

impl Default for HashTableDemo {
    fn default() -> Self {
        Self {
            buckets: vec![Vec::new(); 8],
            num_buckets: 8,
            size: 0,
            strategy: CollisionStrategy::Chaining,
            animation: HashAnimation::Idle,
            speed: 1.0,
            highlight_bucket: None,
            highlight_chain: None,
            collisions: 0,
            message: String::new(),
            pseudocode: Pseudocode::default(),
            rng: Rng::new(42),
        }
    }
}

impl HashTableDemo {
    /// Simple hash function for visualization
    pub fn hash(&self, key: &str) -> usize {
        let mut hash: usize = 0;
        for (i, c) in key.chars().enumerate() {
            hash = hash.wrapping_add((c as usize).wrapping_mul(31_usize.wrapping_pow(i as u32)));
        }
        hash % self.num_buckets
    }

    /// Generate initial data
    fn generate_initial_data(&mut self) {
        self.buckets = vec![Vec::new(); self.num_buckets];
        self.size = 0;

        // Insert some initial entries
        let entries = [
            ("apple", 5),
            ("banana", 3),
            ("cherry", 7),
            ("date", 2),
            ("elderberry", 9),
        ];

        for (key, value) in entries {
            self.insert_immediate(key.to_string(), value);
        }
    }

    /// Insert immediately without animation
    fn insert_immediate(&mut self, key: String, value: i32) {
        let bucket = self.hash(&key);

        // Check if key exists (update)
        for entry in &mut self.buckets[bucket] {
            if entry.key == key {
                entry.value = value;
                return;
            }
        }

        // Add new entry
        self.buckets[bucket].push(HashEntry { key, value });
        self.size += 1;
    }

    /// Start insert animation
    pub fn insert(&mut self, key: String, _value: i32) {
        let hash_value = self.hash(&key);
        self.collisions = 0;
        self.highlight_bucket = None;
        self.highlight_chain = None;
        self.pseudocode = Pseudocode::new("Hash Insert", pc_hash::INSERT);

        self.message = format!("hash(\"{}\") = {} (mod {})", key, hash_value, self.num_buckets);
        self.animation = HashAnimation::Hashing {
            key,
            progress: 0.0,
            hash_value,
        };
    }

    /// Start search animation
    pub fn search(&mut self, key: String) {
        let bucket = self.hash(&key);
        self.collisions = 0;
        self.highlight_bucket = Some(bucket);
        self.highlight_chain = None;
        self.pseudocode = Pseudocode::new("Hash Search", pc_hash::SEARCH);

        self.message = format!("Searching for \"{}\" in bucket {}", key, bucket);
        self.animation = HashAnimation::Searching {
            key,
            bucket,
            probe: 0,
            progress: 0.0,
        };
    }

    /// Get load factor
    pub fn load_factor(&self) -> f32 {
        self.size as f32 / self.num_buckets as f32
    }

    /// Get entry count per bucket (for visualization)
    pub fn bucket_sizes(&self) -> Vec<usize> {
        self.buckets.iter().map(|b| b.len()).collect()
    }

    /// Get all entries in a bucket
    pub fn get_bucket(&self, index: usize) -> &[HashEntry] {
        if index < self.buckets.len() {
            &self.buckets[index]
        } else {
            &[]
        }
    }

    /// Delete a key
    pub fn delete(&mut self, key: &str) {
        let bucket = self.hash(key);
        if let Some(pos) = self.buckets[bucket].iter().position(|e| e.key == key) {
            self.buckets[bucket].remove(pos);
            self.size -= 1;
            self.message = format!("Deleted \"{}\" from bucket {}", key, bucket);
        } else {
            self.message = format!("\"{}\" not found", key);
        }
    }
}

impl Demo for HashTableDemo {
    fn reset(&mut self, seed: u64) {
        self.rng = Rng::new(seed);
        self.animation = HashAnimation::Idle;
        self.highlight_bucket = None;
        self.highlight_chain = None;
        self.collisions = 0;
        self.message.clear();
        self.pseudocode.clear();
        self.generate_initial_data();
    }

    fn step(&mut self, dt: f32) {
        let speed = self.speed * dt * 2.0;

        // Extract animation state to avoid borrow conflicts
        let anim = std::mem::replace(&mut self.animation, HashAnimation::Idle);

        self.animation = match anim {
            HashAnimation::Idle => HashAnimation::Idle,
            HashAnimation::Hashing { key, progress, hash_value } => {
                let new_progress = progress + speed;
                if new_progress >= 1.0 {
                    let bucket = hash_value;
                    self.highlight_bucket = Some(bucket);

                    // Generate random value before insert
                    let rand_val = self.rng.range(1.0, 100.0) as i32;
                    self.insert_immediate(key, rand_val);

                    // Check for collision
                    if self.buckets[bucket].len() > 1 {
                        self.collisions = self.buckets[bucket].len() - 1;
                        self.message = format!("Collision at bucket {} ({} existing entries)", bucket, self.collisions);
                        HashAnimation::Collision { bucket, probe: 0, progress: 0.0 }
                    } else {
                        HashAnimation::Inserting { bucket, progress: 0.0 }
                    }
                } else {
                    HashAnimation::Hashing { key, progress: new_progress, hash_value }
                }
            }
            HashAnimation::Collision { bucket, probe, progress } => {
                let new_progress = progress + speed * 0.5;
                if new_progress >= 1.0 {
                    self.highlight_chain = Some(self.buckets[bucket].len().saturating_sub(1));
                    self.message = format!("Added to chain (length: {})", self.buckets[bucket].len());
                    HashAnimation::Idle
                } else {
                    HashAnimation::Collision { bucket, probe, progress: new_progress }
                }
            }
            HashAnimation::Inserting { bucket, progress } => {
                let new_progress = progress + speed;
                if new_progress >= 1.0 {
                    self.highlight_chain = Some(0);
                    self.message = format!("Inserted in bucket {} - O(1)", bucket);
                    HashAnimation::Idle
                } else {
                    HashAnimation::Inserting { bucket, progress: new_progress }
                }
            }
            HashAnimation::Searching { key, bucket, probe, progress } => {
                let new_progress = progress + speed * 0.7;
                if new_progress >= 1.0 {
                    self.highlight_chain = Some(probe);

                    if probe < self.buckets[bucket].len() {
                        if self.buckets[bucket][probe].key == key {
                            let value = self.buckets[bucket][probe].value;
                            self.message = format!("Found \"{}\" = {} (probes: {})", key, value, probe + 1);
                            HashAnimation::Idle
                        } else {
                            self.collisions += 1;
                            HashAnimation::Searching { key, bucket, probe: probe + 1, progress: 0.0 }
                        }
                    } else {
                        self.message = format!("\"{}\" not found (probes: {})", key, probe);
                        HashAnimation::Idle
                    }
                } else {
                    HashAnimation::Searching { key, bucket, probe, progress: new_progress }
                }
            }
        };
    }

    fn set_param(&mut self, name: &str, value: f32) -> bool {
        match name {
            "speed" => { self.speed = value; true }
            _ => false,
        }
    }

    fn params() -> &'static [ParamMeta] {
        &[
            ParamMeta {
                name: "speed",
                label: "Animation Speed",
                min: 0.25,
                max: 4.0,
                step: 0.25,
                default: 1.0,
            },
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_table_init() {
        let mut demo = HashTableDemo::default();
        demo.reset(42);
        assert_eq!(demo.size, 5);
        assert_eq!(demo.num_buckets, 8);
    }

    #[test]
    fn test_hash_deterministic() {
        let demo = HashTableDemo::default();
        let h1 = demo.hash("apple");
        let h2 = demo.hash("apple");
        assert_eq!(h1, h2);
    }

    #[test]
    fn test_insert_search() {
        let mut demo = HashTableDemo::default();
        demo.reset(42);

        demo.insert_immediate("grape".to_string(), 42);
        let bucket = demo.hash("grape");
        assert!(demo.buckets[bucket].iter().any(|e| e.key == "grape" && e.value == 42));
    }

    #[test]
    fn test_load_factor() {
        let mut demo = HashTableDemo::default();
        demo.reset(42);

        let lf = demo.load_factor();
        assert!(lf > 0.0 && lf < 1.0);
    }
}
