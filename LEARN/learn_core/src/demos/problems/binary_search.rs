//! ===============================================================================
//! FILE: binary_search.rs | LEARN/learn_core/src/demos/problems/binary_search.rs
//! PURPOSE: Binary Search algorithm visualizations
//! MODIFIED: 2026-01-08
//! LAYER: LEARN -> learn_core -> demos -> problems
//! ===============================================================================

use crate::demos::pseudocode::{CodeLine, Pseudocode};
use crate::Demo;

// Static pseudocode for each variant
static BASIC_SEARCH_CODE: &[CodeLine] = &[
    CodeLine::new("left = 0, right = n - 1", 0),
    CodeLine::new("while left <= right:", 0),
    CodeLine::new("mid = (left + right) / 2", 1),
    CodeLine::new("if arr[mid] == target:", 1),
    CodeLine::new("return mid", 2),
    CodeLine::new("elif arr[mid] < target:", 1),
    CodeLine::new("left = mid + 1", 2),
    CodeLine::new("else:", 1),
    CodeLine::new("right = mid - 1", 2),
    CodeLine::new("return -1", 0),
];

static ROTATED_ARRAY_CODE: &[CodeLine] = &[
    CodeLine::new("left = 0, right = n - 1", 0),
    CodeLine::new("while left <= right:", 0),
    CodeLine::new("mid = (left + right) / 2", 1),
    CodeLine::new("if arr[mid] == target: return mid", 1),
    CodeLine::new("# Check which half is sorted", 1),
    CodeLine::new("if arr[left] <= arr[mid]:", 1),
    CodeLine::new("# Left half is sorted", 2),
    CodeLine::new("if arr[left] <= target < arr[mid]:", 2),
    CodeLine::new("right = mid - 1", 3),
    CodeLine::new("else:", 2),
    CodeLine::new("left = mid + 1", 3),
    CodeLine::new("else:", 1),
    CodeLine::new("# Right half is sorted", 2),
    CodeLine::new("if arr[mid] < target <= arr[right]:", 2),
    CodeLine::new("left = mid + 1", 3),
    CodeLine::new("else:", 2),
    CodeLine::new("right = mid - 1", 3),
    CodeLine::new("return -1", 0),
];

static FIRST_LAST_CODE: &[CodeLine] = &[
    CodeLine::new("# Find leftmost occurrence", 0),
    CodeLine::new("left_idx = binary_search_left(arr, target)", 0),
    CodeLine::new("if left_idx == -1: return [-1, -1]", 0),
    CodeLine::new("# Find rightmost occurrence", 0),
    CodeLine::new("right_idx = binary_search_right(arr, target)", 0),
    CodeLine::new("return [left_idx, right_idx]", 0),
];

static MATRIX_2D_CODE: &[CodeLine] = &[
    CodeLine::new("row = 0, col = n - 1", 0),
    CodeLine::new("while row < m and col >= 0:", 0),
    CodeLine::new("if matrix[row][col] == target:", 1),
    CodeLine::new("return True", 2),
    CodeLine::new("elif matrix[row][col] > target:", 1),
    CodeLine::new("col -= 1  # Move left", 2),
    CodeLine::new("else:", 1),
    CodeLine::new("row += 1  # Move down", 2),
    CodeLine::new("return False", 0),
];

/// Animation state for binary search problems
#[derive(Clone, Debug, Default)]
pub struct BinarySearchDemo {
    /// The sorted array
    pub arr: Vec<i32>,
    /// 2D matrix (for matrix problems)
    pub matrix: Vec<Vec<i32>>,
    /// Left bound
    pub left: usize,
    /// Right bound
    pub right: usize,
    /// Current middle
    pub mid: usize,
    /// Target value to find
    pub target: i32,
    /// Found index (if any)
    pub found: Option<usize>,
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
    pub variant: BinarySearchVariant,
    /// Rotation point (for rotated array)
    pub rotation_point: usize,
    /// Result for first/last position
    pub range_result: Option<(usize, usize)>,
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum BinarySearchVariant {
    #[default]
    BasicSearch,
    RotatedArray,
    FirstLastPosition,
    Search2DMatrix,
    MedianTwoArrays,
}

impl BinarySearchDemo {
    pub fn new(variant: BinarySearchVariant) -> Self {
        let mut demo = Self {
            variant,
            ..Default::default()
        };
        demo.reset(42);
        demo
    }

