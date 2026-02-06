//! ===============================================================================
//! FILE: dp_problems.rs | LEARN/learn_core/src/demos/problems/dp_problems.rs
//! PURPOSE: Dynamic Programming algorithm visualizations
//! MODIFIED: 2026-01-08
//! LAYER: LEARN -> learn_core -> demos -> problems
//! ===============================================================================

use crate::demos::pseudocode::{CodeLine, Pseudocode};
use crate::Demo;

// Static pseudocode for each variant
static CLIMBING_STAIRS_CODE: &[CodeLine] = &[
    CodeLine::new("if n <= 2: return n", 0),
    CodeLine::new("prev2, prev1 = 1, 2", 0),
    CodeLine::new("for i in 3..n+1:", 0),
    CodeLine::new("curr = prev1 + prev2", 1),
    CodeLine::new("prev2 = prev1", 1),
    CodeLine::new("prev1 = curr", 1),
    CodeLine::new("return prev1", 0),
];

static LIS_CODE: &[CodeLine] = &[
    CodeLine::new("dp = [1] * n  # Each element is LIS of 1", 0),
    CodeLine::new("for i in 1..n:", 0),
    CodeLine::new("for j in 0..i:", 1),
    CodeLine::new("if nums[j] < nums[i]:", 2),
    CodeLine::new("dp[i] = max(dp[i], dp[j] + 1)", 3),
    CodeLine::new("return max(dp)", 0),
];

static COIN_CHANGE_CODE: &[CodeLine] = &[
    CodeLine::new("dp = [inf] * (amount + 1)", 0),
    CodeLine::new("dp[0] = 0", 0),
    CodeLine::new("for coin in coins:", 0),
    CodeLine::new("for x in coin..amount+1:", 1),
    CodeLine::new("dp[x] = min(dp[x], dp[x-coin] + 1)", 2),
    CodeLine::new("return dp[amount] if dp[amount] != inf else -1", 0),
];

/// Animation state for DP problems
#[derive(Clone, Debug, Default)]
pub struct DPProblemsDemo {
    /// DP table
    pub dp: Vec<i32>,
    /// Input array (for LIS)
    pub nums: Vec<i32>,
    /// Coins (for coin change)
    pub coins: Vec<i32>,
    /// Target amount
    pub amount: i32,
    /// Number of stairs
    pub n: i32,
    /// Current position in iteration
    pub i: usize,
    pub j: usize,
    /// Current coin being processed
    pub current_coin: Option<i32>,
    /// For climbing stairs: prev values
    pub prev1: i32,
    pub prev2: i32,
    /// Highlight cells
    pub highlights: Vec<usize>,
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
    pub variant: DPProblemVariant,
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum DPProblemVariant {
    #[default]
    ClimbingStairs,
    LongestIncreasingSubsequence,
    CoinChange,
}

impl DPProblemsDemo {
    pub fn new(variant: DPProblemVariant) -> Self {
        let mut demo = Self {
            variant,
            ..Default::default()
        };
        demo.reset(42);
        demo
    }

    fn setup_climbing_stairs(&mut self) {
        self.n = 6;
        self.prev1 = 2;
        self.prev2 = 1;
        self.i = 3;
        self.dp = vec![0, 1, 2]; // Base cases
        self.pseudocode = Pseudocode::new("Climbing Stairs", CLIMBING_STAIRS_CODE);
        self.pseudocode.current_line = Some(1);
        self.message = format!("Ways to climb {} stairs", self.n);
    }

    fn setup_lis(&mut self) {
        self.nums = vec![10, 9, 2, 5, 3, 7, 101, 18];
        self.dp = vec![1; self.nums.len()];
        self.i = 1;
        self.j = 0;
        self.pseudocode = Pseudocode::new("Longest Increasing Subsequence", LIS_CODE);
        self.pseudocode.current_line = Some(0);
        self.message = "Find LIS length".to_string();
    }

    fn setup_coin_change(&mut self) {
        self.coins = vec![1, 2, 5];
        self.amount = 11;
        self.dp = vec![i32::MAX; (self.amount + 1) as usize];
        self.dp[0] = 0;
        self.i = 0; // Current coin index
        self.j = 0; // Current amount
        self.current_coin = Some(self.coins[0]);
        self.pseudocode = Pseudocode::new("Coin Change", COIN_CHANGE_CODE);
        self.pseudocode.current_line = Some(0);
        self.message = format!("Min coins for amount {}", self.amount);
    }

    pub fn step_algorithm(&mut self) {
        if self.complete {
            return;
        }

        match self.variant {
            DPProblemVariant::ClimbingStairs => self.step_climbing_stairs(),
            DPProblemVariant::LongestIncreasingSubsequence => self.step_lis(),
            DPProblemVariant::CoinChange => self.step_coin_change(),
        }

        self.step += 1;
    }

