//! ===============================================================================
//! FILE: heap_demo.rs | LEARN/learn_core/src/demos/heap_demo.rs
//! PURPOSE: Heap (Priority Queue) visualization with bubble up/sink down animations
//! MODIFIED: 2026-01-07
//! LAYER: LEARN -> learn_core -> demos
//! ===============================================================================

use super::pseudocode::{heap as pc_heap, Pseudocode};
use crate::{Demo, ParamMeta, Rng, Vec2};

/// Heap type
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HeapType {
    MaxHeap,
    MinHeap,
}

/// Animation state for heap operations
#[derive(Clone, Debug, PartialEq)]
pub enum HeapAnimation {
    Idle,
    Inserting {
        value: i32,
        index: usize,
        progress: f32,
    },
    BubblingUp {
        index: usize,
        progress: f32,
    },
    Extracting {
        progress: f32,
    },
    SinkingDown {
        index: usize,
        progress: f32,
    },
    Swapping {
        i: usize,
        j: usize,
        progress: f32,
    },
}

/// Heap visualization demo
#[derive(Clone)]
pub struct HeapDemo {
    /// Heap elements (array representation)
    pub elements: Vec<i32>,
    /// Heap type (max or min)
    pub heap_type: HeapType,
    /// Animation state
    pub animation: HeapAnimation,
    /// Animation speed
    pub speed: f32,
    /// Currently highlighted indices
    pub highlight: Vec<usize>,
    /// Last extracted value
    pub last_extracted: Option<i32>,
    /// Positions for tree visualization
    pub positions: Vec<Vec2>,
    /// Status message
    pub message: String,
    /// Pseudocode state
    pub pseudocode: Pseudocode,
    /// RNG
    rng: Rng,
}

impl Default for HeapDemo {
    fn default() -> Self {
        Self {
            elements: Vec::with_capacity(15),
            heap_type: HeapType::MaxHeap,
            animation: HeapAnimation::Idle,
            speed: 1.0,
            highlight: Vec::new(),
            last_extracted: None,
            positions: Vec::new(),
            message: String::new(),
            pseudocode: Pseudocode::default(),
            rng: Rng::new(42),
        }
    }
}

impl HeapDemo {
    /// Update node positions for tree visualization
    fn update_positions(&mut self) {
        self.positions.clear();
        for i in 0..self.elements.len() {
            let (x, y) = self.index_to_position(i);
            self.positions.push(Vec2::new(x, y));
        }
    }

    fn index_to_position(&self, index: usize) -> (f32, f32) {
        if index == 0 {
            return (400.0, 50.0);
        }

        let level = (index + 1).ilog2() as usize;
        let level_start = (1 << level) - 1;
        let pos_in_level = index - level_start;

        let width = 700.0 / (1 << level) as f32;
        let x = 50.0 + width * (pos_in_level as f32 + 0.5);
        let y = 50.0 + level as f32 * 70.0;

        (x, y)
    }

    /// Generate initial heap
    fn generate_initial_data(&mut self) {
        self.elements.clear();

        // Insert values to build initial heap
        let values = [50, 30, 40, 10, 20, 35, 25];
        for &val in &values {
            self.insert_immediate(val);
        }
        self.update_positions();
    }

    /// Insert immediately without animation
    fn insert_immediate(&mut self, value: i32) {
        self.elements.push(value);
        let mut idx = self.elements.len() - 1;

        // Bubble up
        while idx > 0 {
            let parent = (idx - 1) / 2;
            if self.should_swap(idx, parent) {
                self.elements.swap(idx, parent);
                idx = parent;
            } else {
                break;
            }
        }
    }

    /// Check if element at idx should be above element at parent
    fn should_swap(&self, idx: usize, parent: usize) -> bool {
        match self.heap_type {
            HeapType::MaxHeap => self.elements[idx] > self.elements[parent],
            HeapType::MinHeap => self.elements[idx] < self.elements[parent],
        }
    }

    /// Check if parent should sink to child
    fn should_sink(&self, parent: usize, child: usize) -> bool {
        if child >= self.elements.len() {
            return false;
        }
        match self.heap_type {
            HeapType::MaxHeap => self.elements[child] > self.elements[parent],
            HeapType::MinHeap => self.elements[child] < self.elements[parent],
        }
    }

