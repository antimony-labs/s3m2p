//! ===============================================================================
//! FILE: bst_demo.rs | LEARN/learn_core/src/demos/bst_demo.rs
//! PURPOSE: Binary Search Tree visualization with search path animations
//! MODIFIED: 2026-01-07
//! LAYER: LEARN -> learn_core -> demos
//! ===============================================================================

use crate::{Demo, ParamMeta, Rng, Vec2};
use super::pseudocode::{Pseudocode, bst as pc_bst};

/// A node in the BST
#[derive(Clone, Debug)]
pub struct BstNode {
    pub value: i32,
    pub left: Option<usize>,
    pub right: Option<usize>,
    pub position: Vec2,
    pub highlight: HighlightState,
}

/// Highlight state for BST nodes
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HighlightState {
    None,
    Searching,  // Currently being examined
    Found,      // Search target found
    Path,       // Part of search path
    Inserting,  // Being inserted
}

/// Animation state for BST operations
#[derive(Clone, Debug, PartialEq)]
pub enum BstAnimation {
    Idle,
    Searching { target: i32, path: Vec<usize>, step: usize, progress: f32 },
    Inserting { value: i32, path: Vec<usize>, step: usize, progress: f32 },
    Deleting { target: i32, step: usize, progress: f32 },
}

/// Binary Search Tree visualization demo
#[derive(Clone)]
pub struct BstDemo {
    /// All nodes
    pub nodes: Vec<BstNode>,
    /// Index of root node
    pub root: Option<usize>,
    /// Animation state
    pub animation: BstAnimation,
    /// Animation speed
    pub speed: f32,
    /// Comparisons made in current operation
    pub comparisons: usize,
    /// Status message
    pub message: String,
    /// Pseudocode state
    pub pseudocode: Pseudocode,
    /// RNG
    rng: Rng,
}

impl Default for BstDemo {
    fn default() -> Self {
        Self {
            nodes: Vec::with_capacity(31),
            root: None,
            animation: BstAnimation::Idle,
            speed: 1.0,
            comparisons: 0,
            message: String::new(),
            pseudocode: Pseudocode::default(),
            rng: Rng::new(42),
        }
    }
}

impl BstDemo {
    /// Calculate positions for all nodes
    fn update_positions(&mut self) {
        if let Some(root_idx) = self.root {
            self.position_subtree(root_idx, 400.0, 50.0, 180.0, 0);
        }
    }

    fn position_subtree(&mut self, idx: usize, x: f32, y: f32, spread: f32, depth: usize) {
        if depth > 5 { return; }

        self.nodes[idx].position = Vec2::new(x, y);

        let next_spread = spread * 0.55;
        let next_y = y + 70.0;

        if let Some(left_idx) = self.nodes[idx].left {
            self.position_subtree(left_idx, x - spread, next_y, next_spread, depth + 1);
        }
        if let Some(right_idx) = self.nodes[idx].right {
            self.position_subtree(right_idx, x + spread, next_y, next_spread, depth + 1);
        }
    }

    /// Generate initial BST
    fn generate_initial_data(&mut self) {
        self.nodes.clear();
        self.root = None;

        // Create a balanced BST
        let values = [50, 25, 75, 12, 37, 62, 87];
        for &val in &values {
            self.insert_immediate(val);
        }
        self.update_positions();
    }

    /// Insert immediately (no animation)
    fn insert_immediate(&mut self, value: i32) -> Vec<usize> {
        let mut path = Vec::new();
        let new_idx = self.nodes.len();
        let new_node = BstNode {
            value,
            left: None,
            right: None,
            position: Vec2::new(400.0, 50.0),
            highlight: HighlightState::None,
        };
        self.nodes.push(new_node);

        if self.root.is_none() {
            self.root = Some(new_idx);
            return path;
        }

        let mut current = self.root;
        while let Some(idx) = current {
            path.push(idx);
            if value < self.nodes[idx].value {
                if self.nodes[idx].left.is_none() {
                    self.nodes[idx].left = Some(new_idx);
                    break;
                }
                current = self.nodes[idx].left;
            } else {
                if self.nodes[idx].right.is_none() {
                    self.nodes[idx].right = Some(new_idx);
                    break;
                }
                current = self.nodes[idx].right;
            }
        }
        path
    }

    /// Clear all highlights
    fn clear_highlights(&mut self) {
        for node in &mut self.nodes {
            node.highlight = HighlightState::None;
        }
    }

    /// Start search animation
    pub fn search(&mut self, target: i32) {
        self.clear_highlights();
        self.comparisons = 0;
        self.pseudocode = Pseudocode::new("BST Search", pc_bst::SEARCH);

        // Compute search path
        let mut path = Vec::new();
        let mut current = self.root;
        let mut found = false;

        while let Some(idx) = current {
            path.push(idx);
            if target == self.nodes[idx].value {
                found = true;
                break;
            } else if target < self.nodes[idx].value {
                current = self.nodes[idx].left;
            } else {
                current = self.nodes[idx].right;
            }
        }

        if path.is_empty() {
            self.message = "Tree is empty!".to_string();
            return;
        }

        let status = if found { "will find" } else { "not in tree" };
        self.message = format!("Searching for {} ({}) - O(log n)", target, status);
        self.animation = BstAnimation::Searching { target, path, step: 0, progress: 0.0 };
    }

