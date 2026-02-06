use glam::Vec2;
use rand::Rng;

#[derive(Clone, Copy, Debug)]
pub struct Obstacle {
    pub center: Vec2,
    pub radius: f32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BoidRole {
    Herbivore,
    Carnivore,
    Scavenger,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BoidState {
    Wander,
    Forage,
    Hunt,
    Flee,
    Reproduce,
    Dead, // Persist for scavenging
}

#[derive(Clone, Copy, Debug)]
pub struct Genome {
    pub role: BoidRole,
    pub max_speed: f32,     // 2.0 - 6.0
    pub agility: f32,       // Turn rate / Force multiplier (0.5 - 2.0)
    pub size: f32,          // 0.5 - 2.0 multiplier
    pub strength: f32,      // Combat/Health (0.5 - 2.0)
    pub sensor_radius: f32, // Vision (40.0 - 120.0)
    pub metabolism: f32,    // Energy cost (0.7 - 1.3)
    pub color_hs: (u16, u8),
}

impl Default for Genome {
    fn default() -> Self {
        Self {
            role: BoidRole::Herbivore,
            max_speed: 3.0,
            agility: 1.0,
            size: 1.0,
            strength: 1.0,
            sensor_radius: 60.0,
            metabolism: 1.0,
            color_hs: (120, 70),
        }
    }
}

impl Genome {
    pub fn random() -> Self {
        let mut rng = rand::thread_rng();
        let role_roll = rng.gen::<f32>();
        let role = if role_roll < 0.6 {
            BoidRole::Herbivore
        } else if role_roll < 0.7 {
            BoidRole::Carnivore
        } else {
            BoidRole::Scavenger
        };

        let max_speed = rng.gen_range(2.0..=4.0);
        let agility = rng.gen_range(0.5..=2.0);
        let size = rng.gen_range(0.5..=2.0);
        let strength = rng.gen_range(0.5..=2.0);
        let sensor_radius = rng.gen_range(40.0..=120.0);
        let metabolism = rng.gen_range(0.7..=1.3);

        let color_hs = Self::compute_color_hs(role, max_speed, metabolism);

        Self {
            role,
            max_speed,
            agility,
            size,
            strength,
            sensor_radius,
            metabolism,
            color_hs,
        }
    }

    /// Compute color based on role, speed, and metabolism
    #[inline]
    fn compute_color_hs(role: BoidRole, max_speed: f32, metabolism: f32) -> (u16, u8) {
        let (base_hue, base_sat) = match role {
            BoidRole::Herbivore => (120, 70), // Green
            BoidRole::Carnivore => (0, 80),   // Red
            BoidRole::Scavenger => (280, 60), // Purple
        };

        let speed_norm = ((max_speed - 2.0) / 2.0).clamp(0.0, 1.0);
        let hue = (base_hue as f32 + speed_norm * 30.0) as u16 % 360;
        let sat = (base_sat as f32 + (metabolism - 0.7) * 20.0) as u8;
        (hue, sat.clamp(50, 100))
    }

    /// Hue from speed (blue=slow, red=fast), saturation from metabolism
    #[inline]
    pub fn color_hs(&self) -> (u16, u8) {
        self.color_hs
    }

    /// Mutate genome with one of 5 evolutionary events
    pub fn mutate(&self) -> Self {
        let mut rng = rand::thread_rng();
        let event_roll = rng.gen::<f32>();

        let mut new_genome = *self;

        // 5 Evolutionary Events
        if event_roll < 0.2 {
            // 1. Gigantism: ++Size/Strength, --Speed/Efficiency
            new_genome.size = (self.size * 1.2).clamp(0.5, 2.0);
            new_genome.strength = (self.strength * 1.2).clamp(0.5, 2.0);
            new_genome.max_speed = (self.max_speed * 0.9).clamp(2.0, 6.0);
            new_genome.metabolism = (self.metabolism * 1.1).clamp(0.7, 1.3);
        } else if event_roll < 0.4 {
            // 2. Miniaturization: --Size, ++Agility/Efficiency
            new_genome.size = (self.size * 0.8).clamp(0.5, 2.0);
            new_genome.agility = (self.agility * 1.2).clamp(0.5, 2.0);
            new_genome.metabolism = (self.metabolism * 0.9).clamp(0.7, 1.3);
        } else if event_roll < 0.6 {
            // 3. Swiftness: ++Speed, --Strength
            new_genome.max_speed = (self.max_speed * 1.2).clamp(2.0, 6.0);
            new_genome.strength = (self.strength * 0.9).clamp(0.5, 2.0);
        } else if event_roll < 0.8 {
            // 4. Hyper-Sense: ++Sensor Radius, --Efficiency
            new_genome.sensor_radius = (self.sensor_radius * 1.3).clamp(40.0, 120.0);
            new_genome.metabolism = (self.metabolism * 1.1).clamp(0.7, 1.3);
        } else if event_roll < 0.81 {
            // 5. Speciation: 1% chance to switch Role
            new_genome.role = match self.role {
                BoidRole::Herbivore => BoidRole::Carnivore,
                BoidRole::Carnivore => BoidRole::Scavenger,
                BoidRole::Scavenger => BoidRole::Herbivore,
            };
        } else {
            // Standard small mutations (19% chance)
            new_genome.max_speed = (self.max_speed * rng.gen_range(0.95..=1.05)).clamp(2.0, 6.0);
            new_genome.agility = (self.agility * rng.gen_range(0.95..=1.05)).clamp(0.5, 2.0);
            new_genome.size = (self.size * rng.gen_range(0.95..=1.05)).clamp(0.5, 2.0);
            new_genome.strength = (self.strength * rng.gen_range(0.95..=1.05)).clamp(0.5, 2.0);
            new_genome.sensor_radius =
                (self.sensor_radius * rng.gen_range(0.95..=1.05)).clamp(40.0, 120.0);
            new_genome.metabolism = (self.metabolism * rng.gen_range(0.95..=1.05)).clamp(0.7, 1.3);
        }

        // Recompute color
        new_genome.color_hs =
            Self::compute_color_hs(new_genome.role, new_genome.max_speed, new_genome.metabolism);

        new_genome
    }
}

/// Generational index - catches use-after-free bugs at zero runtime cost
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BoidHandle {
    pub(crate) index: u16,
    pub(crate) generation: u16,
}

impl BoidHandle {
    pub const INVALID: Self = Self {
        index: u16::MAX,
        generation: 0,
    };

    #[inline]
    pub fn is_valid(&self) -> bool {
        self.index != u16::MAX
    }

    #[inline]
    pub fn index(&self) -> usize {
        self.index as usize
    }
}
