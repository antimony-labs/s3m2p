//! ===============================================================================
//! FILE: binary_tree_demo.rs | LEARN/learn_core/src/demos/binary_tree_demo.rs
//! PURPOSE: Binary tree visualization with traversal animations
//! MODIFIED: 2026-01-07
//! LAYER: LEARN -> learn_core -> demos
//! ===============================================================================

use crate::{Demo, ParamMeta, Rng, Vec2};
use super::pseudocode::{Pseudocode, binary_tree as pc_tree};

/// A node in the binary tree
#[derive(Clone, Debug)]
pub struct TreeNode {
    pub value: i32,
    pub left: Option<usize>,   // Index into nodes vector
    pub right: Option<usize>,  // Index into nodes vector
    pub position: Vec2,        // For rendering
    pub highlight: bool,       // Currently highlighted during traversal
}

/// Traversal order types
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TraversalOrder {
    PreOrder,   // Root -> Left -> Right
    InOrder,    // Left -> Root -> Right
    PostOrder,  // Left -> Right -> Root
    LevelOrder, // BFS by level
}

/// Animation state for binary tree operations
#[derive(Clone, Debug, PartialEq)]
pub enum TreeAnimation {
    Idle,
    Inserting { value: i32, progress: f32 },
    Traversing { order: TraversalOrder, step: usize, progress: f32 },
}

/// Binary tree visualization demo
#[derive(Clone)]
pub struct BinaryTreeDemo {
    /// All nodes
    pub nodes: Vec<TreeNode>,
    /// Index of root node
    pub root: Option<usize>,
    /// Animation state
    pub animation: TreeAnimation,
    /// Animation speed
    pub speed: f32,
    /// Current traversal path (indices)
    pub traversal_path: Vec<usize>,
    /// Status message
    pub message: String,
    /// Current pseudocode state
    pub pseudocode: Pseudocode,
    /// RNG
    rng: Rng,
}

impl Default for BinaryTreeDemo {
    fn default() -> Self {
        Self {
            nodes: Vec::with_capacity(31), // Max 5 levels
            root: None,
            animation: TreeAnimation::Idle,
            speed: 1.0,
            traversal_path: Vec::new(),
            message: String::new(),
            pseudocode: Pseudocode::default(),
            rng: Rng::new(42),
        }
    }
}

impl BinaryTreeDemo {
    /// Calculate positions for all nodes (level-based layout)
    fn update_positions(&mut self) {
        if let Some(root_idx) = self.root {
            self.position_subtree(root_idx, 400.0, 50.0, 180.0, 0);
        }
    }

