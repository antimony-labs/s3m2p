//! ═══════════════════════════════════════════════════════════════════════════════
//! FILE: grid_world.rs | LEARN/learn_core/src/demos/grid_world.rs
//! PURPOSE: Grid World reinforcement learning demo with Q-learning
//! MODIFIED: 2026-01-02
//! LAYER: LEARN → learn_core → demos
//! ═══════════════════════════════════════════════════════════════════════════════
//!
//! # Grid World RL Demo
//!
//! Interactive reinforcement learning visualization:
//! - Agent learns to navigate a grid world
//! - Q-learning with epsilon-greedy exploration
//! - Value function visualization
//! - Policy arrows showing learned behavior

use crate::{Demo, ParamMeta, Rng};

/// Cell types in the grid world
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Empty,
    Wall,
    Goal,
    Pit,
    Start,
}

impl Cell {
    pub fn reward(&self) -> f32 {
        match self {
            Cell::Goal => 1.0,
            Cell::Pit => -1.0,
            Cell::Empty | Cell::Start => -0.01, // Small negative to encourage efficiency
            Cell::Wall => -0.01,
        }
    }

    pub fn is_terminal(&self) -> bool {
        matches!(self, Cell::Goal | Cell::Pit)
    }
}

/// Actions the agent can take
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Action {
    Up = 0,
    Right = 1,
    Down = 2,
    Left = 3,
}

impl Action {
    pub const ALL: [Action; 4] = [Action::Up, Action::Right, Action::Down, Action::Left];

    pub fn delta(&self) -> (i32, i32) {
        match self {
            Action::Up => (0, -1),
            Action::Right => (1, 0),
            Action::Down => (0, 1),
            Action::Left => (-1, 0),
        }
    }

    pub fn from_index(idx: usize) -> Self {
        match idx % 4 {
            0 => Action::Up,
            1 => Action::Right,
            2 => Action::Down,
            _ => Action::Left,
        }
    }

    pub fn symbol(&self) -> char {
        match self {
            Action::Up => '↑',
            Action::Right => '→',
            Action::Down => '↓',
            Action::Left => '←',
        }
    }
}

/// Grid world layout preset
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GridLayout {
    Simple,
    Maze,
    CliffWalk,
    FourRooms,
}

impl GridLayout {
    pub fn name(&self) -> &'static str {
        match self {
            GridLayout::Simple => "Simple",
            GridLayout::Maze => "Maze",
            GridLayout::CliffWalk => "Cliff Walk",
            GridLayout::FourRooms => "Four Rooms",
        }
    }

    pub fn from_index(idx: usize) -> Self {
        match idx % 4 {
            0 => GridLayout::Simple,
            1 => GridLayout::Maze,
            2 => GridLayout::CliffWalk,
            _ => GridLayout::FourRooms,
        }
    }
}

/// Grid World RL Demo
#[derive(Clone)]
pub struct GridWorldDemo {
    // Grid
    pub grid: Vec<Vec<Cell>>,
    pub width: usize,
    pub height: usize,
    pub layout: GridLayout,

    // Agent state
    pub agent_x: usize,
    pub agent_y: usize,
    pub start_x: usize,
    pub start_y: usize,

    // Q-table: [y][x][action] -> value
    pub q_table: Vec<Vec<[f32; 4]>>,

    // Training parameters
    learning_rate: f32,
    discount: f32,
    pub epsilon: f32,
    epsilon_decay: f32,

    // Statistics
    pub episode: usize,
    pub steps_this_episode: usize,
    pub total_reward: f32,
    pub episode_rewards: Vec<f32>,

    // Visualization
    pub show_values: bool,
    pub show_policy: bool,

    // RNG
    rng: Rng,
}

impl Default for GridWorldDemo {
    fn default() -> Self {
        Self {
            grid: Vec::new(),
            width: 8,
            height: 6,
            layout: GridLayout::Simple,
            agent_x: 0,
            agent_y: 0,
            start_x: 0,
            start_y: 0,
            q_table: Vec::new(),
            learning_rate: 0.1,
            discount: 0.95,
            epsilon: 0.3,
            epsilon_decay: 0.995,
            episode: 0,
            steps_this_episode: 0,
            total_reward: 0.0,
            episode_rewards: Vec::new(),
            show_values: true,
            show_policy: true,
            rng: Rng::new(42),
        }
    }
}