    fn step_climbing_stairs(&mut self) {
        if self.i > self.n as usize {
            self.complete = true;
            self.pseudocode.current_line = Some(6);
            self.message = format!("Ways to climb {} stairs: {}", self.n, self.prev1);
            return;
        }

        let curr = self.prev1 + self.prev2;
        self.dp.push(curr);
        self.highlights = vec![self.dp.len() - 1];

        self.pseudocode.current_line = Some(3);
        self.message = format!(
            "Step {}: dp[{}] = {} + {} = {}",
            self.i, self.i, self.prev2, self.prev1, curr
        );

        self.prev2 = self.prev1;
        self.prev1 = curr;
        self.i += 1;
    }

    fn step_lis(&mut self) {
        if self.i >= self.nums.len() {
            self.complete = true;
            let max_lis = *self.dp.iter().max().unwrap_or(&1);
            self.pseudocode.current_line = Some(5);
            self.message = format!("LIS length: {}", max_lis);
            return;
        }

        self.highlights = vec![self.i, self.j];

        if self.nums[self.j] < self.nums[self.i] {
            let new_val = self.dp[self.j] + 1;
            if new_val > self.dp[self.i] {
                self.dp[self.i] = new_val;
                self.pseudocode.current_line = Some(4);
                self.message = format!(
                    "nums[{}]={} < nums[{}]={}, dp[{}] = max({}, {}+1) = {}",
                    self.j,
                    self.nums[self.j],
                    self.i,
                    self.nums[self.i],
                    self.i,
                    self.dp[self.i] - 1,
                    self.dp[self.j],
                    self.dp[self.i]
                );
            } else {
                self.pseudocode.current_line = Some(3);
                self.message = format!(
                    "nums[{}]={} < nums[{}]={}, but dp[{}]={} already >= {}",
                    self.j,
                    self.nums[self.j],
                    self.i,
                    self.nums[self.i],
                    self.i,
                    self.dp[self.i],
                    self.dp[self.j] + 1
                );
            }
        } else {
            self.pseudocode.current_line = Some(3);
            self.message = format!(
                "nums[{}]={} >= nums[{}]={}, skip",
                self.j, self.nums[self.j], self.i, self.nums[self.i]
            );
        }

        self.j += 1;
        if self.j >= self.i {
            self.j = 0;
            self.i += 1;
        }
    }

    fn step_coin_change(&mut self) {
        if self.i >= self.coins.len() {
            self.complete = true;
            let result = if self.dp[self.amount as usize] == i32::MAX {
                -1
            } else {
                self.dp[self.amount as usize]
            };
            self.pseudocode.current_line = Some(5);
            self.message = format!("Min coins for {}: {}", self.amount, result);
            return;
        }

        let coin = self.coins[self.i];
        self.current_coin = Some(coin);

        // Process one amount per step
        if self.j as i32 >= coin && self.j <= self.amount as usize {
            let prev = self.dp[self.j - coin as usize];
            if prev != i32::MAX && prev + 1 < self.dp[self.j] {
                self.dp[self.j] = prev + 1;
                self.pseudocode.current_line = Some(4);
                self.message = format!(
                    "Coin {}: dp[{}] = min({}, dp[{}]+1) = {}",
                    coin,
                    self.j,
                    if self.dp[self.j] == i32::MAX {
                        "inf".to_string()
                    } else {
                        self.dp[self.j].to_string()
                    },
                    self.j - coin as usize,
                    self.dp[self.j]
                );
            } else {
                self.pseudocode.current_line = Some(4);
                self.message = format!(
                    "Coin {}: dp[{}] = {} (no improvement)",
                    coin,
                    self.j,
                    if self.dp[self.j] == i32::MAX {
                        "inf".to_string()
                    } else {
                        self.dp[self.j].to_string()
                    }
                );
            }
            self.highlights = vec![self.j];
        }

        self.j += 1;
        if self.j > self.amount as usize {
            self.j = 0;
            self.i += 1;
            if self.i < self.coins.len() {
                self.pseudocode.current_line = Some(2);
                self.message = format!("Now using coin {}", self.coins[self.i]);
            }
        }
    }

    pub fn get_dp(&self) -> &[i32] {
        &self.dp
    }
}

impl Demo for DPProblemsDemo {
    fn reset(&mut self, _seed: u64) {
        self.step = 0;
        self.complete = false;
        self.timer = 0.0;
        self.dp.clear();
        self.highlights.clear();
        self.i = 0;
        self.j = 0;
        self.current_coin = None;

        match self.variant {
            DPProblemVariant::ClimbingStairs => self.setup_climbing_stairs(),
            DPProblemVariant::LongestIncreasingSubsequence => self.setup_lis(),
            DPProblemVariant::CoinChange => self.setup_coin_change(),
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
