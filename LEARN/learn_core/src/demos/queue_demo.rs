//! ===============================================================================
//! FILE: queue_demo.rs | LEARN/learn_core/src/demos/queue_demo.rs
//! PURPOSE: Queue (FIFO) visualization with enqueue/dequeue animations
//! MODIFIED: 2026-01-07
//! LAYER: LEARN -> learn_core -> demos
//! ===============================================================================

use crate::{Demo, ParamMeta, Rng};
use super::pseudocode::{Pseudocode, queue as pc_queue};

/// Animation state for queue operations
#[derive(Clone, Debug, PartialEq)]
pub enum QueueAnimation {
    Idle,
    Enqueuing { value: i32, progress: f32 },
    Dequeuing { progress: f32 },
    Peeking { progress: f32 },
}

/// Queue visualization demo (FIFO)
#[derive(Clone)]
pub struct QueueDemo {
    /// Queue elements (front is first, back is last)
    pub elements: Vec<i32>,
    /// Maximum capacity for display
    pub max_capacity: usize,
    /// Animation state
    pub animation: QueueAnimation,
    /// Animation speed
    pub speed: f32,
    /// Last dequeued value
    pub last_dequeued: Option<i32>,
    /// Status message
    pub message: String,
    /// Current pseudocode state
    pub pseudocode: Pseudocode,
    /// RNG
    rng: Rng,
}

impl Default for QueueDemo {
    fn default() -> Self {
        Self {
            elements: Vec::with_capacity(10),
            max_capacity: 8,
            animation: QueueAnimation::Idle,
            speed: 1.0,
            last_dequeued: None,
            message: String::new(),
            pseudocode: Pseudocode::default(),
            rng: Rng::new(42),
        }
    }
}

impl QueueDemo {
    /// Generate initial queue
    fn generate_initial_data(&mut self) {
        self.elements.clear();
        for _ in 0..4 {
            let value = self.rng.range(1.0, 99.0) as i32;
            self.elements.push(value);
        }
    }

    /// Start enqueue animation
    pub fn enqueue(&mut self, value: i32) {
        self.pseudocode = Pseudocode::new("Enqueue", pc_queue::ENQUEUE);
        if self.elements.len() >= self.max_capacity {
            self.message = "Queue is full!".to_string();
            self.pseudocode.set_line(1);
            return;
        }
        self.animation = QueueAnimation::Enqueuing { value, progress: 0.0 };
        self.message = format!("Enqueuing {} at back - O(1)", value);
        self.pseudocode.set_line(0);
    }

    /// Start dequeue animation
    pub fn dequeue(&mut self) {
        self.pseudocode = Pseudocode::new("Dequeue", pc_queue::DEQUEUE);
        if self.elements.is_empty() {
            self.message = "Queue is empty!".to_string();
            self.pseudocode.set_line(1);
            return;
        }
        self.animation = QueueAnimation::Dequeuing { progress: 0.0 };
        self.message = "Dequeuing from front - O(1)".to_string();
        self.pseudocode.set_line(0);
    }

    /// Start peek animation
    pub fn peek(&mut self) {
        self.pseudocode = Pseudocode::new("Peek", pc_queue::PEEK);
        if self.elements.is_empty() {
            self.message = "Queue is empty!".to_string();
            self.pseudocode.set_line(1);
            return;
        }
        self.animation = QueueAnimation::Peeking { progress: 0.0 };
        let front = self.elements.first().unwrap();
        self.message = format!("Peek: front = {} - O(1)", front);
        self.pseudocode.set_line(0);
    }

    /// Get front element
    pub fn front(&self) -> Option<i32> {
        self.elements.first().copied()
    }

    /// Get back element
    pub fn back(&self) -> Option<i32> {
        self.elements.last().copied()
    }

    /// Check if queue is empty
    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }

    /// Get queue size
    pub fn len(&self) -> usize {
        self.elements.len()
    }

    /// Get animation progress for enqueuing element
    pub fn enqueuing_value(&self) -> Option<(i32, f32)> {
        match &self.animation {
            QueueAnimation::Enqueuing { value, progress } => Some((*value, *progress)),
            _ => None,
        }
    }

    /// Get dequeue progress
    pub fn dequeuing_progress(&self) -> Option<f32> {
        match &self.animation {
            QueueAnimation::Dequeuing { progress } => Some(*progress),
            _ => None,
        }
    }

    /// Check if peeking
    pub fn is_peeking(&self) -> bool {
        matches!(self.animation, QueueAnimation::Peeking { .. })
    }
}

impl Demo for QueueDemo {
    fn reset(&mut self, seed: u64) {
        self.rng = Rng::new(seed);
        self.animation = QueueAnimation::Idle;
        self.last_dequeued = None;
        self.message.clear();
        self.pseudocode.clear();
        self.generate_initial_data();
    }

    fn step(&mut self, dt: f32) {
        let speed = self.speed * dt * 3.0;

        match &mut self.animation {
            QueueAnimation::Idle => {}
            QueueAnimation::Enqueuing { value, progress } => {
                *progress += speed;
                if *progress < 0.3 {
                    self.pseudocode.set_line(3); // rear = (rear + 1) % capacity
                } else if *progress < 0.7 {
                    self.pseudocode.set_line(4); // queue[rear] = value
                } else {
                    self.pseudocode.set_line(5); // size++
                }
                if *progress >= 1.0 {
                    let val = *value;
                    self.elements.push(val);
                    self.animation = QueueAnimation::Idle;
                }
            }
            QueueAnimation::Dequeuing { progress } => {
                *progress += speed;
                if *progress < 0.3 {
                    self.pseudocode.set_line(3); // value = queue[front]
                } else if *progress < 0.6 {
                    self.pseudocode.set_line(4); // front = (front + 1) % capacity
                } else if *progress < 0.8 {
                    self.pseudocode.set_line(5); // size--
                } else {
                    self.pseudocode.set_line(6); // return value
                }
                if *progress >= 1.0 {
                    if !self.elements.is_empty() {
                        self.last_dequeued = Some(self.elements.remove(0));
                        if let Some(v) = self.last_dequeued {
                            self.message = format!("Dequeued: {}", v);
                        }
                    }
                    self.animation = QueueAnimation::Idle;
                }
            }
            QueueAnimation::Peeking { progress } => {
                *progress += speed;
                self.pseudocode.set_line(3); // return queue[front]
                if *progress >= 1.0 {
                    self.animation = QueueAnimation::Idle;
                }
            }
        }
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
    fn test_queue_enqueue_dequeue() {
        let mut demo = QueueDemo::default();
        demo.reset(42);
        let first = demo.front();

        demo.enqueue(99);
        for _ in 0..100 { demo.step(0.016); }
        assert_eq!(demo.back(), Some(99));
        assert_eq!(demo.front(), first); // Front unchanged

        demo.dequeue();
        for _ in 0..100 { demo.step(0.016); }
        assert_eq!(demo.last_dequeued, first);
    }
}