impl GridWorldDemo {
    /// Generate grid based on layout
    fn generate_grid(&mut self) {
        match self.layout {
            GridLayout::Simple => self.generate_simple(),
            GridLayout::Maze => self.generate_maze(),
            GridLayout::CliffWalk => self.generate_cliff_walk(),
            GridLayout::FourRooms => self.generate_four_rooms(),
        }

        // Initialize Q-table
        self.q_table = vec![vec![[0.0; 4]; self.width]; self.height];
    }

    fn generate_simple(&mut self) {
        self.width = 6;
        self.height = 5;
        self.grid = vec![vec![Cell::Empty; self.width]; self.height];

        // Start position
        self.grid[4][0] = Cell::Start;
        self.start_x = 0;
        self.start_y = 4;

        // Goal
        self.grid[0][5] = Cell::Goal;

        // Some walls
        self.grid[1][2] = Cell::Wall;
        self.grid[2][2] = Cell::Wall;
        self.grid[3][2] = Cell::Wall;

        // A pit
        self.grid[2][4] = Cell::Pit;
    }

    fn generate_maze(&mut self) {
        self.width = 8;
        self.height = 6;
        self.grid = vec![vec![Cell::Empty; self.width]; self.height];

        // Start
        self.grid[5][0] = Cell::Start;
        self.start_x = 0;
        self.start_y = 5;

        // Goal
        self.grid[0][7] = Cell::Goal;

        // Maze walls
        let walls = [
            (1, 0), (1, 1), (1, 2),
            (3, 2), (3, 3), (3, 4), (3, 5),
            (1, 4), (1, 5),
            (5, 1), (5, 2), (5, 3),
        ];
        for (y, x) in walls {
            if y < self.height && x < self.width {
                self.grid[y][x] = Cell::Wall;
            }
        }

        // Pits
        self.grid[4][3] = Cell::Pit;
        self.grid[2][6] = Cell::Pit;
    }

    fn generate_cliff_walk(&mut self) {
        self.width = 10;
        self.height = 4;
        self.grid = vec![vec![Cell::Empty; self.width]; self.height];

        // Start
        self.grid[3][0] = Cell::Start;
        self.start_x = 0;
        self.start_y = 3;

        // Goal
        self.grid[3][9] = Cell::Goal;

        // Cliff (pits along bottom)
        for x in 1..9 {
            self.grid[3][x] = Cell::Pit;
        }
    }

    fn generate_four_rooms(&mut self) {
        self.width = 9;
        self.height = 9;
        self.grid = vec![vec![Cell::Empty; self.width]; self.height];

        // Start
        self.grid[7][1] = Cell::Start;
        self.start_x = 1;
        self.start_y = 7;

        // Goal
        self.grid[1][7] = Cell::Goal;

        // Vertical walls
        for y in 0..self.height {
            if y != 2 && y != 6 {
                self.grid[y][4] = Cell::Wall;
            }
        }

        // Horizontal walls
        for x in 0..self.width {
            if x != 2 && x != 6 {
                self.grid[4][x] = Cell::Wall;
            }
        }

        // Some pits
        self.grid[2][2] = Cell::Pit;
        self.grid[6][6] = Cell::Pit;
    }

    /// Reset agent to start position
    fn reset_agent(&mut self) {
        self.agent_x = self.start_x;
        self.agent_y = self.start_y;
        self.steps_this_episode = 0;
        self.total_reward = 0.0;
    }

    /// Check if position is valid
    fn is_valid(&self, x: i32, y: i32) -> bool {
        x >= 0 && y >= 0 &&
        (x as usize) < self.width && (y as usize) < self.height &&
        self.grid[y as usize][x as usize] != Cell::Wall
    }

