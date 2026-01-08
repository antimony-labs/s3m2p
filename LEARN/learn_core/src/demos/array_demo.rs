//! ===============================================================================
//! FILE: array_demo.rs | LEARN/learn_core/src/demos/array_demo.rs
//! PURPOSE: Array visualization demo with access, insert, delete operations
//! MODIFIED: 2026-01-07
//! LAYER: LEARN -> learn_core -> demos
//! ===============================================================================

use crate::{Demo, ParamMeta, Rng};
use super::pseudocode::{Pseudocode, array as pc_array};

/// Animation state for array operations
#[derive(Clone, Debug, PartialEq)]
pub enum ArrayAnimation {
    Idle,
    Accessing { index: usize, progress: f32 },
    Inserting { index: usize, value: i32, progress: f32 },
    Deleting { index: usize, progress: f32 },
    Shifting { start: usize, direction: i32, progress: f32 },
}

/// Array visualization demo
///
/// Demonstrates:
/// - O(1) index access
/// - O(n) insertion with element shifting
/// - O(n) deletion with element shifting
#[derive(Clone)]
pub struct ArrayDemo {
    /// Current array elements
    pub elements: Vec<Option<i32>>,
    /// Logical size (number of actual elements)
    pub size: usize,
    /// Total capacity
    pub capacity: usize,
    /// Current animation state
    pub animation: ArrayAnimation,
    /// Animation speed multiplier
    pub speed: f32,
    /// Currently highlighted index
    pub highlight_index: Option<usize>,
    /// Last operation message
    pub message: String,
    /// Current pseudocode state
    pub pseudocode: Pseudocode,
    /// RNG for generating values
    rng: Rng,
    seed: u64,
}

impl Default for ArrayDemo {
    fn default() -> Self {
        Self {
            elements: vec![None; 10],
            size: 0,
            capacity: 10,
            animation: ArrayAnimation::Idle,
            speed: 1.0,
            highlight_index: None,
            message: String::new(),
            pseudocode: Pseudocode::default(),
            rng: Rng::new(42),
            seed: 42,
        }
    }
}

impl ArrayDemo {
    /// Initialize with some random elements
    fn generate_initial_data(&mut self) {
        self.elements = vec![None; self.capacity];
        self.size = 5;
        for i in 0..self.size {
            self.elements[i] = Some(self.rng.range(0.0, 100.0) as i32);
        }
    }

    /// Start an access animation
    pub fn access(&mut self, index: usize) {
        if index < self.size {
            self.animation = ArrayAnimation::Accessing { index, progress: 0.0 };
            self.message = format!("Accessing index {} - O(1)", index);
            self.pseudocode = Pseudocode::new("Access", pc_array::ACCESS);
            self.pseudocode.set_line(0);
        } else {
            self.message = format!("Index {} out of bounds (size={})", index, self.size);
            self.pseudocode = Pseudocode::new("Access", pc_array::ACCESS);
            self.pseudocode.set_line(1); // Error case
        }
    }

    /// Start an insert animation
    pub fn insert(&mut self, index: usize, value: i32) {
        self.pseudocode = Pseudocode::new("Insert", pc_array::INSERT);
        if index > self.size {
            self.message = format!("Cannot insert at index {} (size={})", index, self.size);
            self.pseudocode.set_line(1);
            return;
        }
        if self.size >= self.capacity {
            self.message = "Array is full!".to_string();
            self.pseudocode.set_line(2);
            return;
        }
        self.animation = ArrayAnimation::Inserting { index, value, progress: 0.0 };
        self.message = format!("Inserting {} at index {} - O(n-i) shifting", value, index);
        self.pseudocode.set_line(0);
    }

    /// Start a delete animation
    pub fn delete(&mut self, index: usize) {
        self.pseudocode = Pseudocode::new("Delete", pc_array::DELETE);
        if index >= self.size {
            self.message = format!("Cannot delete at index {} (size={})", index, self.size);
            self.pseudocode.set_line(1);
            return;
        }
        self.animation = ArrayAnimation::Deleting { index, progress: 0.0 };
        self.message = format!("Deleting index {} - O(n-i) shifting", index);
        self.pseudocode.set_line(0);
    }

    /// Get element at index (for display)
    pub fn get(&self, index: usize) -> Option<i32> {
        if index < self.capacity {
            self.elements[index]
        } else {
            None
        }
    }

    /// Get the shift offset for an element during animation (for rendering)
    pub fn get_shift_offset(&self, index: usize) -> f32 {
        match &self.animation {
            ArrayAnimation::Inserting { index: ins_idx, progress, .. } => {
                if index >= *ins_idx && index < self.size {
                    // Elements at and after insertion point shift right
                    *progress
                } else {
                    0.0
                }
            }
            ArrayAnimation::Deleting { index: del_idx, progress, .. } => {
                if index > *del_idx && index < self.size {
                    // Elements after deletion point shift left
                    -*progress
                } else {
                    0.0
                }
            }
            _ => 0.0,
        }
    }

