//! ===============================================================================
//! FILE: stack_problems.rs | LEARN/learn_core/src/demos/problems/stack_problems.rs
//! PURPOSE: Stack/Queue algorithm visualizations
//! MODIFIED: 2026-01-08
//! LAYER: LEARN -> learn_core -> demos -> problems
//! ===============================================================================

use crate::demos::pseudocode::{CodeLine, Pseudocode};
use crate::Demo;

// Static pseudocode for each variant
static VALID_PARENS_CODE: &[CodeLine] = &[
    CodeLine::new("stack = []", 0),
    CodeLine::new("pairs = {')':'(', ']':'[', '}':'{'}", 0),
    CodeLine::new("for char in s:", 0),
    CodeLine::new("if char in '([{':", 1),
    CodeLine::new("stack.push(char)", 2),
    CodeLine::new("else:", 1),
    CodeLine::new("if stack.empty() or stack.pop() != pairs[char]:", 2),
    CodeLine::new("return False", 3),
    CodeLine::new("return stack.empty()", 0),
];

static DAILY_TEMPS_CODE: &[CodeLine] = &[
    CodeLine::new("stack = []  # (index, temp)", 0),
    CodeLine::new("result = [0] * n", 0),
    CodeLine::new("for i, temp in enumerate(temps):", 0),
    CodeLine::new("while stack and temp > stack[-1].temp:", 1),
    CodeLine::new("prev_idx = stack.pop().index", 2),
    CodeLine::new("result[prev_idx] = i - prev_idx", 2),
    CodeLine::new("stack.push((i, temp))", 1),
    CodeLine::new("return result", 0),
];

static RPN_CODE: &[CodeLine] = &[
    CodeLine::new("stack = []", 0),
    CodeLine::new("for token in tokens:", 0),
    CodeLine::new("if token is number:", 1),
    CodeLine::new("stack.push(token)", 2),
    CodeLine::new("else:  # operator", 1),
    CodeLine::new("b = stack.pop()", 2),
    CodeLine::new("a = stack.pop()", 2),
    CodeLine::new("result = apply(a, token, b)", 2),
    CodeLine::new("stack.push(result)", 2),
    CodeLine::new("return stack[0]", 0),
];

static HISTOGRAM_CODE: &[CodeLine] = &[
    CodeLine::new("stack = []  # indices", 0),
    CodeLine::new("max_area = 0", 0),
    CodeLine::new("for i in 0..n+1:", 0),
    CodeLine::new("h = heights[i] if i < n else 0", 1),
    CodeLine::new("while stack and h < heights[stack[-1]]:", 1),
    CodeLine::new("height = heights[stack.pop()]", 2),
    CodeLine::new("width = i if !stack else i - stack[-1] - 1", 2),
    CodeLine::new("max_area = max(max_area, height * width)", 2),
    CodeLine::new("stack.push(i)", 1),
    CodeLine::new("return max_area", 0),
];

/// Animation state for stack/queue problems
#[derive(Clone, Debug, Default)]
pub struct StackProblemsDemo {
    /// Input string or array
    pub input: String,
    pub arr: Vec<i32>,
    /// Main stack
    pub stack: Vec<StackItem>,
    /// Secondary stack (for queue implementation)
    pub stack2: Vec<i32>,
    /// Current position in input
    pub pos: usize,
    /// Result array (for temperatures, histogram, etc.)
    pub result: Vec<i32>,
    /// Is valid (for parentheses)
    pub is_valid: Option<bool>,
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
    pub variant: StackProblemVariant,
    /// Highlight indices
    pub highlights: Vec<usize>,
}

#[derive(Clone, Debug)]
pub struct StackItem {
    pub value: i32,
    pub index: usize,
    pub char_val: Option<char>,
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum StackProblemVariant {
    #[default]
    ValidParentheses,
    ReversePolishNotation,
    DailyTemperatures,
    QueueUsingStacks,
    LargestRectangleHistogram,
}

impl StackProblemsDemo {
    pub fn new(variant: StackProblemVariant) -> Self {
        let mut demo = Self {
            variant,
            ..Default::default()
        };
        demo.reset(42);
        demo
    }