    /// Get better child (for sinking)
    fn better_child(&self, parent: usize) -> Option<usize> {
        let left = 2 * parent + 1;
        let right = 2 * parent + 2;

        if left >= self.elements.len() {
            return None;
        }

        if right >= self.elements.len() {
            return Some(left);
        }

        match self.heap_type {
            HeapType::MaxHeap => {
                if self.elements[left] >= self.elements[right] {
                    Some(left)
                } else {
                    Some(right)
                }
            }
            HeapType::MinHeap => {
                if self.elements[left] <= self.elements[right] {
                    Some(left)
                } else {
                    Some(right)
                }
            }
        }
    }

    /// Start insert animation
    pub fn insert(&mut self, value: i32) {
        if self.elements.len() >= 15 {
            self.message = "Heap is full (max 15 elements)".to_string();
            return;
        }

        self.highlight.clear();
        self.pseudocode = Pseudocode::new("Heap Insert", pc_heap::INSERT);
        let index = self.elements.len();
        self.elements.push(value);
        self.update_positions();

        self.animation = HeapAnimation::Inserting {
            value,
            index,
            progress: 0.0,
        };
        self.message = format!("Inserting {} - O(log n) bubble up", value);
    }

    /// Start extract animation
    pub fn extract(&mut self) {
        if self.elements.is_empty() {
            self.message = "Heap is empty!".to_string();
            return;
        }

        self.highlight.clear();
        self.highlight.push(0);
        self.pseudocode = Pseudocode::new("Heap Extract", pc_heap::EXTRACT);

        let type_name = match self.heap_type {
            HeapType::MaxHeap => "max",
            HeapType::MinHeap => "min",
        };
        self.message = format!(
            "Extracting {} ({}) - O(log n) sink down",
            self.elements[0], type_name
        );
        self.animation = HeapAnimation::Extracting { progress: 0.0 };
    }

    /// Peek at top element
    pub fn peek(&self) -> Option<i32> {
        self.elements.first().copied()
    }

    /// Get parent index
    pub fn parent(index: usize) -> Option<usize> {
        if index == 0 {
            None
        } else {
            Some((index - 1) / 2)
        }
    }

    /// Get left child index
    pub fn left_child(index: usize) -> usize {
        2 * index + 1
    }

    /// Get right child index
    pub fn right_child(index: usize) -> usize {
        2 * index + 2
    }

    /// Set heap type
    pub fn set_heap_type(&mut self, heap_type: HeapType) {
        if self.heap_type != heap_type {
            self.heap_type = heap_type;
            self.rebuild_heap();
        }
    }

    /// Rebuild heap after type change
    fn rebuild_heap(&mut self) {
        // Floyd's algorithm - heapify from bottom up
        if self.elements.len() <= 1 {
            return;
        }

        for i in (0..self.elements.len() / 2).rev() {
            self.heapify_down(i);
        }
        self.update_positions();
    }

    fn heapify_down(&mut self, mut idx: usize) {
        loop {
            if let Some(child) = self.better_child(idx) {
                if self.should_sink(idx, child) {
                    self.elements.swap(idx, child);
                    idx = child;
                } else {
                    break;
                }
            } else {
                break;
            }
        }
    }
}

impl Demo for HeapDemo {
    fn reset(&mut self, seed: u64) {
        self.rng = Rng::new(seed);
        self.animation = HeapAnimation::Idle;
        self.highlight.clear();
        self.last_extracted = None;
        self.message.clear();
        self.pseudocode.clear();
        self.heap_type = HeapType::MaxHeap;
        self.generate_initial_data();
    }

