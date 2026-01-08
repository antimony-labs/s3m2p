//! ===============================================================================
//! FILE: balanced_tree_demo.rs | LEARN/learn_core/src/demos/balanced_tree_demo.rs
//! PURPOSE: AVL tree visualization with rotation animations
//! MODIFIED: 2026-01-07
//! LAYER: LEARN -> learn_core -> demos
//! ===============================================================================

use crate::{Demo, ParamMeta, Rng, Vec2};
use super::pseudocode::{Pseudocode, avl as pc_avl};

/// A node in the AVL tree
#[derive(Clone, Debug)]
pub struct AvlNode {
    pub value: i32,
    pub left: Option<usize>,
    pub right: Option<usize>,
    pub height: i32,
    pub position: Vec2,
    pub highlight: HighlightType,
}

/// Highlight type for AVL nodes
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HighlightType {
    None,
    Inserted,
    Rotating,
    Unbalanced,
    Path,
}

/// Rotation type
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RotationType {
    LeftLeft,   // Single right rotation
    RightRight, // Single left rotation
    LeftRight,  // Double: left then right
    RightLeft,  // Double: right then left
}

/// Animation state for AVL operations
#[derive(Clone, Debug, PartialEq)]
pub enum AvlAnimation {
    Idle,
    Inserting { value: i32, path: Vec<usize>, step: usize, progress: f32 },
    CheckingBalance { node: usize, progress: f32 },
    Rotating { node: usize, rotation: RotationType, progress: f32 },
    Rebalancing { path: Vec<usize>, step: usize, progress: f32 },
}

/// AVL tree (balanced BST) visualization demo
#[derive(Clone)]
pub struct BalancedTreeDemo {
    /// All nodes
    pub nodes: Vec<AvlNode>,
    /// Index of root node
    pub root: Option<usize>,
    /// Animation state
    pub animation: AvlAnimation,
    /// Animation speed
    pub speed: f32,
    /// Number of rotations performed
    pub rotation_count: usize,
    /// Last rotation type
    pub last_rotation: Option<RotationType>,
    /// Status message
    pub message: String,
    /// Pseudocode state
    pub pseudocode: Pseudocode,
    /// RNG
    rng: Rng,
}

impl Default for BalancedTreeDemo {
    fn default() -> Self {
        Self {
            nodes: Vec::with_capacity(31),
            root: None,
            animation: AvlAnimation::Idle,
            speed: 1.0,
            rotation_count: 0,
            last_rotation: None,
            message: String::new(),
            pseudocode: Pseudocode::default(),
            rng: Rng::new(42),
        }
    }
}

impl BalancedTreeDemo {
    /// Calculate positions for all nodes
    fn update_positions(&mut self) {
        if let Some(root_idx) = self.root {
            self.position_subtree(root_idx, 400.0, 50.0, 180.0, 0);
        }
    }

