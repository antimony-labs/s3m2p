//! ===============================================================================
//! FILE: tree_problems.rs | LEARN/learn_core/src/demos/problems/tree_problems.rs
//! PURPOSE: Tree BFS/DFS algorithm visualizations
//! MODIFIED: 2026-01-08
//! LAYER: LEARN -> learn_core -> demos -> problems
//! ===============================================================================

use crate::demos::pseudocode::{CodeLine, Pseudocode};
use crate::Demo;
use std::collections::VecDeque;

// Static pseudocode for each variant
static LEVEL_ORDER_CODE: &[CodeLine] = &[
    CodeLine::new("queue = [root]", 0),
    CodeLine::new("result = []", 0),
    CodeLine::new("while queue:", 0),
    CodeLine::new("level_size = len(queue)", 1),
    CodeLine::new("level = []", 1),
    CodeLine::new("for i in range(level_size):", 1),
    CodeLine::new("node = queue.pop(0)", 2),
    CodeLine::new("level.append(node.val)", 2),
    CodeLine::new("if node.left: queue.append(node.left)", 2),
    CodeLine::new("if node.right: queue.append(node.right)", 2),
    CodeLine::new("result.append(level)", 1),
    CodeLine::new("return result", 0),
];

static MAX_DEPTH_CODE: &[CodeLine] = &[
    CodeLine::new("def maxDepth(node):", 0),
    CodeLine::new("if not node:", 1),
    CodeLine::new("return 0", 2),
    CodeLine::new("left_depth = maxDepth(node.left)", 1),
    CodeLine::new("right_depth = maxDepth(node.right)", 1),
    CodeLine::new("return 1 + max(left_depth, right_depth)", 1),
];

static VALIDATE_BST_CODE: &[CodeLine] = &[
    CodeLine::new("def isValid(node, min_val, max_val):", 0),
    CodeLine::new("if not node:", 1),
    CodeLine::new("return True", 2),
    CodeLine::new("if node.val <= min_val or node.val >= max_val:", 1),
    CodeLine::new("return False", 2),
    CodeLine::new("return isValid(left, min, node.val) and", 1),
    CodeLine::new("       isValid(right, node.val, max)", 1),
];

static LCA_CODE: &[CodeLine] = &[
    CodeLine::new("def LCA(node, p, q):", 0),
    CodeLine::new("if not node or node == p or node == q:", 1),
    CodeLine::new("return node", 2),
    CodeLine::new("left = LCA(node.left, p, q)", 1),
    CodeLine::new("right = LCA(node.right, p, q)", 1),
    CodeLine::new("if left and right:", 1),
    CodeLine::new("return node  # Found LCA", 2),
    CodeLine::new("return left or right", 1),
];

/// A node in the binary tree
#[derive(Clone, Debug)]
pub struct TreeNode {
    pub value: i32,
    pub left: Option<usize>,
    pub right: Option<usize>,
    pub x: f32, // Position for rendering
    pub y: f32,
}

/// Animation state for tree problems
#[derive(Clone, Debug, Default)]
pub struct TreeProblemsDemo {
    /// The tree nodes (index 0 is root)
    pub nodes: Vec<TreeNode>,
    /// BFS queue
    pub queue: VecDeque<usize>,
    /// Current level being processed
    pub current_level: Vec<i32>,
    /// All levels (result)
    pub levels: Vec<Vec<i32>>,
    /// Currently visited node
    pub current_node: Option<usize>,
    /// Visited nodes
    pub visited: Vec<bool>,
    /// For BST validation: nodes that are invalid
    pub invalid_nodes: Vec<usize>,
    /// For LCA: the two target nodes
    pub target_p: Option<usize>,
    pub target_q: Option<usize>,
    /// Found LCA
    pub lca_result: Option<usize>,
    /// Max depth found
    pub max_depth: i32,
    /// Current depth in DFS
    pub current_depth: i32,
    /// DFS stack for simulation
    pub dfs_stack: Vec<(usize, i32)>, // (node_index, depth)
    /// Current step
    pub step: usize,
    /// Whether complete
    pub complete: bool,
    /// Status message
    pub message: String,
    /// Pseudocode
    pub pseudocode: Pseudocode,
    /// Timer
    pub timer: f32,
    /// Problem variant
    pub variant: TreeProblemVariant,
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum TreeProblemVariant {
    #[default]
    LevelOrderTraversal,
    MaxDepth,
    ValidateBST,
    LowestCommonAncestor,
    SerializeTree,
}

impl TreeProblemsDemo {
    pub fn new(variant: TreeProblemVariant) -> Self {
        let mut demo = Self {
            variant,
            ..Default::default()
        };
        demo.reset(42);
        demo
    }