    fn position_subtree(&mut self, idx: usize, x: f32, y: f32, spread: f32, depth: usize) {
        if depth > 5 { return; } // Prevent infinite recursion

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

    /// Generate initial tree
    fn generate_initial_data(&mut self) {
        self.nodes.clear();
        self.root = None;

        // Create a balanced tree with 7 nodes
        let values = [50, 30, 70, 20, 40, 60, 80];
        for &val in &values {
            self.insert_immediate(val);
        }
        self.update_positions();
    }

    /// Insert immediately (no animation)
    fn insert_immediate(&mut self, value: i32) {
        let new_idx = self.nodes.len();
        let new_node = TreeNode {
            value,
            left: None,
            right: None,
            position: Vec2::new(400.0, 50.0),
            highlight: false,
        };
        self.nodes.push(new_node);

        if self.root.is_none() {
            self.root = Some(new_idx);
            return;
        }

        // BST-style insertion for initial data
        let mut current = self.root;
        while let Some(idx) = current {
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
    }

    /// Start insert animation
    pub fn insert(&mut self, value: i32) {
        self.pseudocode = Pseudocode::new("Insert", pc_tree::INSERT);
        if self.nodes.len() >= 15 {
            self.message = "Tree is full (max 15 nodes)".to_string();
            return;
        }
        self.animation = TreeAnimation::Inserting { value, progress: 0.0 };
        self.message = format!("Inserting {} into tree", value);
        self.pseudocode.set_line(0);
    }

    /// Start traversal animation
    pub fn traverse(&mut self, order: TraversalOrder) {
        self.traversal_path = self.compute_traversal(order);
        if self.traversal_path.is_empty() {
            self.message = "Tree is empty!".to_string();
            return;
        }

        // Clear all highlights
        for node in &mut self.nodes {
            node.highlight = false;
        }

        // Set appropriate pseudocode based on traversal order
        match order {
            TraversalOrder::PreOrder => {
                self.pseudocode = Pseudocode::new("Pre-order", pc_tree::PREORDER);
            }
            TraversalOrder::InOrder => {
                self.pseudocode = Pseudocode::new("In-order", pc_tree::INORDER);
            }
            TraversalOrder::PostOrder => {
                self.pseudocode = Pseudocode::new("Post-order", pc_tree::POSTORDER);
            }
            TraversalOrder::LevelOrder => {
                self.pseudocode = Pseudocode::new("Level-order", pc_tree::LEVELORDER);
            }
        }
        self.pseudocode.set_line(0);

        let order_name = match order {
            TraversalOrder::PreOrder => "Pre-order (Root→Left→Right)",
            TraversalOrder::InOrder => "In-order (Left→Root→Right)",
            TraversalOrder::PostOrder => "Post-order (Left→Right→Root)",
            TraversalOrder::LevelOrder => "Level-order (BFS)",
        };
        self.animation = TreeAnimation::Traversing { order, step: 0, progress: 0.0 };
        self.message = format!("{} traversal - O(n)", order_name);
    }

    /// Compute traversal order
    fn compute_traversal(&self, order: TraversalOrder) -> Vec<usize> {
        let mut result = Vec::new();
        match order {
            TraversalOrder::PreOrder => self.preorder(self.root, &mut result),
            TraversalOrder::InOrder => self.inorder(self.root, &mut result),
            TraversalOrder::PostOrder => self.postorder(self.root, &mut result),
            TraversalOrder::LevelOrder => self.levelorder(&mut result),
        }
        result
    }

    fn preorder(&self, node: Option<usize>, result: &mut Vec<usize>) {
        if let Some(idx) = node {
            result.push(idx);
            self.preorder(self.nodes[idx].left, result);
            self.preorder(self.nodes[idx].right, result);
        }
    }

    fn inorder(&self, node: Option<usize>, result: &mut Vec<usize>) {
        if let Some(idx) = node {
            self.inorder(self.nodes[idx].left, result);
            result.push(idx);
            self.inorder(self.nodes[idx].right, result);
        }
    }

    fn postorder(&self, node: Option<usize>, result: &mut Vec<usize>) {
        if let Some(idx) = node {
            self.postorder(self.nodes[idx].left, result);
            self.postorder(self.nodes[idx].right, result);
            result.push(idx);
        }
    }

    fn levelorder(&self, result: &mut Vec<usize>) {
        if self.root.is_none() { return; }

        let mut queue = vec![self.root.unwrap()];
        while !queue.is_empty() {
            let idx = queue.remove(0);
            result.push(idx);
            if let Some(left) = self.nodes[idx].left {
                queue.push(left);
            }
            if let Some(right) = self.nodes[idx].right {
                queue.push(right);
            }
        }
    }

    /// Get traversal values as string
    pub fn get_traversal_values(&self) -> Vec<i32> {
        self.traversal_path.iter()
            .filter_map(|&idx| self.nodes.get(idx).map(|n| n.value))
            .collect()
    }

    /// Get current traversal step
    pub fn current_step(&self) -> Option<usize> {
        match &self.animation {
            TreeAnimation::Traversing { step, .. } => Some(*step),
            _ => None,
        }
    }
}

impl Demo for BinaryTreeDemo {
    fn reset(&mut self, seed: u64) {
        self.rng = Rng::new(seed);
        self.animation = TreeAnimation::Idle;
        self.traversal_path.clear();
        self.message.clear();
        self.pseudocode.clear();
        self.generate_initial_data();
    }

    fn step(&mut self, dt: f32) {
        let speed = self.speed * dt * 2.0;

        match &mut self.animation {
            TreeAnimation::Idle => {}
            TreeAnimation::Inserting { value, progress } => {
                *progress += speed;
                // Update pseudocode line based on progress
                if *progress < 0.2 {
                    self.pseudocode.set_line(1); // newNode = createNode
                } else if *progress < 0.5 {
                    self.pseudocode.set_line(4); // queue.enqueue
                } else if *progress < 0.7 {
                    self.pseudocode.set_line(6); // current = queue.dequeue
                } else {
                    self.pseudocode.set_line(7); // if current.left is null
                }
                if *progress >= 1.0 {
                    let val = *value;
                    self.insert_immediate(val);
                    self.update_positions();
                    self.animation = TreeAnimation::Idle;
                }
            }
            TreeAnimation::Traversing { order, step, progress } => {
                *progress += speed * 0.5;
                // Update pseudocode line based on traversal step
                match order {
                    TraversalOrder::PreOrder => {
                        self.pseudocode.set_line(3); // visit(node)
                    }
                    TraversalOrder::InOrder => {
                        self.pseudocode.set_line(4); // visit(node)
                    }
                    TraversalOrder::PostOrder => {
                        self.pseudocode.set_line(5); // visit(node)
                    }
                    TraversalOrder::LevelOrder => {
                        if *progress < 0.5 {
                            self.pseudocode.set_line(4); // node = queue.dequeue
                        } else {
                            self.pseudocode.set_line(5); // visit(node)
                        }
                    }
                }
                if *progress >= 1.0 {
                    // Highlight current node
                    if *step < self.traversal_path.len() {
                        let idx = self.traversal_path[*step];
                        self.nodes[idx].highlight = true;
                        *step += 1;
                        *progress = 0.0;
                    }

                    if *step >= self.traversal_path.len() {
                        let values: Vec<String> = self.get_traversal_values()
                            .iter()
                            .map(|v| v.to_string())
                            .collect();
                        self.message = format!("Traversal: [{}]", values.join(", "));
                        self.animation = TreeAnimation::Idle;
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
    fn test_tree_init() {
        let mut demo = BinaryTreeDemo::default();
        demo.reset(42);
        assert!(demo.root.is_some());
        assert_eq!(demo.nodes.len(), 7);
    }

    #[test]
    fn test_traversal_orders() {
        let mut demo = BinaryTreeDemo::default();
        demo.reset(42);

        // Test that all traversals visit all nodes
        for order in [TraversalOrder::PreOrder, TraversalOrder::InOrder,
                      TraversalOrder::PostOrder, TraversalOrder::LevelOrder] {
            let path = demo.compute_traversal(order);
            assert_eq!(path.len(), 7);
        }
    }

    #[test]
    fn test_inorder_sorted() {
        let mut demo = BinaryTreeDemo::default();
        demo.reset(42);

        demo.traversal_path = demo.compute_traversal(TraversalOrder::InOrder);
        let values = demo.get_traversal_values();

        // In-order traversal of BST should be sorted
        let mut sorted = values.clone();
        sorted.sort();
        assert_eq!(values, sorted);
    }
}
