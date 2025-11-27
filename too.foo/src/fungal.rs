use glam::Vec2;

const MAX_NODES: usize = 2000;
const GROWTH_DISTANCE: f32 = 15.0;

#[derive(Clone, Copy, Debug)]
pub struct FungalNode {
    pub pos: Vec2,
    pub parent_idx: Option<u16>,
    pub health: f32, // 0.0 - 1.0
    pub age: f32,
    pub active: bool,
}

impl Default for FungalNode {
    fn default() -> Self {
        Self {
            pos: Vec2::ZERO,
            parent_idx: None,
            health: 0.0,
            age: 0.0,
            active: false,
        }
    }
}

pub struct FungalNetwork {
    pub nodes: Vec<FungalNode>, 
    pub count: usize,
    pub width: f32,
    pub height: f32,
    pub growth_timer: f32,
}

impl FungalNetwork {
    pub fn new(width: f32, height: f32) -> Self {
        Self {
            nodes: vec![FungalNode::default(); MAX_NODES],
            count: 0,
            width,
            height,
            growth_timer: 0.0,
        }
    }

    pub fn resize(&mut self, width: f32, height: f32) {
        self.width = width;
        self.height = height;
    }

    pub fn spawn_root(&mut self) {
        if self.count >= MAX_NODES { return; }
        
        // For deterministic testing, we might want to inject RNG, 
        // but for this visual sim, internal rand is fine if we test side effects.
        // In tests we can check if count increased.
        let x = self.width / 2.0; // Default to center if not randomizing for test stability? 
        // No, let's keep randomness but expose a method for deterministic seeding if needed.
        // Actually, for the main loop we want random.
        
        use rand::Rng;
        let mut rng = rand::thread_rng();
        let x = rng.gen_range(0.0..self.width);
        let y = rng.gen_range(0.0..self.height);
        
        self.add_node(Vec2::new(x, y), None);
    }

    pub fn seed_at(&mut self, pos: Vec2) {
        if self.count >= MAX_NODES { return; }
        self.add_node(pos, None);
    }

    fn add_node(&mut self, pos: Vec2, parent_idx: Option<u16>) {
        if self.count >= MAX_NODES { return; }
        let idx = self.count;
        self.nodes[idx] = FungalNode {
            pos,
            parent_idx,
            health: 1.0,
            age: 0.0,
            active: true,
        };
        self.count += 1;
    }

    pub fn update(&mut self) {
        self.growth_timer += 1.0;
        
        if self.growth_timer % 60.0 == 0.0 {
            self.spawn_root();
        }

        // Grow
        let mut new_nodes = Vec::new();
        if self.count < MAX_NODES && self.growth_timer % 5.0 == 0.0 {
            use rand::Rng;
            let mut rng = rand::thread_rng();
            
            for i in 0..self.count {
                if !self.nodes[i].active || self.nodes[i].health < 0.5 { continue; }
                
                if rng.gen::<f32>() < 0.02 {
                    let angle = rng.gen_range(0.0..std::f32::consts::TAU);
                    let dir = Vec2::new(angle.cos(), angle.sin());
                    let new_pos = self.nodes[i].pos + dir * GROWTH_DISTANCE;
                    
                    if new_pos.x >= 0.0 && new_pos.x <= self.width && new_pos.y >= 0.0 && new_pos.y <= self.height {
                        new_nodes.push((new_pos, i as u16));
                    }
                }
            }
        }
        
        for (pos, parent) in new_nodes {
            self.add_node(pos, Some(parent));
        }

        // Decay
        for i in 0..self.count {
            if !self.nodes[i].active { continue; }
            self.nodes[i].age += 1.0;
            
            if self.nodes[i].age > 2000.0 {
                self.nodes[i].health -= 0.001;
            }
            
            // Ensure health doesn't become negative NaN, but here we just set active false
            if self.nodes[i].health <= 0.0 {
                self.nodes[i].active = false;
                self.nodes[i].health = 0.0; // Clamp for safety
            }
        }
    }

    pub fn infect(&mut self, pos: Vec2, radius: f32) {
        let radius_sq = radius * radius;
        
        for i in 0..self.count {
            if !self.nodes[i].active { continue; }
            
            let dist_sq = self.nodes[i].pos.distance_squared(pos);
            if dist_sq < radius_sq {
                self.nodes[i].health -= 0.1;
                if self.nodes[i].health < 0.0 { 
                    self.nodes[i].health = 0.0; 
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initialization() {
        let net = FungalNetwork::new(100.0, 100.0);
        assert_eq!(net.count, 0);
        assert_eq!(net.nodes.len(), MAX_NODES);
    }

    #[test]
    fn test_seed_at() {
        let mut net = FungalNetwork::new(100.0, 100.0);
        net.seed_at(Vec2::new(50.0, 50.0));
        assert_eq!(net.count, 1);
        assert!(net.nodes[0].active);
        assert_eq!(net.nodes[0].health, 1.0);
    }

    #[test]
    fn test_infection_decays_health() {
        let mut net = FungalNetwork::new(100.0, 100.0);
        net.seed_at(Vec2::new(50.0, 50.0));
        
        // Infect exactly at the spot
        net.infect(Vec2::new(50.0, 50.0), 10.0);
        
        assert!(net.nodes[0].health < 1.0, "Health should decrease after infection");
        assert!(net.nodes[0].health >= 0.0, "Health should not be negative");
    }

    #[test]
    fn test_infection_clamping() {
        let mut net = FungalNetwork::new(100.0, 100.0);
        net.seed_at(Vec2::new(50.0, 50.0));
        
        // Massive infection loop
        for _ in 0..20 {
            net.infect(Vec2::new(50.0, 50.0), 10.0);
        }
        
        assert_eq!(net.nodes[0].health, 0.0, "Health should clamp to 0.0");
        // Note: active might still be true until update() runs, depending on logic logic. 
        // The update loop handles deactivation.
    }
    
    #[test]
    fn test_bounds_check() {
        let mut net = FungalNetwork::new(10.0, 10.0);
        // Try to grow outside? Hard to force rand, but we can check resize.
        net.resize(50.0, 50.0);
        assert_eq!(net.width, 50.0);
    }
}

