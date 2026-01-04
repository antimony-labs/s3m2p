//! ═══════════════════════════════════════════════════════════════════════════════
//! FILE: lessons.rs | SWARM_ROBOTICS/src/lessons.rs
//! PURPOSE: Swarm Robotics lesson definitions - 20 lessons across 7 phases
//! MODIFIED: 2025-01-XX
//! LAYER: LEARN → SWARM_ROBOTICS
//! ═══════════════════════════════════════════════════════════════════════════════

/// A single Swarm Robotics lesson
pub struct Lesson {
    pub id: usize,
    pub title: &'static str,
    pub subtitle: &'static str,
    pub icon: &'static str,
    pub phase: &'static str,
    /// The hook - why should I care? (1-2 sentences)
    pub why_it_matters: &'static str,
    /// Intuitive explanation - no jargon (2-3 paragraphs)
    pub intuition: &'static str,
    /// What the demo shows
    pub demo_explanation: &'static str,
    /// Key takeaways (what should stick)
    pub key_takeaways: &'static [&'static str],
    /// For those who want to go deeper
    pub going_deeper: &'static str,
    /// Mathematical notation (optional, hidden by default)
    pub math_details: &'static str,
    /// Implementation guide with code prompts and hardware examples
    pub implementation: &'static str,
}

/// Swarm Robotics learning phases
pub static PHASES: &[&str] = &[
    "Welcome to Swarms",
    "Local Rules → Emergent Motion",
    "Consensus (The Backbone)",
    "Coordinated Motion & Formations",
    "Task Allocation",
    "Coverage & Exploration",
    "Robustness & Capstone",
];

