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
    pub angle: f32, // Direction of growth
}

impl Default for FungalNode {
    fn default() -> Self {
        Self {
            pos: Vec2::ZERO,
            parent_idx: None,
            health: 0.0,
            age: 0.0,
            active: false,
            angle: 0.0,
        }
    }
}

pub struct FungalNetwork {
    pub nodes: Vec<FungalNode>, 
    pub count: usize,
    pub width: f32,
    pub height: f32,
    pub growth_timer: f32,
    // Simple spatial binning for optimization
    // Cells store indices of nodes
    spatial_grid: Vec<Vec<u16>>,
    grid_cols: usize,
    grid_rows: usize,
    cell_size: f32,
}

impl FungalNetwork {
    pub fn new(width: f32, height: f32) -> Self {
        let cell_size = 50.0;
        let cols = (width / cell_size).ceil() as usize;
        let rows = (height / cell_size).ceil() as usize;
        
        Self {
            nodes: vec![FungalNode::default(); MAX_NODES],
            count: 0,
            width,
            height,
            growth_timer: 0.0,
            spatial_grid: vec![Vec::new(); cols * rows],
            grid_cols: cols,
            grid_rows: rows,
            cell_size,
        }
    }

    pub fn resize(&mut self, width: f32, height: f32) {
        if self.width != width || self.height != height {
            self.width = width;
            self.height = height;
            self.grid_cols = (width / self.cell_size).ceil() as usize;
            self.grid_rows = (height / self.cell_size).ceil() as usize;
            self.spatial_grid = vec![Vec::new(); self.grid_cols * self.grid_rows];
            self.count = 0; // Reset on resize for simplicity
        }
    }

    fn get_cell_index(&self, pos: Vec2) -> Option<usize> {
        if pos.x < 0.0 || pos.x >= self.width || pos.y < 0.0 || pos.y >= self.height {
            return None;
        }
        let col = (pos.x / self.cell_size) as usize;
        let row = (pos.y / self.cell_size) as usize;
        Some(row * self.grid_cols + col)
    }

    fn add_to_grid(&mut self, idx: u16, pos: Vec2) {
        if let Some(cell_idx) = self.get_cell_index(pos) {
            if cell_idx < self.spatial_grid.len() {
                self.spatial_grid[cell_idx].push(idx);
            }
        }
    }

    fn is_space_occupied(&self, pos: Vec2, radius: f32) -> bool {
        let cell_idx = match self.get_cell_index(pos) {
            Some(idx) => idx,
            None => return true, // Out of bounds is "occupied"
        };
        
        // Check current and neighbor cells
        // Simplified: just check current cell for perf? No, need neighbors.
        // Implementing simple neighbor check logic would be tedious here without a helper.
        // For "Graceful" separation, checking current cell + neighbors is best.
        // Let's iterate a 3x3 block around the cell.
        
        let row = cell_idx / self.grid_cols;
        let col = cell_idx % self.grid_cols;
        
        let radius_sq = radius * radius;

        for r in row.saturating_sub(1)..=(row + 1).min(self.grid_rows - 1) {
            for c in col.saturating_sub(1)..=(col + 1).min(self.grid_cols - 1) {
                let idx = r * self.grid_cols + c;
                for &node_idx in &self.spatial_grid[idx] {
                    let node = &self.nodes[node_idx as usize];
                    if node.active && node.pos.distance_squared(pos) < radius_sq {
                        return true;
                    }
                }
            }
        }
        false
    }

    pub fn spawn_root(&mut self) {
        if self.count >= MAX_NODES { return; }
        
        use rand::Rng;
        let mut rng = rand::thread_rng();
        let x = rng.gen_range(0.0..self.width);
        let y = rng.gen_range(0.0..self.height);
        let angle = rng.gen_range(0.0..std::f32::consts::TAU);
        
        let pos = Vec2::new(x, y);
        if !self.is_space_occupied(pos, GROWTH_DISTANCE * 0.8) {
            self.add_node(pos, None, angle);
        }
    }

    pub fn seed_at(&mut self, pos: Vec2) {
        if self.count >= MAX_NODES { return; }
        use rand::Rng;
        let mut rng = rand::thread_rng();
        let angle = rng.gen_range(0.0..std::f32::consts::TAU);
        
        if !self.is_space_occupied(pos, GROWTH_DISTANCE * 0.5) {
            self.add_node(pos, None, angle);
        }
    }

