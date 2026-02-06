use glam::Vec2;
use rand::Rng;

use super::arena::BoidArena;
use super::types::Genome;

#[derive(Clone, Debug)]
pub struct FoodSource {
    pub position: Vec2,
    pub energy: f32,
    pub max_energy: f32,
    pub radius: f32,
    pub regen_rate: f32,
    pub depleted_timer: f32, // Time since last depletion
}

impl FoodSource {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            position: Vec2::new(x, y),
            energy: 600.0,
            max_energy: 800.0,
            radius: 90.0,
            regen_rate: 5.0,
            depleted_timer: 0.0,
        }
    }

    #[inline]
    pub fn consume(&mut self, amount: f32) -> f32 {
        let taken = amount.min(self.energy);
        self.energy -= taken;
        if taken > 0.0 {
            self.depleted_timer = 0.0;
        }
        taken
    }

    #[inline]
    pub fn regenerate(&mut self, dt: f32, season_multiplier: f32) {
        self.depleted_timer += dt;
        // Faster regen when not being consumed, affected by season
        let regen = self.regen_rate * season_multiplier * dt;
        self.energy = (self.energy + regen).min(self.max_energy);
    }

    #[inline]
    pub fn is_depleted(&self) -> bool {
        self.energy < self.max_energy * 0.1
    }

    #[inline]
    pub fn fullness(&self) -> f32 {
        self.energy / self.max_energy
    }
}

/// Predator zone - dangerous area that drains energy
#[derive(Clone, Debug)]
pub struct PredatorZone {
    pub position: Vec2,
    pub radius: f32,
    pub intensity: f32,
    pub active: bool,
    pub lifetime: f32,
}

impl PredatorZone {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            position: Vec2::new(x, y),
            radius: 120.0,
            intensity: 3.0,
            active: true,
            lifetime: 0.0,
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.lifetime += dt;
        // Predators fade after ~15 seconds
        if self.lifetime > 900.0 {
            self.active = false;
        }
        // Intensity pulses
        self.intensity = 2.0 + (self.lifetime * 0.1).sin() * 1.5;
    }
}

/// World events for drama
#[derive(Clone, Debug)]
pub enum WorldEvent {
    Famine,        // Food stops regenerating
    Bloom,         // Food regenerates 3x faster
    PredatorSpawn, // New predator zone appears
    Migration,     // Boids get pushed in a direction
    Earthquake,    // Randomize all velocities
}

/// Seasonal cycle affects food and mortality
#[derive(Clone, Copy, Debug)]
pub struct SeasonCycle {
    pub time: f32,
    pub period: f32, // ~30 seconds per season
}

impl Default for SeasonCycle {
    fn default() -> Self {
        Self::new()
    }
}

impl SeasonCycle {
    pub fn new() -> Self {
        Self {
            time: 0.0,
            period: 1800.0,
        } // 30 second seasons
    }

    pub fn update(&mut self, dt: f32) {
        self.time += dt;
    }

    /// Returns 0.0-1.0 season phase (0=winter, 0.5=summer)
    #[inline]
    pub fn phase(&self) -> f32 {
        (self.time / self.period).fract()
    }

    /// Food multiplier: low in winter, high in summer
    #[inline]
    pub fn food_multiplier(&self) -> f32 {
        let phase = self.phase();
        // Sinusoidal: 0.3 in winter, 2.0 in summer
        0.3 + 1.7 * (phase * std::f32::consts::TAU).sin().max(0.0)
    }

    /// Returns season name
    pub fn season_name(&self) -> &'static str {
        let phase = self.phase();
        if phase < 0.25 {
            "SPRING"
        } else if phase < 0.5 {
            "SUMMER"
        } else if phase < 0.75 {
            "AUTUMN"
        } else {
            "WINTER"
        }
    }
}

/// Feed boids from food sources - zero allocations
pub fn feed_from_sources<const CAP: usize>(
    arena: &mut BoidArena<CAP>,
    food_sources: &mut [FoodSource],
    season: &SeasonCycle,
) {
    let food_mult = season.food_multiplier();

    for idx in 0..CAP {
        if !arena.alive[idx] {
            continue;
        }

        let pos = arena.positions[idx];

        for food in food_sources.iter_mut() {
            let dist = pos.distance(food.position);
            if dist < food.radius && food.energy > 0.0 {
                // More food in summer, less in winter
                let consumed = food.consume(0.8 + food_mult * 0.4);
                arena.energy[idx] = (arena.energy[idx] + consumed).min(200.0);
                break;
            }
        }
    }

    // Regenerate food based on season
    for food in food_sources.iter_mut() {
        food.regenerate(1.0, food_mult);
    }
}

/// Apply predator zone damage to boids
pub fn apply_predator_zones<const CAP: usize>(
    arena: &mut BoidArena<CAP>,
    predators: &[PredatorZone],
) -> usize {
    let mut kills = 0;

    for idx in 0..CAP {
        if !arena.alive[idx] {
            continue;
        }

        let pos = arena.positions[idx];

        for pred in predators {
            if !pred.active {
                continue;
            }

            let dist = pos.distance(pred.position);
            if dist < pred.radius {
                // Drain energy based on proximity
                let damage = pred.intensity * (1.0 - dist / pred.radius);
                arena.energy[idx] -= damage;

                // Push boids away from predator
                if dist > 1.0 {
                    let flee = (pos - pred.position).normalize() * 2.0;
                    arena.velocities[idx] += flee;
                }

                if arena.energy[idx] <= 0.0 {
                    kills += 1;
                }
            }
        }
    }

    kills
}

