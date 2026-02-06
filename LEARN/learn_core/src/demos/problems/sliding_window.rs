//! ===============================================================================
//! FILE: sliding_window.rs | LEARN/learn_core/src/demos/problems/sliding_window.rs
//! PURPOSE: Sliding Window algorithm visualizations
//! MODIFIED: 2026-01-08
//! LAYER: LEARN -> learn_core -> demos -> problems
//! ===============================================================================

use crate::demos::pseudocode::{CodeLine, Pseudocode};
use crate::Demo;
use std::collections::{HashMap, VecDeque};

// Static pseudocode
static MAX_SUM_CODE: &[CodeLine] = &[
    CodeLine::new("window_sum = 0, max_sum = -inf", 0),
    CodeLine::new("for i in 0..n:", 0),
    CodeLine::new("window_sum += arr[i]", 1),
    CodeLine::new("if i >= k - 1:", 1),
    CodeLine::new("max_sum = max(max_sum, window_sum)", 2),
    CodeLine::new("window_sum -= arr[i - k + 1]", 2),
    CodeLine::new("return max_sum", 0),
];

static LONGEST_SUBSTRING_CODE: &[CodeLine] = &[
    CodeLine::new("char_map = {}, start = 0, max_len = 0", 0),
    CodeLine::new("for end in 0..n:", 0),
    CodeLine::new("char = s[end]", 1),
    CodeLine::new("if char in char_map:", 1),
    CodeLine::new("start = max(start, char_map[char] + 1)", 2),
    CodeLine::new("char_map[char] = end", 1),
    CodeLine::new("max_len = max(max_len, end - start + 1)", 1),
    CodeLine::new("return max_len", 0),
];

static SLIDING_MAX_CODE: &[CodeLine] = &[
    CodeLine::new("deque = [], result = []", 0),
    CodeLine::new("for i in 0..n:", 0),
    CodeLine::new("// Remove indices outside window", 1),
    CodeLine::new("while deque and deque[0] < i - k + 1:", 1),
    CodeLine::new("deque.pop_front()", 2),
    CodeLine::new("// Remove smaller elements", 1),
    CodeLine::new("while deque and arr[deque[-1]] < arr[i]:", 1),
    CodeLine::new("deque.pop_back()", 2),
    CodeLine::new("deque.append(i)", 1),
    CodeLine::new("if i >= k - 1:", 1),
    CodeLine::new("result.append(arr[deque[0]])", 2),
    CodeLine::new("return result", 0),
];

/// Animation state for sliding window problems
#[derive(Clone, Debug, Default)]
pub struct SlidingWindowDemo {
    pub arr: Vec<i32>,
    pub input_str: String,
    pub window_start: usize,
    pub window_end: usize,
    pub k: usize,
    pub window_sum: i32,
    pub best_result: i32,
    pub step: usize,
    pub complete: bool,
    pub message: String,
    pub pseudocode: Pseudocode,
    pub timer: f32,
    pub variant: SlidingWindowVariant,
    pub char_freq: HashMap<char, usize>,
    pub deque: VecDeque<usize>,
    pub results: Vec<i32>,
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum SlidingWindowVariant {
    #[default]
    MaxSumSubarrayK,
    LongestSubstringNoRepeat,
    MinWindowSubstring,
    PermutationInString,
    SlidingWindowMaximum,
}

impl SlidingWindowDemo {
    pub fn new(variant: SlidingWindowVariant) -> Self {
        let mut demo = Self {
            variant,
            ..Default::default()
        };
        demo.reset(42);
        demo
    }

    fn setup_max_sum_k(&mut self) {
        self.arr = vec![2, 1, 5, 1, 3, 2, 8, 1, 3];
        self.k = 3;
        self.window_start = 0;
        self.window_end = 0;
        self.window_sum = 0;
        self.best_result = i32::MIN;
        self.pseudocode = Pseudocode::new("Max Sum Subarray of Size K", MAX_SUM_CODE);
        self.pseudocode.current_line = Some(0);
        self.message = format!("Find max sum of contiguous subarray of size {}", self.k);
    }

    fn setup_longest_substring(&mut self) {
        self.input_str = "abcabcbb".to_string();
        self.arr.clear();
        self.window_start = 0;
        self.window_end = 0;
        self.best_result = 0;
        self.char_freq.clear();
        self.pseudocode = Pseudocode::new(
            "Longest Substring Without Repeating",
            LONGEST_SUBSTRING_CODE,
        );
        self.pseudocode.current_line = Some(0);
        self.message = "Find longest substring without repeating characters".to_string();
    }

    fn setup_sliding_max(&mut self) {
        self.arr = vec![1, 3, -1, -3, 5, 3, 6, 7];
        self.k = 3;
        self.window_start = 0;
        self.window_end = 0;
        self.deque.clear();
        self.results.clear();
        self.pseudocode = Pseudocode::new("Sliding Window Maximum", SLIDING_MAX_CODE);
        self.pseudocode.current_line = Some(0);
        self.message = format!("Find max in each window of size {}", self.k);
    }

