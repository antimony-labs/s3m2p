//! ===============================================================================
//! FILE: linked_list_demo.rs | LEARN/learn_core/src/demos/linked_list_demo.rs
//! PURPOSE: Linked list visualization with pointer-based operations
//! MODIFIED: 2026-01-07
//! LAYER: LEARN -> learn_core -> demos
//! ===============================================================================

use crate::{Demo, ParamMeta, Rng, Vec2};
use super::pseudocode::{Pseudocode, linked_list as pc_linked_list};

/// A node in the linked list
#[derive(Clone, Debug)]
pub struct ListNode {
    pub value: i32,
    pub next: Option<usize>,  // Index into nodes vector
    pub position: Vec2,       // For rendering animation
    pub target_position: Vec2,
}

/// Animation state for linked list operations
#[derive(Clone, Debug, PartialEq)]
pub enum ListAnimation {
    Idle,
    InsertingHead { value: i32, progress: f32 },
    InsertingTail { value: i32, progress: f32 },
    Deleting { index: usize, progress: f32 },
    Searching { value: i32, current: usize, progress: f32 },
    Traversing { current: usize, progress: f32 },
}

/// Linked list visualization demo
#[derive(Clone)]
pub struct LinkedListDemo {
    /// All nodes (some may be "deleted")
    pub nodes: Vec<ListNode>,
    /// Index of head node (None if empty)
    pub head: Option<usize>,
    /// Number of active nodes
    pub size: usize,
    /// Animation state
    pub animation: ListAnimation,
    /// Animation speed
    pub speed: f32,
    /// Currently highlighted node
    pub highlight: Option<usize>,
    /// Status message
    pub message: String,
    /// Current pseudocode state
    pub pseudocode: Pseudocode,
    /// RNG
    rng: Rng,
}

impl Default for LinkedListDemo {
    fn default() -> Self {
        Self {
            nodes: Vec::with_capacity(20),
            head: None,
            size: 0,
            animation: ListAnimation::Idle,
            speed: 1.0,
            highlight: None,
            message: String::new(),
            pseudocode: Pseudocode::default(),
            rng: Rng::new(42),
        }
    }
}

impl LinkedListDemo {
    /// Calculate positions for all nodes
    fn update_positions(&mut self) {
        let start_x = 100.0;
        let spacing = 120.0;
        let y = 200.0;

        let mut current = self.head;
        let mut i = 0;

        while let Some(idx) = current {
            if let Some(node) = self.nodes.get_mut(idx) {
                node.target_position = Vec2::new(start_x + i as f32 * spacing, y);
                current = node.next;
                i += 1;
            } else {
                break;
            }
        }
    }

    /// Generate initial list
    fn generate_initial_data(&mut self) {
        self.nodes.clear();
        self.head = None;
        self.size = 0;

        // Add 4 initial nodes
        for _ in 0..4 {
            let value = self.rng.range(1.0, 99.0) as i32;
            self.insert_tail_immediate(value);
        }
        self.update_positions();

        // Set initial positions to target positions
        for node in &mut self.nodes {
            node.position = node.target_position;
        }
    }

    /// Insert at tail immediately (no animation)
    fn insert_tail_immediate(&mut self, value: i32) {
        let new_idx = self.nodes.len();
        let new_node = ListNode {
            value,
            next: None,
            position: Vec2::new(-50.0, 200.0),
            target_position: Vec2::new(0.0, 200.0),
        };
        self.nodes.push(new_node);

        if self.head.is_none() {
            self.head = Some(new_idx);
        } else {
            // Find tail
            let mut current = self.head;
            let mut tail_idx = None;
            while let Some(idx) = current {
                tail_idx = Some(idx);
                current = self.nodes[idx].next;
            }
            if let Some(tail) = tail_idx {
                self.nodes[tail].next = Some(new_idx);
            }
        }
        self.size += 1;
    }

    /// Start insert at head animation
    pub fn insert_head(&mut self, value: i32) {
        self.animation = ListAnimation::InsertingHead { value, progress: 0.0 };
        self.message = format!("Inserting {} at head - O(1)", value);
        self.pseudocode = Pseudocode::new("Insert Head", pc_linked_list::INSERT_HEAD);
        self.pseudocode.set_line(0);
    }

    /// Start insert at tail animation
    pub fn insert_tail(&mut self, value: i32) {
        self.animation = ListAnimation::InsertingTail { value, progress: 0.0 };
        self.message = format!("Inserting {} at tail - O(n) traversal", value);
        self.pseudocode = Pseudocode::new("Insert Tail", pc_linked_list::INSERT_TAIL);
        self.pseudocode.set_line(0);
    }

    /// Start delete head animation
    pub fn delete_head(&mut self) {
        self.pseudocode = Pseudocode::new("Delete Head", pc_linked_list::DELETE_HEAD);
        if let Some(head_idx) = self.head {
            self.animation = ListAnimation::Deleting { index: head_idx, progress: 0.0 };
            self.message = "Deleting head - O(1)".to_string();
            self.pseudocode.set_line(0);
        } else {
            self.message = "List is empty!".to_string();
            self.pseudocode.set_line(1);
        }
    }

    /// Start search animation
    pub fn search(&mut self, value: i32) {
        self.pseudocode = Pseudocode::new("Search", pc_linked_list::SEARCH);
        if let Some(head_idx) = self.head {
            self.animation = ListAnimation::Searching { value, current: head_idx, progress: 0.0 };
            self.message = format!("Searching for {} - O(n)", value);
            self.pseudocode.set_line(0);
        } else {
            self.message = "List is empty!".to_string();
            self.pseudocode.set_line(8);
        }
    }