    fn position_subtree(&mut self, idx: usize, x: f32, y: f32, spread: f32, depth: usize) {
        if depth > 6 { return; }

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

    /// Generate initial balanced tree
    fn generate_initial_data(&mut self) {
        self.nodes.clear();
        self.root = None;

        // Insert values that create a balanced tree
        let values = [50, 25, 75, 12, 37, 62, 87];
        for &val in &values {
            self.insert_immediate(val);
        }
        self.update_positions();
    }

    /// Get height of a node
    fn height(&self, node: Option<usize>) -> i32 {
        node.map(|idx| self.nodes[idx].height).unwrap_or(0)
    }

    /// Calculate balance factor
    pub fn balance_factor(&self, node: usize) -> i32 {
        let left_h = self.height(self.nodes[node].left);
        let right_h = self.height(self.nodes[node].right);
        left_h - right_h
    }

    /// Update height of a node
    fn update_height(&mut self, node: usize) {
        let left_h = self.height(self.nodes[node].left);
        let right_h = self.height(self.nodes[node].right);
        self.nodes[node].height = 1 + left_h.max(right_h);
    }

    /// Right rotation (for left-left case)
    fn rotate_right(&mut self, y: usize) -> usize {
        let x = self.nodes[y].left.expect("rotate_right needs left child");
        let t2 = self.nodes[x].right;

        // Perform rotation
        self.nodes[x].right = Some(y);
        self.nodes[y].left = t2;

        // Update heights
        self.update_height(y);
        self.update_height(x);

        self.rotation_count += 1;
        x // New root of subtree
    }

    /// Left rotation (for right-right case)
    fn rotate_left(&mut self, x: usize) -> usize {
        let y = self.nodes[x].right.expect("rotate_left needs right child");
        let t2 = self.nodes[y].left;

        // Perform rotation
        self.nodes[y].left = Some(x);
        self.nodes[x].right = t2;

        // Update heights
        self.update_height(x);
        self.update_height(y);

        self.rotation_count += 1;
        y // New root of subtree
    }

    /// Insert and rebalance
    fn insert_immediate(&mut self, value: i32) {
        let new_idx = self.nodes.len();
        let new_node = AvlNode {
            value,
            left: None,
            right: None,
            height: 1,
            position: Vec2::new(400.0, 50.0),
            highlight: HighlightType::None,
        };
        self.nodes.push(new_node);

        if self.root.is_none() {
            self.root = Some(new_idx);
            return;
        }

        self.root = Some(self.insert_recursive(self.root.unwrap(), new_idx));
    }

    fn insert_recursive(&mut self, node: usize, new_idx: usize) -> usize {
        let new_val = self.nodes[new_idx].value;
        let node_val = self.nodes[node].value;

        // Standard BST insert
        if new_val < node_val {
            if let Some(left) = self.nodes[node].left {
                let new_left = self.insert_recursive(left, new_idx);
                self.nodes[node].left = Some(new_left);
            } else {
                self.nodes[node].left = Some(new_idx);
            }
        } else {
            if let Some(right) = self.nodes[node].right {
                let new_right = self.insert_recursive(right, new_idx);
                self.nodes[node].right = Some(new_right);
            } else {
                self.nodes[node].right = Some(new_idx);
            }
        }

        // Update height
        self.update_height(node);

        // Get balance factor
        let balance = self.balance_factor(node);

        // Left Left Case
        if balance > 1 {
            if let Some(left) = self.nodes[node].left {
                if new_val < self.nodes[left].value {
                    return self.rotate_right(node);
                }
                // Left Right Case
                if new_val > self.nodes[left].value {
                    self.nodes[node].left = Some(self.rotate_left(left));
                    return self.rotate_right(node);
                }
            }
        }

        // Right Right Case
        if balance < -1 {
            if let Some(right) = self.nodes[node].right {
                if new_val > self.nodes[right].value {
                    return self.rotate_left(node);
                }
                // Right Left Case
                if new_val < self.nodes[right].value {
                    self.nodes[node].right = Some(self.rotate_right(right));
                    return self.rotate_left(node);
                }
            }
        }

        node
    }

    /// Clear all highlights
    fn clear_highlights(&mut self) {
        for node in &mut self.nodes {
            node.highlight = HighlightType::None;
        }
    }

    /// Start insert animation
    pub fn insert(&mut self, value: i32) {
        if self.nodes.len() >= 15 {
            self.message = "Tree is full (max 15 nodes)".to_string();
            return;
        }

        self.clear_highlights();
        self.rotation_count = 0;
        self.last_rotation = None;
        self.pseudocode = Pseudocode::new("AVL Insert", pc_avl::INSERT);

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
        self.animation = AvlAnimation::Inserting { value, path, step: 0, progress: 0.0 };
    }

    /// Get tree height
    pub fn tree_height(&self) -> i32 {
        self.height(self.root)
    }

    /// Check if tree is balanced
    pub fn is_balanced(&self) -> bool {
        self.check_balanced(self.root)
    }

    fn check_balanced(&self, node: Option<usize>) -> bool {
        match node {
            None => true,
            Some(idx) => {
                let bf = self.balance_factor(idx);
                bf.abs() <= 1
                    && self.check_balanced(self.nodes[idx].left)
                    && self.check_balanced(self.nodes[idx].right)
            }
        }
    }

    /// Get rotation type name
    pub fn rotation_name(rotation: RotationType) -> &'static str {
        match rotation {
            RotationType::LeftLeft => "Right Rotation (LL)",
            RotationType::RightRight => "Left Rotation (RR)",
            RotationType::LeftRight => "Left-Right (LR)",
            RotationType::RightLeft => "Right-Left (RL)",
        }
    }
}

