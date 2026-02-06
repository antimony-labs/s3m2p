//! ===============================================================================
//! FILE: graph_demo.rs | LEARN/learn_core/src/demos/graph_demo.rs
//! PURPOSE: Graph visualization with BFS/DFS traversal animations
//! MODIFIED: 2026-01-07
//! LAYER: LEARN -> learn_core -> demos
//! ===============================================================================

use super::pseudocode::{graph as pc_graph, Pseudocode};
use crate::{Demo, ParamMeta, Rng, Vec2};

/// A vertex in the graph
#[derive(Clone, Debug)]
pub struct Vertex {
    pub id: usize,
    pub label: String,
    pub position: Vec2,
    pub state: VertexState,
}

/// Vertex state for traversal visualization
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VertexState {
    Unvisited,
    Discovered, // In queue/stack (frontier)
    Visited,    // Fully processed
    Current,    // Currently being processed
}

/// An edge in the graph
#[derive(Clone, Debug)]
pub struct Edge {
    pub from: usize,
    pub to: usize,
    pub weight: Option<f32>,
    pub highlighted: bool,
}

/// Graph traversal algorithm
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TraversalAlgorithm {
    BFS, // Breadth-first search
    DFS, // Depth-first search
}

/// Animation state for graph operations
#[derive(Clone, Debug, PartialEq)]
pub enum GraphAnimation {
    Idle,
    Traversing {
        algorithm: TraversalAlgorithm,
        frontier: Vec<usize>,
        visited: Vec<usize>,
        current: Option<usize>,
        progress: f32,
    },
    AddingEdge {
        from: usize,
        to: usize,
        progress: f32,
    },
}

/// Graph visualization demo
#[derive(Clone)]
pub struct GraphDemo {
    /// All vertices
    pub vertices: Vec<Vertex>,
    /// All edges (adjacency list style)
    pub edges: Vec<Edge>,
    /// Is graph directed?
    pub directed: bool,
    /// Animation state
    pub animation: GraphAnimation,
    /// Animation speed
    pub speed: f32,
    /// Traversal order (result)
    pub traversal_order: Vec<usize>,
    /// Status message
    pub message: String,
    /// Pseudocode state
    pub pseudocode: Pseudocode,
    /// RNG
    rng: Rng,
}

impl Default for GraphDemo {
    fn default() -> Self {
        Self {
            vertices: Vec::new(),
            edges: Vec::new(),
            directed: false,
            animation: GraphAnimation::Idle,
            speed: 1.0,
            traversal_order: Vec::new(),
            message: String::new(),
            pseudocode: Pseudocode::default(),
            rng: Rng::new(42),
        }
    }
}

impl GraphDemo {
    /// Generate initial graph
    fn generate_initial_data(&mut self) {
        self.vertices.clear();
        self.edges.clear();

        // Create a sample graph with 6 vertices
        let positions = [
            (300.0, 100.0), // 0: A
            (150.0, 200.0), // 1: B
            (450.0, 200.0), // 2: C
            (100.0, 350.0), // 3: D
            (300.0, 350.0), // 4: E
            (500.0, 350.0), // 5: F
        ];

        let labels = ["A", "B", "C", "D", "E", "F"];

        for (i, ((x, y), label)) in positions.iter().zip(labels.iter()).enumerate() {
            self.vertices.push(Vertex {
                id: i,
                label: label.to_string(),
                position: Vec2::new(*x, *y),
                state: VertexState::Unvisited,
            });
        }

        // Add edges (undirected graph)
        let edges = [
            (0, 1),
            (0, 2), // A-B, A-C
            (1, 3),
            (1, 4), // B-D, B-E
            (2, 4),
            (2, 5), // C-E, C-F
            (3, 4), // D-E
        ];

        for (from, to) in edges {
            self.add_edge_immediate(from, to);
        }
    }

