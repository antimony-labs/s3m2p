//! ===============================================================================
//! FILE: stack_demo.rs | LEARN/learn_core/src/demos/stack_demo.rs
//! PURPOSE: Stack (LIFO) visualization with push/pop animations
//! MODIFIED: 2026-01-07
//! LAYER: LEARN -> learn_core -> demos
//! ===============================================================================

use crate::{Demo, ParamMeta, Rng};
use super::pseudocode::{Pseudocode, stack as pc_stack};

/// Animation state for stack operations
#[derive(Clone, Debug, PartialEq)]
pub enum StackAnimation {
    Idle,
    Pushing { value: i32, progress: f32 },
    Popping { progress: f32 },
    Peeking { progress: f32 },
}

/// Stack visualization demo (LIFO)
#[derive(Clone)]
pub struct StackDemo {
    /// Stack elements (top is last)
    pub elements: Vec<i32>,
    /// Maximum capacity for display
    pub max_capacity: usize,
    /// Animation state
    pub animation: StackAnimation,
    /// Animation speed
    pub speed: f32,
    /// Last popped value
    pub last_popped: Option<i32>,
    /// Status message
    pub message: String,
    /// Current pseudocode state
    pub pseudocode: Pseudocode,
    /// RNG
    rng: Rng,
}

impl Default for StackDemo {
    fn default() -> Self {
        Self {
            elements: Vec::with_capacity(10),
            max_capacity: 8,
            animation: StackAnimation::Idle,
            speed: 1.0,
            last_popped: None,
            message: String::new(),
            pseudocode: Pseudocode::default(),
            rng: Rng::new(42),
        }
    }
}

impl StackDemo {
    /// Generate initial stack
    fn generate_initial_data(&mut self) {
        self.elements.clear();
        for _ in 0..3 {
            let value = self.rng.range(1.0, 99.0) as i32;
            self.elements.push(value);
        }
    }

    /// Start push animation
    pub fn push(&mut self, value: i32) {
        self.pseudocode = Pseudocode::new("Push", pc_stack::PUSH);
        if self.elements.len() >= self.max_capacity {
            self.message = "Stack overflow!".to_string();
            self.pseudocode.set_line(1);
            return;
        }
        self.animation = StackAnimation::Pushing { value, progress: 0.0 };
        self.message = format!("Pushing {} - O(1)", value);
        self.pseudocode.set_line(0);
    }

    /// Start pop animation
    pub fn pop(&mut self) {
        self.pseudocode = Pseudocode::new("Pop", pc_stack::POP);
        if self.elements.is_empty() {
            self.message = "Stack underflow!".to_string();
            self.pseudocode.set_line(1);
            return;
        }
        self.animation = StackAnimation::Popping { progress: 0.0 };
        self.message = "Popping - O(1)".to_string();
        self.pseudocode.set_line(0);
    }

    /// Start peek animation
    pub fn peek(&mut self) {
        self.pseudocode = Pseudocode::new("Peek", pc_stack::PEEK);
        if self.elements.is_empty() {
            self.message = "Stack is empty!".to_string();
            self.pseudocode.set_line(1);
            return;
        }
        self.animation = StackAnimation::Peeking { progress: 0.0 };
        let top = self.elements.last().unwrap();
        self.message = format!("Peek: top = {} - O(1)", top);
        self.pseudocode.set_line(0);
    }

    /// Get top element
    pub fn top(&self) -> Option<i32> {
        self.elements.last().copied()
    }

    /// Check if stack is empty
    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }

    /// Get stack size
    pub fn len(&self) -> usize {
        self.elements.len()
    }

    /// Get animation progress for the pushing element
    pub fn pushing_value(&self) -> Option<(i32, f32)> {
        match &self.animation {
            StackAnimation::Pushing { value, progress } => Some((*value, *progress)),
            _ => None,
        }
    }

    /// Get animation progress for popping
    pub fn popping_progress(&self) -> Option<f32> {
        match &self.animation {
            StackAnimation::Popping { progress } => Some(*progress),
            _ => None,
        }
    }

    /// Check if peeking
    pub fn is_peeking(&self) -> bool {
        matches!(self.animation, StackAnimation::Peeking { .. })
    }
}

impl Demo for StackDemo {
    fn reset(&mut self, seed: u64) {
        self.rng = Rng::new(seed);
        self.animation = StackAnimation::Idle;
        self.last_popped = None;
        self.message.clear();
        self.pseudocode.clear();
        self.generate_initial_data();
    }

    fn step(&mut self, dt: f32) {
        let speed = self.speed * dt * 3.0;

        match &mut self.animation {
            StackAnimation::Idle => {}
            StackAnimation::Pushing { value, progress } => {
                *progress += speed;
                if *progress < 0.3 {
                    self.pseudocode.set_line(3); // top++
                } else if *progress < 0.7 {
                    self.pseudocode.set_line(4); // stack[top] = value
                } else {
                    self.pseudocode.set_line(5); // size++
                }
                if *progress >= 1.0 {
                    let val = *value;
                    self.elements.push(val);
                    self.animation = StackAnimation::Idle;
                }
            }
            StackAnimation::Popping { progress } => {
                *progress += speed;
                if *progress < 0.3 {
                    self.pseudocode.set_line(3); // value = stack[top]
                } else if *progress < 0.6 {
                    self.pseudocode.set_line(4); // top--
                } else if *progress < 0.8 {
                    self.pseudocode.set_line(5); // size--
                } else {
                    self.pseudocode.set_line(6); // return value
                }
                if *progress >= 1.0 {
                    self.last_popped = self.elements.pop();
                    if let Some(v) = self.last_popped {
                        self.message = format!("Popped: {}", v);
                    }
                    self.animation = StackAnimation::Idle;
                }
            }
            StackAnimation::Peeking { progress } => {
                *progress += speed;
                self.pseudocode.set_line(3); // return stack[top]
                if *progress >= 1.0 {
                    self.animation = StackAnimation::Idle;
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
    fn test_stack_push_pop() {
        let mut demo = StackDemo::default();
        demo.reset(42);
        let initial_len = demo.len();

        demo.push(99);
        for _ in 0..100 { demo.step(0.016); }
        assert_eq!(demo.len(), initial_len + 1);
        assert_eq!(demo.top(), Some(99));

        demo.pop();
        for _ in 0..100 { demo.step(0.016); }
        assert_eq!(demo.len(), initial_len);
        assert_eq!(demo.last_popped, Some(99));
    }
}
