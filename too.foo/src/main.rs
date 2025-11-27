use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;
use web_sys::{window, CanvasRenderingContext2d, HtmlCanvasElement, Document, HtmlElement, Performance};
use antimony_core::{
    BoidArena, SpatialGrid, Obstacle, FoodSource, Genome, SimConfig,
    SeasonCycle, PredatorZone,
    compute_flocking_forces, simulation_step, feed_from_sources, get_boid_color,
    apply_predator_zones, trigger_migration, trigger_earthquake,
};
use glam::Vec2;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

// Fixed capacity - no runtime allocations
const ARENA_CAPACITY: usize = 1024;
const CELL_CAPACITY: usize = 32;

// --- Fungal Growth System (Space Colonization / Node Graph) ---
// Replacing dense grid with vector node graph
const MAX_NODES: usize = 2000;
const GROWTH_DISTANCE: f32 = 15.0;
const KILL_DISTANCE: f32 = 30.0;

#[derive(Clone, Copy)]
struct FungalNode {
    pos: Vec2,
    parent_idx: Option<u16>,
    health: f32, // 0.0 - 1.0
    age: f32,
    active: bool,
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

struct FungalNetwork {
    nodes: Vec<FungalNode>, // Fixed capacity vec
    count: usize,
    width: f32,
    height: f32,
    growth_timer: f32,
}

impl FungalNetwork {
    fn new(width: f32, height: f32) -> Self {
        Self {
            nodes: vec![FungalNode::default(); MAX_NODES],
            count: 0,
            width,
            height,
            growth_timer: 0.0,
        }
    }

    fn resize(&mut self, width: f32, height: f32) {
        self.width = width;
        self.height = height;
    }

    // Randomly spawn a new root in empty space
    fn spawn_root(&mut self) {
        if self.count >= MAX_NODES { return; }
        
        use rand::Rng;
        let mut rng = rand::thread_rng();
        
        let x = rng.gen_range(0.0..self.width);
        let y = rng.gen_range(0.0..self.height);
        
        // Find free slot
        let idx = self.count;
        self.nodes[idx] = FungalNode {
            pos: Vec2::new(x, y),
            parent_idx: None,
            health: 1.0,
            age: 0.0,
            active: true,
        };
        self.count += 1;
    }

    // Boids trigger this: "seeding" new growth or "spores"
    fn seed_at(&mut self, pos: Vec2) {
        if self.count >= MAX_NODES { return; }
        
        // Check proximity to existing nodes to prevent clumping? 
        // For simple logic, just spawn if random chance passes
        use rand::Rng;
        let mut rng = rand::thread_rng();
        if rng.gen::<f32>() > 0.05 { return; } // 5% chance per frame per boid? Callee handles frequency

        let idx = self.count;
        self.nodes[idx] = FungalNode {
            pos,
            parent_idx: None,
            health: 1.0,
            age: 0.0,
            active: true,
        };
        self.count += 1;
    }

    fn update(&mut self) {
        self.growth_timer += 1.0;
        
        // Random root spawning
        if self.growth_timer % 60.0 == 0.0 {
            self.spawn_root();
        }

        let mut new_nodes = Vec::new(); // Temp buffer for new growth to avoid mutating self.nodes while iterating
        
        // Grow tips
        // Only grow if count < MAX and some time passed
        if self.count < MAX_NODES && self.growth_timer % 5.0 == 0.0 {
            use rand::Rng;
            let mut rng = rand::thread_rng();
            
            // Iterate active nodes, try to branch
            for i in 0..self.count {
                if !self.nodes[i].active || self.nodes[i].health < 0.5 { continue; }
                
                // Random chance to branch based on health
                if rng.gen::<f32>() < 0.02 {
                    let angle = rng.gen_range(0.0..std::f32::consts::TAU);
                    let dir = Vec2::new(angle.cos(), angle.sin());
                    let new_pos = self.nodes[i].pos + dir * GROWTH_DISTANCE;
                    
                    // Check bounds
                    if new_pos.x < 0.0 || new_pos.x > self.width || new_pos.y < 0.0 || new_pos.y > self.height {
                        continue;
                    }
                    
                    new_nodes.push(FungalNode {
                        pos: new_pos,
                        parent_idx: Some(i as u16),
                        health: 1.0,
                        age: 0.0,
                        active: true,
                    });
                }
            }
        }
        
        // Append new nodes
        for node in new_nodes {
            if self.count < MAX_NODES {
                self.nodes[self.count] = node;
                self.count += 1;
            }
        }

        // Decay / Age
        for i in 0..self.count {
            if !self.nodes[i].active { continue; }
            self.nodes[i].age += 1.0;
            
            // Natural decay of very old nodes
            if self.nodes[i].age > 2000.0 {
                self.nodes[i].health -= 0.001;
            }
            
            if self.nodes[i].health <= 0.0 {
                self.nodes[i].active = false;
            }
        }
        
        // TODO: Compact list if too many dead? For now, simple implementation keeps them.
    }