    pub fn step_algorithm(&mut self) {
        if self.complete {
            return;
        }

        match self.variant {
            SlidingWindowVariant::MaxSumSubarrayK => self.step_max_sum_k(),
            SlidingWindowVariant::LongestSubstringNoRepeat => self.step_longest_substring(),
            SlidingWindowVariant::SlidingWindowMaximum => self.step_sliding_max(),
            _ => {}
        }

        self.step += 1;
    }

    fn step_max_sum_k(&mut self) {
        if self.window_end >= self.arr.len() {
            self.complete = true;
            self.pseudocode.current_line = Some(6);
            self.message = format!("Maximum sum: {}", self.best_result);
            return;
        }

        self.window_sum += self.arr[self.window_end];
        self.pseudocode.current_line = Some(2);
        self.message = format!(
            "Add arr[{}] = {}, window_sum = {}",
            self.window_end, self.arr[self.window_end], self.window_sum
        );

        if self.window_end >= self.k - 1 {
            if self.window_sum > self.best_result {
                self.best_result = self.window_sum;
                self.pseudocode.current_line = Some(4);
                self.message = format!(
                    "New max! Window [{}-{}] sum = {}",
                    self.window_start, self.window_end, self.window_sum
                );
            }
            self.window_sum -= self.arr[self.window_start];
            self.window_start += 1;
        }

        self.window_end += 1;
    }

    fn step_longest_substring(&mut self) {
        let chars: Vec<char> = self.input_str.chars().collect();

        if self.window_end >= chars.len() {
            self.complete = true;
            self.pseudocode.current_line = Some(7);
            self.message = format!("Longest length: {}", self.best_result);
            return;
        }

        let c = chars[self.window_end];
        self.pseudocode.current_line = Some(2);

        if let Some(&prev_idx) = self
            .char_freq
            .get(&c)
            .filter(|&&idx| idx >= self.window_start)
        {
            self.window_start = prev_idx + 1;
            self.pseudocode.current_line = Some(4);
            self.message = format!(
                "'{}' seen at {}, move start to {}",
                c, prev_idx, self.window_start
            );
        } else {
            let len = self.window_end - self.window_start + 1;
            if len as i32 > self.best_result {
                self.best_result = len as i32;
            }
            self.pseudocode.current_line = Some(6);
            self.message = format!(
                "Add '{}', window = \"{}\", length = {}",
                c,
                &self.input_str[self.window_start..=self.window_end],
                len
            );
        }

        self.char_freq.insert(c, self.window_end);
        self.window_end += 1;
    }

    fn step_sliding_max(&mut self) {
        if self.window_end >= self.arr.len() {
            self.complete = true;
            self.pseudocode.current_line = Some(11);
            self.message = format!("Results: {:?}", self.results);
            return;
        }

        while !self.deque.is_empty()
            && self.deque.front().copied().unwrap_or(0) + self.k <= self.window_end
        {
            self.deque.pop_front();
            self.pseudocode.current_line = Some(4);
        }

        while !self.deque.is_empty()
            && self.arr[*self.deque.back().unwrap()] < self.arr[self.window_end]
        {
            self.deque.pop_back();
            self.pseudocode.current_line = Some(7);
        }

        self.deque.push_back(self.window_end);
        self.pseudocode.current_line = Some(8);

        if self.window_end >= self.k - 1 {
            let max_val = self.arr[*self.deque.front().unwrap()];
            self.results.push(max_val);
            self.window_start = self.window_end - self.k + 1;
            self.pseudocode.current_line = Some(10);
            self.message = format!(
                "Window [{}-{}], max = {}",
                self.window_start, self.window_end, max_val
            );
        } else {
            self.message = format!(
                "Building window, added arr[{}] = {}",
                self.window_end, self.arr[self.window_end]
            );
        }

        self.window_end += 1;
    }

    pub fn get_window(&self) -> (usize, usize) {
        (self.window_start, self.window_end.saturating_sub(1))
    }

    pub fn is_in_window(&self, idx: usize) -> bool {
        idx >= self.window_start && idx < self.window_end
    }
}

impl Demo for SlidingWindowDemo {
    fn reset(&mut self, _seed: u64) {
        self.step = 0;
        self.complete = false;
        self.timer = 0.0;
        self.window_sum = 0;
        self.best_result = 0;
        self.char_freq.clear();
        self.deque.clear();
        self.results.clear();

        match self.variant {
            SlidingWindowVariant::MaxSumSubarrayK => self.setup_max_sum_k(),
            SlidingWindowVariant::LongestSubstringNoRepeat => self.setup_longest_substring(),
            SlidingWindowVariant::SlidingWindowMaximum => self.setup_sliding_max(),
            _ => self.setup_max_sum_k(),
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