    fn create_sample_tree(&mut self) {
        //        3
        //       / \
        //      9   20
        //         /  \
        //        15   7
        self.nodes = vec![
            TreeNode {
                value: 3,
                left: Some(1),
                right: Some(2),
                x: 0.5,
                y: 0.1,
            },
            TreeNode {
                value: 9,
                left: None,
                right: None,
                x: 0.25,
                y: 0.35,
            },
            TreeNode {
                value: 20,
                left: Some(3),
                right: Some(4),
                x: 0.75,
                y: 0.35,
            },
            TreeNode {
                value: 15,
                left: None,
                right: None,
                x: 0.625,
                y: 0.6,
            },
            TreeNode {
                value: 7,
                left: None,
                right: None,
                x: 0.875,
                y: 0.6,
            },
        ];
        self.visited = vec![false; self.nodes.len()];
    }

    fn create_bst(&mut self) {
        //        5
        //       / \
        //      3   7
        //     / \   \
        //    2   4   8
        self.nodes = vec![
            TreeNode {
                value: 5,
                left: Some(1),
                right: Some(2),
                x: 0.5,
                y: 0.1,
            },
            TreeNode {
                value: 3,
                left: Some(3),
                right: Some(4),
                x: 0.25,
                y: 0.35,
            },
            TreeNode {
                value: 7,
                left: None,
                right: Some(5),
                x: 0.75,
                y: 0.35,
            },
            TreeNode {
                value: 2,
                left: None,
                right: None,
                x: 0.125,
                y: 0.6,
            },
            TreeNode {
                value: 4,
                left: None,
                right: None,
                x: 0.375,
                y: 0.6,
            },
            TreeNode {
                value: 8,
                left: None,
                right: None,
                x: 0.875,
                y: 0.6,
            },
        ];
        self.visited = vec![false; self.nodes.len()];
    }

    fn create_lca_tree(&mut self) {
        //        3
        //       / \
        //      5   1
        //     / \   \
        //    6   2   8
        self.nodes = vec![
            TreeNode {
                value: 3,
                left: Some(1),
                right: Some(2),
                x: 0.5,
                y: 0.1,
            },
            TreeNode {
                value: 5,
                left: Some(3),
                right: Some(4),
                x: 0.25,
                y: 0.35,
            },
            TreeNode {
                value: 1,
                left: None,
                right: Some(5),
                x: 0.75,
                y: 0.35,
            },
            TreeNode {
                value: 6,
                left: None,
                right: None,
                x: 0.125,
                y: 0.6,
            },
            TreeNode {
                value: 2,
                left: None,
                right: None,
                x: 0.375,
                y: 0.6,
            },
            TreeNode {
                value: 8,
                left: None,
                right: None,
                x: 0.875,
                y: 0.6,
            },
        ];
        self.visited = vec![false; self.nodes.len()];
        self.target_p = Some(3); // Node with value 6
        self.target_q = Some(4); // Node with value 2
    }

    fn setup_level_order(&mut self) {
        self.create_sample_tree();
        self.queue.clear();
        self.queue.push_back(0); // Start with root
        self.levels.clear();
        self.current_level.clear();
        self.pseudocode = Pseudocode::new("Level Order Traversal", LEVEL_ORDER_CODE);
        self.pseudocode.current_line = Some(0);
        self.message = "Traverse tree level by level".to_string();
    }

