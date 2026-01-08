//! ===============================================================================
//! FILE: two_pointers.rs | LEARN/learn_core/src/demos/problems/two_pointers.rs
//! PURPOSE: Two Pointers algorithm visualizations
//! MODIFIED: 2026-01-08
//! LAYER: LEARN -> learn_core -> demos -> problems
//! ===============================================================================

use crate::Demo;
use crate::demos::pseudocode::{Pseudocode, CodeLine};

// Static pseudocode for each variant
static TWO_SUM_CODE: &[CodeLine] = &[
    CodeLine::new("left = 0, right = n-1", 0),
    CodeLine::new("while left < right:", 0),
    CodeLine::new("sum = arr[left] + arr[right]", 1),
    CodeLine::new("if sum == target:", 1),
    CodeLine::new("return (left, right)", 2),
    CodeLine::new("elif sum < target:", 1),
    CodeLine::new("left += 1", 2),
    CodeLine::new("else:", 1),
    CodeLine::new("right -= 1", 2),
    CodeLine::new("return None", 0),
];

static REMOVE_DUPS_CODE: &[CodeLine] = &[
    CodeLine::new("slow = 0", 0),
    CodeLine::new("for fast in 1..n:", 0),
    CodeLine::new("if arr[fast] != arr[slow]:", 1),
    CodeLine::new("slow += 1", 2),
    CodeLine::new("arr[slow] = arr[fast]", 2),
    CodeLine::new("return slow + 1", 0),
];

static CONTAINER_CODE: &[CodeLine] = &[
    CodeLine::new("left = 0, right = n-1", 0),
    CodeLine::new("max_area = 0", 0),
    CodeLine::new("while left < right:", 0),
    CodeLine::new("width = right - left", 1),
    CodeLine::new("height = min(arr[left], arr[right])", 1),
    CodeLine::new("area = width * height", 1),
    CodeLine::new("max_area = max(max_area, area)", 1),
    CodeLine::new("if arr[left] < arr[right]:", 1),
    CodeLine::new("left += 1", 2),
    CodeLine::new("else:", 1),
    CodeLine::new("right -= 1", 2),
    CodeLine::new("return max_area", 0),
];

static RAIN_CODE: &[CodeLine] = &[
    CodeLine::new("left = 0, right = n-1", 0),
    CodeLine::new("left_max = right_max = 0", 0),
    CodeLine::new("water = 0", 0),
    CodeLine::new("while left < right:", 0),
    CodeLine::new("if arr[left] < arr[right]:", 1),
    CodeLine::new("if arr[left] >= left_max:", 2),
    CodeLine::new("left_max = arr[left]", 3),
    CodeLine::new("else:", 2),
    CodeLine::new("water += left_max - arr[left]", 3),
    CodeLine::new("left += 1", 2),
    CodeLine::new("else:", 1),
    CodeLine::new("// similar for right side", 2),
    CodeLine::new("return water", 0),
];

/// Animation state for two pointer problems
#[derive(Clone, Debug, Default)]
pub struct TwoPointersDemo {
    /// The array being processed
    pub arr: Vec<i32>,
    /// Left pointer position
    pub left: usize,
    /// Right pointer position
    pub right: usize,
    /// Target sum (for two sum problems)
    pub target: i32,
    /// Current step in algorithm
    pub step: usize,
    /// Whether algorithm is complete
    pub complete: bool,
    /// Found solution indices
    pub solution: Option<(usize, usize)>,
    /// Status message
    pub message: String,
    /// Pseudocode with current line
    pub pseudocode: Pseudocode,
    /// Animation timer
    pub timer: f32,
    /// Problem variant
    pub variant: TwoPointerVariant,
    /// Current area/water calculated
    pub current_value: i32,
    /// Best value found
    pub best_value: i32,
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum TwoPointerVariant {
    #[default]
    TwoSumSorted,
    RemoveDuplicates,
    ContainerWithMostWater,
    ThreeSum,
    TrappingRainWater,
}

impl TwoPointersDemo {
    pub fn new(variant: TwoPointerVariant) -> Self {
        let mut demo = Self {
            variant,
            ..Default::default()
        };
        demo.reset(42);
        demo
    }

    fn setup_two_sum_sorted(&mut self) {
        self.arr = vec![2, 7, 11, 15, 19, 23, 28];
        self.target = 30;
        self.left = 0;
        self.right = self.arr.len() - 1;
        self.pseudocode = Pseudocode::new("Two Sum II (Sorted)", TWO_SUM_CODE);
        self.pseudocode.current_line = Some(0);
        self.message = format!("Find two numbers that sum to {}", self.target);
    }

    fn setup_remove_duplicates(&mut self) {
        self.arr = vec![1, 1, 2, 2, 2, 3, 4, 4, 5];
        self.left = 0;
        self.right = 1;
        self.pseudocode = Pseudocode::new("Remove Duplicates", REMOVE_DUPS_CODE);
        self.pseudocode.current_line = Some(0);
        self.message = "Remove duplicates in-place".to_string();
    }

    fn setup_container_water(&mut self) {
        self.arr = vec![1, 8, 6, 2, 5, 4, 8, 3, 7];
        self.left = 0;
        self.right = self.arr.len() - 1;
        self.current_value = 0;
        self.best_value = 0;
        self.pseudocode = Pseudocode::new("Container With Most Water", CONTAINER_CODE);
        self.pseudocode.current_line = Some(0);
        self.message = "Find container with most water".to_string();
    }