    /// Take action and return (new_x, new_y, reward, done)
    fn take_action(&mut self, action: Action) -> (usize, usize, f32, bool) {
        let (dx, dy) = action.delta();
        let new_x = self.agent_x as i32 + dx;
        let new_y = self.agent_y as i32 + dy;

        let (final_x, final_y) = if self.is_valid(new_x, new_y) {
            (new_x as usize, new_y as usize)
        } else {
            (self.agent_x, self.agent_y) // Stay in place
        };

        let cell = self.grid[final_y][final_x];
        let reward = cell.reward();
        let done = cell.is_terminal();

        self.agent_x = final_x;
        self.agent_y = final_y;

        (final_x, final_y, reward, done)
    }

    /// Get best action for a state (greedy)
    pub fn best_action(&self, x: usize, y: usize) -> Action {
        let q = &self.q_table[y][x];
        let mut best_idx = 0;
        let mut best_val = q[0];
        for (i, &val) in q.iter().enumerate().skip(1) {
            if val > best_val {
                best_val = val;
                best_idx = i;
            }
        }
        Action::from_index(best_idx)
    }

    /// Get value (max Q) for a state
    pub fn state_value(&self, x: usize, y: usize) -> f32 {
        self.q_table[y][x].iter().cloned().fold(f32::NEG_INFINITY, f32::max)
    }

    /// Select action with epsilon-greedy
    fn select_action(&mut self) -> Action {
        if self.rng.next_f32() < self.epsilon {
            Action::from_index(self.rng.range_int(0, 4) as usize)
        } else {
            self.best_action(self.agent_x, self.agent_y)
        }
    }

    /// Run one step of Q-learning
    fn q_learning_step(&mut self) -> bool {
        let x = self.agent_x;
        let y = self.agent_y;

        // Select and take action
        let action = self.select_action();
        let (new_x, new_y, reward, done) = self.take_action(action);

        // Q-learning update
        let current_q = self.q_table[y][x][action as usize];
        let next_max_q = if done {
            0.0
        } else {
            self.state_value(new_x, new_y)
        };

        let new_q = current_q + self.learning_rate * (reward + self.discount * next_max_q - current_q);
        self.q_table[y][x][action as usize] = new_q;

        self.total_reward += reward;
        self.steps_this_episode += 1;

        if done {
            self.episode_rewards.push(self.total_reward);
            if self.episode_rewards.len() > 100 {
                self.episode_rewards.remove(0);
            }
            self.episode += 1;
            self.epsilon *= self.epsilon_decay;
            self.reset_agent();
        }

        done
    }

    /// Get normalized values for visualization (0-1 range)
    pub fn normalized_values(&self) -> Vec<Vec<f32>> {
        let mut min_v = f32::MAX;
        let mut max_v = f32::MIN;

        for y in 0..self.height {
            for x in 0..self.width {
                if self.grid[y][x] != Cell::Wall {
                    let v = self.state_value(x, y);
                    if v.is_finite() {
                        min_v = min_v.min(v);
                        max_v = max_v.max(v);
                    }
                }
            }
        }

        let range = (max_v - min_v).max(0.01);

        (0..self.height).map(|y| {
            (0..self.width).map(|x| {
                if self.grid[y][x] == Cell::Wall {
                    0.5
                } else {
                    let v = self.state_value(x, y);
                    if v.is_finite() {
                        (v - min_v) / range
                    } else {
                        0.5
                    }
                }
            }).collect()
        }).collect()
    }

    /// Get policy for each cell
    pub fn policy(&self) -> Vec<Vec<Option<Action>>> {
        (0..self.height).map(|y| {
            (0..self.width).map(|x| {
                if self.grid[y][x] == Cell::Wall || self.grid[y][x].is_terminal() {
                    None
                } else {
                    Some(self.best_action(x, y))
                }
            }).collect()
        }).collect()
    }

    /// Average recent reward
    pub fn avg_reward(&self) -> f32 {
        if self.episode_rewards.is_empty() {
            0.0
        } else {
            let recent = self.episode_rewards.iter().rev().take(10);
            recent.clone().sum::<f32>() / recent.count().max(1) as f32
        }
    }
}

