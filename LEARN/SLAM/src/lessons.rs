//! ═══════════════════════════════════════════════════════════════════════════════
//! FILE: lessons.rs | SLAM/src/lessons.rs
//! PURPOSE: SLAM lesson definitions - structured from intuitive to advanced
//! MODIFIED: 2025-12-12
//! LAYER: LEARN → SLAM
//! ═══════════════════════════════════════════════════════════════════════════════
//!
//! Curriculum designed for audience ranging from undergrads to professionals.
//! Each lesson starts with intuition and a demo, then builds to formal concepts.

/// Technical term that can have a popup explanation
#[derive(Clone)]
pub struct Term {
    pub word: &'static str,
    pub short: &'static str,  // One-line explanation
    pub detail: &'static str, // Full explanation for popup
}

/// Glossary of technical terms used across lessons
pub static GLOSSARY: &[Term] = &[
    Term {
        word: "sensor fusion",
        short: "Combining multiple sensors to get a better estimate",
        detail: "Each sensor has strengths and weaknesses. By combining them intelligently, \
                 we can get an estimate that's better than any single sensor alone. \
                 Like using both your eyes for depth perception.",
    },
    Term {
        word: "noise",
        short: "Random errors in sensor measurements",
        detail: "Real sensors aren't perfect. They give slightly different readings each time, \
                 even when measuring the same thing. This randomness is called noise. \
                 Think of static on a radio - the signal is there, but with interference.",
    },
    Term {
        word: "drift",
        short: "Error that accumulates over time",
        detail: "Some sensors have tiny errors that add up. If you integrate a gyroscope \
                 that's slightly off, after an hour you might think you've rotated 10° \
                 when you haven't moved at all. This accumulated error is drift.",
    },
    Term {
        word: "Gaussian",
        short: "Bell-curve shaped probability distribution",
        detail: "Also called 'normal distribution'. Most measurements cluster around the true \
                 value, with fewer measurements far away. The bell curve shape appears \
                 everywhere in nature - heights, test scores, measurement errors.",
    },
    Term {
        word: "covariance",
        short: "How much uncertainty we have",
        detail: "A number (or matrix) that describes how spread out our estimates are. \
                 High covariance = very uncertain, our guess could be way off. \
                 Low covariance = confident, we're pretty sure where it is.",
    },
    Term {
        word: "state",
        short: "Everything we want to know about the system",
        detail: "For a robot, the state might be: position (x, y), orientation (which way \
                 it's facing), and velocity (how fast it's moving). The filter's job is \
                 to estimate this state from noisy sensor data.",
    },
    Term {
        word: "particle",
        short: "One guess about what the state might be",
        detail: "Instead of tracking one estimate, we track hundreds of guesses (particles). \
                 Each particle is a hypothesis: 'maybe the robot is HERE'. Particles that \
                 match sensor readings survive; wrong guesses die off.",
    },
    Term {
        word: "landmark",
        short: "A recognizable feature in the environment",
        detail: "Something the robot can see and recognize - a door, a corner, a unique \
                 pattern. By measuring distances to known landmarks, the robot can \
                 figure out where it is (like navigating by stars).",
    },
    Term {
        word: "loop closure",
        short: "Recognizing you've returned to a place you've been before",
        detail: "When mapping, errors accumulate as you travel. But if you recognize \
                 'I've been here before!', you can correct all the accumulated drift. \
                 This 'closing the loop' snaps the whole map into consistency.",
    },
    Term {
        word: "odometry",
        short: "Estimating position by counting wheel rotations or steps",
        detail: "Calculating where you are based on how much you've moved. \
                 Like counting steps in the dark. It is accurate over short distances \
                 but drifts over time as small errors add up.",
    },
    Term {
        word: "proprioception",
        short: "Internal sensing (Sensing self)",
        detail: "Sensors that measure what the robot is doing internally. \
                 Examples: Encoders (wheel speed), IMU (acceleration/rotation). \
                 These don't need the outside world to work, but they drift.",
    },
    Term {
        word: "exteroception",
        short: "External sensing (Sensing the world)",
        detail: "Sensors that look at the world around the robot. \
                 Examples: Cameras, Lidar, Radar, GPS. \
                 These allow the robot to correct drift by spotting known landmarks.",
    },
];