    fn setup_basic_search(&mut self) {
        self.arr = vec![2, 5, 8, 12, 16, 23, 38, 56, 72, 91];
        self.target = 23;
        self.left = 0;
        self.right = self.arr.len() - 1;
        self.mid = (self.left + self.right) / 2;
        self.pseudocode = Pseudocode::new("Binary Search", BASIC_SEARCH_CODE);
        self.pseudocode.current_line = Some(0);
        self.message = format!("Search for {} in sorted array", self.target);
    }

    fn setup_rotated_array(&mut self) {
        // Array [0,1,2,4,5,6,7] rotated at pivot 4 -> [4,5,6,7,0,1,2]
        self.arr = vec![4, 5, 6, 7, 0, 1, 2];
        self.rotation_point = 4; // Index where rotation happened
        self.target = 0;
        self.left = 0;
        self.right = self.arr.len() - 1;
        self.mid = (self.left + self.right) / 2;
        self.pseudocode = Pseudocode::new("Search Rotated Sorted Array", ROTATED_ARRAY_CODE);
        self.pseudocode.current_line = Some(0);
        self.message = format!("Search for {} in rotated sorted array", self.target);
    }

    fn setup_first_last(&mut self) {
        self.arr = vec![5, 7, 7, 8, 8, 8, 8, 10];
        self.target = 8;
        self.left = 0;
        self.right = self.arr.len() - 1;
        self.mid = (self.left + self.right) / 2;
        self.range_result = None;
        self.pseudocode = Pseudocode::new("Find First and Last Position", FIRST_LAST_CODE);
        self.pseudocode.current_line = Some(0);
        self.message = format!("Find first and last position of {}", self.target);
    }

    fn setup_2d_matrix(&mut self) {
        self.matrix = vec![
            vec![1, 4, 7, 11],
            vec![2, 5, 8, 12],
            vec![3, 6, 9, 16],
            vec![10, 13, 14, 17],
        ];
        self.target = 5;
        self.left = 0; // row
        self.right = 3; // col (start from top-right)
        self.pseudocode = Pseudocode::new("Search 2D Matrix", MATRIX_2D_CODE);
        self.pseudocode.current_line = Some(0);
        self.message = format!("Search for {} in 2D matrix", self.target);
    }

    /// Advance algorithm one step
    pub fn step_algorithm(&mut self) {
        if self.complete {
            return;
        }

        match self.variant {
            BinarySearchVariant::BasicSearch => self.step_basic_search(),
            BinarySearchVariant::RotatedArray => self.step_rotated_array(),
            BinarySearchVariant::FirstLastPosition => self.step_first_last(),
            BinarySearchVariant::Search2DMatrix => self.step_2d_matrix(),
            BinarySearchVariant::MedianTwoArrays => {}
        }

        self.step += 1;
    }

    fn step_basic_search(&mut self) {
        if self.left > self.right {
            self.complete = true;
            self.pseudocode.current_line = Some(9);
            self.message = format!("Target {} not found", self.target);
            return;
        }

        self.mid = (self.left + self.right) / 2;
        self.pseudocode.current_line = Some(2);
        self.message = format!(
            "mid = ({} + {}) / 2 = {}, arr[{}] = {}",
            self.left, self.right, self.mid, self.mid, self.arr[self.mid]
        );

        if self.arr[self.mid] == self.target {
            self.found = Some(self.mid);
            self.complete = true;
            self.pseudocode.current_line = Some(4);
            self.message = format!("Found {} at index {}", self.target, self.mid);
        } else if self.arr[self.mid] < self.target {
            self.pseudocode.current_line = Some(6);
            self.message = format!(
                "{} < {}, search right half",
                self.arr[self.mid], self.target
            );
            self.left = self.mid + 1;
        } else {
            self.pseudocode.current_line = Some(8);
            self.message = format!("{} > {}, search left half", self.arr[self.mid], self.target);
            self.right = self.mid.saturating_sub(1);
        }
    }

    fn step_rotated_array(&mut self) {
        if self.left > self.right {
            self.complete = true;
            self.pseudocode.current_line = Some(17);
            self.message = format!("Target {} not found", self.target);
            return;
        }

        self.mid = (self.left + self.right) / 2;
        self.pseudocode.current_line = Some(2);

        if self.arr[self.mid] == self.target {
            self.found = Some(self.mid);
            self.complete = true;
            self.pseudocode.current_line = Some(3);
            self.message = format!("Found {} at index {}", self.target, self.mid);
            return;
        }

        // Check which half is sorted
        if self.arr[self.left] <= self.arr[self.mid] {
            // Left half is sorted
            self.pseudocode.current_line = Some(5);
            if self.arr[self.left] <= self.target && self.target < self.arr[self.mid] {
                self.right = self.mid.saturating_sub(1);
                self.message = format!(
                    "Left half sorted, target {} in [{}, {}), go left",
                    self.target, self.arr[self.left], self.arr[self.mid]
                );
            } else {
                self.left = self.mid + 1;
                self.message = format!(
                    "Left half sorted, target {} not in range, go right",
                    self.target
                );
            }
        } else {
            // Right half is sorted
            self.pseudocode.current_line = Some(11);
            if self.arr[self.mid] < self.target && self.target <= self.arr[self.right] {
                self.left = self.mid + 1;
                self.message = format!(
                    "Right half sorted, target {} in ({}, {}], go right",
                    self.target, self.arr[self.mid], self.arr[self.right]
                );
            } else {
                self.right = self.mid.saturating_sub(1);
                self.message = format!(
                    "Right half sorted, target {} not in range, go left",
                    self.target
                );
            }
        }
    }

