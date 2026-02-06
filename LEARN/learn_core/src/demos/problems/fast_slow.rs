//! ===============================================================================
//! FILE: fast_slow.rs | LEARN/learn_core/src/demos/problems/fast_slow.rs
//! PURPOSE: Fast/Slow Pointers algorithm visualizations
//! MODIFIED: 2026-01-08
//! LAYER: LEARN -> learn_core -> demos -> problems
//! ===============================================================================

use crate::demos::pseudocode::{CodeLine, Pseudocode};
use crate::Demo;

// Static pseudocode for each variant
static CYCLE_DETECTION_CODE: &[CodeLine] = &[
    CodeLine::new("slow = head, fast = head", 0),
    CodeLine::new("while fast and fast.next:", 0),
    CodeLine::new("slow = slow.next", 1),
    CodeLine::new("fast = fast.next.next", 1),
    CodeLine::new("if slow == fast:", 1),
    CodeLine::new("return True  # Cycle found", 2),
    CodeLine::new("return False  # No cycle", 0),
];

static FIND_MIDDLE_CODE: &[CodeLine] = &[
    CodeLine::new("slow = head, fast = head", 0),
    CodeLine::new("while fast and fast.next:", 0),
    CodeLine::new("slow = slow.next", 1),
    CodeLine::new("fast = fast.next.next", 1),
    CodeLine::new("return slow  # Middle node", 0),
];

static CYCLE_START_CODE: &[CodeLine] = &[
    CodeLine::new("# First, detect cycle", 0),
    CodeLine::new("slow = fast = head", 0),
    CodeLine::new("while fast and fast.next:", 0),
    CodeLine::new("slow = slow.next", 1),
    CodeLine::new("fast = fast.next.next", 1),
    CodeLine::new("if slow == fast: break", 1),
    CodeLine::new("# Find cycle start", 0),
    CodeLine::new("slow = head", 0),
    CodeLine::new("while slow != fast:", 0),
    CodeLine::new("slow = slow.next", 1),
    CodeLine::new("fast = fast.next", 1),
    CodeLine::new("return slow  # Cycle start", 0),
];

static HAPPY_NUMBER_CODE: &[CodeLine] = &[
    CodeLine::new("def sum_squares(n):", 0),
    CodeLine::new("return sum(int(d)**2 for d in str(n))", 1),
    CodeLine::new("slow = n, fast = n", 0),
    CodeLine::new("while True:", 0),
    CodeLine::new("slow = sum_squares(slow)", 1),
    CodeLine::new("fast = sum_squares(sum_squares(fast))", 1),
    CodeLine::new("if fast == 1: return True", 1),
    CodeLine::new("if slow == fast: return False", 1),
];

/// A node in the linked list visualization
#[derive(Clone, Debug)]
pub struct ListNode {
    pub value: i32,
    pub next: Option<usize>, // Index of next node
}

/// Animation state for fast/slow pointer problems
#[derive(Clone, Debug, Default)]
pub struct FastSlowDemo {
    /// The linked list nodes
    pub nodes: Vec<ListNode>,
    /// Cycle connection point (if any)
    pub cycle_to: Option<usize>,
    /// Slow pointer position
    pub slow: usize,
    /// Fast pointer position
    pub fast: usize,
    /// Current number (for happy number)
    pub current_num: i32,
    pub slow_num: i32,
    pub fast_num: i32,
    /// Current step
    pub step: usize,
    /// Whether complete
    pub complete: bool,
    /// Found cycle/result
    pub found: bool,
    /// Status message
    pub message: String,
    /// Pseudocode
    pub pseudocode: Pseudocode,
    /// Timer
    pub timer: f32,
    /// Problem variant
    pub variant: FastSlowVariant,
    /// Phase (for cycle start: 0 = detection, 1 = finding start)
    pub phase: usize,
    /// Meeting point for cycle start
    pub meet_point: Option<usize>,
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum FastSlowVariant {
    #[default]
    LinkedListCycle,
    FindMiddle,
    CycleStart,
    HappyNumber,
}

impl FastSlowDemo {
    pub fn new(variant: FastSlowVariant) -> Self {
        let mut demo = Self {
            variant,
            ..Default::default()
        };
        demo.reset(42);
        demo
    }