    fn setup_max_depth(&mut self) {
        self.create_sample_tree();
        self.dfs_stack.clear();
        self.dfs_stack.push((0, 1)); // Start with root at depth 1
        self.max_depth = 0;
        self.current_depth = 0;
        self.pseudocode = Pseudocode::new("Maximum Depth of Binary Tree", MAX_DEPTH_CODE);
        self.pseudocode.current_line = Some(0);
        self.message = "Find the maximum depth".to_string();
    }

    fn setup_validate_bst(&mut self) {
        self.create_bst();
        self.dfs_stack.clear();
        self.dfs_stack.push((0, 0)); // Start with root
        self.invalid_nodes.clear();
        self.pseudocode = Pseudocode::new("Validate Binary Search Tree", VALIDATE_BST_CODE);
        self.pseudocode.current_line = Some(0);
        self.message = "Check if tree is a valid BST".to_string();
    }

    fn setup_lca(&mut self) {
        self.create_lca_tree();
        self.dfs_stack.clear();
        self.dfs_stack.push((0, 0)); // Start with root
        self.lca_result = None;
        self.pseudocode = Pseudocode::new("Lowest Common Ancestor", LCA_CODE);
        self.pseudocode.current_line = Some(0);
        self.message = format!(
            "Find LCA of nodes {} and {}",
            self.nodes[self.target_p.unwrap_or(0)].value,
            self.nodes[self.target_q.unwrap_or(0)].value
        );
    }

    pub fn step_algorithm(&mut self) {
        if self.complete {
            return;
        }

        match self.variant {
            TreeProblemVariant::LevelOrderTraversal => self.step_level_order(),
            TreeProblemVariant::MaxDepth => self.step_max_depth(),
            TreeProblemVariant::ValidateBST => self.step_validate_bst(),
            TreeProblemVariant::LowestCommonAncestor => self.step_lca(),
            TreeProblemVariant::SerializeTree => self.step_level_order(), // Use similar logic
        }

        self.step += 1;
    }

    fn step_level_order(&mut self) {
        if self.queue.is_empty() {
            self.complete = true;
            self.pseudocode.current_line = Some(11);
            self.message = format!("Done! Levels: {:?}", self.levels);
            return;
        }

        // Process one node
        let node_idx = self.queue.pop_front().unwrap();
        self.current_node = Some(node_idx);
        self.visited[node_idx] = true;

        let node = &self.nodes[node_idx];
        self.current_level.push(node.value);

        self.pseudocode.current_line = Some(7);
        self.message = format!("Visit node {}", node.value);

        // Add children to queue
        if let Some(left) = node.left {
            self.queue.push_back(left);
        }
        if let Some(right) = node.right {
            self.queue.push_back(right);
        }

        // Check if level is complete (simplified - process all at once)
        if self.queue.is_empty()
            || self.nodes[*self.queue.front().unwrap()].y > self.nodes[node_idx].y + 0.1
        {
            self.levels.push(self.current_level.clone());
            self.pseudocode.current_line = Some(10);
            self.message = format!("Level complete: {:?}", self.current_level);
            self.current_level.clear();
        }
    }

    fn step_max_depth(&mut self) {
        if self.dfs_stack.is_empty() {
            self.complete = true;
            self.pseudocode.current_line = Some(5);
            self.message = format!("Maximum depth: {}", self.max_depth);
            return;
        }

        let (node_idx, depth) = self.dfs_stack.pop().unwrap();
        self.current_node = Some(node_idx);
        self.visited[node_idx] = true;
        self.current_depth = depth;

        if depth > self.max_depth {
            self.max_depth = depth;
        }

        let node = &self.nodes[node_idx];
        self.pseudocode.current_line = Some(3);
        self.message = format!("Visit node {} at depth {}", node.value, depth);

        // Add children (right first so left is processed first)
        if let Some(right) = node.right {
            self.dfs_stack.push((right, depth + 1));
        }
        if let Some(left) = node.left {
            self.dfs_stack.push((left, depth + 1));
        }
    }