    fn step_first_last(&mut self) {
        // Simplified: find any occurrence first, then expand
        if self.left > self.right {
            self.complete = true;
            if self.found.is_some() {
                // Find the range
                let idx = self.found.unwrap();
                let mut first = idx;
                let mut last = idx;
                while first > 0 && self.arr[first - 1] == self.target {
                    first -= 1;
                }
                while last < self.arr.len() - 1 && self.arr[last + 1] == self.target {
                    last += 1;
                }
                self.range_result = Some((first, last));
                self.pseudocode.current_line = Some(5);
                self.message = format!("Found {} at positions [{}, {}]", self.target, first, last);
            } else {
                self.pseudocode.current_line = Some(2);
                self.message = format!("Target {} not found", self.target);
            }
            return;
        }

        self.mid = (self.left + self.right) / 2;
        self.pseudocode.current_line = Some(1);

        if self.arr[self.mid] == self.target {
            self.found = Some(self.mid);
            // Continue to find boundaries
            self.message = format!(
                "Found {} at {}, continuing to find range...",
                self.target, self.mid
            );
            self.right = self.mid.saturating_sub(1); // Keep searching left for first occurrence
        } else if self.arr[self.mid] < self.target {
            self.left = self.mid + 1;
            self.message = format!("{} < {}, search right", self.arr[self.mid], self.target);
        } else {
            self.right = self.mid.saturating_sub(1);
            self.message = format!("{} > {}, search left", self.arr[self.mid], self.target);
        }
    }

    fn step_2d_matrix(&mut self) {
        let rows = self.matrix.len();
        let cols = if rows > 0 { self.matrix[0].len() } else { 0 };

        if self.left >= rows || self.right >= cols {
            self.complete = true;
            self.pseudocode.current_line = Some(8);
            self.message = format!("Target {} not found", self.target);
            return;
        }

        let current = self.matrix[self.left][self.right];
        self.pseudocode.current_line = Some(2);
        self.message = format!("At ({}, {}), value = {}", self.left, self.right, current);

        if current == self.target {
            self.found = Some(self.left * cols + self.right);
            self.complete = true;
            self.pseudocode.current_line = Some(3);
            self.message = format!("Found {} at ({}, {})", self.target, self.left, self.right);
        } else if current > self.target {
            self.pseudocode.current_line = Some(5);
            self.right = self.right.saturating_sub(1);
            self.message = format!("{} > {}, move left", current, self.target);
        } else {
            self.pseudocode.current_line = Some(7);
            self.left += 1;
            self.message = format!("{} < {}, move down", current, self.target);
        }
    }

    /// Get search bounds for rendering
    pub fn get_bounds(&self) -> (usize, usize, usize) {
        (self.left, self.right, self.mid)
    }

    /// Check if index is within active search range
    pub fn is_in_range(&self, idx: usize) -> bool {
        idx >= self.left && idx <= self.right
    }

    /// Check if index is the middle element
    pub fn is_mid(&self, idx: usize) -> bool {
        idx == self.mid && !self.complete
    }

    /// Get matrix position for 2D problems
    pub fn get_matrix_pos(&self) -> (usize, usize) {
        (self.left, self.right)
    }
}

impl Demo for BinarySearchDemo {
    fn reset(&mut self, _seed: u64) {
        self.step = 0;
        self.complete = false;
        self.found = None;
        self.timer = 0.0;
        self.range_result = None;

        match self.variant {
            BinarySearchVariant::BasicSearch => self.setup_basic_search(),
            BinarySearchVariant::RotatedArray => self.setup_rotated_array(),
            BinarySearchVariant::FirstLastPosition => self.setup_first_last(),
            BinarySearchVariant::Search2DMatrix => self.setup_2d_matrix(),
            BinarySearchVariant::MedianTwoArrays => self.setup_basic_search(),
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
