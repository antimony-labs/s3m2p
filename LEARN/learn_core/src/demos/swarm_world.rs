//! ═══════════════════════════════════════════════════════════════════════════════
//! FILE: swarm_world.rs | LEARN/learn_core/src/demos/swarm_world.rs
//! PURPOSE: Shared swarm world infrastructure for all swarm demos
//! MODIFIED: 2025-01-XX
//! LAYER: LEARN → learn_core → demos
//! ═══════════════════════════════════════════════════════════════════════════════

use crate::{Rng, Vec2};
use std::collections::HashMap;

/// An agent in the swarm
#[derive(Clone, Debug)]
pub struct Agent {
    pub id: usize,
    pub pos: Vec2,
    pub vel: Vec2,
    pub heading: f32,
    pub value: f32,                   // Generic scalar for consensus demos
    pub health: f32,                  // For robustness demos
    pub assigned_task: Option<usize>, // For allocation demos
}

impl Agent {
    pub fn new(id: usize, pos: Vec2) -> Self {
        Self {
            id,
            pos,
            vel: Vec2::ZERO,
            heading: 0.0,
            value: 0.0,
            health: 1.0,
            assigned_task: None,
        }
    }
}

/// An obstacle in the world
#[derive(Clone, Debug)]
pub struct Obstacle {
    pub center: Vec2,
    pub radius: f32,
}

/// Spatial hash for efficient neighbor search
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Cell {
    x: i32,
    y: i32,
}

/// Shared swarm world infrastructure
pub struct SwarmWorld {
    pub bounds_min: Vec2,
    pub bounds_max: Vec2,
    pub agents: Vec<Agent>,
    pub obstacles: Vec<Obstacle>,
    pub dt: f32,
    pub time: f32,
    pub seed: u64,
    pub rng: Rng,
    cell_size: f32,
    grid: HashMap<Cell, Vec<usize>>,
}

impl SwarmWorld {
    pub fn new(bounds_min: Vec2, bounds_max: Vec2, seed: u64) -> Self {
        let cell_size = 0.1; // Default cell size for neighbor search
        Self {
            bounds_min,
            bounds_max,
            agents: Vec::new(),
            obstacles: Vec::new(),
            dt: 0.016, // ~60 FPS
            time: 0.0,
            seed,
            rng: Rng::new(seed),
            cell_size,
            grid: HashMap::new(),
        }
    }

    /// Add an agent to the world
    pub fn add_agent(&mut self, agent: Agent) {
        self.agents.push(agent);
    }

    /// Add an obstacle to the world
    pub fn add_obstacle(&mut self, obstacle: Obstacle) {
        self.obstacles.push(obstacle);
    }

    /// Build spatial hash grid for neighbor search
    pub fn build_grid(&mut self) {
        self.grid.clear();
        for (i, agent) in self.agents.iter().enumerate() {
            let cell = self.cell_of(agent.pos);
            self.grid.entry(cell).or_default().push(i);
        }
    }

    /// Get cell coordinates for a position
    fn cell_of(&self, pos: Vec2) -> Cell {
        Cell {
            x: (pos.x / self.cell_size).floor() as i32,
            y: (pos.y / self.cell_size).floor() as i32,
        }
    }

    /// Find neighbors within radius using spatial hash
    pub fn find_neighbors(&self, agent_id: usize, radius: f32) -> Vec<usize> {
        let agent = &self.agents[agent_id];
        let cell = self.cell_of(agent.pos);
        let mut neighbors = Vec::new();

        // Check cell and 8 adjacent cells
        for dx in -1..=1 {
            for dy in -1..=1 {
                let check_cell = Cell {
                    x: cell.x + dx,
                    y: cell.y + dy,
                };
                if let Some(agent_ids) = self.grid.get(&check_cell) {
                    for &j in agent_ids {
                        if j == agent_id {
                            continue;
                        }
                        let dist = agent.pos.distance(self.agents[j].pos);
                        if dist <= radius {
                            neighbors.push(j);
                        }
                    }
                }
            }
        }

        neighbors
    }

    /// Update agent positions with Euler integration
    pub fn update_positions(&mut self) {
        for agent in &mut self.agents {
            // Velocity update is handled by demos (they set acceleration)
            // This just updates position based on current velocity

            // Clamp speed (will be set by demos)
            let max_speed = 0.5;
            if agent.vel.length() > max_speed {
                agent.vel = agent.vel.normalize() * max_speed;
            }

            // Update position
            agent.pos += agent.vel * self.dt;

            // Wrap around (torus world)
            let width = self.bounds_max.x - self.bounds_min.x;
            let height = self.bounds_max.y - self.bounds_min.y;

            if agent.pos.x < self.bounds_min.x {
                agent.pos.x += width;
            } else if agent.pos.x > self.bounds_max.x {
                agent.pos.x -= width;
            }

            if agent.pos.y < self.bounds_min.y {
                agent.pos.y += height;
            } else if agent.pos.y > self.bounds_max.y {
                agent.pos.y -= height;
            }

            // Update heading from velocity
            if agent.vel.length() > 0.001 {
                agent.heading = agent.vel.angle();
            }
        }
    }

    /// Compute metrics
    pub fn compute_collisions(&self, collision_radius: f32) -> usize {
        let mut collisions = 0;
        for i in 0..self.agents.len() {
            for j in (i + 1)..self.agents.len() {
                if self.agents[i].pos.distance(self.agents[j].pos) < collision_radius {
                    collisions += 1;
                }
            }
        }
        collisions
    }

    /// Compute minimum separation
    pub fn compute_min_separation(&self) -> f32 {
        let mut min_sep = f32::INFINITY;
        for i in 0..self.agents.len() {
            for j in (i + 1)..self.agents.len() {
                let dist = self.agents[i].pos.distance(self.agents[j].pos);
                min_sep = min_sep.min(dist);
            }
        }
        min_sep
    }

    /// Compute number of connected components
    pub fn compute_components(&self, neighbor_radius: f32) -> usize {
        let mut visited = vec![false; self.agents.len()];
        let mut components = 0;

        for i in 0..self.agents.len() {
            if !visited[i] {
                components += 1;
                self.dfs_component(i, &mut visited, neighbor_radius);
            }
        }

        components
    }

    fn dfs_component(&self, start: usize, visited: &mut [bool], radius: f32) {
        let mut stack = vec![start];
        visited[start] = true;

        while let Some(i) = stack.pop() {
            for j in 0..self.agents.len() {
                if i != j && !visited[j] {
                    let dist = self.agents[i].pos.distance(self.agents[j].pos);
                    if dist <= radius {
                        visited[j] = true;
                        stack.push(j);
                    }
                }
            }
        }
    }

    /// Step the world forward
    pub fn step(&mut self) {
        self.update_positions();
        self.build_grid();
        self.time += self.dt;
    }
}