/// Trigger a migration event - push all boids in a direction
pub fn trigger_migration<const CAP: usize>(
    arena: &mut BoidArena<CAP>,
    direction: Vec2,
    strength: f32,
) {
    for idx in 0..CAP {
        if arena.alive[idx] {
            arena.velocities[idx] += direction * strength;
        }
    }
}

/// Trigger earthquake - randomize velocities
pub fn trigger_earthquake<const CAP: usize>(arena: &mut BoidArena<CAP>) {
    let mut rng = rand::thread_rng();

    for idx in 0..CAP {
        if arena.alive[idx] {
            arena.velocities[idx] = Vec2::new(rng.gen_range(-3.0..3.0), rng.gen_range(-3.0..3.0));
            // Stress from earthquake
            arena.energy[idx] -= 5.0;
        }
    }
}

/// Compute ecosystem diversity score (0.0 = monoculture, 1.0 = highly diverse)
/// Based on role distribution and trait variance
pub fn compute_diversity<const CAP: usize>(arena: &BoidArena<CAP>) -> f32 {
    if arena.alive_count < 10 {
        return 1.0; // Too few to measure, assume diverse
    }

    let mut herbivore_count = 0usize;
    let mut carnivore_count = 0usize;
    let mut scavenger_count = 0usize;
    let mut speed_sum = 0.0f32;
    let mut speed_sq_sum = 0.0f32;

    for idx in arena.iter_alive() {
        match arena.roles[idx] {
            super::types::BoidRole::Herbivore => herbivore_count += 1,
            super::types::BoidRole::Carnivore => carnivore_count += 1,
            super::types::BoidRole::Scavenger => scavenger_count += 1,
        }
        let speed = arena.genes[idx].max_speed;
        speed_sum += speed;
        speed_sq_sum += speed * speed;
    }

    let total = arena.alive_count as f32;

    // Role diversity: Shannon entropy normalized
    let h_frac = herbivore_count as f32 / total;
    let c_frac = carnivore_count as f32 / total;
    let s_frac = scavenger_count as f32 / total;

    let mut entropy = 0.0f32;
    if h_frac > 0.0 {
        entropy -= h_frac * h_frac.log2();
    }
    if c_frac > 0.0 {
        entropy -= c_frac * c_frac.log2();
    }
    if s_frac > 0.0 {
        entropy -= s_frac * s_frac.log2();
    }

    let max_entropy = 3.0f32.log2(); // ~1.58
    let role_diversity = (entropy / max_entropy).clamp(0.0, 1.0);

    // Trait diversity: coefficient of variation of speed
    let speed_mean = speed_sum / total;
    let speed_variance = (speed_sq_sum / total) - (speed_mean * speed_mean);
    let speed_std = speed_variance.max(0.0).sqrt();
    let cv = if speed_mean > 0.0 {
        speed_std / speed_mean
    } else {
        0.0
    };
    // CV of 0.3+ is healthy diversity, normalize
    let trait_diversity = (cv / 0.4).clamp(0.0, 1.0);

    // Combined score (weighted)
    0.7 * role_diversity + 0.3 * trait_diversity
}

/// Trigger mass extinction - kills most boids, resets ecosystem
pub fn trigger_mass_extinction<const CAP: usize>(
    arena: &mut BoidArena<CAP>,
    kill_fraction: f32,
    width: f32,
    height: f32,
) {
    let mut rng = rand::thread_rng();

    let mut killed = 0usize;
    let target_kills = (arena.alive_count as f32 * kill_fraction) as usize;

    for idx in 0..CAP {
        if !arena.alive[idx] {
            continue;
        }
        if killed >= target_kills {
            break;
        }

        // Random chance to survive (larger/stronger have slight advantage)
        let survival_bonus = arena.genes[idx].strength * 0.1;
        if rng.gen::<f32>() > survival_bonus {
            arena.kill(idx);
            killed += 1;
        }
    }

    // Spawn a few diverse founders to reseed (use actual world dimensions)
    let founders = 10.min(CAP - arena.alive_count);
    let margin = 50.0f32.min(width * 0.1).min(height * 0.1);
    for _ in 0..founders {
        let pos = Vec2::new(
            rng.gen_range(margin..(width - margin).max(margin + 1.0)),
            rng.gen_range(margin..(height - margin).max(margin + 1.0)),
        );
        let vel = Vec2::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0));
        arena.spawn(pos, vel, Genome::random());
    }
}

/// Get color components for a boid (hue, saturation, lightness)
#[inline]
pub fn get_boid_color<const CAP: usize>(arena: &BoidArena<CAP>, idx: usize) -> (u16, u8, u8) {
    let (hue, sat) = arena.genes[idx].color_hs();
    let energy_norm = (arena.energy[idx] / 200.0).clamp(0.0, 1.0);
    let lightness = (25.0 + energy_norm * 55.0) as u8;
    (hue, sat, lightness)
}