    fn setup_cycle_detection(&mut self) {
        // Create list: 1 -> 2 -> 3 -> 4 -> 5 -> 6 -> (back to 3)
        self.nodes = vec![
            ListNode {
                value: 1,
                next: Some(1),
            },
            ListNode {
                value: 2,
                next: Some(2),
            },
            ListNode {
                value: 3,
                next: Some(3),
            },
            ListNode {
                value: 4,
                next: Some(4),
            },
            ListNode {
                value: 5,
                next: Some(5),
            },
            ListNode {
                value: 6,
                next: Some(2),
            }, // Cycle back to node 2 (value 3)
        ];
        self.cycle_to = Some(2);
        self.slow = 0;
        self.fast = 0;
        self.pseudocode = Pseudocode::new("Linked List Cycle Detection", CYCLE_DETECTION_CODE);
        self.pseudocode.current_line = Some(0);
        self.message = "Detect if linked list has a cycle".to_string();
    }

    fn setup_find_middle(&mut self) {
        // Create list: 1 -> 2 -> 3 -> 4 -> 5 -> 6 -> 7
        self.nodes = vec![
            ListNode {
                value: 1,
                next: Some(1),
            },
            ListNode {
                value: 2,
                next: Some(2),
            },
            ListNode {
                value: 3,
                next: Some(3),
            },
            ListNode {
                value: 4,
                next: Some(4),
            },
            ListNode {
                value: 5,
                next: Some(5),
            },
            ListNode {
                value: 6,
                next: Some(6),
            },
            ListNode {
                value: 7,
                next: None,
            },
        ];
        self.cycle_to = None;
        self.slow = 0;
        self.fast = 0;
        self.pseudocode = Pseudocode::new("Find Middle of Linked List", FIND_MIDDLE_CODE);
        self.pseudocode.current_line = Some(0);
        self.message = "Find the middle node of the list".to_string();
    }

    fn setup_cycle_start(&mut self) {
        // Create list with cycle: 1 -> 2 -> 3 -> 4 -> 5 -> 6 -> (back to 3)
        self.nodes = vec![
            ListNode {
                value: 1,
                next: Some(1),
            },
            ListNode {
                value: 2,
                next: Some(2),
            },
            ListNode {
                value: 3,
                next: Some(3),
            },
            ListNode {
                value: 4,
                next: Some(4),
            },
            ListNode {
                value: 5,
                next: Some(5),
            },
            ListNode {
                value: 6,
                next: Some(2),
            }, // Cycle back to node 2 (value 3)
        ];
        self.cycle_to = Some(2);
        self.slow = 0;
        self.fast = 0;
        self.phase = 0;
        self.meet_point = None;
        self.pseudocode = Pseudocode::new("Find Cycle Start", CYCLE_START_CODE);
        self.pseudocode.current_line = Some(0);
        self.message = "Find where the cycle begins".to_string();
    }

    fn setup_happy_number(&mut self) {
        self.current_num = 19; // 19 is a happy number
        self.slow_num = 19;
        self.fast_num = 19;
        self.pseudocode = Pseudocode::new("Happy Number", HAPPY_NUMBER_CODE);
        self.pseudocode.current_line = Some(2);
        self.message = format!("Check if {} is a happy number", self.current_num);
    }

    fn sum_of_squares(n: i32) -> i32 {
        let mut num = n;
        let mut sum = 0;
        while num > 0 {
            let digit = num % 10;
            sum += digit * digit;
            num /= 10;
        }
        sum
    }

    pub fn step_algorithm(&mut self) {
        if self.complete {
            return;
        }

        match self.variant {
            FastSlowVariant::LinkedListCycle => self.step_cycle_detection(),
            FastSlowVariant::FindMiddle => self.step_find_middle(),
            FastSlowVariant::CycleStart => self.step_cycle_start(),
            FastSlowVariant::HappyNumber => self.step_happy_number(),
        }

        self.step += 1;
    }

    fn step_cycle_detection(&mut self) {
        // Check if fast can move
        let fast_next = self.nodes.get(self.fast).and_then(|n| n.next);
        let fast_next_next = fast_next.and_then(|idx| self.nodes.get(idx).and_then(|n| n.next));

        if fast_next.is_none() || fast_next_next.is_none() {
            self.complete = true;
            self.found = false;
            self.pseudocode.current_line = Some(6);
            self.message = "No cycle found - fast reached end".to_string();
            return;
        }

        // Move pointers
        self.slow = self.nodes[self.slow].next.unwrap_or(self.slow);
        self.fast = fast_next_next.unwrap();

        self.pseudocode.current_line = Some(3);
        self.message = format!(
            "slow at node {} (val {}), fast at node {} (val {})",
            self.slow, self.nodes[self.slow].value, self.fast, self.nodes[self.fast].value
        );

        // Check if they meet
        if self.slow == self.fast {
            self.complete = true;
            self.found = true;
            self.pseudocode.current_line = Some(5);
            self.message = format!(
                "Cycle detected! Pointers met at node {} (val {})",
                self.slow, self.nodes[self.slow].value
            );
        }
    }