impl Demo for BalancedTreeDemo {
    fn reset(&mut self, seed: u64) {
        self.rng = Rng::new(seed);
        self.animation = AvlAnimation::Idle;
        self.rotation_count = 0;
        self.last_rotation = None;
        self.message.clear();
        self.pseudocode.clear();
        self.generate_initial_data();
    }

    fn step(&mut self, dt: f32) {
        let speed = self.speed * dt * 2.0;

        match &mut self.animation {
            AvlAnimation::Idle => {}
            AvlAnimation::Inserting { value, path, step, progress } => {
                *progress += speed * 0.5;
                if *progress >= 1.0 {
                    if *step < path.len() {
                        // Highlight path nodes
                        let idx = path[*step];
                        self.nodes[idx].highlight = HighlightType::Path;
                        *step += 1;
                        *progress = 0.0;
                    } else {
                        // Insert the new node
                        let val = *value;
                        let old_rotations = self.rotation_count;
                        self.insert_immediate(val);
                        self.update_positions();

                        // Highlight new node
                        let new_idx = self.nodes.len() - 1;
                        self.nodes[new_idx].highlight = HighlightType::Inserted;

                        let rotations = self.rotation_count - old_rotations;
                        if rotations > 0 {
                            self.message = format!("Inserted {} with {} rotation(s)", val, rotations);
                        } else {
                            self.message = format!("Inserted {} (no rebalancing needed)", val);
                        }
                        self.animation = AvlAnimation::Idle;
                    }
                }
            }
            AvlAnimation::CheckingBalance { progress, .. } => {
                *progress += speed;
                if *progress >= 1.0 {
                    self.animation = AvlAnimation::Idle;
                }
            }
            AvlAnimation::Rotating { progress, .. } => {
                *progress += speed;
                if *progress >= 1.0 {
                    self.update_positions();
                    self.animation = AvlAnimation::Idle;
                }
            }
            AvlAnimation::Rebalancing { progress, .. } => {
                *progress += speed;
                if *progress >= 1.0 {
                    self.animation = AvlAnimation::Idle;
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
    fn test_avl_init() {
        let mut demo = BalancedTreeDemo::default();
        demo.reset(42);
        assert!(demo.root.is_some());
        assert_eq!(demo.nodes.len(), 7);
    }

    #[test]
    fn test_avl_balanced() {
        let mut demo = BalancedTreeDemo::default();
        demo.reset(42);
        assert!(demo.is_balanced());
    }

    #[test]
    fn test_balance_factor() {
        let mut demo = BalancedTreeDemo::default();
        demo.reset(42);

        // All nodes should have balance factor in [-1, 1]
        for i in 0..demo.nodes.len() {
            let bf = demo.balance_factor(i);
            assert!(bf >= -1 && bf <= 1, "Node {} has balance factor {}", i, bf);
        }
    }

    #[test]
    fn test_insert_maintains_balance() {
        let mut demo = BalancedTreeDemo::default();
        demo.reset(42);

        // Insert values that would unbalance a regular BST
        demo.insert_immediate(5);
        demo.insert_immediate(3);
        demo.insert_immediate(2);

        assert!(demo.is_balanced());
    }

    #[test]
    fn test_height_calculation() {
        let mut demo = BalancedTreeDemo::default();
        demo.reset(42);

        // Perfect binary tree of 7 nodes has height 3
        let h = demo.tree_height();
        assert!(h <= 3, "Height {} is too large for 7 nodes", h);
    }
}