/// A single SLAM lesson
pub struct Lesson {
    pub id: usize,
    pub title: &'static str,
    pub subtitle: &'static str,
    pub icon: &'static str,
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
}

/// All SLAM lessons - ordered from simple intuition to complex algorithms
pub static LESSONS: &[Lesson] = &[
    // ═══════════════════════════════════════════════════════════════════════════
    // LESSON 0: The Core Problem (Why SLAM is Hard)
    // ═══════════════════════════════════════════════════════════════════════════
    Lesson {
        id: 0,
        title: "The Robot's Dilemma",
        subtitle: "Uncertainty & The Loop",
        icon: "🤖",
        why_it_matters: "Before we solve SLAM, we must feel the pain of NOT having it. \
                         Why can't robots just know where they are?",
        intuition: "<h3>The Dark Hallway Analogy</h3>\n\
            Imagine you are standing in a pitch-black hallway. You want to walk 10 meters forward.\n\n\
            <strong>Strategy A: Counting Steps (Odometry)</strong><br>\
            You close your eyes and count steps. 1, 2, 3... You think you've moved 10 meters. \
            But are you sure? Maybe your stride was short. Maybe you slipped slightly. \
            Without seeing, your uncertainty grows with every step. After 100 meters, \
            you could be anywhere.<br>\n\
            <em>This is <strong>Internal Sensing</strong> (Proprioception). It's smooth but drifts.</em>\n\n\
            <strong>Strategy B: Touching the Wall (Landmarks)</strong><br>\
            Now, imagine you touch a doorframe. You know exactly where that door is on your mental map. \
            Instantly, your uncertainty collapses! You verify: 'Ah, I am at the kitchen door.'<br>\n\
            <em>This is <strong>External Sensing</strong> (Exteroception). It corrects drift.</em>\n\n\
            <strong>The Cycle of SLAM:</strong><br>\
            Robotics is just this dance repeated forever:\n\
            1. <strong>Predict:</strong> Close eyes, take a step (Uncertainty grows 📈)\n\
            2. <strong>Update:</strong> Open eyes, see landmark (Uncertainty shrinks 📉)\n\n\
            <div class=\"mermaid\">\n\
            graph LR\n\
                A[Start] --> B(Predict: Move Step)\n\
                B --> C{Uncertainty Grows}\n\
                C -->|See Landmark| D[Update: Fix Position]\n\
                C -->|No Landmark| B\n\
                D -->|Uncertainty Shrinks| B\n\
            </div>\n\n\
            All the math we will learn—Kalman Filters, Particle Filters, Graph SLAM—is just \
            different ways to mathematically model this 'Open Eyes / Close Eyes' dance.",
        demo_explanation: r#"
                You are in a <strong>Dark Hallway</strong>. The ghost shows your TRUE position; the solid figure shows where you THINK you are.
                <br><br>
                <strong>🎮 Try This:</strong>
                <ol>
                <li><strong>Blind Walk:</strong> Click 'Step Blindly' 10 times. Watch the uncertainty bubble grow and your estimate drift from reality.</li>
                <li><strong>Landmark Fix:</strong> Click 'Touch Wall' when near a door (at 15m, 30m, 45m). See your uncertainty collapse instantly!</li>
                <li><strong>Challenge:</strong> Walk 50 meters staying within 3m of your true position. Strategy: touch walls often!</li>
                <li><strong>Failure Mode:</strong> Walk 30 steps without touching anything. How lost are you now?</li>
                </ol>
                <br>
                <strong>Key Insight:</strong> This is every robot's dilemma—movement creates uncertainty, landmarks fix it.
            "#,
        key_takeaways: &[
            "Internal sensors (odometry) accumulate error over time (Drift)",
            "External sensors (cameras/lidar) fix errors relative to landmarks",
            "SLAM is the cycle of Prediction (Movement) and Correction (Measurement)",
        ],
        going_deeper: "<strong>In Nature:</strong> This is Path Integration (dead reckoning) vs. Allothetic Navigation \
                       (external cues). The hippocampus in mammal brains contains 'grid cells' that perform biological SLAM! \
                       (2014 Nobel Prize in Physiology).<br><br>\
                       <strong>In Robotics:</strong> Every robot faces this dilemma—from Mars rovers (use sun angle as landmarks) \
                       to warehouse robots (use QR codes on floors) to self-driving cars (use lane markings and signs).",
        math_details: "x_t = f(x_{t-1}, u_t)  [Motion Model: Uncertainty increases]\n\
                       z_t = h(x_t)           [Measurement Model: Uncertainty decreases]",
    },

    // ═══════════════════════════════════════════════════════════════════════════
    // LESSON 1: Complementary Filter (Trusting Two Senses)
    // ═══════════════════════════════════════════════════════════════════════════
    Lesson {
        id: 1,
        title: "Complementary Filter",
        subtitle: "Your First Sensor Fusion",
        icon: "🔄",
        why_it_matters: "The first step to solving the dilemma: what if we have TWO internal sensors \
                         that lie in opposite ways? We can make them police each other.",
        intuition: "<h3>The Drunk Friend & The Watch Analogy</h3>\n\
            Imagine you want to know what time it is, but you have two flawed sources:\n\n\
            1. <strong>The Watch (Gyro):</strong> It runs smoothly, but it's fast. Every hour, it gains 1 minute. \
            If you only check it once a day, it's fine. But wait a week, and it's hours off.<br>\
            <em>Problem: Drift (Long-term error)</em>\n\n\
            2. <strong>The Drunk Friend (Accel):</strong> He yells the time at you. He's roughly right on average, \
            but he shouts 'It's 2:03! No, 2:05! No, 2:01!' from second to second.<br>\
            <em>Problem: Noise (Short-term jitter)</em>\n\n\
            <strong>The Solution:</strong><br>\
            Listen to the Watch for second-to-second changes (it's smooth). \
            Listen to the Friend to check if you're roughly on track every hour (he corrects the drift).\n\n\
            <strong>The Real Sensors:</strong> The Gyroscope is the smooth watch (drifts over time). The Accelerometer \
            is the noisy friend (jittery but always knows which way is 'down'). We mix them mathematically.\n\n\
            <div class=\"mermaid\">\n\
            graph TD\n\
                A[Gyroscope] -->|High Pass| C(Integration)\n\
                B[Accelerometer] -->|Low Pass| D(Gravity Vector)\n\
                C --> E((Fusion))\n\
                D --> E\n\
                E --> F[Estimated Angle]\n\
            </div>",
        demo_explanation: "<strong>Red Line:</strong> Accelerometer (noisy but knows 'down')\n\
            <strong>Blue Line:</strong> Gyroscope (smooth but drifts over time)\n\
            <strong>Green Line:</strong> Fused estimate—the best of both!\n\n\
            <strong>🎮 Try This:</strong>\n\
            <ol>\n\
            <li><strong>Extreme Trust in Gyro:</strong> Set α = 0.99. Watch it drift over 30 seconds.</li>\n\
            <li><strong>Extreme Trust in Accel:</strong> Set α = 0.50. See the jitter!</li>\n\
            <li><strong>Find the Sweet Spot:</strong> Try α = 0.96. Notice: smooth AND stable.</li>\n\
            <li><strong>Shake Test:</strong> Watch during rapid movements—which sensor handles it better?</li>\n\
            </ol>\n\n\
            <strong>Real Drones:</strong> This exact filter runs on flight controllers at 1000Hz!",
        key_takeaways: &[
            "Gyroscope = Low Drift, High Smoothness (Good for fast moves)",
            "Accelerometer = High Noise, No Drift (Good for long term)",
            "Complementary Filter blends them: High-pass Gyro + Low-pass Accel",
        ],
        going_deeper: "<strong>The Theory:</strong> This is a frequency-domain approach. We trust the Gyro for high frequencies \
                       (fast changes) and the Accel for low frequencies (gravity).<br><br>\
                       <strong>In Practice:</strong> It's computationally almost free (~10 multiplies), which is why it's on \
                       EVERY flight controller: Betaflight, ArduPilot, PX4. Your DJI drone, racing quad, and even \
                       smartphone camera stabilization all use variations of this filter running at 1000-8000Hz.",
        math_details: "angle = α × (angle + gyro×dt) + (1-α) × accel_angle\n\n\
                       If α = 0.98:\n\
                       98% trust in Gyro integration (prediction)\n\
                       2% trust in Accelerometer (correction)",
    },

    // ═══════════════════════════════════════════════════════════════════════════
    // LESSON 2: Kalman Filter (The Optimal Bet)
    // ═══════════════════════════════════════════════════════════════════════════
    Lesson {
        id: 2,
        title: "Kalman Filter",
        subtitle: "Optimal Sensor Fusion",
        icon: "📊",
        why_it_matters: "In Lesson 1, we guessed α = 0.98. But why 0.98? What if conditions change? \
                         The Kalman Filter calculates the OPTIMAL trust factor for every single moment, automatically.",
        intuition: "<h3>The GPS vs. Speedometer Analogy</h3>\n\
            You are driving in a tunnel. \n\n\
            1. <strong>Prediction (Speedometer):</strong> You see you're going 60mph. Logic says 'I am 1 mile further than before.' \
            But tires can slip, roads can curve. Your uncertainty bubble <strong>grows</strong> 🎈.\n\n\
            2. <strong>Correction (GPS):</strong> Suddenly, the GPS gets a signal! It says 'You are at Exit 4.' \
            This measurement is noisy, but it anchors you. Your uncertainty bubble <strong>shrinks</strong> 🤏.\n\n\
            <strong>The Magic of Kalman Gain:</strong><br>\
            The filter asks: 'Who do I trust more right now?'\n\
            • If GPS is garbage (tunnel), trust speedometer (Gain ~ 0)\n\
            • If tires are slipping (ice), trust GPS (Gain ~ 1)\n\n\
            It dynamically adjusts this trust 100 times a second.",
        demo_explanation: "The <strong>ellipse</strong> represents uncertainty (bigger = less confident).\n\n\
            <strong>Legend:</strong>\n\
            • <strong>Green dot:</strong> True position (hidden from filter)\n\
            • <strong>Cyan dot + ellipse:</strong> Kalman estimate with uncertainty\n\
            • <strong>Yellow dots:</strong> GPS measurements (noisy but absolute)\n\n\
            <strong>🎮 Try This:</strong>\n\
            <ol>\n\
            <li><strong>Watch the Bubble:</strong> See it GROW during movement, SHRINK on GPS update.</li>\n\
            <li><strong>GPS Blackout:</strong> Increase GPS interval to 5s. Watch drift accumulate!</li>\n\
            <li><strong>Noisy GPS:</strong> Increase measurement noise. The filter trusts it less.</li>\n\
            <li><strong>Perfect GPS:</strong> Set measurement noise very low. Filter snaps to GPS instantly.</li>\n\
            </ol>\n\n\
            <strong>Real World:</strong> This is how your phone fuses GPS + accelerometer!",
        key_takeaways: &[
            "Prediction (Motion) always INCREASES uncertainty",
            "Update (Measurement) always DECREASES uncertainty",
            "Kalman Gain is the calculated 'Trust Factor'",
            "Gaussian (Bell Curve) assumption is both its power and its weakness",
        ],
        going_deeper: "<strong>The Theory:</strong> The Kalman filter is the Best Linear Unbiased Estimator (BLUE). \
                       It assumes everything is Gaussian. If your robot hits a wall (non-linear stop), \
                       Kalman fails—that's why we need Lesson 3.<br><br>\
                       <strong>Famous Uses:</strong> Apollo 11 lunar lander used Kalman filtering to land on the moon. \
                       Your phone's GPS uses it to smooth jumpy satellite readings. Tesla cars use Extended Kalman Filters \
                       (EKF) to fuse camera, radar, and ultrasonic sensors for Autopilot.",
        math_details: "1. PREDICT (Bubble Grows):\n\
                       x' = Fx + Bu  (Physics projection)\n\
                       P' = FPFᵀ + Q (Add uncertainty Q)\n\n\
                       2. UPDATE (Bubble Shrinks):\n\
                       K = P'Hᵀ(HP'Hᵀ + R)⁻¹  (Calculate Gain)\n\
                       x = x' + K(z - Hx')    (Weighted Average)\n\
                       P = (I - KH)P'         (Shrink Covariance)",
    },

    // ═══════════════════════════════════════════════════════════════════════════
    // LESSON 3: Particle Filter (The Multiverse)
    // ═══════════════════════════════════════════════════════════════════════════
    Lesson {
        id: 3,
        title: "Particle Filter",
        subtitle: "Monte Carlo Localization",
        icon: "🎯",
        why_it_matters: "Kalman (Lesson 2) assumes you're roughly 'here' (one bell curve). \
                         But what if you wake up with NO idea where you are? Or you might be in Room A OR Room B? \
                         Particle filters can represent ANY uncertainty shape—even multimodal 'I'm in two places'.",
        intuition: "<h3>The 'Kidnapped Robot' Problem</h3>\n\
            Imagine you wake up in a generic office building. You see a hallway. \
            <strong>Hypothesis 1:</strong> 'I'm on floor 1.'\n\
            <strong>Hypothesis 2:</strong> 'I'm on floor 2.'\n\n\
            Kalman cannot handle 'I am in two places at once'. It would average them and say \
            'You are floating between floors'. 💥\n\n\
            <strong>The Solution: The Multiverse (Particles)</strong><br>\
            Instead of tracking ONE position, we simulate 1,000 parallel universe robots.\n\
            • Universe 1 robot is in the kitchen.\n\
            • Universe 2 robot is in the hallway.\n\
            • ...\n\n\
            As real-you walks 5 meters and sees a red door:\n\
            • Universe 1 robot says 'There is no red door in the kitchen'. <strong>DELETE.</strong>\n\
            • Universe 2 robot says 'Yes! Red door matches!' <strong>CLONE/MULTIPLY.</strong>\n\n\
            This Survival of the Fittest algorithm naturally converges on the truth.",
        demo_explanation: "Survival of the fittest—wrong hypotheses die, correct ones multiply!\n\n\
            <strong>Legend:</strong>\n\
            • <strong>Green triangle:</strong> True robot pose (hidden from filter)\n\
            • <strong>Orange dots:</strong> Particles (hypotheses)\n\
            • <strong>Cyan triangle:</strong> Best estimate (weighted average)\n\
            • <strong>Blue squares:</strong> Known landmarks\n\n\
            <strong>🎮 Try This:</strong>\n\
            <ol>\n\
            <li><strong>Step Mode:</strong> Enable it! Watch each phase: PREDICT → UPDATE → RESAMPLE</li>\n\
            <li><strong>Kidnapped Robot:</strong> Click 'Reset Random'. See particles scattered everywhere, then converge!</li>\n\
            <li><strong>Particle Starvation:</strong> Reduce particles to 50. Does it still work?</li>\n\
            <li><strong>Ambiguous Rooms:</strong> Notice when particles split between two possible locations.</li>\n\
            </ol>\n\n\
            <strong>Real World:</strong> Roomba vacuums use this exact algorithm to localize!",
        key_takeaways: &[
            "Particles = Parallel Universe Hypotheses",
            "Resampling = Survival of the Fittest (Evolution)",
            "Can solve Global Localization (Lost Robot problem)",
            "Computationally heavy (simulating 1000 robots takes CPU)",
        ],
        going_deeper: "<strong>The Theory:</strong> This approximates ANY probability distribution using discrete samples \
                       (Monte Carlo). As N → ∞, it becomes perfect. In practice, keeping enough particles \
                       to cover a whole building is hard, so we use 'Adaptive Monte Carlo Localization' (AMCL).<br><br>\
                       <strong>In the Wild:</strong> iRobot Roomba vacuums use particle filters for localization. \
                       ROS (Robot Operating System) ships with AMCL as the default localization package. \
                       Amazon warehouse robots (Kiva) use particle filters to navigate between QR code landmarks.",
        math_details: "For each particle i:\n\
                       1. x_i = motion_model(x_i, u) + noise\n\
                       2. w_i = measurement_prob(z | x_i)\n\
                       3. Draw new set of particles based on weights w_i\n\
                       (High weight = likely to be picked multiple times)",
    },

    // ═══════════════════════════════════════════════════════════════════════════
    // LESSON 4: EKF SLAM (The Chicken & Egg)
    // ═══════════════════════════════════════════════════════════════════════════
    Lesson {
        id: 4,
        title: "EKF SLAM",
        subtitle: "Building the Map While You Navigate",
        icon: "🗺️",
        why_it_matters: "So far, we assumed we HAD a map (Lessons 0-3). But in the real world, \
                         you need a map to localize AND you need to know your location to build a map. \
                         This is the chicken-and-egg problem. SLAM solves BOTH simultaneously!",
        intuition: "<h3>The Unknown Cave Problem</h3>\n\
            So far, we assumed we had a map (we knew where the door/landmark was). \
            What if we don't? \n\n\
            1. To know where I am, I need the map.\n\
            2. To build the map, I need to know where I am.\n\n\
            <strong>The Solution: Correlation (Entanglement)</strong><br>\
            SLAM says: 'Okay, I don't know my location, and I don't know the landmark location. \
            BUT I know relatively how far apart we are!'\n\n\
            If I find out later that I was 1 meter to the left, I instantly know the landmark \
            must also be 1 meter to the left. The robot and map are connected by invisible \
            springs of mathematics (Covariance).",
        demo_explanation: "Watch the <strong>ellipses</strong>—they show uncertainty for both the robot AND the map!\n\n\
            <strong>Legend:</strong>\n\
            • <strong>Robot ellipse:</strong> Where we think the robot is\n\
            • <strong>Landmark ellipses:</strong> Where we think each landmark is\n\
            • <strong>Correlations:</strong> Robot and landmarks are mathematically linked!\n\n\
            <strong>🎮 Try This:</strong>\n\
            <ol>\n\
            <li><strong>Explore:</strong> Move around. Watch robot uncertainty grow.</li>\n\
            <li><strong>First Sighting:</strong> See a new landmark. Its uncertainty starts HUGE.</li>\n\
            <li><strong>Loop Closure Magic:</strong> Return to a landmark you've seen before. \
            Watch EVERYTHING snap into place—robot AND all landmarks!</li>\n\
            <li><strong>The Entanglement:</strong> Notice how landmark ellipses shrink together. They're connected!</li>\n\
            </ol>\n\n\
            <strong>Real World:</strong> Early self-driving car prototypes used EKF SLAM!",
        key_takeaways: &[
            "SLAM estimates robot pose AND map simultaneously",
            "New observations create correlations between estimates",
            "Loop closure (revisiting) dramatically reduces uncertainty",
            "Computational cost grows with number of landmarks",
        ],
        going_deeper: "<strong>The Limitation:</strong> EKF SLAM is O(n²) per update, making it impractical for large maps \
                       (>1000 landmarks). It also struggles with data association—figuring out WHICH landmark you're seeing.<br><br>\
                       <strong>Evolution:</strong> FastSLAM (2002) combined particles + EKFs to break the O(n²) barrier. \
                       Graph SLAM (next lesson) became the modern standard.<br><br>\
                       <strong>Historical Note:</strong> EKF SLAM powered the 2005 DARPA Grand Challenge winner (Stanley). \
                       It's still used in small-scale applications like indoor robot vacuums and educational platforms.",
        math_details: "State vector: [robot_x, robot_y, robot_θ, lm1_x, lm1_y, lm2_x, ...]\n\n\
                       The covariance matrix tracks correlations between ALL pairs:\n\
                       Σ = [Σ_rr  Σ_rm₁  Σ_rm₂ ...]\n\
                           [Σ_m₁r Σ_m₁m₁ Σ_m₁m₂...]\n\
                           [...                    ]\n\n\
                       Observing landmark i updates ALL correlated estimates.",
    },

    // ═══════════════════════════════════════════════════════════════════════════
    // LESSON 5: Graph SLAM (The Rubber Sheet)
    // ═══════════════════════════════════════════════════════════════════════════
    Lesson {
        id: 5,
        title: "Graph SLAM",
        subtitle: "Scaling to Real-World Maps",
        icon: "🔗",

        why_it_matters: "EKF SLAM (Lesson 4) updates the ENTIRE state every step—that's O(n²) and doesn't scale. \
                         Graph SLAM takes a different approach: collect ALL constraints, then solve the whole puzzle at once. \
                         This is how modern self-driving cars actually work!",
        intuition: "<h3>The Rubber Band Graph</h3>\n\
            Forget complex matrices. Imagine every place the robot stood is a pin 📌.\n\
            Every measurement is a rubber band connecting pins.\n\n\
            1. <strong>Odometry:</strong> 'I moved 1m forward' = Rubber band between Pose A and Pose B.\n\
            2. <strong>Loop Closure:</strong> 'Hey, Pose Z looks just like Pose A!' = A strong rubber band connecting the start and end.\n\n\
            If you have drift, the rubber bands are stretched and tight. \
            When you press <strong>Optimize</strong>, the physics engine effectively lets the graph \
            snap into its most comfortable (lowest energy) shape. Everything aligns!\n\n\
            <div class=\"mermaid\">\n\
            graph LR\n\
                P1((Pose 1)) -->|Odom| P2((Pose 2))\n\
                P2 -->|Odom| P3((Pose 3))\n\
                P3 -->|Odom| P4((Pose 4))\n\
                P1 -->|Loop Closure| P4\n\
                style P1 fill:#2E7D32,stroke:#000,stroke-width:2px,color:#fff\n\
                style P4 fill:#C62828,stroke:#000,stroke-width:2px,color:#fff\n\
            </div>\n\n\
            Graph SLAM is basically a giant spring-mass system. We minimize the 'tension' (error) in the springs.",
        demo_explanation: "A graph of poses connected by 'rubber band' constraints!\n\n\
            <strong>Legend:</strong>\n\
            • <strong>Nodes:</strong> Robot poses at each timestep\n\
            • <strong>Blue edges:</strong> Odometry constraints (I moved X meters)\n\
            • <strong>Green edges:</strong> Loop closure constraints (I've been here before!)\n\n\
            <strong>🎮 Try This:</strong>\n\
            <ol>\n\
            <li><strong>Build a Path:</strong> Let the robot move. Watch drift accumulate as it loops.</li>\n\
            <li><strong>Feel the Tension:</strong> The drifted path has 'stretched' constraints.</li>\n\
            <li><strong>Add Loop Closure:</strong> When the robot returns to the start, add a loop closure edge.</li>\n\
            <li><strong>Optimize:</strong> Click it! Watch the entire graph snap into a consistent shape.</li>\n\
            <li><strong>Multiple Loops:</strong> Try adding multiple loop closures. Each one tightens the map!</li>\n\
            </ol>\n\n\
            <strong>Real World:</strong> Google Street View and Tesla Autopilot use Graph SLAM!",
        key_takeaways: &[
            "Don't filter step-by-step; optimize the whole path at once",
            "Every constraint is a spring; Optimization finds the relaxation state",
            "Sparsity: Most places are only connected to their neighbors",
        ],
        going_deeper: "<strong>The Math:</strong> Modern Graph SLAM uses 'Factor Graphs' and sparse linear algebra \
                       to solve systems with millions of variables in milliseconds.<br><br>\
                       <strong>Key Libraries:</strong> g2o (general graph optimization), GTSAM (Georgia Tech), \
                       Ceres Solver (Google). These power most research and production SLAM systems.<br><br>\
                       <strong>In Production:</strong> Google Street View cars map cities using Graph SLAM. \
                       Waymo and Tesla use it for HD map creation. Boston Dynamics' Spot robot builds 3D maps \
                       in real-time. Even your iPhone's LiDAR uses a variant for room scanning!",
        math_details: "minimize E = Σ (z_ij - h(x_i, x_j))²\n\n\
                       We are finding the set of poses {x} that minimizes the total tension \
                       in all the rubber bands (constraints).",
    },
];