    /// Start insert animation
    pub fn insert(&mut self, value: i32) {
        if self.nodes.len() >= 15 {
            self.message = "Tree is full (max 15 nodes)".to_string();
            return;
        }

        self.clear_highlights();
        self.comparisons = 0;
        self.pseudocode = Pseudocode::new("BST Insert", pc_bst::INSERT);

        // Compute insertion path
        let mut path = Vec::new();
        let mut current = self.root;

        while let Some(idx) = current {
            path.push(idx);
            if value < self.nodes[idx].value {
                current = self.nodes[idx].left;
            } else {
                current = self.nodes[idx].right;
            }
        }

        self.message = format!("Inserting {} - O(log n)", value);
        self.animation = BstAnimation::Inserting { value, path, step: 0, progress: 0.0 };
    }

    /// Get tree height
    pub fn height(&self) -> usize {
        self.subtree_height(self.root)
    }

    fn subtree_height(&self, node: Option<usize>) -> usize {
        match node {
            None => 0,
            Some(idx) => {
                let left_h = self.subtree_height(self.nodes[idx].left);
                let right_h = self.subtree_height(self.nodes[idx].right);
                1 + left_h.max(right_h)
            }
        }
    }

    /// Get minimum value
    pub fn min(&self) -> Option<i32> {
        let mut current = self.root;
        let mut min_val = None;
        while let Some(idx) = current {
            min_val = Some(self.nodes[idx].value);
            current = self.nodes[idx].left;
        }
        min_val
    }

    /// Get maximum value
    pub fn max(&self) -> Option<i32> {
        let mut current = self.root;
        let mut max_val = None;
        while let Some(idx) = current {
            max_val = Some(self.nodes[idx].value);
            current = self.nodes[idx].right;
        }
        max_val
    }
}

impl Demo for BstDemo {
    fn reset(&mut self, seed: u64) {
        self.rng = Rng::new(seed);
        self.animation = BstAnimation::Idle;
        self.comparisons = 0;
        self.message.clear();
        self.pseudocode.clear();
        self.generate_initial_data();
    }

    fn step(&mut self, dt: f32) {
        let speed = self.speed * dt * 2.0;

        match &mut self.animation {
            BstAnimation::Idle => {}
            BstAnimation::Searching { target, path, step, progress } => {
                *progress += speed * 0.5;
                if *progress >= 1.0 {
                    if *step < path.len() {
                        // Highlight current node
                        let idx = path[*step];

                        // Mark previous nodes as path
                        for i in 0..*step {
                            self.nodes[path[i]].highlight = HighlightState::Path;
                        }

                        self.comparisons += 1;

                        if self.nodes[idx].value == *target {
                            self.nodes[idx].highlight = HighlightState::Found;
                            self.message = format!("Found {} after {} comparisons", target, self.comparisons);
                            self.animation = BstAnimation::Idle;
                        } else {
                            self.nodes[idx].highlight = HighlightState::Searching;
                            *step += 1;
                            *progress = 0.0;
                        }
                    } else {
                        self.message = format!("{} not found after {} comparisons", target, self.comparisons);
                        self.animation = BstAnimation::Idle;
                    }
                }
            }
            BstAnimation::Inserting { value, path, step, progress } => {
                *progress += speed * 0.5;
                if *progress >= 1.0 {
                    if *step < path.len() {
                        // Highlight path nodes
                        let idx = path[*step];
                        self.nodes[idx].highlight = HighlightState::Path;
                        self.comparisons += 1;
                        *step += 1;
                        *progress = 0.0;
                    } else {
                        // Insert the new node
                        let val = *value;
                        self.insert_immediate(val);
                        self.update_positions();

                        // Highlight the new node
                        let new_idx = self.nodes.len() - 1;
                        self.nodes[new_idx].highlight = HighlightState::Inserting;

                        self.message = format!("Inserted {} after {} comparisons", val, self.comparisons);
                        self.animation = BstAnimation::Idle;
                    }
                }
            }
            BstAnimation::Deleting { progress, .. } => {
                *progress += speed;
                if *progress >= 1.0 {
                    self.animation = BstAnimation::Idle;
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
    fn test_bst_init() {
        let mut demo = BstDemo::default();
        demo.reset(42);
        assert!(demo.root.is_some());
        assert_eq!(demo.nodes.len(), 7);
    }

    #[test]
    fn test_bst_property() {
        let mut demo = BstDemo::default();
        demo.reset(42);

        // Check BST property: left < parent < right
        fn check_bst(demo: &BstDemo, idx: Option<usize>, min: i32, max: i32) -> bool {
            match idx {
                None => true,
                Some(i) => {
                    let val = demo.nodes[i].value;
                    val > min && val < max
                        && check_bst(demo, demo.nodes[i].left, min, val)
                        && check_bst(demo, demo.nodes[i].right, val, max)
                }
            }
        }

        assert!(check_bst(&demo, demo.root, i32::MIN, i32::MAX));
    }

    #[test]
    fn test_min_max() {
        let mut demo = BstDemo::default();
        demo.reset(42);

        assert_eq!(demo.min(), Some(12));
        assert_eq!(demo.max(), Some(87));
    }
}