    fn step(&mut self, dt: f32) {
        let speed = self.speed * dt * 2.0;

        // Extract animation state to avoid borrow conflicts
        let anim = std::mem::replace(&mut self.animation, HeapAnimation::Idle);

        self.animation = match anim {
            HeapAnimation::Idle => HeapAnimation::Idle,
            HeapAnimation::Inserting {
                index,
                value,
                progress,
            } => {
                let new_progress = progress + speed;
                if new_progress >= 1.0 {
                    self.highlight = vec![index];
                    HeapAnimation::BubblingUp {
                        index,
                        progress: 0.0,
                    }
                } else {
                    HeapAnimation::Inserting {
                        index,
                        value,
                        progress: new_progress,
                    }
                }
            }
            HeapAnimation::BubblingUp { index, progress } => {
                let new_progress = progress + speed * 0.7;
                if new_progress >= 1.0 {
                    if index > 0 {
                        let parent = (index - 1) / 2;
                        if self.should_swap(index, parent) {
                            self.elements.swap(index, parent);
                            self.highlight = vec![parent];
                            HeapAnimation::BubblingUp {
                                index: parent,
                                progress: 0.0,
                            }
                        } else {
                            self.message = format!("Inserted at position {}", index);
                            HeapAnimation::Idle
                        }
                    } else {
                        self.message = "Inserted at root".to_string();
                        HeapAnimation::Idle
                    }
                } else {
                    HeapAnimation::BubblingUp {
                        index,
                        progress: new_progress,
                    }
                }
            }
            HeapAnimation::Extracting { progress } => {
                let new_progress = progress + speed;
                if new_progress >= 1.0 {
                    self.last_extracted = Some(self.elements[0]);
                    let last = self.elements.pop().unwrap();
                    if !self.elements.is_empty() {
                        self.elements[0] = last;
                        self.highlight = vec![0];
                        self.update_positions();
                        HeapAnimation::SinkingDown {
                            index: 0,
                            progress: 0.0,
                        }
                    } else {
                        self.message = format!("Extracted: {}", self.last_extracted.unwrap());
                        self.update_positions();
                        HeapAnimation::Idle
                    }
                } else {
                    HeapAnimation::Extracting {
                        progress: new_progress,
                    }
                }
            }
            HeapAnimation::SinkingDown { index, progress } => {
                let new_progress = progress + speed * 0.7;
                if new_progress >= 1.0 {
                    if let Some(child) = self.better_child(index) {
                        if self.should_sink(index, child) {
                            self.elements.swap(index, child);
                            self.highlight = vec![child];
                            HeapAnimation::SinkingDown {
                                index: child,
                                progress: 0.0,
                            }
                        } else {
                            if let Some(v) = self.last_extracted {
                                self.message = format!("Extracted: {}", v);
                            }
                            HeapAnimation::Idle
                        }
                    } else {
                        if let Some(v) = self.last_extracted {
                            self.message = format!("Extracted: {}", v);
                        }
                        HeapAnimation::Idle
                    }
                } else {
                    HeapAnimation::SinkingDown {
                        index,
                        progress: new_progress,
                    }
                }
            }
            HeapAnimation::Swapping { i, j, progress } => {
                let new_progress = progress + speed;
                if new_progress >= 1.0 {
                    HeapAnimation::Idle
                } else {
                    HeapAnimation::Swapping {
                        i,
                        j,
                        progress: new_progress,
                    }
                }
            }
        };
    }

    fn set_param(&mut self, name: &str, value: f32) -> bool {
        match name {
            "speed" => {
                self.speed = value;
                true
            }
            _ => false,
        }
    }

    fn params() -> &'static [ParamMeta] {
        &[ParamMeta {
            name: "speed",
            label: "Animation Speed",
            min: 0.25,
            max: 4.0,
            step: 0.25,
            default: 1.0,
        }]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_heap_init() {
        let mut demo = HeapDemo::default();
        demo.reset(42);
        assert_eq!(demo.elements.len(), 7);
        // Max heap property: parent >= children
        assert!(demo.elements[0] >= demo.elements[1]);
        assert!(demo.elements[0] >= demo.elements[2]);
    }

    #[test]
    fn test_heap_property() {
        let mut demo = HeapDemo::default();
        demo.reset(42);

        // Check max-heap property
        for i in 0..demo.elements.len() {
            let left = 2 * i + 1;
            let right = 2 * i + 2;
            if left < demo.elements.len() {
                assert!(demo.elements[i] >= demo.elements[left]);
            }
            if right < demo.elements.len() {
                assert!(demo.elements[i] >= demo.elements[right]);
            }
        }
    }

    #[test]
    fn test_insert_extract() {
        let mut demo = HeapDemo::default();
        demo.reset(42);

        let initial_max = demo.elements[0];
        demo.insert_immediate(100);
        assert_eq!(demo.elements[0], 100); // New max

        // Extract should remove max
        demo.elements[0] = demo.elements.pop().unwrap();
        demo.heapify_down(0);
        assert_eq!(demo.elements[0], initial_max);
    }
}