    fn add_node(&mut self, pos: Vec2, parent_idx: Option<u16>, angle: f32) {
        if self.count >= MAX_NODES { return; }
        let idx = self.count;
        self.nodes[idx] = FungalNode {
            pos,
            parent_idx,
            health: 1.0,
            age: 0.0,
            active: true,
            angle,
        };
        self.add_to_grid(idx as u16, pos);
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
                
                // Reduced chance to branch to keep it sparse and "creepy"
                if rng.gen::<f32>() < 0.05 {
                    // Whole number degrees: 30 deg = PI/6
                    let current_angle = self.nodes[i].angle;
                    
                    // Branch options: -30, 0, +30 degrees
                    let branches = rng.gen_range(1..=2); // 1 or 2 branches
                    
                    for _ in 0..branches {
                        let offset_deg = rng.gen_range(-1..=1) as f32 * 30.0; // -30, 0, 30
                        let new_angle = current_angle + offset_deg.to_radians();
                        
                        let dir = Vec2::new(new_angle.cos(), new_angle.sin());
                        let new_pos = self.nodes[i].pos + dir * GROWTH_DISTANCE;
                        
                        if new_pos.x >= 0.0 && new_pos.x <= self.width && new_pos.y >= 0.0 && new_pos.y <= self.height {
                            if !self.is_space_occupied(new_pos, GROWTH_DISTANCE * 0.8) {
                                new_nodes.push((new_pos, i as u16, new_angle));
                            }
                        }
                    }
                }
            }
        }
        
        for (pos, parent, angle) in new_nodes {
            self.add_node(pos, Some(parent), angle);
        }

        // Decay
        // Simple: Nodes decay if infected (health < 1.0).
        // Boid contact sets health < 1.0.
        // If health reaches 0, deactivate.
        for i in 0..self.count {
            if !self.nodes[i].active { continue; }
            
            // Natural slow aging
            self.nodes[i].age += 1.0;
            if self.nodes[i].age > 3000.0 {
                self.nodes[i].health -= 0.0005;
            }
            
            // Infection decay (if damaged)
            if self.nodes[i].health < 0.95 {
                self.nodes[i].health -= 0.01; // "Slowly dying"
            }
            
            if self.nodes[i].health <= 0.0 {
                self.nodes[i].active = false;
                self.nodes[i].health = 0.0;
            }
        }
    }

    pub fn infect(&mut self, pos: Vec2, radius: f32) {
        // Use spatial grid for faster infection checks
        let radius_sq = radius * radius;
        
        // Range of cells to check
        if let Some(center_idx) = self.get_cell_index(pos) {
            let row = center_idx / self.grid_cols;
            let col = center_idx % self.grid_cols;
            
            for r in row.saturating_sub(1)..=(row + 1).min(self.grid_rows - 1) {
                for c in col.saturating_sub(1)..=(col + 1).min(self.grid_cols - 1) {
                    let idx = r * self.grid_cols + c;
                    for &node_idx in &self.spatial_grid[idx] {
                        let i = node_idx as usize;
                        if !self.nodes[i].active { continue; }
                        
                        let dist_sq = self.nodes[i].pos.distance_squared(pos);
                        if dist_sq < radius_sq {
                            // Trigger infection state
                            if self.nodes[i].health > 0.9 {
                                self.nodes[i].health = 0.9; // Start the decay process
                            }
                        }
                    }
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
    fn test_infection_triggers_decay() {
        let mut net = FungalNetwork::new(100.0, 100.0);
        net.seed_at(Vec2::new(50.0, 50.0));
        
        // Infect
        net.infect(Vec2::new(50.0, 50.0), 10.0);
        
        // Check immediate impact
        assert!(net.nodes[0].health <= 0.9);
        
        // Update to see decay
        net.update();
        assert!(net.nodes[0].health < 0.9);
    }

    #[test]
    fn test_infection_clamping() {
        let mut net = FungalNetwork::new(100.0, 100.0);
        net.seed_at(Vec2::new(50.0, 50.0));
        net.nodes[0].health = 0.05; // Almost dead
        
        // Infect and update until death
        for _ in 0..10 {
            net.update();
        }
        
        assert_eq!(net.nodes[0].health, 0.0);
        assert!(!net.nodes[0].active);
    }
    
    #[test]
    fn test_space_occupancy() {
        let mut net = FungalNetwork::new(100.0, 100.0);
        net.seed_at(Vec2::new(50.0, 50.0));
        
        // Should be occupied at the seed
        assert!(net.is_space_occupied(Vec2::new(50.0, 50.0), 5.0));
        // Should be empty far away
        assert!(!net.is_space_occupied(Vec2::new(10.0, 10.0), 5.0));
    }
}
