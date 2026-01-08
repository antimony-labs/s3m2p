//! ===============================================================================
//! FILE: graph_problems.rs | LEARN/learn_core/src/demos/problems/graph_problems.rs
//! PURPOSE: Graph algorithm visualizations
//! MODIFIED: 2026-01-08
//! LAYER: LEARN -> learn_core -> demos -> problems
//! ===============================================================================

use crate::Demo;
use crate::demos::pseudocode::{Pseudocode, CodeLine};
use std::collections::VecDeque;

// Static pseudocode for each variant
static NUM_ISLANDS_CODE: &[CodeLine] = &[
    CodeLine::new("count = 0", 0),
    CodeLine::new("for i in range(rows):", 0),
    CodeLine::new("for j in range(cols):", 1),
    CodeLine::new("if grid[i][j] == '1':", 2),
    CodeLine::new("count += 1", 3),
    CodeLine::new("dfs(grid, i, j)  # Mark island", 3),
    CodeLine::new("return count", 0),
];

static CLONE_GRAPH_CODE: &[CodeLine] = &[
    CodeLine::new("visited = {}", 0),
    CodeLine::new("def clone(node):", 0),
    CodeLine::new("if not node: return None", 1),
    CodeLine::new("if node in visited:", 1),
    CodeLine::new("return visited[node]", 2),
    CodeLine::new("copy = Node(node.val)", 1),
    CodeLine::new("visited[node] = copy", 1),
    CodeLine::new("for neighbor in node.neighbors:", 1),
    CodeLine::new("copy.neighbors.append(clone(neighbor))", 2),
    CodeLine::new("return copy", 1),
];

static COURSE_SCHEDULE_CODE: &[CodeLine] = &[
    CodeLine::new("# Build adjacency list", 0),
    CodeLine::new("graph = defaultdict(list)", 0),
    CodeLine::new("in_degree = [0] * numCourses", 0),
    CodeLine::new("# Topological sort (Kahn's)", 0),
    CodeLine::new("queue = [i for i if in_degree[i] == 0]", 0),
    CodeLine::new("completed = 0", 0),
    CodeLine::new("while queue:", 0),
    CodeLine::new("course = queue.pop(0)", 1),
    CodeLine::new("completed += 1", 1),
    CodeLine::new("for next in graph[course]:", 1),
    CodeLine::new("in_degree[next] -= 1", 2),
    CodeLine::new("if in_degree[next] == 0:", 2),
    CodeLine::new("queue.append(next)", 3),
    CodeLine::new("return completed == numCourses", 0),
];

static WORD_LADDER_CODE: &[CodeLine] = &[
    CodeLine::new("queue = [(beginWord, 1)]", 0),
    CodeLine::new("visited = {beginWord}", 0),
    CodeLine::new("while queue:", 0),
    CodeLine::new("word, steps = queue.pop(0)", 1),
    CodeLine::new("if word == endWord:", 1),
    CodeLine::new("return steps", 2),
    CodeLine::new("for i in range(len(word)):", 1),
    CodeLine::new("for c in 'a'..'z':", 2),
    CodeLine::new("new_word = word[:i] + c + word[i+1:]", 3),
    CodeLine::new("if new_word in wordList and not visited:", 3),
    CodeLine::new("queue.append((new_word, steps + 1))", 4),
    CodeLine::new("return 0", 0),
];

/// A cell in the grid (for islands)
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Cell {
    Water,
    Land,
    Visited,
}

/// Animation state for graph problems
#[derive(Clone, Debug, Default)]
pub struct GraphProblemsDemo {
    /// Grid for islands problem
    pub grid: Vec<Vec<Cell>>,
    pub rows: usize,
    pub cols: usize,
    /// Island count
    pub island_count: i32,
    /// Current position
    pub current_pos: Option<(usize, usize)>,
    /// BFS/DFS queue/stack
    pub queue: VecDeque<(usize, usize)>,
    /// For course schedule
    pub courses: Vec<i32>,      // Course numbers
    pub in_degrees: Vec<i32>,   // In-degree for each course
    pub completed: Vec<usize>,  // Completed courses
    pub processing: Option<usize>,
    /// For word ladder
    pub words: Vec<String>,
    pub current_word: String,
    pub target_word: String,
    pub word_path: Vec<String>,
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
    pub variant: GraphProblemVariant,
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum GraphProblemVariant {
    #[default]
    NumberOfIslands,
    CloneGraph,
    CourseSchedule,
    WordLadder,
}

impl GraphProblemsDemo {
    pub fn new(variant: GraphProblemVariant) -> Self {
        let mut demo = Self {
            variant,
            ..Default::default()
        };
        demo.reset(42);
        demo
    }