    fn setup_valid_parentheses(&mut self) {
        self.input = "{[()]}".to_string();
        self.stack.clear();
        self.pos = 0;
        self.is_valid = None;
        self.pseudocode = Pseudocode::new("Valid Parentheses", VALID_PARENS_CODE);
        self.pseudocode.current_line = Some(0);
        self.message = format!("Check if \"{}\" has valid parentheses", self.input);
    }

    fn setup_daily_temperatures(&mut self) {
        self.arr = vec![73, 74, 75, 71, 69, 72, 76, 73];
        self.result = vec![0; self.arr.len()];
        self.stack.clear();
        self.pos = 0;
        self.pseudocode = Pseudocode::new("Daily Temperatures", DAILY_TEMPS_CODE);
        self.pseudocode.current_line = Some(0);
        self.message = "Find days until warmer temperature".to_string();
    }

    fn setup_rpn(&mut self) {
        // Represents: (2 + 3) * 4 = 20
        self.input = "2 3 + 4 *".to_string();
        self.stack.clear();
        self.pos = 0;
        self.pseudocode = Pseudocode::new("Reverse Polish Notation", RPN_CODE);
        self.pseudocode.current_line = Some(0);
        self.message = "Evaluate: 2 3 + 4 * = (2+3)*4".to_string();
    }

    fn setup_histogram(&mut self) {
        self.arr = vec![2, 1, 5, 6, 2, 3];
        self.result = vec![0];
        self.stack.clear();
        self.pos = 0;
        self.pseudocode = Pseudocode::new("Largest Rectangle in Histogram", HISTOGRAM_CODE);
        self.pseudocode.current_line = Some(0);
        self.message = "Find largest rectangle in histogram".to_string();
    }

    /// Advance algorithm one step
    pub fn step_algorithm(&mut self) {
        if self.complete {
            return;
        }

        match self.variant {
            StackProblemVariant::ValidParentheses => self.step_valid_parentheses(),
            StackProblemVariant::DailyTemperatures => self.step_daily_temperatures(),
            StackProblemVariant::ReversePolishNotation => self.step_rpn(),
            StackProblemVariant::LargestRectangleHistogram => self.step_histogram(),
            StackProblemVariant::QueueUsingStacks => {}
        }

        self.step += 1;
    }

    fn step_valid_parentheses(&mut self) {
        let chars: Vec<char> = self.input.chars().collect();

        if self.pos >= chars.len() {
            self.complete = true;
            self.is_valid = Some(self.stack.is_empty());
            self.pseudocode.current_line = Some(8);
            self.message = if self.stack.is_empty() {
                "Valid! Stack is empty".to_string()
            } else {
                format!("Invalid! {} unclosed brackets", self.stack.len())
            };
            return;
        }

        let c = chars[self.pos];
        self.pseudocode.current_line = Some(3);

        if c == '(' || c == '[' || c == '{' {
            self.stack.push(StackItem {
                value: 0,
                index: self.pos,
                char_val: Some(c),
            });
            self.pseudocode.current_line = Some(4);
            self.message = format!("Push '{}' onto stack", c);
        } else {
            let matching = match c {
                ')' => '(',
                ']' => '[',
                '}' => '{',
                _ => ' ',
            };

            if let Some(top) = self.stack.pop() {
                if top.char_val == Some(matching) {
                    self.pseudocode.current_line = Some(6);
                    self.message = format!("'{}' matches '{}', pop stack", c, matching);
                } else {
                    self.complete = true;
                    self.is_valid = Some(false);
                    self.pseudocode.current_line = Some(7);
                    self.message = format!("Mismatch! '{}' doesn't match '{:?}'", c, top.char_val);
                    return;
                }
            } else {
                self.complete = true;
                self.is_valid = Some(false);
                self.pseudocode.current_line = Some(6);
                self.message = format!("Invalid! No opening bracket for '{}'", c);
                return;
            }
        }

        self.pos += 1;
    }