/// All Swarm Robotics lessons - ordered from simple intuition to complex algorithms
pub static LESSONS: &[Lesson] = &[
    // ═══════════════════════════════════════════════════════════════════════════
    // PHASE 0: Welcome to Swarms (Onboarding + primitives)
    // ═══════════════════════════════════════════════════════════════════════════
    Lesson {
        id: 0,
        title: "No Boss, No Problem",
        subtitle: "Emergence from Local Rules",
        icon: "🐜",
        phase: "Welcome to Swarms",
        why_it_matters: "Ants don't have WiFi, but they still out-schedule your team. How? \
                         They follow simple local rules that create global intelligence.",
        intuition: "<h3>The Ant Colony Analogy</h3>\n\
            Watch an ant colony. No ant is in charge. No ant has a map. Yet they find food, \
            build nests, and coordinate thousands of workers.<br><br>\n\
            <strong>The Secret:</strong> Each ant follows three rules:\n\
            <ol>\n\
            <li>If you see food, pick it up and head home</li>\n\
            <li>If you smell pheromone, follow it</li>\n\
            <li>If you're carrying food, drop pheromone</li>\n\
            </ol>\n\
            <strong>Emergence:</strong> These simple rules create complex behavior. The colony \
            'decides' which path is shortest. The colony 'assigns' workers to tasks. But no \
            single ant knows the plan.<br><br>\n\
            <strong>In Robotics:</strong> We'll learn to program robots the same way—give each \
            robot simple rules, and watch the swarm solve problems no single robot could.",
        demo_explanation: r#"
            <strong>🎮 Try This:</strong>
            <ol>
            <li><strong>Toggle Rules:</strong> Turn off separation. Watch robots collide!</li>
            <li><strong>Turn Off Alignment:</strong> See chaos—no coordinated motion.</li>
            <li><strong>All Rules On:</strong> Suddenly, order emerges. Flocking appears!</li>
            <li><strong>Challenge:</strong> Reach stable flock with zero collisions in 30 seconds.</li>
            </ol>
            <br>
            <strong>Key Insight:</strong> No robot knows the global plan. Each follows local rules. \
            The swarm's behavior emerges from these interactions.
        "#,
        key_takeaways: &[
            "Emergence: Complex global behavior from simple local rules",
            "No central controller needed—distributed intelligence",
            "Each robot only needs to sense neighbors, not the whole swarm",
        ],
        going_deeper: "<strong>In Nature:</strong> Flocking birds, schooling fish, and ant colonies \
                       all use emergence. Biologists call this 'swarm intelligence.'<br><br>\
                       <strong>In Engineering:</strong> Swarm robotics enables scalable systems. \
                       Add 100 robots? Just spawn 100 agents—no redesign needed.",
        math_details: r#"
<h4>State Update Loop</h4>
<p>Each robot updates its state based on local rules:</p>

$$p_i(t+1) = p_i(t) + v_i(t) \cdot \Delta t$$

$$v_i(t+1) = v_i(t) + a_i(t) \cdot \Delta t$$

<p><strong>Where:</strong></p>
<ul>
<li>$p_i$ = Position of robot $i$</li>
<li>$v_i$ = Velocity of robot $i$</li>
<li>$a_i$ = Acceleration (from local rules)</li>
<li>$\Delta t$ = Time step</li>
</ul>

<h4>Bounded Actuation</h4>
<p>Real robots have limits:</p>

$$|v_i| \leq v_{max}, \quad |a_i| \leq a_{max}$$

<p>This prevents infinite acceleration and models physical constraints.</p>
        "#,
        implementation: r#"
<h4>Basic Agent Structure</h4>
<pre>
struct Agent {
    pos: Vec2,
    vel: Vec2,
    max_speed: f32,
    max_accel: f32,
}

impl Agent {
    fn update(&mut self, acceleration: Vec2, dt: f32) {
        // Clamp acceleration
        let accel = acceleration.normalize() * acceleration.length().min(self.max_accel);
        
        // Update velocity
        self.vel += accel * dt;
        
        // Clamp speed
        if self.vel.length() > self.max_speed {
            self.vel = self.vel.normalize() * self.max_speed;
        }
        
        // Update position
        self.pos += self.vel * dt;
    }
}
</pre>

<h4>LLM Prompt: First Swarm</h4>
<pre>"Create a Rust struct SwarmWorld with 50 agents in a 1x1 unit square.
Each agent starts at random position with random velocity.
Implement Euler integration step() that updates all agents.
Add boundary wrapping (torus world)."</pre>
        "#,
    },
    Lesson {
        id: 1,
        title: "What Does a Robot Know?",
        subtitle: "Local Sensing & Partial Observability",
        icon: "👁️",
        phase: "Welcome to Swarms",
        why_it_matters: "A robot can't see everything. It only knows what's nearby. \
                         This limitation is actually a feature—it enables scalable swarms.",
        intuition: "<h3>The Blindfolded Party Analogy</h3>\n\
            Imagine you're at a party, blindfolded. You can only:\n\
            <ul>\n\
            <li>Hear voices within 3 meters</li>\n\
            <li>Feel if someone bumps into you</li>\n\
            <li>Know your own position relative to the room</li>\n\
            </ul>\n\
            <strong>What you CAN'T do:</strong> See the whole party. Know where everyone is. \
            See the exit.<br><br>\n\
            <strong>What you CAN do:</strong> Follow nearby voices. Avoid collisions. \
            Eventually find the exit by exploring.<br><br>\n\
            <strong>In Robotics:</strong> Each robot has a 'sensing radius.' It only knows \
            neighbors within that radius. This is 'partial observability'—you see part of \
            the world, not all of it.",
        demo_explanation: r#"
            <strong>Visualization:</strong>
            <ul>
            <li><strong>Circle around robot:</strong> Sensing radius</li>
            <li><strong>Lines:</strong> Connections to neighbors within radius</li>
            <li><strong>Gray robots:</strong> Outside sensing range (unknown)</li>
            </ul>
            <br>
            <strong>🎮 Try This:</strong>
            <ol>
            <li><strong>Shrink Radius:</strong> Reduce sensing radius. Watch the swarm fragment!</li>
            <li><strong>Expand Radius:</strong> Increase it. More connections = better coordination.</li>
            <li><strong>Noisy Sensing:</strong> Add noise to distance measurements. See how it affects behavior.</li>
            </ol>
            <br>
            <strong>Key Insight:</strong> Local sensing enables scalability. Add 1000 robots? \
            Each still only processes ~10 neighbors.
        "#,
        key_takeaways: &[
            "Partial observability: robots only sense nearby neighbors",
            "Sensing radius determines connectivity and coordination",
            "Noise in sensing creates uncertainty that must be handled",
        ],
        going_deeper: "<strong>In Nature:</strong> Fish schools use lateral line sensing—they \
                       feel water pressure from nearby fish, not vision of the whole school.<br><br>\
                       <strong>In Engineering:</strong> Local sensing reduces communication overhead. \
                       Instead of broadcasting to all robots, each robot only talks to neighbors.",
        math_details: r#"
<h4>Neighbor Set</h4>
<p>Robot $i$ can sense neighbors within radius $r$:</p>

$$N_i = \{j : \|p_i - p_j\| \leq r\}$$

<p><strong>Where:</strong></p>
<ul>
<li>$N_i$ = Set of neighbors of robot $i$</li>
<li>$r$ = Sensing radius</li>
<li>$p_i, p_j$ = Positions of robots $i$ and $j$</li>
</ul>

<h4>Noisy Measurements</h4>
<p>Real sensors have noise:</p>

$$\tilde{d}_{ij} = \|p_i - p_j\| + \epsilon$$

<p>Where $\epsilon \sim \mathcal{N}(0, \sigma^2)$ is Gaussian noise.</p>

<h4>Connectivity</h4>
<p>The neighbor graph is connected if there's a path between any two robots:</p>

$$\text{Connected} \Leftrightarrow \forall i,j \exists \text{path } i \to j$$

<p>If the graph disconnects, consensus becomes impossible.</p>
        "#,
        implementation: r#"
<h4>Neighbor Search</h4>
<pre>
impl SwarmWorld {
    fn find_neighbors(&self, agent_id: usize, radius: f32) -> Vec<usize> {
        let agent = &self.agents[agent_id];
        let mut neighbors = Vec::new();
        
        for (j, other) in self.agents.iter().enumerate() {
            if j == agent_id { continue; }
            
            let dist = agent.pos.distance(other.pos);
            if dist <= radius {
                neighbors.push(j);
            }
        }
        
        neighbors
    }
}
</pre>

<h4>LLM Prompt: Spatial Hash</h4>
<pre>"Optimize neighbor search using uniform grid spatial hashing.
Divide world into cells of size 'radius'.
For each agent, only check neighbors in its cell + 8 adjacent cells.
This reduces O(n²) to O(n) for large swarms."</pre>
        "#,
    },
    Lesson {
        id: 2,
        title: "Graphs Are The Swarm",
        subtitle: "Communication Topology",
        icon: "🕸️",
        phase: "Welcome to Swarms",
        why_it_matters: "The communication graph IS the algorithm. Change the topology, \
                         change the behavior. Understanding graphs is understanding swarms.",
        intuition: r#"<h3>The Telephone Game Analogy</h3>
            Remember the telephone game? One person whispers to the next, message spreads 
            through the chain. But what if the chain breaks?<br><br>
            <strong>In Swarms:</strong> Each robot is a node. Each connection is an edge. 
            The 'graph' is who talks to whom.<br><br>
            <strong>Key Insight:</strong> The graph structure determines EVERYTHING:
            <ul>
            <li><strong>Connected graph:</strong> Information spreads everywhere</li>
            <li><strong>Disconnected:</strong> Swarm splits into isolated groups</li>
            <li><strong>Dense graph:</strong> Fast consensus, but lots of communication</li>
            <li><strong>Sparse graph:</strong> Slow consensus, but efficient</li>
            </ul>
            <strong>The Math:</strong> We'll use graph Laplacian $L$ to analyze connectivity. 
            The second-smallest eigenvalue $\lambda_2$ tells us how 'well-connected' the graph is."#,
        demo_explanation: r#"
            <strong>Visualization:</strong>
            <ul>
            <li><strong>Nodes:</strong> Robots (circles)</li>
            <li><strong>Edges:</strong> Communication links (lines)</li>
            <li><strong>Colors:</strong> Different connected components</li>
            </ul>
            <br>
            <strong>🎮 Try This:</strong>
            <ol>
            <li><strong>Watch Formation:</strong> As robots move, edges appear/disappear</li>
            <li><strong>Break Graph:</strong> Reduce sensing radius until graph splits</li>
            <li><strong>Count Components:</strong> See how many isolated groups form</li>
            <li><strong>Challenge:</strong> Keep graph connected while minimizing edges</li>
            </ol>
            <br>
            <strong>Key Insight:</strong> The graph topology determines if consensus is possible. \
            If disconnected, different groups will converge to different values.
        "#,
        key_takeaways: &[
            "Graph = nodes (robots) + edges (communication links)",
            "Connected graph is required for global consensus",
            "Laplacian matrix $L$ encodes graph structure",
            r#"$\lambda_2$ (algebraic connectivity) measures how well-connected"#,
        ],
        going_deeper: "<strong>In Theory:</strong> Graph theory is the mathematical foundation \
                       of distributed algorithms. Many swarm behaviors reduce to graph problems.<br><br>\
                       <strong>In Practice:</strong> Network topology affects everything—consensus speed, \
                       fault tolerance, energy consumption. Design the graph, design the behavior.",
        math_details: r#"
<h4>Adjacency Matrix</h4>
<p>For graph with $n$ nodes, adjacency matrix $A$ is:</p>

$$A_{ij} = \begin{cases}
1 & \text{if } (i,j) \text{ is an edge} \\
0 & \text{otherwise}
\end{cases}$$

<h4>Degree Matrix</h4>
<p>Diagonal matrix with node degrees:</p>

$$D_{ii} = \sum_j A_{ij}$$

<h4>Laplacian Matrix</h4>
<p>The graph Laplacian is:</p>

$$L = D - A$$

<p><strong>Properties:</strong></p>
<ul>
<li>$L$ is symmetric and positive semi-definite</li>
<li>Smallest eigenvalue: $\lambda_1 = 0$ (always)</li>
<li>Second-smallest: $\lambda_2 > 0$ if and only if graph is connected</li>
<li>$\lambda_2$ is called 'algebraic connectivity'</li>
</ul>

<h4>Why $\lambda_2$ Matters</h4>
<p>For consensus, convergence rate is proportional to $\lambda_2$:</p>

$$\text{Error} \propto e^{-\lambda_2 t}$$

<p>Larger $\lambda_2$ = faster consensus.</p>
        "#,
        implementation: r#"
<h4>Build Adjacency Matrix</h4>
<pre>
fn build_adjacency(agents: &[Agent], radius: f32) -> Vec<Vec<bool>> {
    let n = agents.len();
    let mut adj = vec![vec![false; n]; n];
    
    for i in 0..n {
        for j in (i+1)..n {
            let dist = agents[i].pos.distance(agents[j].pos);
            if dist <= radius {
                adj[i][j] = true;
                adj[j][i] = true;
            }
        }
    }
    
    adj
}
</pre>

<h4>Compute Laplacian</h4>
<pre>
fn compute_laplacian(adj: &[Vec<bool>]) -> Vec<Vec<f32>> {
    let n = adj.len();
    let mut L = vec![vec![0.0; n]; n];
    
    for i in 0..n {
        let degree = adj[i].iter().filter(|&&x| x).count() as f32;
        L[i][i] = degree;
        
        for j in 0..n {
            if adj[i][j] {
                L[i][j] = -1.0;
            }
        }
    }
    
    L
}
</pre>

<h4>LLM Prompt: Find Components</h4>
<pre>"Implement DFS (depth-first search) to find connected components of a graph.
Return vector of component IDs for each node.
Use this to detect if swarm has split into isolated groups."</pre>
        "#,
    },
    // ═══════════════════════════════════════════════════════════════════════════
    // PHASE 1: Local Rules → Emergent Motion
    // ═══════════════════════════════════════════════════════════════════════════
    Lesson {
        id: 3,
        title: "Boids Flocking",
        subtitle: "Separation, Alignment, Cohesion",
        icon: "🐦",
        phase: "Local Rules → Emergent Motion",
        why_it_matters: "Boids created realistic flocking in 1986 with just three rules. \
                         It's the foundation of swarm motion—and it's beautiful.",
        intuition: "<h3>The Three Rules</h3>\n\
            <strong>1. Separation:</strong> Don't crowd neighbors. Steer away if too close.<br>\n\
            <strong>2. Alignment:</strong> Match velocity of neighbors. Fly in the same direction.<br>\n\
            <strong>3. Cohesion:</strong> Steer toward average position of neighbors. Stay with the group.<br><br>\n\
            <strong>The Magic:</strong> These three forces, weighted and combined, create \
            realistic flocking. No robot plans the path. No robot knows the destination. \
            Yet the swarm moves as one.<br><br>\n\
            <strong>In Nature:</strong> This matches how birds actually flock. Biologists \
            confirmed boids matches real bird behavior.",
        demo_explanation: r#"
            <strong>🎮 Try This:</strong>
            <ol>
            <li><strong>Separation Only:</strong> Set cohesion=0, alignment=0. Robots scatter!</li>
            <li><strong>Cohesion Only:</strong> Set separation=0. Robots clump into a ball.</li>
            <li><strong>All Three:</strong> Perfect flocking emerges.</li>
            <li><strong>Add Obstacles:</strong> Watch the swarm flow around barriers.</li>
            <li><strong>Challenge:</strong> Navigate through obstacle field without collisions.</li>
            </ol>
            <br>
            <strong>Key Insight:</strong> Weighted combination of simple forces creates complex behavior.
        "#,
        key_takeaways: &[
            "Separation: avoid crowding neighbors",
            "Alignment: match neighbor velocities",
            "Cohesion: steer toward group center",
            "Weighted combination creates emergent flocking",
        ],
        going_deeper: "<strong>History:</strong> Craig Reynolds created boids in 1986 for \
                       computer graphics. It revolutionized animation.<br><br>\
                       <strong>Applications:</strong> Used in movies (Batman Returns, Lion King), \
                       games (crowd simulation), and robotics (UAV swarms).",
        math_details: r#"
<h4>Separation Force</h4>
<p>Steer away from neighbors that are too close:</p>

$$a_i^{sep} = k_{sep} \sum_{j \in N_i} \frac{p_i - p_j}{\|p_i - p_j\|^2 + \epsilon}$$

<p>Where $\epsilon$ prevents division by zero.</p>

<h4>Alignment Force</h4>
<p>Match average velocity of neighbors:</p>

$$a_i^{ali} = k_{ali} \left( \frac{1}{|N_i|} \sum_{j \in N_i} v_j - v_i \right)$$

<h4>Cohesion Force</h4>
<p>Steer toward center of neighbor group:</p>

$$a_i^{coh} = k_{coh} \left( \frac{1}{|N_i|} \sum_{j \in N_i} p_j - p_i \right)$$

<h4>Total Acceleration</h4>
<p>Weighted sum:</p>

$$a_i = a_i^{sep} + a_i^{ali} + a_i^{coh}$$

<p>Typical weights: $k_{sep} = 1.5$, $k_{ali} = 1.0$, $k_{coh} = 0.8$</p>
        "#,
        implementation: r#"
<h4>Boids Implementation</h4>
<pre>
fn compute_boids_forces(agent: &Agent, neighbors: &[&Agent], 
                         k_sep: f32, k_ali: f32, k_coh: f32) -> Vec2 {
    let mut sep = Vec2::ZERO;
    let mut ali = Vec2::ZERO;
    let mut coh = Vec2::ZERO;
    
    let n = neighbors.len() as f32;
    if n == 0.0 { return Vec2::ZERO; }
    
    for neighbor in neighbors {
        let diff = agent.pos - neighbor.pos;
        let dist_sq = diff.length_squared() + 0.01; // epsilon
        
        // Separation
        sep += diff / dist_sq;
        
        // Alignment
        ali += neighbor.vel;
        
        // Cohesion
        coh += neighbor.pos;
    }
    
    sep = sep.normalize() * k_sep;
    ali = (ali / n - agent.vel).normalize() * k_ali;
    coh = ((coh / n) - agent.pos).normalize() * k_coh;
    
    sep + ali + coh
}
</pre>

<h4>LLM Prompt: Obstacle Avoidance</h4>
<pre>"Add obstacle avoidance to boids:
- For each obstacle, compute repulsive force
- Force magnitude: k / (distance - radius)²
- Add to total acceleration
- Test with circular obstacles"</pre>
        "#,
    },
    Lesson {
        id: 4,
        title: "Potential Fields",
        subtitle: "Attract & Repel",
        icon: "🧲",
        phase: "Local Rules → Emergent Motion",
        why_it_matters: "Potential fields create natural motion—robots flow like water. \
                         But they can get stuck. Understanding why is key to robust swarms.",
        intuition: "<h3>The Magnet Analogy</h3>\n\
            Imagine robots are magnets:\n\
            <ul>\n\
            <li><strong>Repulsion:</strong> Like poles repel. Robots push each other away.</li>\n\
            <li><strong>Attraction:</strong> Opposite poles attract. Robots pull together.</li>\n\
            </ul>\n\
            <strong>Potential Energy:</strong> We define a 'potential' function $U(p)$ that \
            represents energy at each point. Robots move downhill (negative gradient) to \
            minimize energy.<br><br>\n\
            <strong>The Problem:</strong> Potential fields can have local minima—valleys \
            where robots get stuck. Like a ball rolling into a depression, robots can't escape.",
        demo_explanation: r#"
            <strong>🎮 Try This:</strong>
            <ol>
            <li><strong>Pure Repulsion:</strong> Watch robots spread into a ring</li>
            <li><strong>Pure Attraction:</strong> See them clump into a tight cluster</li>
            <li><strong>Mixed:</strong> Balance creates stable formations</li>
            <li><strong>Get Stuck:</strong> Reduce noise. Watch robots trap in local minima</li>
            <li><strong>Challenge:</strong> Reach target formation without getting stuck</li>
            </ol>
            <br>
            <strong>Key Insight:</strong> Potential fields are elegant but fragile. \
            Local minima are the enemy.
        "#,
        key_takeaways: &[
            "Potential field: energy function $U(p)$",
            r#"Robots follow negative gradient: $a = -\nabla U$"#,
            r#"Repulsion: $U \propto 1/d^p$"#,
            r#"Attraction: $U \propto d^2$"#,
            "Local minima cause robots to get stuck",
        ],
        going_deeper: "<strong>In Theory:</strong> Potential fields are gradient descent on \
                       an energy landscape. Local minima are inherent to non-convex optimization.<br><br>\
                       <strong>In Practice:</strong> Add noise, random walks, or escape heuristics \
                       to avoid getting stuck.",
        math_details: r#"
<h4>Repulsive Potential</h4>
<p>Forces robots apart:</p>

$$U_{rep}(d) = \frac{k_{rep}}{d^p}$$

<p>Where $d$ is distance, $p$ is power (typically 2 or 3).</p>

<h4>Attractive Potential</h4>
<p>Forces robots together:</p>

$$U_{att}(d) = k_{att} \cdot d^2$$

<h4>Gradient Descent</h4>
<p>Robot acceleration follows negative gradient:</p>

$$a_i = -\nabla_{p_i} \sum_{j \neq i} U(\|p_i - p_j\|)$$

<h4>Local Minima</h4>
<p>A local minimum occurs when:</p>

$$\nabla U = 0, \quad \nabla^2 U > 0$$

<p>At these points, robots have zero acceleration but aren't at the global minimum.</p>
        "#,
        implementation: r#"
<h4>Potential Field Implementation</h4>
<pre>
fn compute_potential_force(agent: &Agent, neighbors: &[&Agent],
                           k_rep: f32, k_att: f32, p: f32) -> Vec2 {
    let mut force = Vec2::ZERO;
    
    for neighbor in neighbors {
        let diff = agent.pos - neighbor.pos;
        let dist = diff.length().max(0.01);
        
        // Repulsion
        let rep_mag = k_rep / dist.powi(p as i32);
        force += diff.normalize() * rep_mag;
        
        // Attraction
        let att_mag = k_att * dist;
        force -= diff.normalize() * att_mag;
    }
    
    force
}
</pre>

<h4>LLM Prompt: Escape Local Minima</h4>
<pre>"Add escape mechanism for local minima:
- Detect when robot velocity is near zero but force is non-zero
- Add random 'kick' force with probability p
- Or implement wall-following heuristic
- Test with U-shaped obstacle"</pre>
        "#,
    },
    Lesson {
        id: 5,
        title: "Escaping Local Minima",
        subtitle: "Why Your Swarm Gets Stuck",
        icon: "🕳️",
        phase: "Local Rules → Emergent Motion",
        why_it_matters: "Potential fields are elegant, but robots get stuck. \
                         Learning to escape local minima is essential for robust swarms.",
        intuition: "<h3>The Ball in a Bowl Analogy</h3>\n\
            Drop a ball in a bowl. It rolls to the bottom and stops. That's a local minimum—\
            the lowest point nearby, but not the lowest point overall.<br><br>\n\
            <strong>In Swarms:</strong> Robots following potential fields can get trapped in \
            'energy valleys.' They can't escape because all forces point inward.<br><br>\n\
            <strong>Solutions:</strong>\n\
            <ul>\n\
            <li><strong>Add Noise:</strong> Random kicks help escape</li>\n\
            <li><strong>Wall Following:</strong> Follow boundaries to escape</li>\n\
            <li><strong>Random Walk:</strong> Occasional random direction changes</li>\n\
            <li><strong>Hybrid Approach:</strong> Switch to different algorithm when stuck</li>\n\
            </ul>",
        demo_explanation: r#"
            <strong>🎮 Try This:</strong>
            <ol>
            <li><strong>Watch Stuck Robots:</strong> Reduce noise. See robots trap in corners</li>
            <li><strong>Add Noise:</strong> Increase random force. Watch robots escape</li>
            <li><strong>Wall Following:</strong> Enable boundary-following mode</li>
            <li><strong>Challenge:</strong> Navigate U-shaped obstacle without getting stuck</li>
            </ol>
            <br>
            <strong>Key Insight:</strong> Deterministic algorithms can get stuck. \
            Stochasticity (randomness) enables escape.
        "#,
        key_takeaways: &[
            "Local minima trap robots in potential fields",
            "Stochasticity (noise) enables escape",
            "Wall-following is a deterministic escape heuristic",
            "Hybrid approaches combine multiple strategies",
        ],
        going_deeper: "<strong>In Theory:</strong> This is the exploration-exploitation tradeoff. \
                       Deterministic = exploit (efficient but can get stuck). \
                       Stochastic = explore (inefficient but finds global optimum).<br><br>\
                       <strong>In Practice:</strong> Most real swarm systems use hybrid approaches—\
                       deterministic most of the time, stochastic when stuck.",
        math_details: r#"
<h4>Detecting Stuck State</h4>
<p>Robot is stuck if velocity is low but force is high:</p>

$$\text{stuck} \Leftrightarrow \|v_i\| < \epsilon_v \text{ and } \|a_i\| > \epsilon_a$$

<h4>Random Walk Escape</h4>
<p>Add random force with probability $p$:</p>

$$a_i \leftarrow a_i + \begin{cases}
\mathcal{N}(0, \sigma^2) & \text{with probability } p \\
0 & \text{otherwise}
\end{cases}$$

<h4>Wall Following</h4>
<p>When stuck, follow boundary tangent:</p>

$$a_i \leftarrow a_i + k_{wall} \cdot t_{wall}$$

<p>Where $t_{wall}$ is tangent to nearest obstacle boundary.</p>
        "#,
        implementation: r#"
<h4>Stuck Detection</h4>
<pre>
fn is_stuck(agent: &Agent, force: Vec2, 
            vel_threshold: f32, force_threshold: f32) -> bool {
    agent.vel.length() < vel_threshold && 
    force.length() > force_threshold
}
</pre>

<h4>Escape Mechanisms</h4>
<pre>
fn escape_local_minima(agent: &mut Agent, rng: &mut Rng,
                      noise_strength: f32, wall_tangent: Option<Vec2>) {
    if is_stuck(agent, ...) {
        // Option 1: Random kick
        let kick = Vec2::from_angle(
            rng.range(0.0, TAU),
            noise_strength
        );
        agent.accel += kick;
        
        // Option 2: Wall following
        if let Some(tangent) = wall_tangent {
            agent.accel += tangent * 0.5;
        }
    }
}
</pre>

<h4>LLM Prompt: Hybrid Controller</h4>
<pre>"Implement hybrid controller:
- Use potential field when not stuck
- Switch to random walk when stuck for >2 seconds
- Switch back to potential field when velocity increases
- Test in U-shaped obstacle field"</pre>
        "#,
    },
    // ═══════════════════════════════════════════════════════════════════════════
    // PHASE 2: Consensus (The Backbone of Coordination)
    // ═══════════════════════════════════════════════════════════════════════════
    Lesson {
        id: 6,
        title: "Average Consensus",
        subtitle: "Agree on a Number Without a Boss",
        icon: "🤝",
        phase: "Consensus (The Backbone)",
        why_it_matters: "How do robots agree on a value without a leader? Consensus algorithms \
                         enable distributed decision-making—the foundation of swarm coordination.",
        intuition: r#"<h3>The Pizza Party Analogy</h3>
            You and friends want to split a pizza. No one is in charge. How do you agree on 
            how many slices each person gets?<br><br>
            <strong>Consensus Algorithm:</strong>
            <ol>
            <li>Each person says their number</li>
            <li>Everyone averages with neighbors</li>
            <li>Repeat until everyone has the same number</li>
            </ol>
            <strong>The Math:</strong> $x_i(t+1) = x_i(t) + \alpha \sum_{j \in N_i} (x_j(t) - x_i(t))$<br><br>
            <strong>Key Insight:</strong> If the communication graph is connected, everyone 
            converges to the average. No leader needed!"#,
        demo_explanation: r#"
            <strong>🎮 Try This:</strong>
            <ol>
            <li><strong>Watch Convergence:</strong> Start with random values. See them converge to average</li>
            <li><strong>Break Graph:</strong> Disconnect graph. See groups converge to different values</li>
            <li><strong>Tune Alpha:</strong> Too high = oscillations. Too low = slow convergence</li>
            <li><strong>Challenge:</strong> Reach consensus error < 0.01 in minimum time</li>
            </ol>
            <br>
            <strong>Key Insight:</strong> Connected graph + proper step size = guaranteed consensus.
        "#,
        key_takeaways: &[
            "Consensus: robots agree on a value without a leader",
            r#"Update rule: $x_i \leftarrow x_i + \alpha \sum_{j \in N_i} (x_j - x_i)$"#,
            "Convergence requires connected communication graph",
            r#"Step size $\alpha$ affects convergence speed and stability"#,
        ],
        going_deeper: "<strong>In Theory:</strong> Consensus is a fundamental distributed algorithm. \
                       Used in sensor networks, distributed computing, and swarm robotics.<br><br>\
                       <strong>In Practice:</strong> Consensus enables swarms to agree on: target \
                       positions, task assignments, formation shapes, and more.",
        math_details: r#"
<h4>Discrete Consensus Update</h4>
<p>Each robot updates its value:</p>

$$x_i(t+1) = x_i(t) + \alpha \sum_{j \in N_i} (x_j(t) - x_i(t))$$

<p>In matrix form:</p>

$$x(t+1) = (I - \alpha L) x(t)$$

<p>Where $L$ is the Laplacian matrix.</p>

<h4>Convergence Condition</h4>
<p>For stability, step size must satisfy:</p>

$$0 < \alpha < \frac{2}{\lambda_{max}(L)}$$

<p>Where $\lambda_{max}$ is the largest eigenvalue of $L$.</p>

<h4>Convergence Rate</h4>
<p>Error decays exponentially:</p>

$$\|x(t) - \bar{x}\| \leq e^{-\alpha \lambda_2 t} \|x(0) - \bar{x}\|$$

<p>Where $\lambda_2$ is the second-smallest eigenvalue (algebraic connectivity).</p>
        "#,
        implementation: r#"
<h4>Consensus Implementation</h4>
<pre>
fn consensus_step(agents: &mut [Agent], alpha: f32, neighbors: &[Vec<usize>]) {
    let mut updates = vec![0.0; agents.len()];
    
    for i in 0..agents.len() {
        let mut sum_diff = 0.0;
        for &j in &neighbors[i] {
            sum_diff += agents[j].value - agents[i].value;
        }
        updates[i] = alpha * sum_diff;
    }
    
    for i in 0..agents.len() {
        agents[i].value += updates[i];
    }
}
</pre>

<h4>LLM Prompt: Convergence Detection</h4>
<pre>"Add convergence detection:
- Compute consensus error: stddev of all values
- Stop when error < threshold
- Return number of iterations to convergence
- Test with different graph topologies"</pre>
        "#,
    },
    Lesson {
        id: 7,
        title: "Laplacian Flow",
        subtitle: "Continuous-Time Consensus",
        icon: "🌊",
        phase: "Consensus (The Backbone)",
        why_it_matters: "Continuous-time consensus is smoother and easier to analyze. \
                         It's the foundation for many advanced swarm algorithms.",
        intuition: r#"<h3>The Thermostat Analogy</h3>
            Imagine rooms connected by open doors. Each room has a different temperature. 
            Heat flows from hot to cold. Eventually, all rooms reach the same temperature.<br><br>
            <strong>In Swarms:</strong> Instead of discrete updates, robots continuously adjust. 
            The rate of change is proportional to the difference with neighbors.<br><br>
            <strong>The Math:</strong> $\dot{x}_i = -\sum_{j \in N_i} (x_i - x_j)$<br><br>
            <strong>Key Insight:</strong> This is gradient descent on the disagreement energy 
            $E = \frac{1}{2}\sum_{(i,j) \in E} (x_i - x_j)^2$."#,
        demo_explanation: r#"
            <strong>🎮 Try This:</strong>
            <ol>
            <li><strong>Watch Smooth Convergence:</strong> Compare to discrete consensus</li>
            <li><strong>Plot Error:</strong> See exponential decay</li>
            <li><strong>Vary Connectivity:</strong> More edges = faster convergence</li>
            <li><strong>Challenge:</strong> Achieve consensus error < 0.001 in minimum time</li>
            </ol>
            <br>
            <strong>Key Insight:</strong> Continuous-time is smoother and easier to analyze mathematically.
        "#,
        key_takeaways: &[
            r#"Continuous-time: $\dot{x}_i = -\sum_{j \in N_i} (x_i - x_j)$"#,
            "Equivalent to gradient descent on disagreement energy",
            r#"Convergence rate: $e^{-\lambda_2 t}$"#,
            "Smoother than discrete-time consensus",
        ],
        going_deeper: "<strong>In Theory:</strong> Continuous-time consensus is a linear system. \
                       Stability analysis uses eigenvalues of the Laplacian.<br><br>\
                       <strong>In Practice:</strong> Implemented as discrete-time with small time steps. \
                       The continuous analysis guides parameter selection.",
        math_details: r#"
<h4>Continuous-Time Consensus</h4>
<p>Differential equation:</p>

$$\dot{x}_i = -\sum_{j \in N_i} (x_i - x_j)$$

<p>In matrix form:</p>

$$\dot{x} = -L x$$

<p>Where $L$ is the Laplacian matrix.</p>

<h4>Solution</h4>
<p>The solution is:</p>

$$x(t) = e^{-Lt} x(0)$$

<p>For connected graph, converges to:</p>

$$\lim_{t \to \infty} x(t) = \frac{1}{n} \mathbf{1}^T x(0) \cdot \mathbf{1}$$

<p>Where $\mathbf{1}$ is the vector of all ones.</p>

<h4>Convergence Rate</h4>
<p>Error decays as:</p>

$$\|x(t) - \bar{x}\| \leq e^{-\lambda_2 t} \|x(0) - \bar{x}\|$$

<p>Where $\lambda_2$ is algebraic connectivity.</p>
        "#,
        implementation: r#"
<h4>Continuous-Time Implementation</h4>
<pre>
fn laplacian_flow_step(agents: &mut [Agent], dt: f32, neighbors: &[Vec<usize>]) {
    let mut derivatives = vec![0.0; agents.len()];
    
    for i in 0..agents.len() {
        let mut sum_diff = 0.0;
        for &j in &neighbors[i] {
            sum_diff += agents[j].value - agents[i].value;
        }
        derivatives[i] = -sum_diff;
    }
    
    for i in 0..agents.len() {
        agents[i].value += derivatives[i] * dt;
    }
}
</pre>

<h4>LLM Prompt: Adaptive Step Size</h4>
<pre>"Implement adaptive step size for Euler integration:
- Estimate error using Richardson extrapolation
- Adjust dt to keep error below threshold
- Ensure stability (dt < 1/λ_max)
- Test with varying graph connectivity"</pre>
        "#,
    },
    Lesson {
        id: 8,
        title: "Gossip Consensus",
        subtitle: "Asynchronous & Robust",
        icon: "💬",
        phase: "Consensus (The Backbone)",
        why_it_matters: "Real robots don't synchronize perfectly. Gossip algorithms work \
                         asynchronously and handle packet loss—essential for real swarms.",
        intuition: "<h3>The Rumor Spreading Analogy</h3>\n\
            How does a rumor spread through a crowd? Not everyone talks at once. People \
            randomly pair up and share information. Eventually, everyone hears the rumor.<br><br>\n\
            <strong>Gossip Algorithm:</strong>\n\
            <ol>\n\
            <li>Randomly pick a neighbor</li>\n\
            <li>Average your values</li>\n\
            <li>Repeat</li>\n\
            </ol>\n\
            <strong>Key Insight:</strong> No global synchronization needed. Works with packet loss. \
            Robust to failures.",
        demo_explanation: r#"
            <strong>🎮 Try This:</strong>
            <ol>
            <li><strong>Watch Random Updates:</strong> See pairs update randomly</li>
            <li><strong>Add Packet Loss:</strong> Some updates fail. Still converges!</li>
            <li><strong>Compare to Sync:</strong> Gossip is slower but more robust</li>
            <li><strong>Challenge:</strong> Reach consensus with 50% packet loss</li>
            </ol>
            <br>
            <strong>Key Insight:</strong> Asynchronous algorithms trade speed for robustness.
        "#,
        key_takeaways: &[
            "Gossip: randomly pair neighbors and average",
            "Asynchronous: no global synchronization needed",
            "Robust to packet loss and failures",
            "Slower than synchronous consensus but more practical",
        ],
        going_deeper: "<strong>In Theory:</strong> Gossip converges in expectation. Variance \
                       across runs is higher than synchronous algorithms.<br><br>\
                       <strong>In Practice:</strong> Used in sensor networks, distributed databases, \
                       and any system where synchronization is expensive or impossible.",
        math_details: r#"
<h4>Gossip Update</h4>
<p>Randomly select edge $(i,j)$:</p>

$$x_i, x_j \leftarrow \frac{x_i + x_j}{2}$$

<h4>Expected Convergence</h4>
<p>In expectation, converges to average:</p>

$$\mathbb{E}[x(t)] = \bar{x}$$

<h4>Convergence Rate</h4>
<p>Expected error decays as:</p>

$$\mathbb{E}[\|x(t) - \bar{x}\|^2] \leq (1 - \frac{\lambda_2}{n})^t \|x(0) - \bar{x}\|^2$$

<p>Slower than synchronous but robust to failures.</p>
        "#,
        implementation: r#"
<h4>Gossip Implementation</h4>
<pre>
fn gossip_step(agents: &mut [Agent], rng: &mut Rng, 
               neighbors: &[Vec<usize>], packet_loss: f32) {
    // Randomly select an agent
    let i = rng.range(0, agents.len());
    if neighbors[i].is_empty() { return; }
    
    // Randomly select neighbor
    let j_idx = rng.range(0, neighbors[i].len());
    let j = neighbors[i][j_idx];
    
    // Packet loss?
    if rng.range(0.0, 1.0) < packet_loss { return; }
    
    // Average values
    let avg = (agents[i].value + agents[j].value) / 2.0;
    agents[i].value = avg;
    agents[j].value = avg;
}
</pre>

<h4>LLM Prompt: Push-Sum</h4>
<pre>"Implement push-sum algorithm for directed graphs:
- Each agent maintains (sum, weight) pair
- Split and send to neighbors
- Estimate = sum / weight
- Test with directed communication graph"</pre>
        "#,
    },
    // ═══════════════════════════════════════════════════════════════════════════
    // PHASE 3: Coordinated Motion & Formations
    // ═══════════════════════════════════════════════════════════════════════════
    Lesson {
        id: 9,
        title: "Rendezvous",
        subtitle: "Meet at One Point",
        icon: "📍",
        phase: "Coordinated Motion & Formations",
        why_it_matters: "Getting robots to meet at a point is fundamental. It's consensus \
                         applied to positions—the foundation of formation control.",
        intuition: "<h3>The Meeting Point Analogy</h3>\n\
            You and friends want to meet. No one knows where. How do you agree on a location?<br><br>\n\
            <strong>Rendezvous Algorithm:</strong>\n\
            <ol>\n\
            <li>Each robot moves toward average position of neighbors</li>\n\
            <li>Repeat until all robots converge to one point</li>\n\
            </ol>\n\
            <strong>Key Insight:</strong> This is consensus on positions. If the graph is connected, \
            all robots converge to the centroid.",
        demo_explanation: r#"
            <strong>🎮 Try This:</strong>
            <ol>
            <li><strong>Watch Convergence:</strong> See robots converge to centroid</li>
            <li><strong>Leader Election:</strong> One robot doesn't move. Others converge to it</li>
            <li><strong>Break Graph:</strong> Disconnect. See multiple meeting points</li>
            <li><strong>Challenge:</strong> Rendezvous in minimum time</li>
            </ol>
            <br>
            <strong>Key Insight:</strong> Rendezvous = consensus on positions.
        "#,
        key_takeaways: &[
            "Rendezvous: robots meet at one point",
            "Move toward average neighbor position",
            "Converges to centroid if graph is connected",
            "Foundation for formation control",
        ],
        going_deeper: "<strong>In Theory:</strong> Rendezvous is position consensus. Same math \
                       as value consensus, applied to 2D/3D positions.<br><br>\
                       <strong>In Practice:</strong> Used for: meeting points, formation assembly, \
                       and as a building block for more complex behaviors.",
        math_details: r#"
<h4>Rendezvous Update</h4>
<p>Each robot moves toward average neighbor position:</p>

$$p_i(t+1) = p_i(t) + \alpha \left( \frac{1}{|N_i|} \sum_{j \in N_i} p_j(t) - p_i(t) \right)$$

<h4>Convergence</h4>
<p>If graph is connected, converges to:</p>

$$\lim_{t \to \infty} p_i(t) = \frac{1}{n} \sum_{j=1}^n p_j(0)$$

<p>The centroid of initial positions.</p>
        "#,
        implementation: r#"
<h4>Rendezvous Implementation</h4>
<pre>
fn rendezvous_step(agents: &mut [Agent], alpha: f32, neighbors: &[Vec<usize>]) {
    for i in 0..agents.len() {
        if neighbors[i].is_empty() { continue; }
        
        let mut avg_pos = Vec2::ZERO;
        for &j in &neighbors[i] {
            avg_pos += agents[j].pos;
        }
        avg_pos /= neighbors[i].len() as f32;
        
        let direction = (avg_pos - agents[i].pos) * alpha;
        agents[i].vel = direction;
    }
}
</pre>

<h4>LLM Prompt: Leader-Follower</h4>
<pre>"Implement leader-follower rendezvous:
- Designate one robot as leader (doesn't move)
- Followers move toward leader + neighbors
- Test with leader at different positions
- Compare convergence time to standard rendezvous"</pre>
        "#,
    },
    Lesson {
        id: 10,
        title: "Distance-Based Formations",
        subtitle: "Maintain Shape",
        icon: "🔷",
        phase: "Coordinated Motion & Formations",
        why_it_matters: "Formations enable coordinated motion—robots maintain relative positions \
                         while moving. Essential for collaborative tasks.",
        intuition: "<h3>The Rigid Body Analogy</h3>\n\
            Imagine robots connected by rigid rods. The rods maintain fixed distances. \
            The formation can move and rotate, but shape stays fixed.<br><br>\n\
            <strong>Formation Control:</strong>\n\
            <ul>\n\
            <li>Define target distances $d_{ij}$ between pairs</li>\n\
            <li>Robots move to minimize distance errors</li>\n\
            <li>Formation moves as a unit</li>\n\
            </ul>\n\
            <strong>Key Insight:</strong> Graph rigidity determines if formation is maintainable. \
            Too few edges = formation collapses.",
        demo_explanation: r#"
            <strong>🎮 Try This:</strong>
            <ol>
            <li><strong>Triangle Formation:</strong> Maintain equilateral triangle</li>
            <li><strong>Square Formation:</strong> Maintain square shape</li>
            <li><strong>Reduce Edges:</strong> Remove edges. See formation fail</li>
            <li><strong>Challenge:</strong> Maintain formation while moving to target</li>
            </ol>
            <br>
            <strong>Key Insight:</strong> Formation requires sufficient connectivity (rigidity).
        "#,
        key_takeaways: &[
            "Formation: maintain relative positions",
            "Control law minimizes distance errors",
            "Graph rigidity determines feasibility",
            "Too few edges = formation collapses",
        ],
        going_deeper: "<strong>In Theory:</strong> Rigid graph theory determines minimum edges \
                       needed. In 2D, need $2n-3$ edges for rigidity.<br><br>\
                       <strong>In Practice:</strong> Formations enable: coordinated search, \
                       collaborative manipulation, and aesthetic swarm displays.",
        math_details: r#"
<h4>Formation Energy</h4>
<p>Energy function:</p>

$$E = \sum_{(i,j) \in E} (\|p_i - p_j\| - d_{ij})^2$$

<h4>Gradient Control</h4>
<p>Robots move to minimize energy:</p>

$$a_i = -k \sum_{j:(i,j) \in E} (\|p_i - p_j\| - d_{ij}) \frac{p_i - p_j}{\|p_i - p_j\|}$$

<h4>Rigidity</h4>
<p>Formation is rigid if distance constraints uniquely determine positions (up to translation/rotation).</p>
        "#,
        implementation: r#"
<h4>Formation Control</h4>
<pre>
fn formation_step(agents: &mut [Agent], target_distances: &[(usize, usize, f32)],
                  k: f32) {
    for i in 0..agents.len() {
        let mut force = Vec2::ZERO;
        
        for &(j, k, d_target) in target_distances {
            if j == i {
                let diff = agents[i].pos - agents[k].pos;
                let dist = diff.length();
                let error = dist - d_target;
                force += diff.normalize() * error * k;
            }
        }
        
        agents[i].accel = force;
    }
}
</pre>

<h4>LLM Prompt: Rigidity Check</h4>
<pre>"Implement rigidity check for 2D formation:
- Count edges: need 2n-3 for rigidity
- Check if distance constraints are independent
- Warn if formation is under-constrained
- Test with triangle, square, line formations"</pre>
        "#,
    },
    Lesson {
        id: 11,
        title: "Cyclic Pursuit",
        subtitle: "Chasing Creates Order",
        icon: "🎠",
        phase: "Coordinated Motion & Formations",
        why_it_matters: "Cyclic pursuit creates beautiful patterns—robots chase neighbors \
                         in a cycle, forming rotating polygons. Simple rule, complex behavior.",
        intuition: "<h3>The Carousel Analogy</h3>\n\
            Imagine robots arranged in a circle. Each robot chases the next one. What happens?<br><br>\n\
            <strong>Result:</strong> The formation rotates! Robots maintain relative positions \
            while orbiting a center point.<br><br>\n\
            <strong>The Math:</strong> $a_i = k R(\theta) (p_{i+1} - p_i)$<br><br>\n\
            <strong>Key Insight:</strong> The rotation matrix $R(\theta)$ creates the pursuit \
            angle. Different angles create different patterns.",
        demo_explanation: r#"
            <strong>🎮 Try This:</strong>
            <ol>
            <li><strong>Watch Rotation:</strong> See polygon rotate around center</li>
            <li><strong>Vary Angle:</strong> Change pursuit angle. See different patterns</li>
            <li><strong>Add Robots:</strong> More robots = more sides to polygon</li>
            <li><strong>Challenge:</strong> Create stable rotating square formation</li>
            </ol>
            <br>
            <strong>Key Insight:</strong> Simple pursuit rule creates complex coordinated motion.
        "#,
        key_takeaways: &[
            "Cyclic pursuit: each robot chases next",
            "Creates rotating polygon formations",
            "Pursuit angle determines pattern",
            "Stable for certain angles and gains",
        ],
        going_deeper: "<strong>In Theory:</strong> Cyclic pursuit is a special case of formation \
                       control. Stability analysis uses eigenvalues of the pursuit matrix.<br><br>\
                       <strong>In Practice:</strong> Used for: coordinated surveillance, \
                       aesthetic displays, and as a building block for more complex behaviors.",
        math_details: r#"
<h4>Cyclic Pursuit</h4>
<p>Each robot pursues next with rotation:</p>

$$a_i = k R(\theta) (p_{i+1} - p_i)$$

<p>Where $R(\theta)$ rotates by angle $\theta$:</p>

$$R(\theta) = \begin{bmatrix}
\cos\theta & -\sin\theta \\
\sin\theta & \cos\theta
\end{bmatrix}$$

<h4>Stability</h4>
<p>For $n$ robots, stable if:</p>

$$k > 0, \quad \theta \in (-\pi/2, \pi/2)$$

<p>Converges to regular polygon rotating around centroid.</p>
        "#,
        implementation: r#"
<h4>Cyclic Pursuit</h4>
<pre>
fn cyclic_pursuit_step(agents: &mut [Agent], k: f32, theta: f32) {
    let n = agents.len();
    let rot = Mat2::rotation(theta);
    
    for i in 0..n {
        let next = (i + 1) % n;
        let diff = agents[next].pos - agents[i].pos;
        let pursuit = rot * diff;
        agents[i].accel = pursuit * k;
    }
}
</pre>

<h4>LLM Prompt: Multi-Cycle</h4>
<pre>"Implement multi-cycle pursuit:
- Divide robots into multiple cycles
- Each cycle pursues independently
- Test with 2 cycles of 4 robots each
- Visualize separate rotating formations"</pre>
        "#,
    },
    // ═══════════════════════════════════════════════════════════════════════════
    // PHASE 4: Task Allocation
    // ═══════════════════════════════════════════════════════════════════════════
    Lesson {
        id: 12,
        title: "Why Greedy Fails",
        subtitle: "The Assignment Problem",
        icon: "📦",
        phase: "Task Allocation",
        why_it_matters: "Assigning robots to tasks seems simple—just pick the nearest. \
                         But greedy assignment fails spectacularly. Understanding why is key.",
        intuition: "<h3>The Warehouse Analogy</h3>\n\
            You have 3 robots and 3 packages. Greedy: each robot takes nearest package. \
            Seems optimal, right?<br><br>\n\
            <strong>The Problem:</strong> What if all packages are far from one robot, but \
            close to others? Greedy creates imbalance.<br><br>\n\
            <strong>Example:</strong>\n\
            <ul>\n\
            <li>Robot A: 10m to package 1, 100m to package 2</li>\n\
            <li>Robot B: 10m to package 1, 10m to package 2</li>\n\
            </ul>\n\
            Greedy: Both take package 1. Robot A travels 100m. Total: 110m.<br>\
            Optimal: A takes 1, B takes 2. Total: 20m.<br><br>\
            <strong>Key Insight:</strong> Greedy is locally optimal but globally suboptimal.",
        demo_explanation: r#"
            <strong>🎮 Try This:</strong>
            <ol>
            <li><strong>Greedy Assignment:</strong> Watch robots take nearest tasks</li>
            <li><strong>See Imbalance:</strong> Some robots overloaded, others idle</li>
            <li><strong>Optimal Baseline:</strong> Compare to optimal assignment</li>
            <li><strong>Challenge:</strong> Minimize total travel distance</li>
            </ol>
            <br>
            <strong>Key Insight:</strong> Greedy fails when tasks are clustered.
        "#,
        key_takeaways: &[
            "Greedy: each robot takes nearest task",
            "Fails when tasks are clustered",
            "Creates imbalance and inefficiency",
            "Optimal assignment requires global view",
        ],
        going_deeper: "<strong>In Theory:</strong> Assignment problem is bipartite matching. \
                       Greedy is $O(n)$ but suboptimal. Optimal is $O(n^3)$ (Hungarian algorithm).<br><br>\
                       <strong>In Practice:</strong> For large swarms, need distributed algorithms \
                       that approximate optimal without global knowledge.",
        math_details: r#"
<h4>Assignment Problem</h4>
<p>Minimize total cost:</p>

$$\min \sum_{i,j} c_{ij} x_{ij}$$

<p>Subject to:</p>

$$\sum_j x_{ij} = 1 \quad \forall i$$

$$\sum_i x_{ij} = 1 \quad \forall j$$

<p>Where $x_{ij} \in \{0,1\}$ indicates assignment.</p>

<h4>Greedy Algorithm</h4>
<p>For each robot, assign to nearest unassigned task:</p>

$$j^* = \arg\min_{j \text{ unassigned}} c_{ij}$$

<h4>Why It Fails</h4>
<p>Greedy doesn't consider future assignments. Can create 'bottlenecks' where \
one robot gets all far tasks.</p>
        "#,
        implementation: r#"
<h4>Greedy Assignment</h4>
<pre>
fn greedy_assignment(robots: &[Vec2], tasks: &[Vec2]) -> Vec<usize> {
    let mut assignments = vec![usize::MAX; robots.len()];
    let mut assigned = vec![false; tasks.len()];
    
    for i in 0..robots.len() {
        let mut best_j = None;
        let mut best_dist = f32::INFINITY;
        
        for j in 0..tasks.len() {
            if assigned[j] { continue; }
            let dist = robots[i].distance(tasks[j]);
            if dist < best_dist {
                best_dist = dist;
                best_j = Some(j);
            }
        }
        
        if let Some(j) = best_j {
            assignments[i] = j;
            assigned[j] = true;
        }
    }
    
    assignments
}
</pre>

<h4>LLM Prompt: Optimal Baseline</h4>
<pre>"Implement Hungarian algorithm for optimal assignment:
- Build cost matrix
- Use Kuhn-Munkres algorithm
- Compare total cost to greedy
- Test with varying task distributions"</pre>
        "#,
    },
    Lesson {
        id: 13,
        title: "Auction Algorithm",
        subtitle: "Distributed Task Allocation",
        icon: "🔨",
        phase: "Task Allocation",
        why_it_matters: "Auction algorithms enable distributed task allocation—robots bid on \
                         tasks without central coordination. Scalable and robust.",
        intuition: r#"<h3>The Auction Analogy</h3>
            Imagine an auction. Items (tasks) have prices. Bidders (robots) bid based on 
            value minus price. Highest bidder wins. Prices increase. Repeat.<br><br>
            <strong>Auction Algorithm:</strong>
            <ol>
            <li>Robots bid on tasks: $b_{ij} = value_{ij} - price_j$</li>
            <li>Highest bidder wins task</li>
            <li>Update price: $price_j \leftarrow price_j + \epsilon$</li>
            <li>Repeat until convergence</li>
            </ol>
            <strong>Key Insight:</strong> Prices coordinate robots. No central controller needed!"#,
        demo_explanation: r#"
            <strong>🎮 Try This:</strong>
            <ol>
            <li><strong>Watch Bidding:</strong> See robots bid on tasks</li>
            <li><strong>See Prices Rise:</strong> Popular tasks get expensive</li>
            <li><strong>Convergence:</strong> Watch assignment stabilize</li>
            <li><strong>Add Delay:</strong> Simulate communication delay. Still works!</li>
            <li><strong>Challenge:</strong> Minimize total cost with communication constraints</li>
            </ol>
            <br>
            <strong>Key Insight:</strong> Prices coordinate distributed assignment.
        "#,
        key_takeaways: &[
            "Auction: robots bid on tasks",
            "Bid = value - price",
            "Prices coordinate without central controller",
            "Converges to near-optimal assignment",
        ],
        going_deeper: r#"<strong>In Theory:</strong> Auction algorithm converges to $\epsilon$-optimal 
                       assignment. $\epsilon$ controls tradeoff between optimality and convergence speed.<br><br>
                       <strong>In Practice:</strong> Used in: multi-robot task allocation, 
                       distributed computing, and any system needing distributed coordination."#,
        math_details: r#"
<h4>Bidding Rule</h4>
<p>Robot $i$ bids on task $j$:</p>

$$b_{ij} = r_{ij} - c \cdot d_{ij} - p_j$$

<p>Where:</p>
<ul>
<li>$r_{ij}$ = reward for robot $i$ doing task $j$</li>
<li>$c$ = cost per unit distance</li>
<li>$d_{ij}$ = distance from robot $i$ to task $j$</li>
<li>$p_j$ = current price of task $j$</li>
</ul>

<h4>Price Update</h4>
<p>After assignment, update price:</p>

$$p_j \leftarrow p_j + \epsilon$$

<p>Where $\epsilon$ is small positive constant.</p>

<h4>Convergence</h4>
<p>Algorithm converges when no robot wants to switch tasks. \
Assignment is $\epsilon$-optimal.</p>
        "#,
        implementation: r#"
<h4>Auction Algorithm</h4>
<pre>
fn auction_step(robots: &[Agent], tasks: &mut [Task], epsilon: f32) -> bool {
    let mut changed = false;
    
    for i in 0..robots.len() {
        // Find best task
        let mut best_j = None;
        let mut best_bid = f32::NEG_INFINITY;
        
        for j in 0..tasks.len() {
            let dist = robots[i].pos.distance(tasks[j].pos);
            let bid = tasks[j].reward - dist - tasks[j].price;
            if bid > best_bid {
                best_bid = bid;
                best_j = Some(j);
            }
        }
        
        // Update assignment
        if let Some(j) = best_j {
            if robots[i].assigned_task != Some(j) {
                changed = true;
                robots[i].assigned_task = Some(j);
                tasks[j].price += epsilon;
            }
        }
    }
    
    changed
}
</pre>

<h4>LLM Prompt: Communication Delay</h4>
<pre>"Add communication delay to auction:
- Messages arrive after delay
- Robots use stale price information
- Test convergence with varying delays
- Compare to synchronous auction"</pre>
        "#,
    },
    Lesson {
        id: 14,
        title: "Exploration vs Exploitation",
        subtitle: "Multi-Armed Bandits",
        icon: "🎰",
        phase: "Task Allocation",
        why_it_matters: "Robots must balance exploring new areas vs exploiting known good areas. \
                         Multi-armed bandits provide the optimal strategy.",
        intuition: "<h3>The Restaurant Analogy</h3>\n\
            You're in a new city. Do you:\n\
            <ul>\n\
            <li>Go to the same restaurant every day? (Exploit—safe but boring)</li>\n\
            <li>Try a new restaurant every day? (Explore—risky but might find better)</li>\n\
            </ul>\n\
            <strong>Optimal Strategy:</strong> Start exploring. As you learn, shift to exploiting \
            the best options. But always explore a little.<br><br>\n\
            <strong>In Swarms:</strong> Robots explore unknown areas vs exploit known good areas. \
            UCB1 algorithm balances this optimally.",
        demo_explanation: r#"
            <strong>🎮 Try This:</strong>
            <ol>
            <li><strong>Pure Exploitation:</strong> Always pick best known. Misses better options</li>
            <li><strong>Pure Exploration:</strong> Always pick random. Wastes time</li>
            <li><strong>UCB1:</strong> Optimal balance. See regret decrease</li>
            <li><strong>Challenge:</strong> Minimize regret over 1000 steps</li>
            </ol>
            <br>
            <strong>Key Insight:</strong> Exploration-exploitation tradeoff is fundamental to learning.
        "#,
        key_takeaways: &[
            "Exploration: try new options",
            "Exploitation: use best known option",
            "UCB1: optimal balance",
            "Regret: difference from optimal",
        ],
        going_deeper: "<strong>In Theory:</strong> Multi-armed bandits is a fundamental \
                       learning problem. UCB1 achieves logarithmic regret.<br><br>\
                       <strong>In Practice:</strong> Used in: recommendation systems, A/B testing, \
                       and any system needing to learn while acting.",
        math_details: r#"
<h4>UCB1 Algorithm</h4>
<p>Select arm (task) with highest upper confidence bound:</p>

$$UCB_i = \bar{x}_i + c \sqrt{\frac{\ln t}{n_i}}$$

<p>Where:</p>
<ul>
<li>$\bar{x}_i$ = average reward from arm $i$</li>
<li>$n_i$ = number of times arm $i$ selected</li>
<li>$t$ = total number of selections</li>
<li>$c$ = exploration constant (typically $\sqrt{2}$)</li>
</ul>

<h4>Regret</h4>
<p>Cumulative regret:</p>

$$R_T = \sum_{t=1}^T (\mu^* - \mu_{i_t})$$

<p>Where $\mu^*$ is optimal arm mean, $\mu_{i_t}$ is selected arm mean.</p>

<h4>UCB1 Regret</h4>
<p>UCB1 achieves:</p>

$$R_T = O(\ln T)$$

<p>Logarithmic regret is optimal.</p>
        "#,
        implementation: r#"
<h4>UCB1 Implementation</h4>
<pre>
struct UCB1Arm {
    rewards: Vec<f32>,
    count: usize,
}

impl UCB1Arm {
    fn ucb(&self, total_count: usize, c: f32) -> f32 {
        if self.count == 0 { return f32::INFINITY; }
        
        let avg = self.rewards.iter().sum::<f32>() / self.count as f32;
        let exploration = c * (total_count as f32 / self.count as f32).ln().sqrt();
        avg + exploration
    }
}
</pre>

<h4>LLM Prompt: Thompson Sampling</h4>
<pre>"Implement Thompson Sampling as alternative to UCB1:
- Maintain Beta distribution for each arm
- Sample from distributions
- Select arm with highest sample
- Compare regret to UCB1"</pre>
        "#,
    },
    // ═══════════════════════════════════════════════════════════════════════════
    // PHASE 5: Coverage & Exploration
    // ═══════════════════════════════════════════════════════════════════════════
    Lesson {
        id: 15,
        title: "Voronoi Coverage",
        subtitle: "Lloyd's Algorithm",
        icon: "🗺️",
        phase: "Coverage & Exploration",
        why_it_matters: "Covering an area efficiently is fundamental—search and rescue, \
                         surveillance, agriculture. Voronoi coverage is optimal.",
        intuition: "<h3>The Territory Analogy</h3>\n\
            Imagine dividing a country into regions. Each region is closest to one city. \
            That's a Voronoi diagram.<br><br>\n\
            <strong>Coverage Problem:</strong> Place robots to minimize maximum distance \
            to any point. Lloyd's algorithm solves this iteratively:\n\
            <ol>\n\
            <li>Compute Voronoi cells (each robot's territory)</li>\n\
            <li>Move robots to centroids of their cells</li>\n\
            <li>Repeat</li>\n\
            </ol>\n\
            <strong>Key Insight:</strong> This minimizes coverage error—the maximum distance \
            from any point to nearest robot.",
        demo_explanation: r#"
            <strong>🎮 Try This:</strong>
            <ol>
            <li><strong>Watch Voronoi Cells:</strong> See territories form</li>
            <li><strong>See Robots Move:</strong> To centroids of cells</li>
            <li><strong>Coverage Error:</strong> Watch it decrease over time</li>
            <li><strong>Add Obstacles:</strong> See cells adapt</li>
            <li><strong>Challenge:</strong> Minimize coverage error in minimum time</li>
            </ol>
            <br>
            <strong>Key Insight:</strong> Voronoi coverage is optimal for uniform coverage.
        "#,
        key_takeaways: &[
            "Voronoi diagram: partition space by nearest robot",
            "Lloyd's algorithm: move to cell centroids",
            "Minimizes coverage error",
            "Converges to optimal configuration",
        ],
        going_deeper: "<strong>In Theory:</strong> Voronoi coverage minimizes the maximum \
                       distance. Lloyd's algorithm is gradient descent on this objective.<br><br>\
                       <strong>In Practice:</strong> Used in: sensor networks, UAV surveillance, \
                       and any application needing area coverage.",
        math_details: r#"
<h4>Voronoi Cell</h4>
<p>Cell for robot $i$:</p>

$$V_i = \{p : \|p - p_i\| \leq \|p - p_j\| \quad \forall j \neq i\}$$

<h4>Coverage Error</h4>
<p>Maximum distance from any point to nearest robot:</p>

$$E = \max_{p \in \Omega} \min_i \|p - p_i\|$$

<h4>Centroid</h4>
<p>Centroid of Voronoi cell:</p>

$$c_i = \frac{\int_{V_i} p \, dp}{\int_{V_i} dp}$$

<h4>Lloyd's Update</h4>
<p>Move robot to centroid:</p>

$$p_i \leftarrow c_i$$

<p>This decreases coverage error.</p>
        "#,
        implementation: r#"
<h4>Voronoi Coverage</h4>
<pre>
fn voronoi_coverage_step(agents: &mut [Agent], world: &World) {
    // Compute Voronoi cells (approximate on grid)
    let mut cells = vec![Vec::new(); agents.len()];
    
    for y in 0..grid_height {
        for x in 0..grid_width {
            let p = grid_to_world(x, y);
            let nearest = find_nearest_agent(p, agents);
            cells[nearest].push(p);
        }
    }
    
    // Move to centroids
    for i in 0..agents.len() {
        let centroid = compute_centroid(&cells[i]);
        agents[i].target = centroid;
    }
}
</pre>

<h4>LLM Prompt: Weighted Voronoi</h4>
<pre>"Implement weighted Voronoi coverage:
- Each robot has different sensing radius
- Voronoi cells weighted by radius
- Test with heterogeneous swarm
- Compare coverage to uniform Voronoi"</pre>
        "#,
    },
    Lesson {
        id: 16,
        title: "Frontier Exploration",
        subtitle: "Mapping Unknown Areas",
        icon: "🔍",
        phase: "Coverage & Exploration",
        why_it_matters: "Exploring unknown environments efficiently is crucial—search and rescue, \
                         mapping, inspection. Frontier exploration minimizes overlap.",
        intuition: "<h3>The Explorer Analogy</h3>\n\
            You're exploring a cave. Do you:\n\
            <ul>\n\
            <li>Go back to explored areas? (Wasteful—already mapped)</li>\n\
            <li>Go to unexplored areas? (Efficient—new information)</li>\n\
            </ul>\n\
            <strong>Frontier:</strong> Boundary between explored and unexplored. \
            Frontier exploration sends robots to frontiers.<br><br>\n\
            <strong>Key Insight:</strong> This maximizes information gain—each step reveals \
            maximum new area.",
        demo_explanation: r#"
            <strong>🎮 Try This:</strong>
            <ol>
            <li><strong>Watch Exploration:</strong> See map fill in</li>
            <li><strong>See Frontiers:</strong> Boundary between known/unknown</li>
            <li><strong>Robot Assignment:</strong> Each robot goes to nearest frontier</li>
            <li><strong>Compare Strategies:</strong> Random vs frontier exploration</li>
            <li><strong>Challenge:</strong> Map entire area in minimum time</li>
            </ol>
            <br>
            <strong>Key Insight:</strong> Frontier exploration minimizes redundant exploration.
        "#,
        key_takeaways: &[
            "Frontier: boundary between explored/unexplored",
            "Frontier exploration maximizes information gain",
            "Assign robots to nearest frontiers",
            "Minimizes overlap and exploration time",
        ],
        going_deeper: "<strong>In Theory:</strong> Frontier exploration is greedy information \
                       maximization. Each step reveals maximum new area.<br><br>\
                       <strong>In Practice:</strong> Used in: SLAM, search and rescue, and any \
                       application needing efficient exploration.",
        math_details: r#"
<h4>Information Gain</h4>
<p>Information gain from exploring point $p$:</p>

$$I(p) = \text{Area}(U(p) \cap \text{Unknown})$$

<p>Where $U(p)$ is sensing region around $p$.</p>

<h4>Frontier Detection</h4>
<p>Point $p$ is on frontier if:</p>

$$\exists q \in N(p) : \text{Explored}(p) \land \neg\text{Explored}(q)$$

<p>Where $N(p)$ is neighborhood of $p$.</p>

<h4>Assignment</h4>
<p>Assign robot $i$ to frontier:</p>

$$f^* = \arg\min_{f \in \text{Frontiers}} \|p_i - f\|$$

        "#,
        implementation: r#"
<h4>Frontier Exploration</h4>
<pre>
fn frontier_exploration_step(agents: &mut [Agent], map: &OccupancyGrid) {
    let frontiers = detect_frontiers(map);
    
    for agent in agents.iter_mut() {
        if frontiers.is_empty() { continue; }
        
        // Find nearest frontier
        let mut nearest = None;
        let mut min_dist = f32::INFINITY;
        
        for &frontier in &frontiers {
            let dist = agent.pos.distance(frontier);
            if dist < min_dist {
                min_dist = dist;
                nearest = Some(frontier);
            }
        }
        
        if let Some(target) = nearest {
            agent.target = target;
        }
    }
}
</pre>

<h4>LLM Prompt: Multi-Robot Coordination</h4>
<pre>"Add coordination to frontier exploration:
- Avoid assigning multiple robots to same frontier
- Use auction algorithm for frontier assignment
- Test with varying numbers of robots
- Compare exploration time to greedy assignment"</pre>
        "#,
    },
    Lesson {
        id: 17,
        title: "Pheromones & Stigmergy",
        subtitle: "Indirect Coordination",
        icon: "🐝",
        phase: "Coverage & Exploration",
        why_it_matters: "Ants coordinate without direct communication—they leave pheromone trails. \
                         Stigmergy enables scalable indirect coordination.",
        intuition: "<h3>The Ant Trail Analogy</h3>\n\
            Ants find food and return to nest, leaving pheromone. Other ants follow strong \
            trails. Over time, shortest path gets strongest trail.<br><br>\n\
            <strong>Stigmergy:</strong> Coordination through environment modification. \
            Robots modify environment (pheromone field), environment guides behavior.<br><br>\n\
            <strong>Key Insight:</strong> No direct communication needed. Scalable to millions \
            of agents.",
        demo_explanation: r#"
            <strong>🎮 Try This:</strong>
            <ol>
            <li><strong>Watch Trails Form:</strong> See pheromone accumulate</li>
            <li><strong>See Decay:</strong> Old trails fade</li>
            <li><strong>Follow Gradients:</strong> Robots follow pheromone gradients</li>
            <li><strong>Multiple Sources:</strong> See trails compete</li>
            <li><strong>Challenge:</strong> Find shortest path to goal using pheromones</li>
            </ol>
            <br>
            <strong>Key Insight:</strong> Environment modification enables indirect coordination.
        "#,
        key_takeaways: &[
            "Stigmergy: coordination through environment",
            "Pheromone: virtual chemical trail",
            "Decay: old information fades",
            "Diffusion: information spreads",
        ],
        going_deeper: "<strong>In Nature:</strong> Ants, termites, and bees all use stigmergy. \
                       It's the foundation of social insect coordination.<br><br>\
                       <strong>In Engineering:</strong> Used in: routing algorithms, optimization, \
                       and any system needing scalable coordination.",
        math_details: r#"
<h4>Pheromone Update</h4>
<p>Pheromone field evolves:</p>

$$\frac{\partial P}{\partial t} = -\rho P + \kappa \nabla^2 P + D$$

<p>Where:</p>
<ul>
<li>$\rho$ = decay rate</li>
<li>$\kappa$ = diffusion coefficient</li>
<li>$D$ = deposit (from robots)</li>
</ul>

<h4>Discrete Update</h4>
<p>On grid:</p>

$$P_{ij}(t+1) = (1-\rho) P_{ij}(t) + \kappa \sum_{k \in N(i,j)} (P_k(t) - P_{ij}(t)) + D_{ij}(t)$$

<h4>Robot Motion</h4>
<p>Robots follow gradient:</p>

$$a_i = k \nabla P(p_i)$$

        "#,
        implementation: r#"
<h4>Pheromone Field</h4>
<pre>
struct PheromoneField {
    grid: Vec<Vec<f32>>,
    decay: f32,
    diffusion: f32,
}

impl PheromoneField {
    fn update(&mut self, deposits: &[Vec2]) {
        // Decay
        for row in &mut self.grid {
            for cell in row {
                *cell *= 1.0 - self.decay;
            }
        }
        
        // Diffusion (simplified)
        // ... diffusion step ...
        
        // Deposit
        for deposit in deposits {
            let (x, y) = world_to_grid(deposit);
            self.grid[y][x] += 1.0;
        }
    }
}
</pre>

<h4>LLM Prompt: Ant Colony Optimization</h4>
<pre>"Implement ant colony optimization for path finding:
- Ants deposit pheromone on paths
- Pheromone strength = 1 / path length
- Ants choose paths probabilistically based on pheromone
- Test on graph with multiple paths"</pre>
        "#,
    },
    // ═══════════════════════════════════════════════════════════════════════════
    // PHASE 6: Robustness & Capstone
    // ═══════════════════════════════════════════════════════════════════════════
    Lesson {
        id: 18,
        title: "When Robots Lie",
        subtitle: "Robust Consensus",
        icon: "🛡️",
        phase: "Robustness & Capstone",
        why_it_matters: "Real systems have failures—robots crash, sensors fail, or worse, \
                         robots lie (Byzantine failures). Robust algorithms handle these.",
        intuition: "<h3>The Liar Analogy</h3>\n\
            You're trying to agree on a number with friends. But one friend keeps lying, \
            saying random values. What do you do?<br><br>\n\
            <strong>Mean Consensus:</strong> Average all values. Liar's value corrupts result.<br>\
            <strong>Median Consensus:</strong> Take median. Liar's value is ignored (if less \
            than 50% are liars).<br>\
            <strong>Trimmed Mean:</strong> Remove outliers, then average. More robust.<br><br>\
            <strong>Key Insight:</strong> Robust aggregation functions ignore outliers.",
        demo_explanation: r#"
            <strong>🎮 Try This:</strong>
            <ol>
            <li><strong>Add Malicious Robot:</strong> One robot sends wrong values</li>
            <li><strong>Mean Consensus:</strong> See it fail—consensus corrupted</li>
            <li><strong>Median Consensus:</strong> See it succeed—outlier ignored</li>
            <li><strong>Vary Malicious Fraction:</strong> See tolerance limits</li>
            <li><strong>Challenge:</strong> Reach consensus with 30% malicious robots</li>
            </ol>
            <br>
            <strong>Key Insight:</strong> Robust algorithms tolerate failures.
        "#,
        key_takeaways: &[
            "Byzantine failure: robot sends wrong values",
            "Mean consensus fails with malicious robots",
            "Median consensus tolerates <50% malicious",
            "Trimmed mean balances robustness and efficiency",
        ],
        going_deeper: "<strong>In Theory:</strong> Byzantine consensus requires $>2/3$ honest \
                       robots. This is a fundamental limit.<br><br>\
                       <strong>In Practice:</strong> Real systems use: median, trimmed mean, \
                       or cryptographic signatures to detect liars.",
        math_details: r#"
<h4>Mean Consensus</h4>
<p>Standard consensus:</p>

$$x_i \leftarrow \frac{1}{|N_i|} \sum_{j \in N_i} x_j$$

<p>Fails if any $x_j$ is malicious.</p>

<h4>Median Consensus</h4>
<p>Robust to outliers:</p>

$$x_i \leftarrow \text{median}(\{x_j : j \in N_i\})$$

<p>Tolerates up to 50% malicious values.</p>

<h4>Trimmed Mean</h4>
<p>Remove outliers, then average:</p>

$$x_i \leftarrow \frac{1}{|N_i| - 2f} \sum_{j \in \text{trimmed}(N_i)} x_j$$

<p>Where $f$ is number of outliers to trim.</p>
        "#,
        implementation: r#"
<h4>Robust Consensus</h4>
<pre>
fn robust_consensus_step(agents: &mut [Agent], neighbors: &[Vec<usize>],
                        method: RobustMethod) {
    for i in 0..agents.len() {
        if agents[i].malicious { continue; } // Skip malicious agents
        
        let values: Vec<f32> = neighbors[i].iter()
            .map(|&j| agents[j].value)
            .collect();
        
        agents[i].value = match method {
            RobustMethod::Mean => values.iter().sum::<f32>() / values.len() as f32,
            RobustMethod::Median => median(&values),
            RobustMethod::TrimmedMean(f) => trimmed_mean(&values, f),
        };
    }
}
</pre>

<h4>LLM Prompt: Byzantine Agreement</h4>
<pre>"Implement Byzantine agreement algorithm:
- Requires >2/3 honest robots
- Uses voting and majority rule
- Test with varying malicious fractions
- Compare to median consensus"</pre>
        "#,
    },
    Lesson {
        id: 19,
        title: "Swarm Sandbox",
        subtitle: "Design, Tune, Score",
        icon: "🎮",
        phase: "Robustness & Capstone",
        why_it_matters: "Real swarm design requires balancing multiple objectives. \
                         The capstone lets you combine everything you've learned.",
        intuition: "<h3>The Design Challenge</h3>\n\
            You've learned:\n\
            <ul>\n\
            <li>Flocking for coordinated motion</li>\n\
            <li>Consensus for agreement</li>\n\
            <li>Task allocation for efficiency</li>\n\
            <li>Coverage for exploration</li>\n\
            </ul>\n\
            <strong>Now Combine Them:</strong> Design a swarm that:\n\
            <ul>\n\
            <li>Finds targets efficiently</li>\n\
            <li>Maintains formation</li>\n\
            <li>Avoids collisions</li>\n\
            <li>Minimizes energy</li>\n\
            </ul>\n\
            <strong>Key Insight:</strong> Real swarms combine multiple behaviors. \
            The challenge is tuning parameters to balance objectives.",
        demo_explanation: r#"
            <strong>🎮 Try This:</strong>
            <ol>
            <li><strong>Choose Scenario:</strong> Search, formation, coverage, etc.</li>
            <li><strong>Tune Parameters:</strong> Adjust weights, gains, thresholds</li>
            <li><strong>Run Simulation:</strong> See how your design performs</li>
            <li><strong>View Scoreboard:</strong> Multiple metrics (time, energy, collisions)</li>
            <li><strong>Challenge:</strong> Maximize score on chosen scenario</li>
            </ol>
            <br>
            <strong>Key Insight:</strong> Swarm design is multi-objective optimization.
        "#,
        key_takeaways: &[
            "Real swarms combine multiple behaviors",
            "Design requires balancing objectives",
            "Pareto optimal: can't improve one without hurting another",
            "Tuning is iterative and scenario-dependent",
        ],
        going_deeper: "<strong>In Theory:</strong> Multi-objective optimization has no single \
                       optimal solution. Pareto front shows tradeoffs.<br><br>\
                       <strong>In Practice:</strong> Real swarm design is iterative: \
                       simulate, measure, adjust, repeat.",
        math_details: r#"
<h4>Multi-Objective Optimization</h4>
<p>Minimize vector of objectives:</p>

$$\min f(x) = [f_1(x), f_2(x), \ldots, f_k(x)]$$

<p>No single optimal—Pareto front shows tradeoffs.</p>

<h4>Pareto Dominance</h4>
<p>Solution $x$ dominates $y$ if:</p>

$$\forall i: f_i(x) \leq f_i(y) \land \exists j: f_j(x) < f_j(y)$$

<h4>Score Function</h4>
<p>Weighted combination:</p>

$$S = w_1 \cdot \text{time} + w_2 \cdot \text{energy} + w_3 \cdot \text{collisions}$$

<p>Where weights reflect priorities.</p>
        "#,
        implementation: r#"
<h4>Capstone Scenario</h4>
<pre>
struct Scenario {
    name: String,
    objectives: Vec<Objective>,
    initial_conditions: InitialConditions,
}

struct Score {
    time: f32,
    energy: f32,
    collisions: usize,
    coverage: f32,
    total: f32,
}

fn evaluate_scenario(swarm: &Swarm, scenario: &Scenario) -> Score {
    // Run simulation
    // Measure metrics
    // Compute weighted score
    Score { ... }
}
</pre>

<h4>LLM Prompt: Parameter Tuning</h4>
<pre>"Implement parameter tuning for capstone:
- Use grid search or random search
- Evaluate each parameter set
- Return best parameters
- Visualize parameter-performance landscape"</pre>
        "#,
    },
];


