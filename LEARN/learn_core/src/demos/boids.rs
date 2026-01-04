//! ═══════════════════════════════════════════════════════════════════════════════
//! FILE: boids.rs | LEARN/learn_core/src/demos/boids.rs
//! PURPOSE: Boids flocking demo - separation, alignment, cohesion
//! MODIFIED: 2025-01-XX
//! LAYER: LEARN → learn_core → demos
//! ═══════════════════════════════════════════════════════════════════════════════

use crate::{Demo, ParamMeta, Vec2};
use super::swarm_world::{Agent, Obstacle, SwarmWorld};

/// Boids flocking demo
pub struct BoidsDemo {
    pub world: SwarmWorld,
    pub num_agents: usize,
    pub neighbor_radius: f32,
    pub k_sep: f32,
    pub k_ali: f32,
    pub k_coh: f32,
    pub k_obs: f32,
    pub v_max: f32,
    pub max_accel: f32,
}

impl Default for BoidsDemo {
    fn default() -> Self {
        Self {
            world: SwarmWorld::new(Vec2::new(0.0, 0.0), Vec2::new(1.0, 1.0), 42),
            num_agents: 120,
            neighbor_radius: 0.12,
            k_sep: 1.4,
            k_ali: 1.0,
            k_coh: 0.8,
            k_obs: 2.0,
            v_max: 0.35,
            max_accel: 2.0,
        }
    }
}

impl Demo for BoidsDemo {
    fn reset(&mut self, seed: u64) {
        self.world = SwarmWorld::new(Vec2::new(0.0, 0.0), Vec2::new(1.0, 1.0), seed);
        self.world.agents.clear();
        self.world.obstacles.clear();

        // Initialize agents randomly
        for i in 0..self.num_agents {
            let pos = Vec2::new(
                self.world.rng.range(0.1, 0.9),
                self.world.rng.range(0.1, 0.9),
            );
            let vel = Vec2::from_angle(
                self.world.rng.range(0.0, std::f32::consts::TAU),
                self.world.rng.range(0.05, 0.15),
            );
            let mut agent = Agent::new(i, pos);
            agent.vel = vel;
            self.world.add_agent(agent);
        }

        // Add some obstacles
        self.world.add_obstacle(Obstacle {
            center: Vec2::new(0.3, 0.5),
            radius: 0.08,
        });
        self.world.add_obstacle(Obstacle {
            center: Vec2::new(0.7, 0.5),
            radius: 0.08,
        });

        self.world.build_grid();
    }

    fn step(&mut self, dt: f32) {
        self.world.dt = dt;
        
        // Compute boids forces for each agent
        for i in 0..self.world.agents.len() {
            let neighbors = self.world.find_neighbors(i, self.neighbor_radius);
            
            let mut accel = Vec2::ZERO;
            
            // Separation
            let mut sep = Vec2::ZERO;
            for &j in &neighbors {
                let diff = self.world.agents[i].pos - self.world.agents[j].pos;
                let dist_sq = diff.length_squared() + 0.01; // epsilon
                sep += diff / dist_sq;
            }
            if neighbors.len() > 0 {
                sep = sep.normalize() * self.k_sep;
            }
            accel += sep;
            
            // Alignment
            if neighbors.len() > 0 {
                let mut avg_vel = Vec2::ZERO;
                for &j in &neighbors {
                    avg_vel += self.world.agents[j].vel;
                }
                avg_vel = avg_vel * (1.0 / neighbors.len() as f32);
                let ali = (avg_vel - self.world.agents[i].vel) * self.k_ali;
                accel += ali;
            }
            
            // Cohesion
            if neighbors.len() > 0 {
                let mut avg_pos = Vec2::ZERO;
                for &j in &neighbors {
                    avg_pos += self.world.agents[j].pos;
                }
                avg_pos = avg_pos * (1.0 / neighbors.len() as f32);
                let coh = (avg_pos - self.world.agents[i].pos) * self.k_coh;
                accel += coh;
            }
            
            // Obstacle avoidance
            for obs in &self.world.obstacles {
                let diff = self.world.agents[i].pos - obs.center;
                let dist = diff.length();
                if dist < obs.radius + 0.05 {
                    let avoid_dist = (dist - obs.radius).max(0.01);
                    let avoid_force = diff.normalize() * self.k_obs / (avoid_dist * avoid_dist);
                    accel += avoid_force;
                }
            }
            
            // Clamp acceleration
            if accel.length() > self.max_accel {
                accel = accel.normalize() * self.max_accel;
            }
            
            // Update velocity
            self.world.agents[i].vel += accel * dt;
            
            // Clamp speed
            if self.world.agents[i].vel.length() > self.v_max {
                self.world.agents[i].vel = self.world.agents[i].vel.normalize() * self.v_max;
            }
        }
        
        // Update positions
        self.world.step();
    }

    fn set_param(&mut self, name: &str, value: f32) -> bool {
        match name {
            "num_agents" => {
                self.num_agents = value as usize;
                true
            }
            "neighbor_radius" => {
                self.neighbor_radius = value;
                true
            }
            "k_sep" => {
                self.k_sep = value;
                true
            }
            "k_ali" => {
                self.k_ali = value;
                true
            }
            "k_coh" => {
                self.k_coh = value;
                true
            }
            "k_obs" => {
                self.k_obs = value;
                true
            }
            "v_max" => {
                self.v_max = value;
                true
            }
            _ => false,
        }
    }

    fn params() -> &'static [ParamMeta] {
        &[
            ParamMeta {
                name: "num_agents",
                label: "Number of Agents",
                min: 20.0,
                max: 400.0,
                step: 10.0,
                default: 120.0,
            },
            ParamMeta {
                name: "neighbor_radius",
                label: "Neighbor Radius",
                min: 0.03,
                max: 0.25,
                step: 0.01,
                default: 0.12,
            },
            ParamMeta {
                name: "k_sep",
                label: "Separation Strength",
                min: 0.0,
                max: 3.0,
                step: 0.1,
                default: 1.4,
            },
            ParamMeta {
                name: "k_ali",
                label: "Alignment Strength",
                min: 0.0,
                max: 3.0,
                step: 0.1,
                default: 1.0,
            },
            ParamMeta {
                name: "k_coh",
                label: "Cohesion Strength",
                min: 0.0,
                max: 3.0,
                step: 0.1,
                default: 0.8,
            },
            ParamMeta {
                name: "k_obs",
                label: "Obstacle Avoidance",
                min: 0.0,
                max: 6.0,
                step: 0.2,
                default: 2.0,
            },
            ParamMeta {
                name: "v_max",
                label: "Max Speed",
                min: 0.05,
                max: 1.0,
                step: 0.05,
                default: 0.35,
            },
        ]
    }
}