    fn setup_trapping_rain(&mut self) {
        self.arr = vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1];
        self.left = 0;
        self.right = self.arr.len() - 1;
        self.current_value = 0;
        self.best_value = 0;
        self.pseudocode = Pseudocode::new("Trapping Rain Water", RAIN_CODE);
        self.pseudocode.current_line = Some(0);
        self.message = "Calculate trapped rain water".to_string();
    }

    /// Advance the algorithm one step
    pub fn step_algorithm(&mut self) {
        if self.complete {
            return;
        }

        match self.variant {
            TwoPointerVariant::TwoSumSorted => self.step_two_sum(),
            TwoPointerVariant::RemoveDuplicates => self.step_remove_duplicates(),
            TwoPointerVariant::ContainerWithMostWater => self.step_container_water(),
            TwoPointerVariant::TrappingRainWater => self.step_trapping_rain(),
            TwoPointerVariant::ThreeSum => {}
        }

        self.step += 1;
    }

    fn step_two_sum(&mut self) {
        if self.left >= self.right {
            self.complete = true;
            self.pseudocode.current_line = Some(9);
            self.message = "No solution found".to_string();
            return;
        }

        let sum = self.arr[self.left] + self.arr[self.right];
        self.pseudocode.current_line = Some(2);
        self.message = format!(
            "arr[{}] + arr[{}] = {} + {} = {}",
            self.left, self.right, self.arr[self.left], self.arr[self.right], sum
        );

        if sum == self.target {
            self.solution = Some((self.left, self.right));
            self.complete = true;
            self.pseudocode.current_line = Some(4);
            self.message = format!(
                "Found! {} + {} = {}",
                self.arr[self.left], self.arr[self.right], self.target
            );
        } else if sum < self.target {
            self.pseudocode.current_line = Some(6);
            self.left += 1;
            self.message = format!("{} < {}, move left pointer right", sum, self.target);
        } else {
            self.pseudocode.current_line = Some(8);
            self.right -= 1;
            self.message = format!("{} > {}, move right pointer left", sum, self.target);
        }
    }

    fn step_remove_duplicates(&mut self) {
        if self.right >= self.arr.len() {
            self.complete = true;
            self.pseudocode.current_line = Some(5);
            self.message = format!("Done! New length: {}", self.left + 1);
            return;
        }

        self.pseudocode.current_line = Some(2);
        if self.arr[self.right] != self.arr[self.left] {
            self.left += 1;
            self.arr[self.left] = self.arr[self.right];
            self.pseudocode.current_line = Some(4);
            self.message = format!(
                "Found new value {}, write to position {}",
                self.arr[self.left], self.left
            );
        } else {
            self.message = format!(
                "arr[{}] = {} equals arr[{}], skip",
                self.right, self.arr[self.right], self.left
            );
        }
        self.right += 1;
    }

    fn step_container_water(&mut self) {
        if self.left >= self.right {
            self.complete = true;
            self.pseudocode.current_line = Some(11);
            self.message = format!("Maximum area: {}", self.best_value);
            return;
        }

        let width = (self.right - self.left) as i32;
        let height = self.arr[self.left].min(self.arr[self.right]);
        self.current_value = width * height;
        self.best_value = self.best_value.max(self.current_value);

        self.pseudocode.current_line = Some(6);
        self.message = format!(
            "Width: {}, Height: min({}, {}) = {}, Area: {}",
            width, self.arr[self.left], self.arr[self.right], height, self.current_value
        );

        if self.arr[self.left] < self.arr[self.right] {
            self.left += 1;
        } else {
            self.right -= 1;
        }
    }

    fn step_trapping_rain(&mut self) {
        if self.left >= self.right {
            self.complete = true;
            self.pseudocode.current_line = Some(12);
            self.message = format!("Total water: {}", self.best_value);
            return;
        }

        self.pseudocode.current_line = Some(4);

        if self.arr[self.left] < self.arr[self.right] {
            self.left += 1;
            self.message = format!("Left height {} < right {}, move left",
                self.arr[self.left.saturating_sub(1)], self.arr[self.right]);
        } else {
            self.right -= 1;
            self.message = format!("Right height {} <= left {}, move right",
                self.arr[self.right + 1], self.arr[self.left]);
        }
    }

    /// Check if a position is highlighted
    pub fn is_highlighted(&self, idx: usize) -> bool {
        match self.variant {
            TwoPointerVariant::TwoSumSorted => {
                idx == self.left || idx == self.right || self.solution.map_or(false, |(l, r)| idx == l || idx == r)
            }
            TwoPointerVariant::RemoveDuplicates => {
                idx == self.left || idx == self.right
            }
            TwoPointerVariant::ContainerWithMostWater | TwoPointerVariant::TrappingRainWater => {
                idx >= self.left && idx <= self.right
            }
            TwoPointerVariant::ThreeSum => false,
        }
    }
}

impl Demo for TwoPointersDemo {
    fn reset(&mut self, _seed: u64) {
        self.step = 0;
        self.complete = false;
        self.solution = None;
        self.timer = 0.0;
        self.current_value = 0;
        self.best_value = 0;

        match self.variant {
            TwoPointerVariant::TwoSumSorted => self.setup_two_sum_sorted(),
            TwoPointerVariant::RemoveDuplicates => self.setup_remove_duplicates(),
            TwoPointerVariant::ContainerWithMostWater => self.setup_container_water(),
            TwoPointerVariant::TrappingRainWater => self.setup_trapping_rain(),
            TwoPointerVariant::ThreeSum => self.setup_two_sum_sorted(),
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