    // Infection: Boids kill nodes they touch
    fn infect(&mut self, pos: Vec2, radius: f32) {
        let radius_sq = radius * radius;
        
        for i in 0..self.count {
            if !self.nodes[i].active { continue; }
            
            let dist_sq = self.nodes[i].pos.distance_squared(pos);
            if dist_sq < radius_sq {
                // Infection! Rapid decay
                self.nodes[i].health -= 0.1;
                // Propagate? Maybe later. Simple cutting for now.
            }
        }
    }

    fn draw(&self, ctx: &CanvasRenderingContext2d) {
        ctx.set_line_cap("round");
        
        // Draw branches
        for i in 0..self.count {
            if !self.nodes[i].active { continue; }
            
            if let Some(parent_idx) = self.nodes[i].parent_idx {
                let parent = self.nodes[parent_idx as usize];
                if parent.active {
                    let health = self.nodes[i].health;
                    let alpha = 0.2 + health * 0.6;
                    let width = 0.5 + health * 2.5;
                    
                    // Color shift based on health: Green -> Brown/Grey
                    let hue = 120.0 * health; // 120=Green, 0=Red/Brown ish
                    
                    ctx.set_stroke_style(&JsValue::from_str(&format!("hsla({}, 60%, 50%, {})", hue, alpha)));
                    ctx.set_line_width(width as f64);
                    
                    ctx.begin_path();
                    ctx.move_to(parent.pos.x as f64, parent.pos.y as f64);
                    ctx.line_to(self.nodes[i].pos.x as f64, self.nodes[i].pos.y as f64);
                    ctx.stroke();
                }
            } else {
                // Root node
                let health = self.nodes[i].health;
                ctx.set_fill_style(&JsValue::from_str(&format!("hsla(120, 60%, 50%, {})", 0.3 * health)));
                ctx.begin_path();
                ctx.arc(self.nodes[i].pos.x as f64, self.nodes[i].pos.y as f64, (2.0 * health) as f64, 0.0, std::f64::consts::TAU).unwrap();
                ctx.fill();
            }
        }
    }
}

/// Simulation state tracking
struct SimulationStats {
    max_speed_record: f32,
    max_generation: u16,
    total_births: u64,
    total_deaths: u64,
}

/// Append a log event to the console-log div
fn log_event(document: &Document, msg: &str, event_class: &str) {
    if let Some(console_log) = document.get_element_by_id("console-log") {
        if let Ok(p) = document.create_element("p") {
            p.set_text_content(Some(msg));
            let _ = p.set_attribute("class", event_class);
            let _ = console_log.append_child(&p);
            
            if let Ok(html_el) = console_log.dyn_into::<HtmlElement>() {
                html_el.set_scroll_top(html_el.scroll_height());
            }
        }
    }
}

struct World {
    arena: BoidArena<ARENA_CAPACITY>,
    grid: SpatialGrid<CELL_CAPACITY>,
    obstacles: Vec<Obstacle>,
    food_sources: Vec<FoodSource>,
    fungal_network: FungalNetwork, // CHANGED: Replaced FungalGrid
    predators: Vec<PredatorZone>,
    season: SeasonCycle,
    config: SimConfig,
    width: f32,
    height: f32,
    event_cooldown: f32,
    last_season: &'static str,
}

const BOID_SIZE: f32 = 6.0;
const VISION_RADIUS: f32 = 60.0;

fn scan_dom_obstacles(document: &Document) -> Vec<Obstacle> {
    let mut obstacles = Vec::new();
    let elements = document.get_elements_by_class_name("monolith");
    
    for i in 0..elements.length() {
        if let Some(element) = elements.item(i) {
            let rect = element.get_bounding_client_rect();
            let center_x = rect.left() as f32 + rect.width() as f32 / 2.0;
            let center_y = rect.top() as f32 + rect.height() as f32 / 2.0;
            let radius = (rect.width().max(rect.height()) as f32) / 2.0;
            
            obstacles.push(Obstacle {
                center: Vec2::new(center_x, center_y),
                radius,
            });
        }
    }
    obstacles
}

fn is_paused() -> bool {
    let window = window().unwrap();
    let location = window.location();
    if let Ok(search) = location.search() {
        search.contains("paused=true")
    } else {
        false
    }
}

fn draw_robot_boid(ctx: &CanvasRenderingContext2d, x: f64, y: f64, angle: f64, color: &str, size: f64) {
    ctx.save();
    ctx.translate(x, y).unwrap();
    ctx.rotate(angle).unwrap();
    
    ctx.set_stroke_style(&JsValue::from_str(color));
    ctx.set_line_width(1.5);
    
    // Chevron / Drone Shape
    //   \
    //   /
    ctx.begin_path();
    ctx.move_to(-size, -size * 0.8);
    ctx.line_to(size, 0.0);
    ctx.line_to(-size, size * 0.8);
    ctx.line_to(-size * 0.5, 0.0); // Indent at back
    ctx.close_path();
    
    ctx.stroke();
    
    // Engine glow
    ctx.set_fill_style(&JsValue::from_str("rgba(255, 255, 255, 0.8)"));
    ctx.begin_path();
    ctx.arc(-size * 0.5, 0.0, size * 0.3, 0.0, std::f64::consts::TAU).unwrap();
    ctx.fill();

    ctx.restore();
}

fn main() {
    console_error_panic_hook::set_once();

    let window = window().unwrap();
    let document = window.document().unwrap();
    let canvas = document
        .get_element_by_id("simulation")
        .unwrap()
        .dyn_into::<HtmlCanvasElement>()
        .unwrap();

    let paused = is_paused();

    let w = window.inner_width().unwrap().as_f64().unwrap();
    let h = window.inner_height().unwrap().as_f64().unwrap();
    canvas.set_width(w as u32);
    canvas.set_height(h as u32);

    // Resize handler
    {
        let canvas = canvas.clone();
        let window_for_closure = window.clone();
        let closure = Closure::wrap(Box::new(move || {
            let w = window_for_closure.inner_width().unwrap().as_f64().unwrap();
            let h = window_for_closure.inner_height().unwrap().as_f64().unwrap();
            canvas.set_width(w as u32);
            canvas.set_height(h as u32);
        }) as Box<dyn FnMut()>);
        window.add_event_listener_with_callback("resize", closure.as_ref().unchecked_ref()).unwrap();
        closure.forget();
    }

    let ctx = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()
        .unwrap();

    let width = w as f32;
    let height = h as f32;

    // Initialize arena with starting population
    let mut arena: BoidArena<ARENA_CAPACITY> = BoidArena::new();
    let mut rng = rand::thread_rng();
    use rand::Rng;
    
    for _ in 0..150 {
        let pos = Vec2::new(
            rng.gen_range(0.0..width),
            rng.gen_range(0.0..height),
        );
        let vel = Vec2::new(
            rng.gen_range(-1.0..1.0),
            rng.gen_range(-1.0..1.0),
        );
        arena.spawn(pos, vel, Genome::random());
    }

    let grid = SpatialGrid::new(width, height, VISION_RADIUS);
    let obstacles = scan_dom_obstacles(&document);
    
    // No fixed food sources anymore, boids forage the network
    let food_sources = vec![];

    // Initialize Fungal Network
    let mut fungal_network = FungalNetwork::new(width, height);
    
    // Initial seeding
    for _ in 0..10 {
        fungal_network.spawn_root();
    }

    let mut config = SimConfig::default();
    config.reproduction_threshold = 140.0;
    config.base_mortality = 0.00005;

    let state = Rc::new(RefCell::new(World {
        arena,
        grid,
        obstacles,
        food_sources,
        fungal_network,
        predators: Vec::new(),
        season: SeasonCycle::new(),
        config,
        width,
        height,
        event_cooldown: 0.0,
        last_season: "SPRING",
    }));

    // Cache DOM element references
    let stat_pop = document.get_element_by_id("stat-pop");
    let stat_gen = document.get_element_by_id("stat-gen");
    let stat_fps = document.get_element_by_id("stat-fps");

    let performance: Performance = window.performance().unwrap();

    let f: Rc<RefCell<Option<Closure<dyn FnMut()>>>> = Rc::new(RefCell::new(None));
    let g = f.clone();

    let state_clone = state.clone();
    let document_clone = document.clone();
    let mut frame_count: u32 = 0;
    let mut last_time = performance.now();
    let mut fps_accumulator = 0.0;
    let mut fps_frame_count = 0;
    let mut stats = SimulationStats {
        max_speed_record: 0.0,
        max_generation: 0,
        total_births: 0,
        total_deaths: 0,
    };
    
    *g.borrow_mut() = Some(Closure::new(move || {
        let mut s = state_clone.borrow_mut();
        frame_count += 1;
        
        // FPS calculation
        let current_time = performance.now();
        let delta = current_time - last_time;
        last_time = current_time;
        fps_accumulator += delta;
        fps_frame_count += 1;
        
        // Rescan DOM obstacles occasionally
        if frame_count % 60 == 0 {
            s.obstacles = scan_dom_obstacles(&document_clone);
        }
        
        // Update dashboard every 30 frames
        if frame_count % 30 == 0 {
            let alive_count = s.arena.alive_count;
            
            if let Some(ref el) = stat_pop {
                el.set_text_content(Some(&format!("POP: {}", alive_count)));
            }
            
            // Find max generation
            let mut max_gen: u16 = 0;
            let mut max_speed: f32 = 0.0;
            for idx in s.arena.iter_alive() {
                max_gen = max_gen.max(s.arena.generation[idx]);
                max_speed = max_speed.max(s.arena.genes[idx].max_speed);
            }
            
            if let Some(ref el) = stat_gen {
                el.set_text_content(Some(&format!("GEN: {}", max_gen)));
            }
            
            if fps_frame_count > 0 && fps_accumulator > 0.0 {
                let avg_fps = (fps_frame_count as f64 / fps_accumulator) * 1000.0;
                if let Some(ref el) = stat_fps {
                    el.set_text_content(Some(&format!("FPS: {:.0}", avg_fps)));
                }
                fps_accumulator = 0.0;
                fps_frame_count = 0;
            }
            
            // Log events
            if max_speed > stats.max_speed_record + 0.1 {
                stats.max_speed_record = max_speed;
                log_event(&document_clone, &format!("⚡ SPEED RECORD: {:.2}", max_speed), "event-record");
            }
            
            if max_gen > stats.max_generation {
                stats.max_generation = max_gen;
                if max_gen % 5 == 0 {
                    log_event(&document_clone, &format!("🧬 GEN {} reached", max_gen), "event-birth");
                }
            }
        }

        // Update canvas dimensions
        let canvas_w = ctx.canvas().unwrap().width() as f32;
        let canvas_h = ctx.canvas().unwrap().height() as f32;
        
        if s.width != canvas_w || s.height != canvas_h {
            s.width = canvas_w;
            s.height = canvas_h;
            s.grid.resize(canvas_w, canvas_h);
            s.fungal_network.resize(canvas_w, canvas_h);
        }

        // === SIMULATION STEP ===
        
        // Destructure to get separate borrows
        let World { 
            arena, 
            grid, 
            obstacles, 
            food_sources,
            fungal_network,
            predators,
            season,
            config, 
            width: world_w, 
            height: world_h,
            event_cooldown,
            last_season,
            ..
        } = &mut *s;
        
        // Update season
        season.update(1.0);
        
        // Check for season change
        let current_season = season.season_name();
        if current_season != *last_season {
            *last_season = current_season;
            log_event(&document_clone, &format!("🌍 {} has arrived!", current_season), "event-record");
            
            // Winter is harsh
            if current_season == "WINTER" {
                log_event(&document_clone, "❄ Resources are scarce...", "event-death");
            } else if current_season == "SUMMER" {
                log_event(&document_clone, "☀ Abundance! Food plentiful!", "event-birth");
            }
        }
        
        // Update Fungal Network
        fungal_network.update();
        
        // Boids interactions with network
        // 1. Spore Trail: Chance to seed new root at boid pos
        // 2. Infection: Kill nodes within radius
        
        for idx in arena.iter_alive() {
            let pos = arena.positions[idx];
            
            // Seed (Spore)
            // Small chance
            use rand::Rng;
            let mut rng = rand::thread_rng();
            if rng.gen::<f32>() < 0.005 {
                fungal_network.seed_at(pos);
            }
            
            // Infect (Shrink/Kill)
            // Only check if we are actually near nodes (Spatial hash would be better, but brute force for now with MAX_NODES=2000 is okayish)
            // For perf, maybe only check subset of boids?
            // Or limit check frequency?
            if frame_count % 2 == 0 {
                fungal_network.infect(pos, BOID_SIZE * 3.0);
            }
        }
        
        // Update predators
        for pred in predators.iter_mut() {
            pred.update(1.0);
        }
        predators.retain(|p| p.active);
        
        // 1. Build spatial grid
        grid.build(arena);
        
        // 2. Compute flocking forces (writes to arena.scratch_accel)
        compute_flocking_forces(arena, grid, VISION_RADIUS, obstacles);
        
        // 3. Feed from food sources (season-affected)
        // Replaced by feeding from fungal network? 
        // For now, let's keep food_sources empty and maybe add logic later to feed from network nodes.
        // feed_from_sources(arena, food_sources, season);
        
        // Obstacle feeding - still works near monoliths
        let obstacle_feeders: Vec<usize> = (0..ARENA_CAPACITY)
            .filter(|&idx| arena.alive[idx])
            .filter(|&idx| {
                obstacles.iter().any(|obs| {
                    arena.positions[idx].distance(obs.center) < 150.0
                })
            })
            .collect();
        
        for idx in obstacle_feeders {
            arena.energy[idx] = (arena.energy[idx] + 0.8 * season.food_multiplier()).min(200.0);
        }
        
        // Apply predator damage
        let predator_kills = apply_predator_zones(arena, predators);
        if predator_kills > 0 {
            log_event(&document_clone, &format!("🩸 Predator claimed {} victims!", predator_kills), "event-death");
        }
        
        // 4. Run simulation step (movement, reproduction, death)
        let (births, deaths) = simulation_step(
            arena,
            grid,
            config,
            *world_w,
            *world_h,
            1.0,
        );
        
        if deaths > 15 {
            log_event(&document_clone, &format!("☠ {} died", deaths), "event-death");
        }
        
        let _ = births; // Suppress unused warnings

        // === RENDERING ===
        
        // Background
        ctx.set_fill_style(&JsValue::from_str("#0a0a12"));
        ctx.fill_rect(0.0, 0.0, canvas_w as f64, canvas_h as f64);
        
        // Draw Fungal Network
        fungal_network.draw(&ctx);
        
        // Draw predators
        for pred in &s.predators {
            if !pred.active { continue; }
            
            let pulse = 0.5 + 0.5 * (pred.lifetime * 5.0).sin();
            let alpha = 0.3 * pulse;
            
            // Tech Danger Zone
            ctx.set_stroke_style(&JsValue::from_str(&format!("rgba(255, 0, 50, {})", alpha)));
            ctx.set_line_width(2.0);
            ctx.begin_path();
            ctx.arc(pred.position.x as f64, pred.position.y as f64, pred.radius as f64, 0.0, std::f64::consts::TAU).unwrap();
            ctx.stroke();
        }

        // Draw Robots (Boids)
        for idx in s.arena.iter_alive() {
            let pos = s.arena.positions[idx];
            let vel = s.arena.velocities[idx];
            let angle = vel.y.atan2(vel.x);
            let (hue, sat, light) = get_boid_color(&s.arena, idx);
            
            let color = format!("hsl({}, {}%, {}%)", hue, sat, light);
            draw_robot_boid(&ctx, pos.x as f64, pos.y as f64, angle as f64, &color, BOID_SIZE as f64);
        }
        
        // Trails
        ctx.set_global_alpha(0.2);
        for idx in s.arena.iter_alive() {
            if s.arena.energy[idx] > 100.0 {
                let pos = s.arena.positions[idx];
                let vel = s.arena.velocities[idx];
                let speed = vel.length();
                if speed > 2.0 {
                    let trail_end = pos - vel.normalize() * speed * 8.0; // Longer trails
                    
                    ctx.begin_path();
                    ctx.move_to(pos.x as f64, pos.y as f64);
                    ctx.line_to(trail_end.x as f64, trail_end.y as f64);
                    
                    let (h, s_val, l) = get_boid_color(&s.arena, idx);
                    ctx.set_stroke_style(&JsValue::from_str(&format!("hsl({}, {}%, {}%)", h, s_val, l)));
                    ctx.set_line_width(1.0);
                    ctx.stroke();
                }
            }
        }
        ctx.set_global_alpha(1.0);

        if !paused {
            web_sys::window()
                .unwrap()
                .request_animation_frame(f.borrow().as_ref().unwrap().as_ref().unchecked_ref())
                .unwrap();
        }
    }));

    window
        .request_animation_frame(g.borrow().as_ref().unwrap().as_ref().unchecked_ref())
        .unwrap();
}