    /// Get traversal order of nodes
    pub fn get_traversal_order(&self) -> Vec<usize> {
        let mut order = Vec::new();
        let mut current = self.head;
        while let Some(idx) = current {
            order.push(idx);
            current = self.nodes.get(idx).and_then(|n| n.next);
        }
        order
    }
}

impl Demo for LinkedListDemo {
    fn reset(&mut self, seed: u64) {
        self.rng = Rng::new(seed);
        self.animation = ListAnimation::Idle;
        self.highlight = None;
        self.message.clear();
        self.pseudocode.clear();
        self.generate_initial_data();
    }

    fn step(&mut self, dt: f32) {
        let speed = self.speed * dt * 2.0;

        // Animate node positions
        for node in &mut self.nodes {
            let diff = node.target_position - node.position;
            if diff.length() > 0.1 {
                node.position = node.position + diff * 0.1;
            } else {
                node.position = node.target_position;
            }
        }

        match &mut self.animation {
            ListAnimation::Idle => {}
            ListAnimation::InsertingHead { value, progress } => {
                *progress += speed;
                // Update pseudocode line based on progress
                if *progress < 0.3 {
                    self.pseudocode.set_line(1); // newNode = createNode
                } else if *progress < 0.6 {
                    self.pseudocode.set_line(2); // newNode.next = head
                } else if *progress < 0.8 {
                    self.pseudocode.set_line(3); // head = newNode
                } else {
                    self.pseudocode.set_line(4); // size++
                }
                if *progress >= 1.0 {
                    let val = *value;
                    let new_idx = self.nodes.len();
                    let new_node = ListNode {
                        value: val,
                        next: self.head,
                        position: Vec2::new(-50.0, 200.0),
                        target_position: Vec2::new(0.0, 0.0),
                    };
                    self.nodes.push(new_node);
                    self.head = Some(new_idx);
                    self.size += 1;
                    self.update_positions();
                    self.highlight = Some(new_idx);
                    self.animation = ListAnimation::Idle;
                }
            }
            ListAnimation::InsertingTail { value, progress } => {
                *progress += speed;
                // Update pseudocode line based on progress
                if *progress < 0.2 {
                    self.pseudocode.set_line(1); // newNode = createNode
                } else if *progress < 0.4 {
                    self.pseudocode.set_line(5); // current = head
                } else if *progress < 0.7 {
                    self.pseudocode.set_line(6); // while current.next != null
                } else if *progress < 0.9 {
                    self.pseudocode.set_line(8); // current.next = newNode
                } else {
                    self.pseudocode.set_line(9); // size++
                }
                if *progress >= 1.0 {
                    let val = *value;
                    self.insert_tail_immediate(val);
                    self.update_positions();
                    self.highlight = Some(self.nodes.len() - 1);
                    self.animation = ListAnimation::Idle;
                }
            }
            ListAnimation::Deleting { index, progress } => {
                *progress += speed;
                // Update pseudocode line based on progress
                if *progress < 0.3 {
                    self.pseudocode.set_line(3); // temp = head
                } else if *progress < 0.6 {
                    self.pseudocode.set_line(4); // head = head.next
                } else if *progress < 0.8 {
                    self.pseudocode.set_line(5); // delete temp
                } else {
                    self.pseudocode.set_line(6); // size--
                }
                if *progress >= 1.0 {
                    let idx = *index;
                    if self.head == Some(idx) {
                        self.head = self.nodes[idx].next;
                        self.size -= 1;
                    }
                    self.update_positions();
                    self.highlight = None;
                    self.animation = ListAnimation::Idle;
                }
            }
            ListAnimation::Searching { value, current, progress } => {
                *progress += speed * 0.5;
                // Update pseudocode line based on progress
                if *progress < 0.3 {
                    self.pseudocode.set_line(3); // while current != null
                } else if *progress < 0.6 {
                    self.pseudocode.set_line(4); // if current.value == value
                } else {
                    self.pseudocode.set_line(6); // current = current.next
                }
                if *progress >= 1.0 {
                    let val = *value;
                    let cur = *current;
                    self.highlight = Some(cur);
                    if self.nodes[cur].value == val {
                        self.message = format!("Found {} at position!", val);
                        self.pseudocode.set_line(5); // return index (Found!)
                        self.animation = ListAnimation::Idle;
                    } else if let Some(next_idx) = self.nodes[cur].next {
                        self.animation = ListAnimation::Searching {
                            value: val,
                            current: next_idx,
                            progress: 0.0,
                        };
                    } else {
                        self.message = format!("{} not found in list", val);
                        self.pseudocode.set_line(8); // return NOT_FOUND
                        self.animation = ListAnimation::Idle;
                    }
                }
            }
            ListAnimation::Traversing { current, progress } => {
                *progress += speed * 0.5;
                if *progress >= 1.0 {
                    let cur = *current;
                    self.highlight = Some(cur);
                    if let Some(next_idx) = self.nodes[cur].next {
                        self.animation = ListAnimation::Traversing {
                            current: next_idx,
                            progress: 0.0,
                        };
                    } else {
                        self.animation = ListAnimation::Idle;
                    }
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
    fn test_linked_list_init() {
        let mut demo = LinkedListDemo::default();
        demo.reset(42);
        assert_eq!(demo.size, 4);
        assert!(demo.head.is_some());
    }

    #[test]
    fn test_insert_head() {
        let mut demo = LinkedListDemo::default();
        demo.reset(42);
        let initial_size = demo.size;
        demo.insert_head(99);
        for _ in 0..100 { demo.step(0.016); }
        assert_eq!(demo.size, initial_size + 1);
    }
}