impl Demo for GridWorldDemo {
    fn reset(&mut self, seed: u64) {
        self.rng = Rng::new(seed);
        self.episode = 0;
        self.epsilon = 0.3;
        self.episode_rewards.clear();

        self.generate_grid();
        self.reset_agent();
    }

    fn step(&mut self, _dt: f32) {
        // Run multiple steps per frame for faster learning
        for _ in 0..5 {
            self.q_learning_step();
        }
    }

    fn set_param(&mut self, name: &str, value: f32) -> bool {
        match name {
            "learning_rate" => {
                self.learning_rate = value.clamp(0.01, 1.0);
                true
            }
            "discount" => {
                self.discount = value.clamp(0.0, 0.99);
                true
            }
            "epsilon" => {
                self.epsilon = value.clamp(0.0, 1.0);
                true
            }
            "layout" => {
                self.layout = GridLayout::from_index(value as usize);
                self.generate_grid();
                self.reset_agent();
                self.episode = 0;
                self.episode_rewards.clear();
                true
            }
            "show_values" => {
                self.show_values = value > 0.5;
                true
            }
            "show_policy" => {
                self.show_policy = value > 0.5;
                true
            }
            _ => false,
        }
    }

    fn params() -> &'static [ParamMeta] {
        &[
            ParamMeta {
                name: "learning_rate",
                label: "Learning Rate (α)",
                min: 0.01,
                max: 1.0,
                step: 0.05,
                default: 0.1,
            },
            ParamMeta {
                name: "discount",
                label: "Discount (γ)",
                min: 0.0,
                max: 0.99,
                step: 0.05,
                default: 0.95,
            },
            ParamMeta {
                name: "epsilon",
                label: "Exploration (ε)",
                min: 0.0,
                max: 1.0,
                step: 0.05,
                default: 0.3,
            },
            ParamMeta {
                name: "layout",
                label: "Grid Layout",
                min: 0.0,
                max: 3.0,
                step: 1.0,
                default: 0.0,
            },
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_agent_finds_goal() {
        let mut demo = GridWorldDemo::default();
        demo.layout = GridLayout::Simple;
        demo.learning_rate = 0.5;
        demo.reset(42);

        // Train for many episodes
        for _ in 0..10000 {
            demo.step(0.016);
        }

        // Should have completed some episodes
        assert!(demo.episode > 100, "Should complete episodes: {}", demo.episode);

        // Average reward should be positive (reaching goal)
        let avg = demo.avg_reward();
        assert!(avg > 0.0, "Should learn to reach goal: avg={}", avg);
    }

    #[test]
    fn test_grid_generation() {
        let mut demo = GridWorldDemo::default();

        for layout in [GridLayout::Simple, GridLayout::Maze, GridLayout::CliffWalk, GridLayout::FourRooms] {
            demo.layout = layout;
            demo.generate_grid();

            // Should have grid
            assert!(!demo.grid.is_empty());
            assert_eq!(demo.grid.len(), demo.height);
            assert_eq!(demo.grid[0].len(), demo.width);

            // Should have exactly one goal
            let goals: usize = demo.grid.iter().flatten().filter(|&&c| c == Cell::Goal).count();
            assert_eq!(goals, 1, "Layout {:?} should have exactly one goal", layout);
        }
    }

    #[test]
    fn test_wall_blocks_movement() {
        let mut demo = GridWorldDemo::default();
        demo.layout = GridLayout::Simple;
        demo.reset(42);

        // Find a wall position
        let mut wall_pos = None;
        for y in 0..demo.height {
            for x in 0..demo.width {
                if demo.grid[y][x] == Cell::Wall {
                    wall_pos = Some((x, y));
                    break;
                }
            }
        }

        if let Some((wx, wy)) = wall_pos {
            // Try to move into wall from adjacent cell
            demo.agent_x = if wx > 0 { wx - 1 } else { wx + 1 };
            demo.agent_y = wy;

            let old_x = demo.agent_x;
            demo.take_action(if wx > 0 { Action::Right } else { Action::Left });

            // Should not have moved into wall
            assert_ne!(demo.agent_x, wx, "Should not move into wall");
            assert_eq!(demo.agent_x, old_x, "Should stay in place");
        }
    }
}