    /// Add edge immediately without animation
    fn add_edge_immediate(&mut self, from: usize, to: usize) {
        self.edges.push(Edge {
            from,
            to,
            weight: None,
            highlighted: false,
        });
        if !self.directed {
            self.edges.push(Edge {
                from: to,
                to: from,
                weight: None,
                highlighted: false,
            });
        }
    }

    /// Get neighbors of a vertex
    pub fn neighbors(&self, vertex: usize) -> Vec<usize> {
        self.edges
            .iter()
            .filter(|e| e.from == vertex)
            .map(|e| e.to)
            .collect()
    }

    /// Clear all vertex/edge states
    fn clear_states(&mut self) {
        for v in &mut self.vertices {
            v.state = VertexState::Unvisited;
        }
        for e in &mut self.edges {
            e.highlighted = false;
        }
        self.traversal_order.clear();
    }

    /// Start BFS traversal
    pub fn bfs(&mut self, start: usize) {
        if start >= self.vertices.len() {
            self.message = "Invalid start vertex".to_string();
            return;
        }

        self.clear_states();
        self.vertices[start].state = VertexState::Discovered;
        self.pseudocode = Pseudocode::new("Breadth-First Search", pc_graph::BFS);

        self.message = format!("BFS from {} - O(V + E)", self.vertices[start].label);
        self.animation = GraphAnimation::Traversing {
            algorithm: TraversalAlgorithm::BFS,
            frontier: vec![start],
            visited: Vec::new(),
            current: None,
            progress: 0.0,
        };
    }

    /// Start DFS traversal
    pub fn dfs(&mut self, start: usize) {
        if start >= self.vertices.len() {
            self.message = "Invalid start vertex".to_string();
            return;
        }

        self.clear_states();
        self.vertices[start].state = VertexState::Discovered;
        self.pseudocode = Pseudocode::new("Depth-First Search", pc_graph::DFS);

        self.message = format!("DFS from {} - O(V + E)", self.vertices[start].label);
        self.animation = GraphAnimation::Traversing {
            algorithm: TraversalAlgorithm::DFS,
            frontier: vec![start],
            visited: Vec::new(),
            current: None,
            progress: 0.0,
        };
    }

    /// Add edge with animation
    pub fn add_edge(&mut self, from: usize, to: usize) {
        if from >= self.vertices.len() || to >= self.vertices.len() {
            self.message = "Invalid vertices".to_string();
            return;
        }

        self.message = format!(
            "Adding edge {}-{}",
            self.vertices[from].label, self.vertices[to].label
        );
        self.animation = GraphAnimation::AddingEdge {
            from,
            to,
            progress: 0.0,
        };
    }

    /// Get traversal order as labels
    pub fn get_traversal_labels(&self) -> Vec<String> {
        self.traversal_order
            .iter()
            .filter_map(|&id| self.vertices.get(id).map(|v| v.label.clone()))
            .collect()
    }

    /// Highlight edge between two vertices
    fn highlight_edge(&mut self, from: usize, to: usize) {
        for edge in &mut self.edges {
            if edge.from == from && edge.to == to {
                edge.highlighted = true;
            }
        }
    }
}

impl Demo for GraphDemo {
    fn reset(&mut self, seed: u64) {
        self.rng = Rng::new(seed);
        self.animation = GraphAnimation::Idle;
        self.traversal_order.clear();
        self.message.clear();
        self.pseudocode.clear();
        self.generate_initial_data();
    }