    fn setup_islands(&mut self) {
        // 4x5 grid with 3 islands
        self.grid = vec![
            vec![Cell::Land, Cell::Land, Cell::Water, Cell::Water, Cell::Water],
            vec![Cell::Land, Cell::Land, Cell::Water, Cell::Water, Cell::Water],
            vec![Cell::Water, Cell::Water, Cell::Land, Cell::Water, Cell::Water],
            vec![Cell::Water, Cell::Water, Cell::Water, Cell::Land, Cell::Land],
        ];
        self.rows = 4;
        self.cols = 5;
        self.island_count = 0;
        self.current_pos = None;
        self.queue.clear();
        self.pseudocode = Pseudocode::new("Number of Islands", NUM_ISLANDS_CODE);
        self.pseudocode.current_line = Some(0);
        self.message = "Count islands in grid".to_string();
    }

    fn setup_clone_graph(&mut self) {
        // Simplified: show concept with adjacency list
        self.courses = vec![1, 2, 3, 4];
        self.in_degrees = vec![0, 0, 0, 0];
        self.completed.clear();
        self.pseudocode = Pseudocode::new("Clone Graph", CLONE_GRAPH_CODE);
        self.pseudocode.current_line = Some(0);
        self.message = "Clone undirected graph".to_string();
    }

    fn setup_course_schedule(&mut self) {
        // 4 courses with prerequisites: 1->0, 2->0, 3->1, 3->2
        // Course 0 has no prereqs, 1 and 2 need 0, 3 needs 1 and 2
        self.courses = vec![0, 1, 2, 3];
        self.in_degrees = vec![0, 1, 1, 2]; // 0:0, 1:1, 2:1, 3:2
        self.completed.clear();
        self.queue.clear();
        // Start with course 0 (no prerequisites)
        self.queue.push_back((0, 0));
        self.processing = None;
        self.pseudocode = Pseudocode::new("Course Schedule", COURSE_SCHEDULE_CODE);
        self.pseudocode.current_line = Some(0);
        self.message = "Can we complete all courses?".to_string();
    }

    fn setup_word_ladder(&mut self) {
        self.current_word = "hit".to_string();
        self.target_word = "cog".to_string();
        self.words = vec!["hot".to_string(), "dot".to_string(), "dog".to_string(),
                          "lot".to_string(), "log".to_string(), "cog".to_string()];
        self.word_path = vec![self.current_word.clone()];
        self.queue.clear();
        self.pseudocode = Pseudocode::new("Word Ladder", WORD_LADDER_CODE);
        self.pseudocode.current_line = Some(0);
        self.message = format!("Transform '{}' to '{}'", self.current_word, self.target_word);
    }

    pub fn step_algorithm(&mut self) {
        if self.complete {
            return;
        }

        match self.variant {
            GraphProblemVariant::NumberOfIslands => self.step_islands(),
            GraphProblemVariant::CloneGraph => self.step_clone_graph(),
            GraphProblemVariant::CourseSchedule => self.step_course_schedule(),
            GraphProblemVariant::WordLadder => self.step_word_ladder(),
        }

        self.step += 1;
    }

