use glam::Vec2;
use rand::Rng;

use super::types::{BoidHandle, BoidRole, BoidState, Genome};

/// Structure of Arrays layout for cache-friendly iteration
/// All arrays are the same length (CAPACITY)
pub struct BoidArena<const CAPACITY: usize> {
    // SoA layout - each field in its own array for cache locality
    pub positions: Vec<Vec2>,
    pub velocities: Vec<Vec2>,
    pub genes: Vec<Genome>,
    pub roles: Vec<BoidRole>,
    pub states: Vec<BoidState>,
    pub energy: Vec<f32>,
    pub age: Vec<f32>,
    pub generation: Vec<u16>,

    // Metadata
    pub alive: Vec<bool>,
    gen: Vec<u16>, // Generation counter for handles

    // Free list (indices of dead slots)
    free_list: Vec<u16>,
    free_count: usize,

    // Active count for fast iteration
    pub alive_count: usize,

    // Pre-allocated scratch buffers (avoid per-frame allocations)
    pub scratch_accel: Vec<Vec2>,
    pub scratch_density: Vec<u8>,
}

impl<const CAPACITY: usize> BoidArena<CAPACITY> {
    pub fn new() -> Self {
        let mut arena = Self {
            positions: vec![Vec2::ZERO; CAPACITY],
            velocities: vec![Vec2::ZERO; CAPACITY],
            genes: vec![Genome::default(); CAPACITY],
            roles: vec![BoidRole::Herbivore; CAPACITY],
            states: vec![BoidState::Wander; CAPACITY],
            energy: vec![0.0; CAPACITY],
            age: vec![0.0; CAPACITY],
            generation: vec![0; CAPACITY],
            alive: vec![false; CAPACITY],
            gen: vec![0; CAPACITY],
            free_list: vec![0; CAPACITY],
            free_count: CAPACITY,
            alive_count: 0,
            scratch_accel: vec![Vec2::ZERO; CAPACITY],
            scratch_density: vec![0; CAPACITY],
        };

        // Initialize free list (all slots available)
        for i in 0..CAPACITY {
            arena.free_list[i] = i as u16;
        }

        arena
    }

    /// Spawn a new boid, returns handle. O(1) operation.
    #[inline]
    pub fn spawn(&mut self, pos: Vec2, vel: Vec2, genes: Genome) -> BoidHandle {
        if self.free_count == 0 {
            return BoidHandle::INVALID;
        }

        self.free_count -= 1;
        let idx = self.free_list[self.free_count] as usize;

        self.positions[idx] = pos;
        self.velocities[idx] = vel;
        self.genes[idx] = genes;
        self.roles[idx] = genes.role;
        self.states[idx] = BoidState::Wander;
        self.energy[idx] = 100.0;
        self.age[idx] = 0.0;
        self.generation[idx] = 0;
        self.alive[idx] = true;
        self.gen[idx] = self.gen[idx].wrapping_add(1);
        self.alive_count += 1;

        BoidHandle {
            index: idx as u16,
            generation: self.gen[idx],
        }
    }

    /// Spawn with inherited traits (for reproduction)
    #[inline]
    pub fn spawn_child(&mut self, parent_idx: usize) -> BoidHandle {
        if self.free_count == 0 || !self.alive[parent_idx] {
            return BoidHandle::INVALID;
        }

        let mut rng = rand::thread_rng();
        let pos = self.positions[parent_idx];
        let vel = Vec2::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0));
        let genes = self.genes[parent_idx].mutate();
        let parent_gen = self.generation[parent_idx];

        // Cost to parent
        self.energy[parent_idx] -= 50.0;

        self.free_count -= 1;
        let idx = self.free_list[self.free_count] as usize;

        self.positions[idx] = pos;
        self.velocities[idx] = vel;
        self.genes[idx] = genes;
        self.roles[idx] = genes.role;
        self.states[idx] = BoidState::Wander;
        self.energy[idx] = 80.0;
        self.age[idx] = 0.0;
        self.generation[idx] = parent_gen + 1;
        self.alive[idx] = true;
        self.gen[idx] = self.gen[idx].wrapping_add(1);
        self.alive_count += 1;

        BoidHandle {
            index: idx as u16,
            generation: self.gen[idx],
        }
    }

    /// Kill a boid, returns slot to free list. O(1) operation.
    /// Sets state to Dead for scavenging before cleanup
    #[inline]
    pub fn kill(&mut self, idx: usize) {
        if idx < CAPACITY && self.alive[idx] {
            self.states[idx] = BoidState::Dead;
            self.alive[idx] = false;
            self.free_list[self.free_count] = idx as u16;
            self.free_count += 1;
            self.alive_count -= 1;
        }
    }

    /// Check if handle is still valid
    #[inline]
    pub fn is_alive(&self, handle: BoidHandle) -> bool {
        let idx = handle.index as usize;
        idx < CAPACITY && self.alive[idx] && self.gen[idx] == handle.generation
    }

    /// Iterate over all alive boid indices
    #[inline]
    pub fn iter_alive(&self) -> impl Iterator<Item = usize> + '_ {
        (0..CAPACITY).filter(|&i| self.alive[i])
    }

    /// Get remaining capacity
    #[inline]
    pub fn remaining_capacity(&self) -> usize {
        self.free_count
    }
}

impl<const CAPACITY: usize> Default for BoidArena<CAPACITY> {
    fn default() -> Self {
        Self::new()
    }
}