    fn step_find_middle(&mut self) {
        // Check if fast can move
        let fast_next = self.nodes.get(self.fast).and_then(|n| n.next);
        let fast_next_next = fast_next.and_then(|idx| self.nodes.get(idx).and_then(|n| n.next));

        if fast_next.is_none() || fast_next_next.is_none() {
            self.complete = true;
            self.found = true;
            self.pseudocode.current_line = Some(4);
            self.message = format!(
                "Middle found! Node {} (val {})",
                self.slow, self.nodes[self.slow].value
            );
            return;
        }

        // Move pointers
        self.slow = self.nodes[self.slow].next.unwrap_or(self.slow);
        self.fast = fast_next_next.unwrap();

        self.pseudocode.current_line = Some(2);
        self.message = format!(
            "slow at node {} (val {}), fast at node {} (val {})",
            self.slow, self.nodes[self.slow].value, self.fast, self.nodes[self.fast].value
        );
    }

    fn step_cycle_start(&mut self) {
        if self.phase == 0 {
            // Phase 0: Detect cycle
            let fast_next = self.nodes.get(self.fast).and_then(|n| n.next);
            let fast_next_next = fast_next.and_then(|idx| self.nodes.get(idx).and_then(|n| n.next));

            self.slow = self.nodes[self.slow].next.unwrap_or(self.slow);
            self.fast = fast_next_next.unwrap_or(self.fast);

            self.pseudocode.current_line = Some(4);
            self.message = format!(
                "Phase 1: slow at {}, fast at {}",
                self.nodes[self.slow].value, self.nodes[self.fast].value
            );

            if self.slow == self.fast {
                self.meet_point = Some(self.slow);
                self.phase = 1;
                self.slow = 0; // Reset slow to head
                self.pseudocode.current_line = Some(7);
                self.message = format!(
                    "Met at node {}! Reset slow to head",
                    self.nodes[self.fast].value
                );
            }
        } else {
            // Phase 1: Find cycle start
            if self.slow == self.fast {
                self.complete = true;
                self.found = true;
                self.pseudocode.current_line = Some(11);
                self.message = format!(
                    "Cycle starts at node {} (val {})",
                    self.slow, self.nodes[self.slow].value
                );
                return;
            }

            self.slow = self.nodes[self.slow].next.unwrap_or(self.slow);
            self.fast = self.nodes[self.fast].next.unwrap_or(self.fast);

            self.pseudocode.current_line = Some(10);
            self.message = format!(
                "Phase 2: slow at {}, fast at {}",
                self.nodes[self.slow].value, self.nodes[self.fast].value
            );
        }
    }

    fn step_happy_number(&mut self) {
        self.slow_num = Self::sum_of_squares(self.slow_num);
        self.fast_num = Self::sum_of_squares(Self::sum_of_squares(self.fast_num));

        self.pseudocode.current_line = Some(5);
        self.message = format!("slow = {}, fast = {}", self.slow_num, self.fast_num);

        if self.fast_num == 1 {
            self.complete = true;
            self.found = true;
            self.pseudocode.current_line = Some(6);
            self.message = format!("{} is a happy number!", self.current_num);
        } else if self.slow_num == self.fast_num {
            self.complete = true;
            self.found = false;
            self.pseudocode.current_line = Some(7);
            self.message = format!(
                "{} is NOT a happy number (cycle detected)",
                self.current_num
            );
        }
    }

    pub fn get_pointers(&self) -> (usize, usize) {
        (self.slow, self.fast)
    }
}

impl Demo for FastSlowDemo {
    fn reset(&mut self, _seed: u64) {
        self.step = 0;
        self.complete = false;
        self.found = false;
        self.timer = 0.0;
        self.phase = 0;
        self.meet_point = None;
        self.nodes.clear();

        match self.variant {
            FastSlowVariant::LinkedListCycle => self.setup_cycle_detection(),
            FastSlowVariant::FindMiddle => self.setup_find_middle(),
            FastSlowVariant::CycleStart => self.setup_cycle_start(),
            FastSlowVariant::HappyNumber => self.setup_happy_number(),
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