    /// Check if an element should be fading out (during delete)
    pub fn is_fading(&self, index: usize) -> Option<f32> {
        match &self.animation {
            ArrayAnimation::Deleting { index: del_idx, progress, .. } => {
                if index == *del_idx {
                    Some(1.0 - *progress)
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    /// Check if an element should be fading in (during insert)
    pub fn is_appearing(&self, index: usize) -> Option<(i32, f32)> {
        match &self.animation {
            ArrayAnimation::Inserting { index: ins_idx, value, progress } => {
                if index == *ins_idx && *progress > 0.5 {
                    Some((*value, (*progress - 0.5) * 2.0))
                } else {
                    None
                }
            }
            _ => None,
        }
    }
}

impl Demo for ArrayDemo {
    fn reset(&mut self, seed: u64) {
        self.seed = seed;
        self.rng = Rng::new(seed);
        self.animation = ArrayAnimation::Idle;
        self.highlight_index = None;
        self.message.clear();
        self.pseudocode.clear();
        self.generate_initial_data();
    }

    fn step(&mut self, dt: f32) {
        let speed = self.speed * dt * 2.0;

        match &mut self.animation {
            ArrayAnimation::Idle => {}
            ArrayAnimation::Accessing { index, progress } => {
                *progress += speed * 3.0;
                // Update pseudocode line based on progress
                if *progress < 0.5 {
                    self.pseudocode.set_line(1); // bounds check
                } else {
                    self.pseudocode.set_line(3); // return arr[index]
                }
                if *progress >= 1.0 {
                    self.highlight_index = Some(*index);
                    self.animation = ArrayAnimation::Idle;
                }
            }
            ArrayAnimation::Inserting { index, value, progress } => {
                *progress += speed;
                // Update pseudocode line based on progress
                if *progress < 0.2 {
                    self.pseudocode.set_line(3); // comment about shifting
                } else if *progress < 0.6 {
                    self.pseudocode.set_line(4); // for loop
                } else if *progress < 0.8 {
                    self.pseudocode.set_line(5); // arr[i+1] = arr[i]
                } else if *progress < 0.9 {
                    self.pseudocode.set_line(6); // arr[index] = value
                } else {
                    self.pseudocode.set_line(7); // size++
                }
                if *progress >= 1.0 {
                    // Complete the insertion
                    let idx = *index;
                    let val = *value;
                    // Shift elements right
                    for i in (idx..self.size).rev() {
                        self.elements[i + 1] = self.elements[i];
                    }
                    self.elements[idx] = Some(val);
                    self.size += 1;
                    self.highlight_index = Some(idx);
                    self.animation = ArrayAnimation::Idle;
                }
            }
            ArrayAnimation::Deleting { index, progress } => {
                *progress += speed;
                // Update pseudocode line based on progress
                if *progress < 0.2 {
                    self.pseudocode.set_line(3); // comment about shifting
                } else if *progress < 0.6 {
                    self.pseudocode.set_line(4); // for loop
                } else if *progress < 0.8 {
                    self.pseudocode.set_line(5); // arr[i] = arr[i+1]
                } else {
                    self.pseudocode.set_line(6); // size--
                }
                if *progress >= 1.0 {
                    // Complete the deletion
                    let idx = *index;
                    // Shift elements left
                    for i in idx..self.size - 1 {
                        self.elements[i] = self.elements[i + 1];
                    }
                    self.elements[self.size - 1] = None;
                    self.size -= 1;
                    self.highlight_index = None;
                    self.animation = ArrayAnimation::Idle;
                }
            }
            ArrayAnimation::Shifting { progress, .. } => {
                *progress += speed;
                if *progress >= 1.0 {
                    self.animation = ArrayAnimation::Idle;
                }
            }
        }
    }

    fn set_param(&mut self, name: &str, value: f32) -> bool {
        match name {
            "speed" => {
                self.speed = value;
                true
            }
            "access_index" => {
                self.access(value as usize);
                true
            }
            "insert_index" => {
                // Store for combined with insert_value
                true
            }
            "insert_value" => {
                // Would need to be combined with index
                true
            }
            "delete_index" => {
                self.delete(value as usize);
                true
            }
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
    fn test_array_init() {
        let mut demo = ArrayDemo::default();
        demo.reset(42);
        assert_eq!(demo.size, 5);
        assert!(demo.elements[0].is_some());
    }

    #[test]
    fn test_access() {
        let mut demo = ArrayDemo::default();
        demo.reset(42);
        demo.access(2);
        assert!(matches!(demo.animation, ArrayAnimation::Accessing { index: 2, .. }));
    }

    #[test]
    fn test_insert() {
        let mut demo = ArrayDemo::default();
        demo.reset(42);
        let initial_size = demo.size;
        demo.insert(0, 99);
        // Complete the animation
        for _ in 0..100 {
            demo.step(0.016);
        }
        assert_eq!(demo.size, initial_size + 1);
        assert_eq!(demo.elements[0], Some(99));
    }

    #[test]
    fn test_delete() {
        let mut demo = ArrayDemo::default();
        demo.reset(42);
        let initial_size = demo.size;
        demo.delete(0);
        // Complete the animation
        for _ in 0..100 {
            demo.step(0.016);
        }
        assert_eq!(demo.size, initial_size - 1);
    }
}