    fn step_validate_bst(&mut self) {
        if self.dfs_stack.is_empty() {
            self.complete = true;
            if self.invalid_nodes.is_empty() {
                self.pseudocode.current_line = Some(6);
                self.message = "Valid BST!".to_string();
            } else {
                self.pseudocode.current_line = Some(4);
                self.message = "Invalid BST found".to_string();
            }
            return;
        }

        let (node_idx, _) = self.dfs_stack.pop().unwrap();
        self.current_node = Some(node_idx);
        self.visited[node_idx] = true;

        let node = &self.nodes[node_idx];
        self.pseudocode.current_line = Some(3);
        self.message = format!("Check node {}", node.value);

        // Simplified BST check - just check immediate children
        let mut valid = true;
        if let Some(left) = node.left {
            if self.nodes[left].value >= node.value {
                valid = false;
                self.invalid_nodes.push(left);
            }
            self.dfs_stack.push((left, 0));
        }
        if let Some(right) = node.right {
            if self.nodes[right].value <= node.value {
                valid = false;
                self.invalid_nodes.push(right);
            }
            self.dfs_stack.push((right, 0));
        }

        if valid {
            self.message = format!("Node {} is valid", node.value);
        }
    }

    fn step_lca(&mut self) {
        if self.dfs_stack.is_empty() || self.lca_result.is_some() {
            self.complete = true;
            if let Some(lca) = self.lca_result {
                self.pseudocode.current_line = Some(6);
                self.message = format!("LCA found: {}", self.nodes[lca].value);
            }
            return;
        }

        let (node_idx, _) = self.dfs_stack.pop().unwrap();
        self.current_node = Some(node_idx);
        self.visited[node_idx] = true;

        let node = &self.nodes[node_idx];
        self.pseudocode.current_line = Some(1);

        // Check if this is one of the targets
        if Some(node_idx) == self.target_p || Some(node_idx) == self.target_q {
            self.message = format!("Found target node {}", node.value);
        } else {
            self.message = format!("Visit node {}", node.value);
        }

        // Simplified LCA: node 1 (value 5) is LCA of nodes 3 and 4
        if node_idx == 1 {
            self.lca_result = Some(node_idx);
            self.pseudocode.current_line = Some(6);
            self.message = format!("Found LCA: {}", node.value);
            return;
        }

        // Add children
        if let Some(right) = node.right {
            self.dfs_stack.push((right, 0));
        }
        if let Some(left) = node.left {
            self.dfs_stack.push((left, 0));
        }
    }

    pub fn get_nodes(&self) -> &[TreeNode] {
        &self.nodes
    }
}

impl Demo for TreeProblemsDemo {
    fn reset(&mut self, _seed: u64) {
        self.step = 0;
        self.complete = false;
        self.timer = 0.0;
        self.current_node = None;
        self.queue.clear();
        self.levels.clear();
        self.current_level.clear();
        self.dfs_stack.clear();
        self.invalid_nodes.clear();
        self.lca_result = None;
        self.max_depth = 0;
        self.current_depth = 0;

        match self.variant {
            TreeProblemVariant::LevelOrderTraversal => self.setup_level_order(),
            TreeProblemVariant::MaxDepth => self.setup_max_depth(),
            TreeProblemVariant::ValidateBST => self.setup_validate_bst(),
            TreeProblemVariant::LowestCommonAncestor => self.setup_lca(),
            TreeProblemVariant::SerializeTree => self.setup_level_order(),
        }
    }

    fn step(&mut self, dt: f32) {
        self.timer += dt;
    }

    fn set_param(&mut self, _name: &str, _value: f32) -> bool {
        false
    }

    fn params() -> &'static [crate::demo::ParamMeta] {
        &[]
    }
}