    fn step_islands(&mut self) {
        // If we have cells in queue, process them (DFS for current island)
        if let Some((r, c)) = self.queue.pop_front() {
            self.current_pos = Some((r, c));
            self.grid[r][c] = Cell::Visited;
            self.pseudocode.current_line = Some(5);
            self.message = format!("Mark ({}, {}) as visited", r, c);

            // Add unvisited land neighbors
            let directions = [(0i32, 1i32), (0, -1), (1, 0), (-1, 0)];
            for (dr, dc) in directions {
                let nr = r as i32 + dr;
                let nc = c as i32 + dc;
                if nr >= 0 && nr < self.rows as i32 && nc >= 0 && nc < self.cols as i32 {
                    let (nr, nc) = (nr as usize, nc as usize);
                    if self.grid[nr][nc] == Cell::Land {
                        self.queue.push_back((nr, nc));
                    }
                }
            }
            return;
        }

        // Find next unvisited land cell
        for r in 0..self.rows {
            for c in 0..self.cols {
                if self.grid[r][c] == Cell::Land {
                    self.island_count += 1;
                    self.current_pos = Some((r, c));
                    self.queue.push_back((r, c));
                    self.pseudocode.current_line = Some(4);
                    self.message = format!("Found island {} at ({}, {})", self.island_count, r, c);
                    return;
                }
            }
        }

        // No more land cells
        self.complete = true;
        self.pseudocode.current_line = Some(6);
        self.message = format!("Total islands: {}", self.island_count);
    }

    fn step_clone_graph(&mut self) {
        // Simplified visualization of cloning
        if self.completed.len() >= self.courses.len() {
            self.complete = true;
            self.pseudocode.current_line = Some(9);
            self.message = "Graph cloned successfully!".to_string();
            return;
        }

        let node = self.completed.len();
        self.completed.push(node);
        self.processing = Some(node);
        self.pseudocode.current_line = Some(5);
        self.message = format!("Clone node {}", self.courses[node]);
    }

    fn step_course_schedule(&mut self) {
        if self.queue.is_empty() {
            self.complete = true;
            let success = self.completed.len() == self.courses.len();
            self.pseudocode.current_line = Some(13);
            self.message = if success {
                "Can complete all courses!".to_string()
            } else {
                "Cannot complete all courses (cycle detected)".to_string()
            };
            return;
        }

        let (course, _) = self.queue.pop_front().unwrap();
        self.processing = Some(course);
        self.completed.push(course);
        self.pseudocode.current_line = Some(8);
        self.message = format!("Complete course {}", course);

        // Update in-degrees and add newly available courses
        // Simplified: courses 1,2 unlock after 0, course 3 unlocks after 1 and 2
        if course == 0 {
            self.in_degrees[1] -= 1;
            self.in_degrees[2] -= 1;
            if self.in_degrees[1] == 0 {
                self.queue.push_back((1, 0));
            }
            if self.in_degrees[2] == 0 {
                self.queue.push_back((2, 0));
            }
        } else if course == 1 || course == 2 {
            self.in_degrees[3] -= 1;
            if self.in_degrees[3] == 0 {
                self.queue.push_back((3, 0));
            }
        }
    }

    fn step_word_ladder(&mut self) {
        // Pre-defined path: hit -> hot -> dot -> dog -> cog
        let path = ["hit", "hot", "dot", "dog", "cog"];

        if self.word_path.len() >= path.len() {
            self.complete = true;
            self.pseudocode.current_line = Some(5);
            self.message = format!("Found! Path length: {}", self.word_path.len());
            return;
        }

        let next_word = path[self.word_path.len()].to_string();
        let prev_word = self.word_path.last().unwrap().clone();

        // Find the changed character
        let changed_pos = prev_word.chars()
            .zip(next_word.chars())
            .position(|(a, b)| a != b)
            .unwrap_or(0);

        self.current_word = next_word.clone();
        self.word_path.push(next_word.clone());

        self.pseudocode.current_line = Some(8);
        self.message = format!(
            "Change '{}' -> '{}' at position {}",
            prev_word, next_word, changed_pos
        );
    }

    pub fn get_grid(&self) -> &Vec<Vec<Cell>> {
        &self.grid
    }
}

impl Demo for GraphProblemsDemo {
    fn reset(&mut self, _seed: u64) {
        self.step = 0;
        self.complete = false;
        self.timer = 0.0;
        self.queue.clear();
        self.completed.clear();
        self.processing = None;
        self.word_path.clear();
        self.island_count = 0;
        self.current_pos = None;

        match self.variant {
            GraphProblemVariant::NumberOfIslands => self.setup_islands(),
            GraphProblemVariant::CloneGraph => self.setup_clone_graph(),
            GraphProblemVariant::CourseSchedule => self.setup_course_schedule(),
            GraphProblemVariant::WordLadder => self.setup_word_ladder(),
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