    fn step_daily_temperatures(&mut self) {
        if self.pos >= self.arr.len() {
            self.complete = true;
            self.pseudocode.current_line = Some(7);
            self.message = format!("Done! Result: {:?}", self.result);
            return;
        }

        let temp = self.arr[self.pos];
        self.pseudocode.current_line = Some(3);

        // Pop all colder temperatures
        while !self.stack.is_empty() && temp > self.stack.last().unwrap().value {
            let prev = self.stack.pop().unwrap();
            self.result[prev.index] = (self.pos - prev.index) as i32;
            self.pseudocode.current_line = Some(5);
            self.message = format!(
                "{} > {}, days until warmer = {}",
                temp,
                prev.value,
                self.pos - prev.index
            );
        }

        self.stack.push(StackItem {
            value: temp,
            index: self.pos,
            char_val: None,
        });
        self.pseudocode.current_line = Some(6);
        if self.message.is_empty() || !self.message.contains(">") {
            self.message = format!("Push temp {} at index {}", temp, self.pos);
        }

        self.pos += 1;
    }

    fn step_rpn(&mut self) {
        let tokens: Vec<&str> = self.input.split_whitespace().collect();

        if self.pos >= tokens.len() {
            self.complete = true;
            self.pseudocode.current_line = Some(9);
            let result = self.stack.first().map(|s| s.value).unwrap_or(0);
            self.message = format!("Result: {}", result);
            return;
        }

        let token = tokens[self.pos];
        self.pseudocode.current_line = Some(2);

        if let Ok(num) = token.parse::<i32>() {
            self.stack.push(StackItem {
                value: num,
                index: self.pos,
                char_val: None,
            });
            self.pseudocode.current_line = Some(3);
            self.message = format!("Push number {}", num);
        } else {
            // Operator
            let b = self.stack.pop().map(|s| s.value).unwrap_or(0);
            let a = self.stack.pop().map(|s| s.value).unwrap_or(0);
            let result = match token {
                "+" => a + b,
                "-" => a - b,
                "*" => a * b,
                "/" => a / b,
                _ => 0,
            };
            self.stack.push(StackItem {
                value: result,
                index: self.pos,
                char_val: None,
            });
            self.pseudocode.current_line = Some(8);
            self.message = format!("{} {} {} = {}", a, token, b, result);
        }

        self.pos += 1;
    }

    fn step_histogram(&mut self) {
        // Process one bar at a time
        let n = self.arr.len();
        let h = if self.pos < n { self.arr[self.pos] } else { 0 };

        if self.pos > n {
            self.complete = true;
            self.pseudocode.current_line = Some(9);
            self.message = format!("Maximum area: {}", self.result[0]);
            return;
        }

        self.pseudocode.current_line = Some(4);

        // Pop while current height is smaller
        while !self.stack.is_empty() && h < self.arr[self.stack.last().unwrap().index] {
            let top = self.stack.pop().unwrap();
            let height = self.arr[top.index];
            let width = if self.stack.is_empty() {
                self.pos as i32
            } else {
                (self.pos - self.stack.last().unwrap().index - 1) as i32
            };
            let area = height * width;
            if area > self.result[0] {
                self.result[0] = area;
            }
            self.pseudocode.current_line = Some(7);
            self.message = format!("Pop height {}, width = {}, area = {}", height, width, area);
        }

        if self.pos <= n {
            self.stack.push(StackItem {
                value: h,
                index: self.pos,
                char_val: None,
            });
            if self.message.is_empty() || !self.message.contains("Pop") {
                self.message = format!("Push index {} (height {})", self.pos, h);
            }
        }

        self.pos += 1;
    }

    /// Get stack contents for rendering
    pub fn get_stack(&self) -> &[StackItem] {
        &self.stack
    }

    /// Get current position
    pub fn get_pos(&self) -> usize {
        self.pos
    }
}

impl Demo for StackProblemsDemo {
    fn reset(&mut self, _seed: u64) {
        self.step = 0;
        self.complete = false;
        self.is_valid = None;
        self.timer = 0.0;
        self.stack.clear();
        self.stack2.clear();
        self.pos = 0;
        self.result.clear();
        self.highlights.clear();

        match self.variant {
            StackProblemVariant::ValidParentheses => self.setup_valid_parentheses(),
            StackProblemVariant::DailyTemperatures => self.setup_daily_temperatures(),
            StackProblemVariant::ReversePolishNotation => self.setup_rpn(),
            StackProblemVariant::LargestRectangleHistogram => self.setup_histogram(),
            StackProblemVariant::QueueUsingStacks => self.setup_valid_parentheses(),
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