    fn step(&mut self, dt: f32) {
        let speed = self.speed * dt * 1.5;

        // Extract animation state to avoid borrow conflicts
        let anim = std::mem::replace(&mut self.animation, GraphAnimation::Idle);

        self.animation = match anim {
            GraphAnimation::Idle => GraphAnimation::Idle,
            GraphAnimation::Traversing {
                algorithm,
                mut frontier,
                mut visited,
                current,
                progress,
            } => {
                let new_progress = progress + speed;
                if new_progress >= 1.0 {
                    // Process current vertex
                    if let Some(curr) = current {
                        self.vertices[curr].state = VertexState::Visited;
                        visited.push(curr);
                        self.traversal_order.push(curr);
                        GraphAnimation::Traversing {
                            algorithm,
                            frontier,
                            visited,
                            current: None,
                            progress: 0.0,
                        }
                    } else if !frontier.is_empty() {
                        // Get next vertex from frontier
                        let next = match algorithm {
                            TraversalAlgorithm::BFS => frontier.remove(0), // Queue: front
                            TraversalAlgorithm::DFS => frontier.pop().unwrap(), // Stack: back
                        };

                        // Set as current
                        self.vertices[next].state = VertexState::Current;

                        // Add unvisited neighbors to frontier
                        let neighbors = self.neighbors(next);
                        for &n in &neighbors {
                            if self.vertices[n].state == VertexState::Unvisited {
                                self.vertices[n].state = VertexState::Discovered;
                                frontier.push(n);
                                self.highlight_edge(next, n);
                            }
                        }

                        GraphAnimation::Traversing {
                            algorithm,
                            frontier,
                            visited,
                            current: Some(next),
                            progress: 0.0,
                        }
                    } else {
                        // Traversal complete
                        let labels = self.get_traversal_labels();
                        let alg_name = match algorithm {
                            TraversalAlgorithm::BFS => "BFS",
                            TraversalAlgorithm::DFS => "DFS",
                        };
                        self.message = format!("{}: [{}]", alg_name, labels.join(" → "));
                        GraphAnimation::Idle
                    }
                } else {
                    GraphAnimation::Traversing {
                        algorithm,
                        frontier,
                        visited,
                        current,
                        progress: new_progress,
                    }
                }
            }
            GraphAnimation::AddingEdge { from, to, progress } => {
                let new_progress = progress + speed;
                if new_progress >= 1.0 {
                    self.add_edge_immediate(from, to);
                    self.message = format!(
                        "Added edge {}-{}",
                        self.vertices[from].label, self.vertices[to].label
                    );
                    GraphAnimation::Idle
                } else {
                    GraphAnimation::AddingEdge {
                        from,
                        to,
                        progress: new_progress,
                    }
                }
            }
        };
    }

    fn set_param(&mut self, name: &str, value: f32) -> bool {
        match name {
            "speed" => {
                self.speed = value;
                true
            }
            _ => false,
        }
    }

    fn params() -> &'static [ParamMeta] {
        &[ParamMeta {
            name: "speed",
            label: "Animation Speed",
            min: 0.25,
            max: 4.0,
            step: 0.25,
            default: 1.0,
        }]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_graph_init() {
        let mut demo = GraphDemo::default();
        demo.reset(42);
        assert_eq!(demo.vertices.len(), 6);
        // 7 undirected edges = 14 directed edges
        assert_eq!(demo.edges.len(), 14);
    }

    #[test]
    fn test_neighbors() {
        let mut demo = GraphDemo::default();
        demo.reset(42);

        // A (0) should have neighbors B (1) and C (2)
        let neighbors = demo.neighbors(0);
        assert!(neighbors.contains(&1));
        assert!(neighbors.contains(&2));
    }

    #[test]
    fn test_bfs_visits_all() {
        let mut demo = GraphDemo::default();
        demo.reset(42);

        // Run BFS to completion
        demo.bfs(0);
        for _ in 0..100 {
            demo.step(0.1);
        }

        // All vertices should be visited
        assert_eq!(demo.traversal_order.len(), 6);
    }

    #[test]
    fn test_dfs_visits_all() {
        let mut demo = GraphDemo::default();
        demo.reset(42);

        // Run DFS to completion
        demo.dfs(0);
        for _ in 0..100 {
            demo.step(0.1);
        }

        // All vertices should be visited
        assert_eq!(demo.traversal_order.len(), 6);
    }
}
