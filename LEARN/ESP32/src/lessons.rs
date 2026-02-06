//! ═══════════════════════════════════════════════════════════════════════════════
//! FILE: lessons.rs | ESP32/src/lessons.rs
//! PURPOSE: Electronics course - from basic circuits to ESP32 capstone
//! MODIFIED: 2025-12-30
//! LAYER: LEARN → ESP32
//! ═══════════════════════════════════════════════════════════════════════════════

/// Demo type for a lesson
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DemoType {
    /// No interactive demo - story/theory content only
    Static,
    /// Interactive canvas demo
    Canvas,
    /// Static content with interactive calculator widget
    Calculator,
}

/// Technical term that can have a tooltip explanation
#[derive(Clone)]
pub struct Term {
    pub word: &'static str,
    pub short: &'static str,
    pub detail: &'static str,
}

/// Glossary of terms used across lessons
pub static GLOSSARY: &[Term] = &[
    Term {
        word: "Ohm's Law",
        short: "V = I × R (voltage equals current times resistance)",
        detail: "The fundamental relationship in DC circuits. If you know any two of voltage, current, or resistance, you can calculate the third.",
    },
    Term {
        word: "power",
        short: "P = V × I (watts)",
        detail: "Power is energy per unit time. In electronics, P = V × I. High power means heat—components have maximum power ratings.",
    },
    Term {
        word: "voltage divider",
        short: "Two resistors in series create a fraction of the input voltage",
        detail: "V_out = V_in × (R2 / (R1 + R2)). Used for level shifting, sensor scaling, and battery monitoring.",
    },
    Term {
        word: "RC time constant",
        short: "τ = R × C (how fast a capacitor charges/discharges)",
        detail: "After one time constant (τ), a capacitor charges to ~63% of the final voltage. After 5τ, it's essentially fully charged.",
    },
    Term {
        word: "pull-up",
        short: "A resistor that makes an input read HIGH by default",
        detail: "A pull-up (external or internal) prevents a GPIO input from floating. \
                 For a button-to-GND wiring, enable an internal pull-up or add ~10kΩ to 3.3V.",
    },
    Term {
        word: "floating",
        short: "An input with no defined HIGH/LOW state",
        detail: "Floating inputs pick up noise and randomly flip. Always use pull-ups or pull-downs.",
    },
    Term {
        word: "strapping pin",
        short: "A GPIO that affects boot mode on reset",
        detail: "Some ESP32 pins are sampled on reset to decide boot configuration. \
                 External circuits (like buttons) can accidentally force a bad boot mode.",
    },
    Term {
        word: "open-drain",
        short: "Outputs can pull LOW, but need pull-ups for HIGH",
        detail: "I²C uses open-drain so multiple devices can share a wire safely. \
                 No device actively drives HIGH; pull-up resistors do that.",
    },
    Term {
        word: "duty cycle",
        short: "Fraction of time the PWM signal is HIGH",
        detail: "Duty cycle controls average power: 25% duty means HIGH for 1/4 of each period.",
    },
    Term {
        word: "quantization",
        short: "Rounding a continuous value into discrete steps",
        detail: "ADCs quantize voltages into integer codes; PWM duty also quantizes into timer steps.",
    },
    Term {
        word: "attenuation",
        short: "ADC setting that changes the measurable voltage range",
        detail: "Higher attenuation allows measuring higher voltages but may reduce linearity/accuracy.",
    },
    Term {
        word: "ACK",
        short: "An acknowledge bit (SDA LOW on the 9th clock)",
        detail: "In I²C, the receiver pulls SDA LOW after a byte to acknowledge it was received.",
    },
    Term {
        word: "NACK",
        short: "A no-acknowledge bit (SDA HIGH on the 9th clock)",
        detail: "In I²C, NACK often means 'no device responded' or 'stop sending me data'.",
    },
    Term {
        word: "deep sleep",
        short: "ESP32 power mode that consumes microamps",
        detail: "In deep sleep, the CPU stops, RAM is lost (unless using RTC memory), and only RTC peripherals run. Wake sources include timer, GPIO, or touch.",
    },
    Term {
        word: "power budget",
        short: "Calculating total energy consumption over time",
        detail: "Sum (current × time) for each mode (active, sleep, transmit). Critical for battery-powered devices.",
    },
];

/// A single Electronics lesson
pub struct Lesson {
    pub id: usize,
    pub title: &'static str,
    pub subtitle: &'static str,
    pub icon: &'static str,
    pub phase: &'static str,
    pub demo_type: DemoType,
    /// The hook - why should I care?
    pub why_it_matters: &'static str,
    /// Intuitive explanation + mini lab (HTML allowed)
    pub intuition: &'static str,
    /// What the demo shows / how to use controls
    pub demo_explanation: &'static str,
    /// What should stick
    pub key_takeaways: &'static [&'static str],
    /// Deeper notes (expandable)
    pub going_deeper: &'static str,
    /// Timing / formulas / details (expandable)
    pub math_details: &'static str,
    /// Implementation guide with code prompts and hardware examples
    pub implementation: &'static str,
}

/// All Electronics lessons - ordered from basic circuits to ESP32 capstone
pub static LESSONS: &[Lesson] = &[
    // ═══════════════════════════════════════════════════════════════════════════
    // PHASE 0: The Promise + Safety
    // ═══════════════════════════════════════════════════════════════════════════
    Lesson {
        id: 0,
        title: "Course Map",
        subtitle: "What You'll Build",
        icon: "🗺️",
        phase: "The Promise + Safety",
        demo_type: DemoType::Static,
        why_it_matters:
            "By the end of this course, you'll build a <strong>battery-powered ESP32 environmental monitor</strong> \
             that runs for months on a single charge. This is the roadmap.",
        intuition: r#"
            <h3>The Capstone Project</h3>
            You'll build a <strong>battery-powered ESP32 environmental monitor</strong> that:
            <ul>
              <li>Measures temperature, humidity, and pressure via I²C sensor</li>
              <li>Transmits data over Wi‑Fi every 5 minutes</li>
              <li>Sleeps in deep sleep mode between readings</li>
              <li>Runs for <strong>months</strong> on a single LiPo battery</li>
            </ul>

            <div class="mermaid">
            flowchart TD
                Start[Power On] --> Wake[Wake from Deep Sleep]
                Wake --> Read[Read I2C Sensor]
                Read --> WiFi[Connect WiFi + Transmit]
                WiFi --> Sleep[Deep Sleep 5min]
                Sleep --> Wake
            </div>

            <h3>Why This Matters</h3>
            Most tutorials show you how to blink an LED. Real projects need to:
            <ul>
              <li><strong>Understand circuits</strong> — voltage dividers, current limiting, power budgets</li>
              <li><strong>Master microcontrollers</strong> — GPIO, PWM, ADC, I²C, UART</li>
              <li><strong>Optimize power</strong> — deep sleep, duty cycling, efficient protocols</li>
              <li><strong>Ship something</strong> — validation, enclosure, deployment</li>
            </ul>

            <h3>The Learning Path</h3>
            We'll start with <strong>basic electronics</strong> (Ohm's Law, components), then <strong>microcontroller fundamentals</strong> \
            (GPIO, PWM, ADC, I²C), then <strong>ESP32 specifics</strong> (deep sleep, Wi‑Fi), and finally <strong>put it all together</strong> \
            in the capstone.
        "#,
        demo_explanation: r#"
            This is a course overview — no demo yet! Scroll down to see the full curriculum organized by phase.
        "#,
        key_takeaways: &[
            "The capstone is a battery-powered ESP32 environmental monitor",
            "You'll learn circuits → microcontrollers → ESP32 → capstone",
            "Every lesson builds toward the final project",
            "Real projects require power optimization and careful design",
        ],
        going_deeper:
            "This course uses <strong>Rust</strong> as the primary toolchain (esp-hal / esp-idf). \
             Rust's memory safety and zero-cost abstractions make it ideal for embedded systems. \
             If you're new to Rust, don't worry — we'll cover the essentials as we go.",
        math_details: r#"
The capstone power budget (rough estimate):
  Active mode: 80mA × 2s = 160mAs per reading
  Deep sleep: 10µA × 298s = 2.98mAs per cycle
  Total per 5min cycle: ~163mAs

  For a 2000mAh battery:
  Cycles = (2000mAh × 3600s/h) / 163mAs ≈ 44,000 cycles
  Lifetime ≈ 44,000 × 5min ≈ 153 days

  (Real-world will be less due to Wi‑Fi connection overhead, but this shows the math.)
        "#,
        implementation: r#"
<h4>Hardware Shopping List</h4>
<ul>
<li>ESP32-DevKitC or ESP32-WROOM-32 module</li>
<li>SHT31 or BME280 sensor (I²C, 3.3V)</li>
<li>LiPo battery (3.7V, 2000mAh recommended)</li>
<li>LiPo charger/protection board (TP4056 + DW01)</li>
<li>Breadboard + jumper wires</li>
<li>10kΩ resistors (for I²C pull-ups)</li>
</ul>

<h4>Software Setup</h4>
<pre>
# Install Rust toolchain
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install espup (ESP32 Rust toolchain installer)
cargo install espup

# Set up ESP32 Rust environment
espup install
</pre>

<h4>LLM Prompt: Project Scaffold</h4>
<pre>"Create a Rust project scaffold for ESP32 using esp-hal.
Include: Cargo.toml with esp32-hal dependency, main.rs with basic
blink LED example, and README with build/flash instructions.
Target: ESP32-WROOM-32"</pre>
        "#,
    },
    Lesson {
        id: 1,
        title: "Safety & Tools",
        subtitle: "ESD, LiPo Hazards, Multimeter Basics",
        icon: "⚠️",
        phase: "The Promise + Safety",
        demo_type: DemoType::Static,
        why_it_matters:
            "Electronics can <strong>hurt you</strong> (burns, shocks) and <strong>hurt your components</strong> (ESD, overcurrent). \
             Learn the rules before you touch anything.",
        intuition: r#"
            <h3>ESD (Electrostatic Discharge)</h3>
            Your body builds up static charge (walking on carpet, removing a sweater). When you touch a component, \
            that charge can <strong>destroy</strong> sensitive ICs instantly — even if you don't feel a shock.

            <strong>Rules:</strong>
            <ul>
              <li>Touch a grounded metal surface before handling components</li>
              <li>Work on an anti-static mat if possible</li>
              <li>Store ICs in anti-static bags</li>
            </ul>

            <h3>LiPo Battery Hazards</h3>
            Lithium polymer batteries are <strong>dangerous</strong> if mishandled:
            <ul>
              <li><strong>Never</strong> short-circuit (can cause fire)</li>
              <li><strong>Never</strong> overcharge (use a proper charger)</li>
              <li><strong>Never</strong> puncture or crush</li>
              <li>If a battery swells, stop using it immediately</li>
            </ul>

            <h3>Current Limiting</h3>
            GPIO pins can only source/sink ~40mA on ESP32. Driving motors, LEDs, or other loads directly can \
            <strong>damage the chip</strong>. Always use transistors/MOSFETs or driver ICs for high-current loads.

            <h3>Multimeter Basics</h3>
            Your multimeter is your best friend:
            <ul>
              <li><strong>Voltage mode</strong>: Measure voltage across components (parallel)</li>
              <li><strong>Current mode</strong>: Measure current through a circuit (series, break the circuit)</li>
              <li><strong>Resistance mode</strong>: Measure resistor values (power off!)</li>
              <li><strong>Continuity mode</strong>: Check if wires are connected (beeps)</li>
            </ul>

            <h3>Wiring Discipline</h3>
            Messy wiring causes bugs:
            <ul>
              <li>Use color coding (red = power, black = ground)</li>
              <li>Keep wires short to reduce noise</li>
              <li>Double-check connections before powering on</li>
              <li>Use a breadboard power supply with current limiting</li>
            </ul>
        "#,
        demo_explanation: r#"
            Safety is about habits, not demos. Review this lesson before every lab session.
        "#,
        key_takeaways: &[
            "ESD can destroy ICs — always discharge yourself first",
            "LiPo batteries are dangerous if mishandled — use proper chargers",
            "GPIO pins have current limits — use drivers for high-current loads",
            "Multimeters measure voltage (parallel), current (series), resistance (power off)",
        ],
        going_deeper:
            "Professional labs use ESD wrist straps connected to ground. For hobby projects, touching a grounded \
             metal surface (like a computer case) before handling components is usually sufficient. \
             For LiPo safety, always use a protection circuit (DW01) and a proper charger (TP4056).",
        math_details: r#"
Power dissipation in a component:
  P = I² × R  (for resistors)
  P = V × I   (general)

Example: GPIO driving LED with 220Ω resistor at 3.3V:
  I = V / R = 3.3V / 220Ω = 15mA
  P_resistor = I² × R = (0.015A)² × 220Ω = 0.0495W (safe)
  P_LED = V × I = ~2V × 15mA = 0.03W (check LED rating)

ESP32 GPIO max: ~40mA source, ~28mA sink
        "#,
        implementation: r#"
<h4>Essential Tools</h4>
<ul>
<li><strong>Multimeter</strong> — $20-50, measures voltage/current/resistance</li>
<li><strong>Breadboard</strong> — $5-10, for prototyping without soldering</li>
<li><strong>Jumper wires</strong> — $5, male-to-male for breadboards</li>
<li><strong>Resistor kit</strong> — $10, common values (220Ω, 1kΩ, 10kΩ, etc.)</li>
<li><strong>LEDs</strong> — $2, for visual feedback</li>
<li><strong>Breadboard power supply</strong> — $10-20, with current limiting</li>
</ul>

<h4>Safety Checklist</h4>
<ol>
<li>Discharge static before handling components</li>
<li>Verify power supply voltage before connecting</li>
<li>Check for short circuits with multimeter continuity mode</li>
<li>Use current-limiting power supply when possible</li>
<li>Never leave LiPo batteries unattended while charging</li>
</ol>
        "#,
    },

    // ═══════════════════════════════════════════════════════════════════════════
    // PHASE 1: DC Circuits
    // ═══════════════════════════════════════════════════════════════════════════
    Lesson {
        id: 2,
        title: "Ohm's Law + Power",
        subtitle: "V = I × R, P = V × I",
        icon: "⚡",
        phase: "DC Circuits",
        demo_type: DemoType::Canvas,
        why_it_matters:
            "Ohm's Law is the <strong>foundation</strong> of all electronics. Without it, you're guessing. \
             Power tells you if components will overheat and fail.",
        intuition: r#"
            <h3>Ohm's Law</h3>
            The relationship between voltage (V), current (I), and resistance (R):
            <strong>V = I × R</strong>

            <div class="mermaid">
            flowchart LR
                V[Voltage V] -->|Ohm's Law| I[Current I]
                R[Resistance R] -->|Ohm's Law| I
                V -->|Power| P[Power P]
                I -->|Power| P
            </div>

            <h3>What It Means</h3>
            <ul>
              <li><strong>Voltage</strong> (V): The "pressure" pushing electrons</li>
              <li><strong>Current</strong> (I): The flow of electrons (amps)</li>
              <li><strong>Resistance</strong> (R): How much the circuit "resists" current (ohms)</li>
            </ul>

            <h3>Power</h3>
            Power is energy per unit time: <strong>P = V × I</strong>
            <ul>
              <li>Power becomes <strong>heat</strong></li>
              <li>Components have <strong>maximum power ratings</strong></li>
              <li>Exceed the rating → component fails (sometimes spectacularly)</li>
            </ul>

            <h3>Real-World Example</h3>
            ESP32 GPIO pin at 3.3V driving an LED with 220Ω resistor:
            <ul>
              <li>I = V / R = 3.3V / 220Ω = 15mA</li>
              <li>P_resistor = I² × R = (0.015A)² × 220Ω = 0.05W (safe, resistors are usually 0.25W+)</li>
              <li>P_LED = V_LED × I = ~2V × 15mA = 0.03W (check LED datasheet)</li>
            </ul>
        "#,
        demo_explanation: r#"
            Adjust <strong>Voltage</strong> and <strong>Resistance</strong> to see how current and power change.
            <br><br>
            Watch for:
            <ul>
              <li><strong>High current</strong> → components heat up (red indicator)</li>
              <li><strong>Safe operating zone</strong> → green indicator</li>
              <li><strong>Power limit</strong> → what happens if you exceed component ratings</li>
            </ul>
        "#,
        key_takeaways: &[
            "V = I × R — know any two, calculate the third",
            "P = V × I — power becomes heat",
            "Components have maximum power ratings — exceed them at your peril",
            "Always calculate current before connecting components",
        ],
        going_deeper:
            "Ohm's Law applies to <strong>linear</strong> (resistive) circuits. Diodes, transistors, and other \
             non-linear components have more complex relationships. For DC circuits with resistors, Ohm's Law is king.",
        math_details: r#"
Ohm's Law:
  V = I × R
  I = V / R
  R = V / I

Power:
  P = V × I
  P = I² × R  (substitute V = I×R)
  P = V² / R  (substitute I = V/R)

Example calculations:
  Given: V = 5V, R = 1000Ω
  I = 5V / 1000Ω = 0.005A = 5mA
  P = 5V × 0.005A = 0.025W = 25mW

Component ratings:
  Resistor: usually 0.25W (1/4W) or 0.5W (1/2W)
  LED: typically 20mA max current, ~0.1W power
  ESP32 GPIO: 40mA max source, 28mA max sink
        "#,
        implementation: r#"
<h4>LLM Prompt: Current Calculator</h4>
<pre>"Write a Rust function that calculates current and power given voltage and resistance.
Include safety checks: warn if current exceeds 40mA (ESP32 GPIO limit) or
if power exceeds 0.25W (common resistor rating). Return Result with error messages."</pre>

<h4>Lab Exercise</h4>
<ol>
<li>Measure a 220Ω resistor with multimeter (should read ~220Ω)</li>
<li>Connect LED + 220Ω resistor to 3.3V power supply</li>
<li>Measure voltage across resistor (should be ~1.3V if LED drops 2V)</li>
<li>Calculate current: I = V_resistor / R = 1.3V / 220Ω ≈ 5.9mA</li>
<li>Verify LED is bright but not dim (if dim, resistor is too large)</li>
</ol>
        "#,
    },
    Lesson {
        id: 3,
        title: "Series/Parallel + Voltage Divider",
        subtitle: "Resistor Combinations",
        icon: "🔗",
        phase: "DC Circuits",
        demo_type: DemoType::Calculator,
        why_it_matters:
            "Real circuits combine resistors. Series adds resistance; parallel reduces it. \
             Voltage dividers are everywhere — sensors, level shifting, battery monitoring.",
        intuition: r#"
            <h3>Series Resistors</h3>
            Resistors in series: <strong>R_total = R1 + R2 + R3 + ...</strong>
            <ul>
              <li>Current is the <strong>same</strong> through all resistors</li>
              <li>Voltage <strong>drops</strong> across each resistor</li>
              <li>Total resistance is the <strong>sum</strong></li>
            </ul>

            <h3>Parallel Resistors</h3>
            Resistors in parallel: <strong>1/R_total = 1/R1 + 1/R2 + 1/R3 + ...</strong>
            <ul>
              <li>Voltage is the <strong>same</strong> across all resistors</li>
              <li>Current <strong>splits</strong> between branches</li>
              <li>Total resistance is <strong>less</strong> than the smallest resistor</li>
            </ul>

            <h3>Voltage Divider</h3>
            Two resistors in series create a fraction of the input voltage:
            <strong>V_out = V_in × (R2 / (R1 + R2))</strong>

            <div class="mermaid">
            flowchart LR
                Vin["V_in"] --> R1["R1"]
                R1 --> Vout["V_out"]
                Vout --> R2["R2"]
                R2 --> GND["GND"]
            </div>
            <p><em>V_out = V_in × (R2 / (R1 + R2))</em></p>

            <h3>Why It Matters</h3>
            Voltage dividers are used for:
            <ul>
              <li><strong>Battery monitoring</strong> — scale battery voltage to ADC range</li>
              <li><strong>Level shifting</strong> — convert 5V signals to 3.3V</li>
              <li><strong>Sensor scaling</strong> — adjust sensor output to microcontroller range</li>
            </ul>
        "#,
        demo_explanation: r#"
            Use the calculator to explore:
            <ul>
              <li><strong>Series</strong>: Enter R1 and R2, see total resistance</li>
              <li><strong>Parallel</strong>: Enter R1 and R2, see total resistance (always less!)</li>
              <li><strong>Voltage Divider</strong>: Enter V_in, R1, R2, see V_out</li>
            </ul>
        "#,
        key_takeaways: &[
            "Series: R_total = R1 + R2 (resistance adds)",
            "Parallel: 1/R_total = 1/R1 + 1/R2 (resistance reduces)",
            "Voltage divider: V_out = V_in × (R2 / (R1 + R2))",
            "Voltage dividers are used for battery monitoring and level shifting",
        ],
        going_deeper:
            "For parallel resistors, if R1 = R2, then R_total = R1/2. If you have N equal resistors in parallel, \
             R_total = R / N. Voltage dividers have a <strong>loading effect</strong> — if you connect a load \
             (like an ADC), the effective resistance changes. Use high-impedance inputs or buffer with an op-amp.",
        math_details: r#"
Series resistors:
  R_total = R1 + R2 + R3 + ...

Parallel resistors:
  1/R_total = 1/R1 + 1/R2 + 1/R3 + ...

  For two resistors:
  R_total = (R1 × R2) / (R1 + R2)

Voltage divider:
  V_out = V_in × (R2 / (R1 + R2))

  Current through divider:
  I = V_in / (R1 + R2)

  Power in each resistor:
  P_R1 = I² × R1
  P_R2 = I² × R2

Example: Battery monitoring
  Battery: 4.2V (fully charged LiPo)
  Want: 0-3.3V for ESP32 ADC
  Use: R1 = 10kΩ, R2 = 27kΩ
  V_out = 4.2V × (27k / (10k + 27k)) = 4.2V × 0.73 = 3.07V (safe!)
        "#,
        implementation: r#"
<h4>LLM Prompt: Voltage Divider Calculator</h4>
<pre>"Write a Rust function that calculates voltage divider output given V_in, R1, R2.
Include validation: warn if output exceeds 3.3V (ESP32 max) or if current
through divider exceeds 1mA (wasteful for battery-powered devices)."</pre>

<h4>Lab Exercise: Battery Monitor</h4>
<ol>
<li>Design voltage divider: 4.2V LiPo → 0-3.3V for ADC</li>
<li>Calculate R1 and R2 (hint: use R1=10kΩ, R2=27kΩ)</li>
<li>Build circuit on breadboard</li>
<li>Measure V_out with multimeter (should be ~3.07V at 4.2V input)</li>
<li>Verify it scales linearly as battery voltage changes</li>
</ol>
        "#,
    },
    Lesson {
        id: 4,
        title: "Ground, References, and Measurement Gotchas",
        subtitle: "Why 'Ground' Is Not Magic",
        icon: "🌍",
        phase: "DC Circuits",
        demo_type: DemoType::Static,
        why_it_matters:
            "\"Ground\" is just a <strong>reference point</strong>. Understanding this prevents measurement errors, \
             floating circuits, and mysterious bugs.",
        intuition: r#"
            <h3>What Is Ground?</h3>
            <strong>Ground</strong> is just a <strong>reference point</strong> — voltage is always measured <em>relative</em> to something.
            <ul>
              <li>In circuits, ground is usually <strong>0V</strong> (the negative terminal of your power supply)</li>
              <li>It's not "magic" — it's just a convenient reference</li>
              <li>All voltages are measured <strong>relative to ground</strong></li>
            </ul>

            <h3>Common Ground</h3>
            All components in a circuit must share the <strong>same ground</strong>:
            <ul>
              <li>ESP32 ground → power supply ground → sensor ground</li>
              <li>If grounds aren't connected, the circuit won't work</li>
              <li>This is the #1 cause of "it works on the breadboard but not in the enclosure"</li>
            </ul>

            <h3>Measurement Gotchas</h3>
            <ul>
              <li><strong>Floating measurements</strong>: If a circuit isn't powered, multimeter readings are meaningless</li>
              <li><strong>AC vs DC</strong>: Multimeters have separate modes — use DC for battery/microcontroller circuits</li>
              <li><strong>Probe placement</strong>: Voltage is measured <em>across</em> components (parallel), current is measured <em>through</em> (series)</li>
            </ul>

            <h3>Why It Matters</h3>
            Many bugs come from:
            <ul>
              <li>Missing ground connections</li>
              <li>Measuring voltage with circuit powered off</li>
              <li>Using AC mode instead of DC mode</li>
              <li>Not understanding that voltage is relative</li>
            </ul>
        "#,
        demo_explanation: r#"
            Ground is a concept, not a demo. Review this before troubleshooting circuits.
        "#,
        key_takeaways: &[
            "Ground is just a reference point (usually 0V)",
            "All components must share the same ground",
            "Voltage is always measured relative to ground",
            "Missing ground connections cause mysterious bugs",
        ],
        going_deeper:
            "In AC power systems, 'ground' refers to earth ground (literally connected to the earth). \
             In DC circuits, it's just the negative terminal. In mixed-signal systems (analog + digital), \
             you might have separate <strong>analog ground</strong> and <strong>digital ground</strong> \
             connected at a single point to reduce noise coupling.",
        math_details: r#"
Voltage is always relative:
  V_AB = V_A - V_B

  If B is ground (0V):
  V_AB = V_A - 0 = V_A

  So "voltage at point A" means "voltage relative to ground"

Example:
  Battery: 3.7V (positive terminal relative to negative)
  ESP32: VCC = 3.3V (relative to GND pin)
  If ESP32 GND connects to battery negative:
    VCC pin = 3.3V relative to battery negative
    Battery positive = 3.7V relative to battery negative
    Difference = 0.4V (this is why you need a regulator!)
        "#,
        implementation: r#"
<h4>Ground Checklist</h4>
<ol>
<li>Verify all components share the same ground</li>
<li>Check ground connections with multimeter continuity mode</li>
<li>Use DC mode (not AC) for battery/microcontroller measurements</li>
<li>Measure voltage <em>across</em> components (parallel), current <em>through</em> (series)</li>
<li>Power on the circuit before measuring</li>
</ol>

<h4>Troubleshooting</h4>
<p><strong>Symptom:</strong> Circuit doesn't work, but voltages look correct</p>
<p><strong>Check:</strong> Is ground connected? Use continuity mode to verify.</p>

<p><strong>Symptom:</strong> Readings jump around randomly</p>
<p><strong>Check:</strong> Is circuit powered? Are you using DC mode?</p>
        "#,
    },

    // ═══════════════════════════════════════════════════════════════════════════
    // PHASE 2: Components
    // ═══════════════════════════════════════════════════════════════════════════
    Lesson {
        id: 5,
        title: "Capacitors + RC Time Constant",
        subtitle: "Charging, Filtering, Timing",
        icon: "🔋",
        phase: "Components",
        demo_type: DemoType::Canvas,
        why_it_matters:
            "Capacitors <strong>store energy</strong>, <strong>filter noise</strong>, and create <strong>time delays</strong>. \
             The RC time constant (τ = R × C) tells you how fast they charge/discharge.",
        intuition: r#"
            <h3>What Is a Capacitor?</h3>
            A capacitor stores <strong>electrical energy</strong> in an electric field. Think of it as a tiny rechargeable battery \
            that charges/discharges very quickly.

            <div class="mermaid">
            flowchart LR
                V["Voltage"] -->|Charges| C["Capacitor"]
                C -->|Discharges| R["Resistor"]
                R -->|Time Constant| Tau["τ"]
            </div>
            <p><em>Time constant: τ = R × C</em></p>

            <h3>RC Time Constant</h3>
            When charging through a resistor: <strong>τ = R × C</strong>
            <ul>
              <li>After <strong>1τ</strong>: capacitor charges to ~63% of final voltage</li>
              <li>After <strong>5τ</strong>: capacitor is essentially fully charged (~99%)</li>
              <li>Larger R or C → slower charging</li>
            </ul>

            <h3>Why It Matters</h3>
            Capacitors are used for:
            <ul>
              <li><strong>Power supply filtering</strong> — smooth out voltage ripples</li>
              <li><strong>Debouncing</strong> — hardware debounce for buttons</li>
              <li><strong>Timing circuits</strong> — create delays</li>
              <li><strong>Coupling</strong> — block DC, pass AC signals</li>
            </ul>
        "#,
        demo_explanation: r#"
            Adjust <strong>Resistance</strong> and <strong>Capacitance</strong> to see how the charging curve changes.
            <br><br>
            Watch for:
            <ul>
              <li><strong>Time constant (τ)</strong> — how long until 63% charge</li>
              <li><strong>Charging curve</strong> — exponential rise</li>
              <li><strong>Effect of R and C</strong> — larger values = slower charging</li>
            </ul>
        "#,
        key_takeaways: &[
            "Capacitors store energy in an electric field",
            "RC time constant: τ = R × C",
            "After 1τ: ~63% charged, after 5τ: ~99% charged",
            "Capacitors filter noise, create delays, and smooth power supplies",
        ],
        going_deeper:
            "Capacitors have <strong>ESR</strong> (Equivalent Series Resistance) and <strong>leakage current</strong>. \
             For power supply filtering, use <strong>ceramic</strong> capacitors (low ESR, good for high frequencies) \
             and <strong>electrolytic</strong> capacitors (high capacitance, good for low frequencies). \
             For timing, use <strong>film</strong> or <strong>ceramic</strong> capacitors (stable, low leakage).",
        math_details: r#"
RC charging equation:
  V(t) = V_final × (1 - e^(-t/τ))

  Where:
  τ = R × C (time constant in seconds)
  t = time
  V_final = final voltage

At t = τ:
  V(τ) = V_final × (1 - e^(-1)) ≈ V_final × 0.632

At t = 5τ:
  V(5τ) = V_final × (1 - e^(-5)) ≈ V_final × 0.993

Example:
  R = 10kΩ = 10,000Ω
  C = 100µF = 0.0001F
  τ = 10,000 × 0.0001 = 1 second

  After 1 second: ~63% charged
  After 5 seconds: ~99% charged
        "#,
        implementation: r#"
<h4>LLM Prompt: RC Charging Simulator</h4>
<pre>"Write a Rust function that simulates RC charging given R, C, V_final, and time.
Return voltage at that time using V(t) = V_final × (1 - e^(-t/τ)).
Include helper function to calculate time constant τ = R × C."</pre>

<h4>Lab Exercise: Hardware Debounce</h4>
<ol>
<li>Build RC circuit: Button → 10kΩ resistor → 100nF capacitor → GND</li>
<li>Connect capacitor to GPIO input (with pull-up enabled)</li>
<li>Press button rapidly — observe smooth transition (no bounce)</li>
<li>Measure time constant: τ = 10kΩ × 100nF = 1ms</li>
<li>Compare to software debouncing (which is easier but uses CPU)</li>
</ol>
        "#,
    },
    Lesson {
        id: 6,
        title: "Diodes + LEDs",
        subtitle: "Current Limiting, Polarity, Brightness",
        icon: "💡",
        phase: "Components",
        demo_type: DemoType::Static,
        why_it_matters:
            "LEDs are <strong>diodes</strong> that emit light. They have <strong>polarity</strong> (direction matters) \
             and need <strong>current limiting</strong> (resistors) to prevent destruction.",
        intuition: r#"
            <h3>What Is a Diode?</h3>
            A diode is a <strong>one-way valve</strong> for electricity:
            <ul>
              <li><strong>Forward bias</strong>: Current flows (low resistance)</li>
              <li><strong>Reverse bias</strong>: Current blocked (high resistance)</li>
              <li>Diodes have a <strong>voltage drop</strong> (~0.7V for silicon, ~2V for LEDs)</li>
            </ul>

            <h3>LEDs (Light Emitting Diodes)</h3>
            LEDs are diodes that emit light when current flows:
            <ul>
              <li><strong>Anode</strong> (+): Longer leg, connects to positive</li>
              <li><strong>Cathode</strong> (-): Shorter leg, connects to ground</li>
              <li><strong>Voltage drop</strong>: ~2V (red) to ~3.5V (blue/white)</li>
              <li><strong>Current</strong>: Typically 20mA max (check datasheet)</li>
            </ul>

            <h3>Current Limiting</h3>
            LEDs <strong>must</strong> have a current-limiting resistor:
            <ul>
              <li>Without a resistor, LED draws too much current → <strong>destroys itself</strong></li>
              <li>Formula: R = (V_supply - V_LED) / I_desired</li>
              <li>Example: 3.3V supply, 2V LED, 15mA desired → R = (3.3 - 2) / 0.015 = 87Ω (use 100Ω)</li>
            </ul>

            <h3>Brightness Myths</h3>
            <ul>
              <li><strong>Higher voltage ≠ brighter</strong> — current determines brightness</li>
              <li><strong>PWM dimming</strong> — change duty cycle, not voltage</li>
              <li><strong>Color affects voltage drop</strong> — blue/white need more voltage than red</li>
            </ul>
        "#,
        demo_explanation: r#"
            LEDs are simple but critical. Review polarity and current limiting before connecting.
        "#,
        key_takeaways: &[
            "Diodes are one-way valves — direction matters",
            "LEDs have polarity: anode (+) to positive, cathode (-) to ground",
            "Always use a current-limiting resistor with LEDs",
            "Brightness is controlled by current (or PWM duty cycle), not voltage",
        ],
        going_deeper:
            "LEDs have a <strong>forward voltage</strong> (V_f) that varies by color. Red: ~1.8V, Green: ~2.1V, \
             Blue/White: ~3.0-3.5V. For PWM dimming, use frequencies >100Hz to avoid visible flicker. \
             For battery-powered devices, use lower current (5-10mA) to save power.",
        math_details: r#"
Current-limiting resistor:
  R = (V_supply - V_LED) / I_desired

Example calculations:
  V_supply = 3.3V
  V_LED = 2V (red LED)
  I_desired = 15mA = 0.015A

  R = (3.3V - 2V) / 0.015A = 1.3V / 0.015A = 87Ω

  Use standard value: 100Ω (slightly safer, I = 13mA)

Power in resistor:
  P = I² × R = (0.013A)² × 100Ω = 0.017W (safe, <0.25W)

Power in LED:
  P = V_LED × I = 2V × 0.013A = 0.026W
        "#,
        implementation: r#"
<h4>LLM Prompt: LED Resistor Calculator</h4>
<pre>"Write a Rust function that calculates current-limiting resistor for LED.
Input: V_supply, V_LED, I_desired. Output: resistor value (round to nearest
standard value: 220Ω, 330Ω, 470Ω, 1kΩ, etc.). Include validation: warn if
calculated resistor < 100Ω (may be too low) or if I_desired > 20mA."</pre>

<h4>Lab Exercise</h4>
<ol>
<li>Identify LED polarity (longer leg = anode)</li>
<li>Calculate resistor: 3.3V supply, 2V LED, 15mA → R = 87Ω (use 100Ω)</li>
<li>Build circuit: 3.3V → resistor → LED anode → LED cathode → GND</li>
<li>Measure current with multimeter (should be ~13mA)</li>
<li>Try reversing LED — it won't light (proves polarity matters!)</li>
</ol>
        "#,
    },
    Lesson {
        id: 7,
        title: "Transistors/MOSFETs as Switches",
        subtitle: "Why GPIO Can't Drive Everything",
        icon: "🔀",
        phase: "Components",
        demo_type: DemoType::Static,
        why_it_matters:
            "GPIO pins can only source/sink ~40mA. Motors, high-power LEDs, and relays need <strong>more current</strong>. \
             Transistors and MOSFETs act as <strong>switches</strong> controlled by GPIO.",
        intuition: r#"
            <h3>The Problem</h3>
            ESP32 GPIO pins have limits:
            <ul>
              <li><strong>Source</strong>: ~40mA (driving HIGH)</li>
              <li><strong>Sink</strong>: ~28mA (driving LOW)</li>
              <li>Motors need <strong>hundreds of milliamps</strong></li>
              <li>High-power LEDs need <strong>100mA+</strong></li>
            </ul>

            <h3>The Solution: Transistors</h3>
            Transistors act as <strong>switches</strong>:
            <ul>
              <li><strong>Base/Gate</strong>: Control pin (connected to GPIO)</li>
              <li><strong>Collector/Drain</strong>: High-current path</li>
              <li><strong>Emitter/Source</strong>: Ground/reference</li>
              <li>Small current at base → large current flows through collector</li>
            </ul>

            <h3>MOSFETs vs BJTs</h3>
            <ul>
              <li><strong>BJT (Bipolar Junction Transistor)</strong>: Current-controlled, needs base resistor</li>
              <li><strong>MOSFET</strong>: Voltage-controlled, very low gate current, better for GPIO</li>
              <li>For ESP32, use <strong>logic-level MOSFETs</strong> (turn on at 3.3V)</li>
            </ul>

            <h3>Common Applications</h3>
            <ul>
              <li><strong>Motor control</strong> — H-bridge with MOSFETs</li>
              <li><strong>High-power LEDs</strong> — MOSFET switches</li>
              <li><strong>Relays</strong> — transistor drives relay coil</li>
              <li><strong>Power gating</strong> — turn subsystems on/off</li>
            </ul>
        "#,
        demo_explanation: r#"
            Transistors are switches, not demos. Review before designing high-current circuits.
        "#,
        key_takeaways: &[
            "GPIO pins have current limits (~40mA source, ~28mA sink)",
            "Transistors/MOSFETs act as switches controlled by GPIO",
            "MOSFETs are voltage-controlled and better for GPIO than BJTs",
            "Use logic-level MOSFETs for 3.3V ESP32 GPIO",
        ],
        going_deeper:
            "For motor control, use an <strong>H-bridge</strong> (4 MOSFETs) to control direction and speed. \
             Always include <strong>flyback diodes</strong> to protect against back-EMF. For PWM motor control, \
             use MOSFETs with low R_ds(on) to minimize heat. Common logic-level MOSFETs: IRLZ44N, IRF540N \
             (but check V_gs threshold — must be < 3.3V).",
        math_details: r#"
MOSFET as switch:
  When V_gs > V_threshold: MOSFET turns ON (low resistance)
  When V_gs < V_threshold: MOSFET turns OFF (high resistance)

Power dissipation in MOSFET:
  P = I² × R_ds(on)

  Example:
  I = 1A (motor current)
  R_ds(on) = 0.1Ω (typical for logic-level MOSFET)
  P = (1A)² × 0.1Ω = 0.1W (may need heatsink if >0.5W)

Gate current (MOSFET):
  I_gate ≈ 0 (voltage-controlled, negligible current)

  vs BJT base current:
  I_base = I_collector / β (β = current gain, typically 100-300)
  I_base = 1A / 100 = 10mA (needs base resistor!)
        "#,
        implementation: r#"
<h4>LLM Prompt: MOSFET Switch Driver</h4>
<pre>"Write Rust code to control a logic-level MOSFET from ESP32 GPIO.
Include: GPIO setup (output mode), turn ON/OFF functions, and safety
check to ensure GPIO is not driving more than 40mA (MOSFET gate current
is negligible, so this is fine). Target: esp-hal crate."</pre>

<h4>Lab Exercise: High-Power LED</h4>
<ol>
<li>Get logic-level MOSFET (e.g., IRLZ44N)</li>
<li>Connect: GPIO → MOSFET gate, 5V → LED → MOSFET drain, MOSFET source → GND</li>
<li>Program ESP32: GPIO HIGH = LED ON, GPIO LOW = LED OFF</li>
<li>Measure current through LED (should be limited by power supply, not GPIO)</li>
<li>Verify GPIO current is negligible (< 1mA)</li>
</ol>
        "#,
    },

    // ═══════════════════════════════════════════════════════════════════════════
    // PHASE 3: Microcontrollers
    // ═══════════════════════════════════════════════════════════════════════════
    Lesson {
        id: 8,
        title: "GPIO Inputs",
        subtitle: "Floating, Pull-ups, Debouncing",
        icon: "🔘",
        phase: "Microcontrollers",
        demo_type: DemoType::Canvas,
        why_it_matters:
            "A real button press should be <strong>one</strong> event. Without debouncing you get phantom presses, \
             double-clicks, and flaky menus — even with perfect code everywhere else.",
        intuition: r#"
            <h3>Goal</h3>
            Turn a noisy mechanical switch into a clean digital signal you can trust.

            <h3>What's actually happening</h3>
            A switch is two pieces of metal touching. When they first touch, they bounce for a few milliseconds:
            HIGH/LOW/HIGH/LOW… then finally settle. Your CPU is fast enough to see all those transitions.

            <div class="mermaid">
            flowchart LR
                Raw[RawGPIO] --> Debounce[WaitForStability]
                Debounce --> Clean[CleanState]
            </div>

            <h3>Floating Inputs</h3>
            An unconnected GPIO input is <strong>floating</strong> — it picks up noise and randomly flips between HIGH and LOW.
            <ul>
              <li><strong>Pull-up</strong>: Resistor to VCC (makes input HIGH by default)</li>
              <li><strong>Pull-down</strong>: Resistor to GND (makes input LOW by default)</li>
              <li>ESP32 has <strong>internal pull-ups</strong> you can enable in software</li>
            </ul>

            <h3>ESP‑WROOM‑32 wiring note</h3>
            Use a <strong>button-to-GND</strong> wiring and enable an internal <strong>pull-up</strong>:
            <ul>
              <li>GPIO → Button → GND</li>
              <li>Enable internal pull-up in software</li>
            </ul>
            Avoid using a <strong>strapping pin</strong> (e.g. GPIO0/2/12/15) for a button unless you know why.
            Also note: GPIO34–39 are input-only and typically require an <em>external</em> pull-up/down.

            <h3>Mini-lab</h3>
            Try a debounce window of 20–50ms for common tactile switches. If you still see false triggers,
            increase it. If the button feels "laggy", decrease it.
        "#,
        demo_explanation: r#"
            The top timeline is the <strong>raw</strong> GPIO signal (with bounce). The bottom is the <strong>debounced</strong> output.
            <br><br>
            Use:
            <ul>
              <li><strong>Bounce Severity</strong>: how "messy" the button is</li>
              <li><strong>Sample Rate</strong>: how often we check the pin</li>
              <li><strong>Debounce Window</strong>: how long the signal must stay stable before we accept a change</li>
            </ul>
        "#,
        key_takeaways: &[
            "Mechanical switches bounce for a few milliseconds",
            "A floating input will randomly flip — use pull-ups/pull-downs",
            "Debouncing is a small state machine: detect change → wait for stability → accept",
            "On ESP32, some pins are input-only (34–39) and some are strapping pins (boot-sensitive)",
        ],
        going_deeper:
            "Hardware debounce (RC + Schmitt trigger) can reduce CPU work and improve EMI robustness. \
             For event-driven code, interrupts still need debouncing — either by masking interrupts for a window, \
             or by using a timer task to confirm stability.",
        math_details: r#"
Definitions:
  sample_rate = N samples / second
  debounce_window = W seconds

Rule of thumb:
  W should be several times the worst-case bounce duration.

Simple debounce state machine:
  if raw != pending:
      pending = raw
      stable_time = 0
  else:
      stable_time += dt
      if stable_time >= W:
          debounced = pending
        "#,
        implementation: r#"
<h4>LLM Prompt: GPIO Debounce</h4>
<pre>"Write Rust code for ESP32 GPIO debouncing using esp-hal.
Input: GPIO pin configured as input with pull-up.
Implement state machine: detect change → wait for stability → accept.
Include configurable debounce window (default 50ms). Handle both
polling and interrupt-driven modes."</pre>

<h4>Lab Exercise</h4>
<ol>
<li>Connect button: GPIO → Button → GND</li>
<li>Enable internal pull-up in code</li>
<li>Implement debounce state machine</li>
<li>Test with different debounce windows (20ms, 50ms, 100ms)</li>
<li>Observe: too short = false triggers, too long = laggy feel</li>
</ol>
        "#,
    },
    Lesson {
        id: 9,
        title: "Timers + PWM",
        subtitle: "LEDC: Duty, Frequency, Resolution",
        icon: "📶",
        phase: "Microcontrollers",
        demo_type: DemoType::Canvas,
        why_it_matters:
            "PWM is the workhorse for controlling <strong>brightness</strong>, <strong>motor speed</strong>, and <strong>power</strong> using only digital pins.",
        intuition: r#"
            <h3>The core idea</h3>
            PWM rapidly switches a pin HIGH/LOW. The load (LED, motor, filter capacitor) averages it.
            The <strong>duty cycle</strong> sets the average output.

            <div class="mermaid">
            flowchart LR
                Counter["Timer Counter"] --> Cmp{"Counter &lt; Duty?"}
                Cmp -->|yes| Hi["GPIO HIGH"]
                Cmp -->|no| Lo["GPIO LOW"]
            </div>

            <h3>ESP‑WROOM‑32 note (LEDC)</h3>
            ESP32 has the LEDC peripheral: multiple channels of hardware PWM.
            The key tradeoff is:
            <ul>
              <li>Higher frequency → fewer bits of duty resolution (bigger steps)</li>
              <li>More resolution → lower max frequency</li>
            </ul>

            <h3>Mini-lab</h3>
            Choose a frequency high enough to avoid flicker (LED) or audible noise (motor), then pick the highest duty resolution you can afford.
        "#,
        demo_explanation: r#"
            The top plot is the digital PWM output. The bottom plot shows a smoothed "average" output.
            <br><br>
            Change <strong>Duty</strong> to change average power. Change <strong>Resolution</strong> to see duty steps (quantization).
            Change <strong>Smoothing</strong> to simulate an LED (fast) vs motor (slow) response.
        "#,
        key_takeaways: &[
            "Duty controls average output power",
            "Frequency affects flicker/audible noise and switching losses",
            "Resolution controls the smallest duty step (quantization)",
            "ESP32 LEDC provides multiple PWM channels in hardware",
        ],
        going_deeper:
            "LEDs are not linear to human vision. Many projects apply gamma correction (e.g. duty ≈ brightness^2.2) \
             so dimming feels smooth. Motors often need a driver (H-bridge / MOSFET) and a flyback path; \
             do not drive motors directly from GPIO pins.",
        math_details: r#"
Definitions:
  duty = t_on / T
  frequency = 1 / T

Average output (ideal):
  V_avg ≈ duty * V_high

Duty resolution:
  steps = 2^bits
  duty_quantized = round(duty*(steps-1)) / (steps-1)
        "#,
        implementation: r#"
<h4>LLM Prompt: PWM Control</h4>
<pre>"Write Rust code for ESP32 PWM using esp-hal LEDC peripheral.
Configure: channel, frequency (500Hz), resolution (8 bits), duty cycle (50%).
Include function to update duty cycle dynamically. Target: ESP32-WROOM-32."</pre>

<h4>Lab Exercise</h4>
<ol>
<li>Connect LED to PWM-capable GPIO pin</li>
<li>Configure LEDC channel: 500Hz, 8-bit resolution</li>
<li>Set duty cycle to 50% — LED should be half brightness</li>
<li>Vary duty cycle from 0% to 100% — observe smooth dimming</li>
<li>Try different frequencies (100Hz, 1kHz, 10kHz) — observe flicker at low frequencies</li>
</ol>
        "#,
    },
    Lesson {
        id: 10,
        title: "ADC Reading",
        subtitle: "Quantization, Noise, Averaging",
        icon: "📊",
        phase: "Microcontrollers",
        demo_type: DemoType::Canvas,
        why_it_matters:
            "Sensors are analog. The ADC is how your ESP32 reads knobs, light, temperature, battery voltage, and more.",
        intuition: r#"
            <h3>The core idea</h3>
            An ADC converts a voltage into an integer code. More bits → smaller steps, but noise can still dominate.

            <div class="mermaid">
            flowchart LR
                V[AnalogVoltage] --> Sample[Sample]
                Sample --> Q[Quantize]
                Q --> Code[DigitalCode]
                Code --> Avg[Average]
            </div>

            <h3>ESP‑WROOM‑32 notes</h3>
            <ul>
              <li><strong>ADC1</strong> works alongside Wi‑Fi. <strong>ADC2</strong> is often blocked when Wi‑Fi is active.</li>
              <li>Input range depends on <strong>attenuation</strong> (0dB..11dB).</li>
              <li>ADC pins are input-only — great for sensing, not for outputs.</li>
            </ul>

            <h3>Mini-lab</h3>
            Add noise, then increase averaging. You should see a more stable reading — but with more lag.
        "#,
        demo_explanation: r#"
            Gray is the underlying analog signal. Orange is the quantized ADC result. Green is a moving-average filter.
            <br><br>
            Reduce bits to see bigger steps. Increase noise to see jitter. Increase averaging to smooth jitter.
            Change attenuation to change full-scale range (Vfs).
        "#,
        key_takeaways: &[
            "Quantization turns continuous voltage into discrete codes",
            "Noise can dominate; averaging reduces noise at the cost of responsiveness",
            "On ESP32: prefer ADC1 when using Wi‑Fi",
            "Attenuation changes measurable voltage range (Vfs)",
        ],
        going_deeper:
            "Real ESP32 ADC readings can be non-linear, especially at high attenuation. \
             For better results: use calibration (if available), limit input impedance, and average multiple samples. \
             For battery sensing, use a divider and keep max voltage below 3.3V (and within chosen attenuation range).",
        math_details: r#"
Let Vfs be full-scale voltage and N be bits:
  levels = 2^N - 1
  code = round( (V / Vfs) * levels )
  V_quantized = (code / levels) * Vfs

Quantization error (ideal) is about ±0.5 LSB.
        "#,
        implementation: r#"
<h4>LLM Prompt: ADC Reading</h4>
<pre>"Write Rust code for ESP32 ADC reading using esp-hal.
Configure: ADC1 channel, attenuation (11dB for 0-3.3V), sample averaging (8 samples).
Include function to read voltage (convert ADC code to volts).
Handle ADC2 limitation (blocked when Wi‑Fi active)."</pre>

<h4>Lab Exercise</h4>
<ol>
<li>Connect potentiometer: 3.3V → pot → GND, wiper to ADC pin</li>
<li>Configure ADC: 12-bit, 11dB attenuation</li>
<li>Read ADC value and convert to voltage</li>
<li>Add averaging (8 samples) — observe reduced noise</li>
<li>Rotate potentiometer — verify linear response</li>
</ol>
        "#,
    },
    Lesson {
        id: 11,
        title: "I²C Communication",
        subtitle: "Addressing, ACK/NAK, Clock Stretching",
        icon: "🔗",
        phase: "Microcontrollers",
        demo_type: DemoType::Canvas,
        why_it_matters:
            "I²C lets you connect many peripherals (IMUs, OLEDs, environmental sensors) using only two wires.",
        intuition: r#"
            <h3>The core idea</h3>
            I²C (Inter-Integrated Circuit) is a two-wire bus protocol that lets you connect many devices to a microcontroller:
            <ul>
              <li><strong>SDA</strong> (Serial Data): Carries the actual data bits</li>
              <li><strong>SCL</strong> (Serial Clock): Synchronizes data transfer</li>
            </ul>
            Both lines are <strong>open-drain</strong> and require pull-up resistors (typically 2.2kΩ–10kΩ) to 3.3V.

            <h3>Why Open-Drain?</h3>
            Open-drain means devices can only pull the line LOW, never HIGH. The pull-up resistor pulls it HIGH when no device is pulling LOW.
            This allows multiple devices to share the same bus safely:
            <ul>
              <li>Any device can pull the line LOW (wired-AND logic)</li>
              <li>If one device pulls LOW, the whole bus goes LOW</li>
              <li>All devices must release for the bus to go HIGH</li>
              <li>No conflicts — devices can't fight over the bus</li>
            </ul>

            <h3>Addressing: How Devices Are Selected</h3>
            Each I²C device has a unique 7-bit address (0x08 to 0x77). The master sends the address first:
            <ul>
              <li>7 address bits identify the device</li>
              <li>1 R/W bit: 0 = write (master→slave), 1 = read (slave→master)</li>
              <li>Combined into one 8-bit byte on the wire</li>
            </ul>
            Example: Address 0x3C (0b0111100) + Write (0) = byte 0x78 (0b01111000)

            <h3>Protocol Flow</h3>
            <div class="mermaid">
            sequenceDiagram
                participant Master
                participant Slave
                Master->>Slave: START (SDA↓ while SCL high)
                Master->>Slave: Address + R/W (8 bits, MSB first)
                Slave-->>Master: ACK (SDA LOW on 9th clock)
                Master->>Slave: Data Byte 1 (8 bits)
                Slave-->>Master: ACK
                Master->>Slave: Data Byte 2 (8 bits)
                Slave-->>Master: ACK
                Master->>Slave: STOP (SDA↑ while SCL high)
            </div>

            <h3>ACK vs NACK</h3>
            After every 8 data bits, there's a 9th clock cycle for acknowledgment:
            <ul>
              <li><strong>ACK</strong> (Acknowledge): Slave pulls SDA LOW → "I received the byte"</li>
              <li><strong>NACK</strong> (Not Acknowledge): Slave leaves SDA HIGH → "Error" or "Stop sending"</li>
            </ul>
            NACK can mean:
            <ul>
              <li>No device at that address (device disconnected)</li>
              <li>Device busy (can't accept more data)</li>
              <li>Read complete (master sends NACK to signal "last byte")</li>
            </ul>

            <h3>Clock Stretching</h3>
            Sometimes a slave needs more time to process data. It can hold SCL LOW (clock stretching):
            <ul>
              <li>Slave pulls SCL LOW after receiving a byte</li>
              <li>Master waits (SCL is shared, so master sees it LOW)</li>
              <li>Slave releases SCL when ready</li>
              <li>Master continues the transaction</li>
            </ul>
            This allows slow devices (like EEPROMs) to work on fast buses.

            <h3>Common I²C Devices</h3>
            Many sensors and displays use I²C:
            <ul>
              <li><strong>Sensors</strong>: SHT31 (temp/humidity), BMP280 (pressure), MPU6050 (IMU), TCS34725 (color)</li>
              <li><strong>Displays</strong>: SSD1306 OLED (0x3C), LCD backpacks</li>
              <li><strong>Storage</strong>: EEPROMs (AT24C32, etc.)</li>
              <li><strong>Real-time clocks</strong>: DS1307, DS3231</li>
            </ul>

            <h3>ESP‑WROOM‑32 Notes</h3>
            <ul>
              <li>Default pins: GPIO21 (SDA) and GPIO22 (SCL)</li>
              <li>ESP32 can route I²C to other pins (software I²C)</li>
              <li>Hardware I²C is faster and more reliable than software bit-banging</li>
              <li>Keep everything at 3.3V logic unless you add proper level shifting</li>
              <li>Internal pull-ups exist but are weak (~45kΩ); external 2.2kΩ–10kΩ recommended</li>
            </ul>

            <h3>Speed Limits</h3>
            I²C has several speed modes:
            <ul>
              <li><strong>Standard mode</strong>: 100 kHz (most common)</li>
              <li><strong>Fast mode</strong>: 400 kHz</li>
              <li><strong>Fast mode plus</strong>: 1 MHz</li>
            </ul>
            Bus capacitance limits speed — longer wires = slower max speed. Keep wires short and use proper pull-ups.

            <h3>Multi-Master</h3>
            I²C supports multiple masters on the same bus:
            <ul>
              <li>Masters use arbitration: if two masters start simultaneously, the one sending LOW wins</li>
              <li>The losing master backs off and retries later</li>
              <li>Rare in embedded systems (usually one master, many slaves)</li>
            </ul>

            <h3>Mini-lab</h3>
            Increase NAK chance and see how a transaction aborts. Add clock stretching and watch SCL LOW extend.
            Try different addresses and observe how only the addressed device responds.
        "#,
        demo_explanation: r#"
            Top line is SCL, bottom is SDA. Watch for:
            <ul>
              <li><strong>START</strong>: SDA falls while SCL is high</li>
              <li><strong>STOP</strong>: SDA rises while SCL is high</li>
              <li><strong>ACK</strong>/<strong>NACK</strong>: the 9th clock after each byte</li>
            </ul>
        "#,
        key_takeaways: &[
            "I²C uses two wires (SDA, SCL) with open-drain outputs requiring pull-up resistors",
            "7-bit addresses (0x08–0x77) + 1 R/W bit = 8 bits on wire",
            "START: SDA falls while SCL high; STOP: SDA rises while SCL high",
            "ACK (SDA LOW) = success; NACK (SDA HIGH) = error or end of read",
            "Clock stretching lets slow slaves pause the master by holding SCL LOW",
            "Multiple devices share the bus; addressing selects which responds",
            "Standard speed: 100 kHz; Fast: 400 kHz; bus capacitance limits max speed",
        ],
        going_deeper: r#"
            <h4>Troubleshooting I²C Issues</h4>
            <ul>
              <li><strong>No response (NACK)</strong>: Check address, power, wiring, pull-ups</li>
              <li><strong>Garbled data</strong>: Reduce bus speed, check for loose connections</li>
              <li><strong>Bus stuck LOW</strong>: A device may be holding SDA/SCL LOW; power cycle</li>
              <li><strong>Address conflicts</strong>: Some devices have address-select pins (A0, A1)</li>
            </ul>

            <h4>Address Scanning</h4>
            Scan addresses 0x08–0x77: send START + address + R/W, check for ACK.
            This identifies all devices on the bus — essential for debugging.

            <h4>Pull-up Resistor Selection</h4>
            <ul>
              <li>Too weak (high resistance): Bus rises slowly, limits speed</li>
              <li>Too strong (low resistance): High current when pulled LOW, wastes power</li>
              <li>Sweet spot: 2.2kΩ–10kΩ for 3.3V (depends on bus capacitance)</li>
              <li>Long wires need stronger pull-ups (lower resistance)</li>
            </ul>

            <h4>Reading vs Writing</h4>
            <ul>
              <li><strong>Write</strong>: Master sends data bytes, slave ACKs each</li>
              <li><strong>Read</strong>: Master sends address+R, slave sends data, master ACKs (or NACKs last byte)</li>
              <li>Some devices need register writes before reads (e.g., "read temperature register")</li>
            </ul>
        "#,
        math_details: r#"
I²C frame (write, 7-bit address):
  START
  [A6 A5 A4 A3 A2 A1 A0 W]
  ACK
  [D7 D6 D5 D4 D3 D2 D1 D0]
  ACK
  STOP
        "#,
        implementation: r#"
<h4>LLM Prompt: I²C Sensor Reading</h4>
<pre>"Write Rust code for ESP32 I²C communication using esp-hal.
Connect to SHT31 sensor (address 0x44). Implement: write register,
read register, read temperature/humidity. Include error handling
for NACK and timeout. Use GPIO21 (SDA) and GPIO22 (SCL)."</pre>

<h4>Lab Exercise</h4>
<ol>
<li>Connect SHT31: VCC→3.3V, GND→GND, SDA→GPIO21, SCL→GPIO22</li>
<li>Add 10kΩ pull-up resistors on SDA and SCL to 3.3V</li>
<li>Implement I²C address scan — verify sensor at 0x44</li>
<li>Read temperature register — verify reasonable values</li>
<li>Add error handling for disconnected sensor (NACK)</li>
</ol>
        "#,
    },
    Lesson {
        id: 12,
        title: "UART Logging + Debugging",
        subtitle: "Serial Communication Workflow",
        icon: "📡",
        phase: "Microcontrollers",
        demo_type: DemoType::Static,
        why_it_matters:
            "UART (serial) is your <strong>lifeline</strong> for debugging. Print statements, logging, and interactive \
             commands all use UART. Without it, you're debugging blind.",
        intuition: r#"
            <h3>What Is UART?</h3>
            UART (Universal Asynchronous Receiver-Transmitter) is a simple serial protocol:
            <ul>
              <li><strong>TX</strong> (transmit): ESP32 sends data</li>
              <li><strong>RX</strong> (receive): ESP32 receives data</li>
              <li><strong>Baud rate</strong>: Speed (e.g., 115200 bits/second)</li>
              <li><strong>No clock</strong>: Asynchronous (start/stop bits)</li>
            </ul>

            <h3>Why It Matters</h3>
            UART is used for:
            <ul>
              <li><strong>Debugging</strong> — print statements, logs</li>
              <li><strong>Configuration</strong> — interactive commands</li>
              <li><strong>Data logging</strong> — stream sensor data</li>
              <li><strong>Boot messages</strong> — ESP32 prints boot info</li>
            </ul>

            <h3>Common Baud Rates</h3>
            <ul>
              <li><strong>9600</strong>: Slow but reliable</li>
              <li><strong>115200</strong>: Standard for ESP32 (fast enough, reliable)</li>
              <li><strong>921600</strong>: Maximum (may have errors on long cables)</li>
            </ul>

            <h3>Debugging Workflow</h3>
            <ol>
              <li>Connect USB-to-serial adapter (or use ESP32's built-in USB)</li>
              <li>Open serial monitor (115200 baud, 8N1)</li>
              <li>Add print statements at key points</li>
              <li>Watch output in real-time</li>
            </ol>
        "#,
        demo_explanation: r#"
            UART is a communication protocol, not a visual demo. Review this before debugging your code.
        "#,
        key_takeaways: &[
            "UART is serial communication: TX (send), RX (receive)",
            "Standard baud rate for ESP32: 115200",
            "UART is essential for debugging — print statements, logs",
            "Use serial monitor to view ESP32 output in real-time",
        ],
        going_deeper:
            "ESP32 has multiple UART peripherals (UART0, UART1, UART2). UART0 is usually used for boot messages \
             and debugging. For production, disable debug prints or use a logging framework that can be disabled \
             at compile time. For high-speed data logging, consider using a faster baud rate or switching to SPI.",
        math_details: r#"
UART frame (8N1):
  Start bit (0) + 8 data bits + Stop bit (1) = 10 bits per byte

At 115200 baud:
  Time per bit = 1 / 115200 = 8.68µs
  Time per byte = 10 × 8.68µs = 86.8µs
  Max throughput = 11520 bytes/second

Print overhead:
  "Hello World\\n" = 12 bytes = 1.04ms at 115200 baud
  Too many prints can slow down your code!
        "#,
        implementation: r#"
<h4>LLM Prompt: UART Logging</h4>
<pre>"Write Rust code for ESP32 UART logging using esp-hal.
Configure: UART0, 115200 baud, 8N1. Implement logging macro that
can be disabled at compile time. Include functions: log_info,
log_error, log_debug. Format: [LEVEL] message\\n"</pre>

<h4>Lab Exercise</h4>
<ol>
<li>Connect USB-to-serial adapter (or use ESP32's USB port)</li>
<li>Open serial monitor: 115200 baud, 8N1, no flow control</li>
<li>Add print statement in main loop — verify output</li>
<li>Add logging at different levels (info, error, debug)</li>
<li>Test with sensor reading — log values every second</li>
</ol>
        "#,
    },
    Lesson {
        id: 13,
        title: "5V vs 3.3V Logic + Level Shifting",
        subtitle: "ESP32 Rules of Engagement",
        icon: "⚡",
        phase: "Microcontrollers",
        demo_type: DemoType::Static,
        why_it_matters:
            "ESP32 GPIO pins are <strong>3.3V logic</strong>. Connecting 5V signals can <strong>damage</strong> the chip. \
             Level shifters convert between voltage levels safely.",
        intuition: r#"
            <h3>The Problem</h3>
            ESP32 GPIO pins:
            <ul>
              <li><strong>Maximum input voltage</strong>: 3.6V (absolute max)</li>
              <li><strong>Logic HIGH</strong>: ~2.4V minimum</li>
              <li><strong>Logic LOW</strong>: ~0.8V maximum</li>
            </ul>
            Connecting 5V signals can <strong>destroy</strong> the ESP32!

            <h3>The Solution: Level Shifters</h3>
            Level shifters convert between voltage levels:
            <ul>
              <li><strong>Bidirectional</strong>: Works for both input and output</li>
              <li><strong>Unidirectional</strong>: One direction only (cheaper)</li>
              <li><strong>Voltage divider</strong>: Simple but only works one way (input only)</li>
            </ul>

            <h3>Common Scenarios</h3>
            <ul>
              <li><strong>5V sensor → ESP32</strong>: Use level shifter or voltage divider</li>
              <li><strong>ESP32 → 5V device</strong>: Use level shifter (ESP32 output is too low)</li>
              <li><strong>3.3V sensor → ESP32</strong>: Direct connection (no shifter needed)</li>
            </ul>

            <h3>Voltage Divider for Input</h3>
            Simple but only works one way:
            <ul>
              <li>5V → 10kΩ → ESP32 input</li>
              <li>ESP32 input → 20kΩ → GND</li>
              <li>Output: 5V × (20k / (10k + 20k)) = 3.33V (safe!)</li>
            </ul>
        "#,
        demo_explanation: r#"
            Level shifting is about safety, not demos. Review this before connecting any 5V devices.
        "#,
        key_takeaways: &[
            "ESP32 GPIO is 3.3V logic — never connect 5V directly",
            "Level shifters convert between voltage levels safely",
            "Voltage dividers work for 5V → 3.3V input (one way only)",
            "Always check sensor/logic voltage before connecting",
        ],
        going_deeper:
            "For bidirectional communication (like I²C), use a dedicated level shifter IC (e.g., TXB0104, PCA9306). \
             For one-way signals, a simple voltage divider or resistor + zener diode works. For production, \
             prefer dedicated level shifter ICs — they're more reliable and handle edge cases better.",
        math_details: r#"
Voltage divider (5V → 3.3V):
  R1 = 10kΩ (top resistor)
  R2 = 20kΩ (bottom resistor)
  V_out = V_in × (R2 / (R1 + R2))
  V_out = 5V × (20k / 30k) = 3.33V

Current through divider:
  I = V_in / (R1 + R2) = 5V / 30kΩ = 167µA (negligible)

Power dissipation:
  P_R1 = I² × R1 = (0.000167A)² × 10kΩ = 0.28mW
  P_R2 = I² × R2 = (0.000167A)² × 20kΩ = 0.56mW
        "#,
        implementation: r#"
<h4>LLM Prompt: Level Shifter Interface</h4>
<pre>"Write Rust code for ESP32 to interface with 5V device via level shifter.
Include: voltage divider calculation helper, safety check to warn if
input voltage > 3.6V, and documentation on when to use level shifter
vs voltage divider."</pre>

<h4>Lab Exercise</h4>
<ol>
<li>Identify sensor/logic voltage (check datasheet)</li>
<li>If 5V: design voltage divider or use level shifter</li>
<li>Build circuit: 5V device → level shifter → ESP32</li>
<li>Measure voltage at ESP32 input (should be < 3.6V)</li>
<li>Test communication — verify data integrity</li>
</ol>
        "#,
    },

    // ═══════════════════════════════════════════════════════════════════════════
    // PHASE 4: ESP32 Deep Dive
    // ═══════════════════════════════════════════════════════════════════════════
    Lesson {
        id: 14,
        title: "ESP32 Pins: Strapping Pins, Input-Only, Boot Traps",
        subtitle: "Pin Configuration Gotchas",
        icon: "📌",
        phase: "ESP32 Deep Dive",
        demo_type: DemoType::Static,
        why_it_matters:
            "ESP32 pins have <strong>special functions</strong>. Using the wrong pin can prevent booting, cause \
             unreliable behavior, or damage the chip.",
        intuition: r#"
            <h3>Strapping Pins</h3>
            Some pins are sampled at boot to configure the ESP32:
            <ul>
              <li><strong>GPIO0</strong>: Boot mode (LOW = download mode, HIGH = normal boot)</li>
              <li><strong>GPIO2</strong>: Boot configuration</li>
              <li><strong>GPIO12</strong>: Flash voltage (LOW = 3.3V, HIGH = 1.8V)</li>
              <li><strong>GPIO15</strong>: Boot configuration</li>
            </ul>
            <strong>Don't use these for buttons or outputs</strong> unless you understand the implications!

            <h3>Input-Only Pins</h3>
            GPIO34–39 are <strong>input-only</strong>:
            <ul>
              <li>Cannot be configured as outputs</li>
              <li>No internal pull-ups/pull-downs</li>
              <li>Must use <strong>external</strong> pull-up/down resistors</li>
              <li>Good for: ADC inputs, sensor inputs</li>
            </ul>

            <h3>Safe GPIO Pins</h3>
            These pins are generally safe for general-purpose use:
            <ul>
              <li><strong>GPIO4, 5, 16, 17, 18, 19</strong>: Safe for I/O</li>
              <li><strong>GPIO21, 22</strong>: Default I²C (but can be remapped)</li>
              <li><strong>GPIO25, 26</strong>: Safe for I/O</li>
            </ul>

            <h3>Boot Traps</h3>
            Common mistakes:
            <ul>
              <li>Button on GPIO0 → ESP32 won't boot normally</li>
              <li>Output on input-only pin → doesn't work, no error</li>
              <li>Pull-up on GPIO12 → wrong flash voltage → boot failure</li>
            </ul>
        "#,
        demo_explanation: r#"
            Pin configuration is critical but not visual. Review this before designing your circuit.
        "#,
        key_takeaways: &[
            "Strapping pins (GPIO0, 2, 12, 15) affect boot — use carefully",
            "GPIO34–39 are input-only — no outputs, no internal pull-ups",
            "Always check pin functions before using them",
            "Use safe GPIO pins (4, 5, 16–19, 21, 22, 25, 26) for general I/O",
        ],
        going_deeper:
            "ESP32 pin functions are documented in the datasheet. For production designs, create a pin assignment \
             table that documents each pin's function and any constraints. Use GPIO0 for boot mode selection only \
             if you need to enter download mode frequently. For most projects, avoid strapping pins entirely.",
        math_details: r#"
Pin current limits:
  GPIO source: ~40mA max
  GPIO sink: ~28mA max
  Total chip current: ~600mA (check datasheet for your variant)

Pin voltage levels:
  Input HIGH: > 2.4V (0.7 × VDD)
  Input LOW: < 0.8V (0.3 × VDD)
  Output HIGH: ~3.0V (VDD - 0.3V)
  Output LOW: < 0.1V
        "#,
        implementation: r#"
<h4>LLM Prompt: Pin Configuration Validator</h4>
<pre>"Write Rust code to validate ESP32 pin configuration.
Check: pin is not strapping pin, pin is not input-only if used as output,
pin is not reserved for special function. Return Result with error messages.
Include helper function to get pin capabilities (input, output, ADC, etc.)."</pre>

<h4>Lab Exercise</h4>
<ol>
<li>Create pin assignment table for your project</li>
<li>Verify no strapping pins used for I/O</li>
<li>Verify input-only pins have external pull-ups</li>
<li>Test boot with all pins connected — verify no boot issues</li>
<li>Document pin functions in code comments</li>
</ol>
        "#,
    },
    Lesson {
        id: 15,
        title: "Deep Sleep: Wake Sources, State, RTC Memory",
        subtitle: "Power Optimization",
        icon: "😴",
        phase: "ESP32 Deep Dive",
        demo_type: DemoType::Static,
        why_it_matters:
            "Deep sleep reduces power consumption to <strong>microamps</strong>. This is essential for battery-powered \
             devices that need to run for months.",
        intuition: r#"
            <h3>What Is Deep Sleep?</h3>
            In deep sleep:
            <ul>
              <li><strong>CPU stops</strong> — no code execution</li>
              <li><strong>RAM is lost</strong> — unless using RTC memory</li>
              <li><strong>Only RTC peripherals run</strong> — RTC timer, RTC GPIO</li>
              <li><strong>Power consumption</strong>: ~10µA (vs ~80mA active)</li>
            </ul>

            <h3>Wake Sources</h3>
            ESP32 can wake from:
            <ul>
              <li><strong>Timer</strong> — RTC timer (most common)</li>
              <li><strong>GPIO</strong> — External interrupt (button, sensor)</li>
              <li><strong>Touch</strong> — Touch pad interrupt</li>
              <li><strong>ULP</strong> — Ultra Low Power coprocessor</li>
            </ul>

            <h3>RTC Memory</h3>
            RTC memory persists through deep sleep:
            <ul>
              <li><strong>RTC_SLOW_MEM</strong> — ~8KB, slow access</li>
              <li><strong>RTC_FAST_MEM</strong> — ~8KB, fast access</li>
              <li>Use for: state, sensor data, configuration</li>
              <li>Regular RAM is <strong>lost</strong> on wake</li>
            </ul>

            <h3>Typical Workflow</h3>
            <ol>
              <li>Save state to RTC memory</li>
              <li>Configure wake source (timer, GPIO)</li>
              <li>Enter deep sleep</li>
              <li>Wake up → restore state from RTC memory</li>
              <li>Do work → repeat</li>
            </ol>
        "#,
        demo_explanation: r#"
            Deep sleep is a power mode, not a visual demo. Review this before implementing power optimization.
        "#,
        key_takeaways: &[
            "Deep sleep reduces power to ~10µA (vs ~80mA active)",
            "CPU stops, RAM is lost (unless using RTC memory)",
            "Wake sources: timer, GPIO, touch, ULP",
            "Use RTC memory to persist state through deep sleep",
        ],
        going_deeper:
            "ESP32 has multiple sleep modes: light sleep (keeps RAM, faster wake), deep sleep (loses RAM, \
             slowest wake), hibernation (lowest power, slowest wake). For periodic sensor readings, \
             deep sleep with timer wake is ideal. For event-driven applications, GPIO wake is better.",
        math_details: r#"
Power budget example:
  Active mode: 80mA × 2s = 160mAs per reading
  Deep sleep: 10µA × 298s = 2.98mAs per cycle
  Total per 5min cycle: ~163mAs

For 2000mAh battery:
  Cycles = (2000mAh × 3600s/h) / 163mAs ≈ 44,000 cycles
  Lifetime ≈ 44,000 × 5min ≈ 153 days

Wake time:
  Deep sleep → active: ~200ms (RTC timer wake)
  Light sleep → active: ~5ms (faster but uses more power)
        "#,
        implementation: r#"
<h4>LLM Prompt: Deep Sleep Implementation</h4>
<pre>"Write Rust code for ESP32 deep sleep with timer wake using esp-hal.
Include: save state to RTC memory, configure wake timer (5 minutes),
enter deep sleep, restore state on wake. Handle RTC memory allocation
and error cases."</pre>

<h4>Lab Exercise</h4>
<ol>
<li>Implement deep sleep with 5-minute timer wake</li>
<li>Save sensor reading to RTC memory before sleep</li>
<li>Measure current consumption: active vs deep sleep</li>
<li>Verify state persists through sleep (read RTC memory on wake)</li>
<li>Calculate battery lifetime based on power budget</li>
</ol>
        "#,
    },

    // ═══════════════════════════════════════════════════════════════════════════
    // PHASE 5: Capstone - Battery Environmental Monitor
    // ═══════════════════════════════════════════════════════════════════════════
    Lesson {
        id: 16,
        title: "Sensor Selection + I²C Wiring",
        subtitle: "SHT31/BME280 Integration",
        icon: "🌡️",
        phase: "Capstone",
        demo_type: DemoType::Static,
        why_it_matters:
            "The capstone uses an <strong>I²C environmental sensor</strong> (temperature, humidity, pressure). \
             Proper wiring and address scanning are critical.",
        intuition: r#"
            <h3>Sensor Options</h3>
            Common I²C environmental sensors:
            <ul>
              <li><strong>SHT31</strong>: Temperature + humidity, 0x44 address</li>
              <li><strong>BME280</strong>: Temperature + humidity + pressure, 0x76 or 0x77</li>
              <li><strong>SHT40</strong>: Temperature + humidity, improved accuracy</li>
            </ul>

            <h3>I²C Wiring</h3>
            Standard I²C connections:
            <ul>
              <li><strong>VCC</strong> → 3.3V (not 5V!)</li>
              <li><strong>GND</strong> → GND</li>
              <li><strong>SDA</strong> → GPIO21 (or configured pin)</li>
              <li><strong>SCL</strong> → GPIO22 (or configured pin)</li>
              <li><strong>Pull-ups</strong>: 10kΩ resistors on SDA and SCL to 3.3V</li>
            </ul>

            <h3>Address Scanning</h3>
            First step: verify sensor is connected:
            <ol>
              <li>Scan I²C bus for all addresses (0x08–0x77)</li>
              <li>Look for ACK responses</li>
              <li>Verify expected address matches datasheet</li>
            </ol>

            <h3>Common Issues</h3>
            <ul>
              <li><strong>No device found</strong>: Check wiring, pull-ups, power</li>
              <li><strong>Wrong address</strong>: Some sensors have address-select pins</li>
              <li><strong>NACK errors</strong>: Sensor not ready, check timing</li>
            </ul>
        "#,
        demo_explanation: r#"
            Sensor wiring is physical, not visual. Review this before connecting your sensor.
        "#,
        key_takeaways: &[
            "Use 3.3V sensors (not 5V) for direct ESP32 connection",
            "I²C requires pull-up resistors (10kΩ) on SDA and SCL",
            "Always scan I²C bus to verify sensor address",
            "Check datasheet for address-select pins if address doesn't match",
        ],
        going_deeper:
            "SHT31 has excellent accuracy (±2% RH, ±0.3°C) and low power consumption (~2.7mA active, \
             <1µA sleep). BME280 adds pressure sensing but uses more power. For battery-powered devices, \
             SHT31 is often the better choice. Always check sensor power consumption in datasheet.",
        math_details: r#"
I²C pull-up calculation:
  V_CC = 3.3V
  I_max = 3mA (I²C spec)
  R_min = V_CC / I_max = 3.3V / 0.003A = 1100Ω

  Use 10kΩ (standard value, provides ~330µA current)

Capacitance limit:
  C_bus_max = 400pF (I²C spec)
  With 10kΩ pull-ups: works up to ~1m cable length

Sensor power:
  SHT31: 2.7mA active, <1µA sleep
  BME280: 3.6µA sleep, 338µA active
        "#,
        implementation: r#"
<h4>LLM Prompt: I²C Address Scanner</h4>
<pre>"Write Rust code for ESP32 I²C address scanner using esp-hal.
Scan addresses 0x08–0x77, attempt read/write, report which addresses
respond with ACK. Include timeout handling and error reporting.
Useful for debugging I²C connections."</pre>

<h4>Lab Exercise</h4>
<ol>
<li>Connect SHT31: VCC→3.3V, GND→GND, SDA→GPIO21, SCL→GPIO22</li>
<li>Add 10kΩ pull-ups on SDA and SCL to 3.3V</li>
<li>Run I²C address scanner — verify sensor at 0x44</li>
<li>Read temperature register — verify reasonable values</li>
<li>Read humidity register — verify 0–100% range</li>
</ol>
        "#,
    },
    Lesson {
        id: 17,
        title: "Firmware Architecture",
        subtitle: "Task Loop, State Machine, Error Budget",
        icon: "🏗️",
        phase: "Capstone",
        demo_type: DemoType::Static,
        why_it_matters:
            "Good firmware architecture makes code <strong>maintainable</strong>, <strong>testable</strong>, and \
             <strong>reliable</strong>. A clear state machine prevents bugs.",
        intuition: r#"
            <h3>State Machine</h3>
            The capstone has clear states:
            <ul>
              <li><strong>INIT</strong>: Setup GPIO, I²C, Wi‑Fi</li>
              <li><strong>READ_SENSOR</strong>: Read temperature/humidity</li>
              <li><strong>CONNECT_WIFI</strong>: Establish Wi‑Fi connection</li>
              <li><strong>TRANSMIT</strong>: Send data to server</li>
              <li><strong>SLEEP</strong>: Enter deep sleep, wait for wake</li>
            </ul>

            <h3>Error Handling</h3>
            Every operation can fail:
            <ul>
              <li><strong>Sensor read fails</strong> → retry (with limit), then sleep</li>
              <li><strong>Wi‑Fi connect fails</strong> → retry (with limit), then sleep</li>
              <li><strong>Transmit fails</strong> → retry (with limit), then sleep</li>
              <li><strong>Error budget</strong>: Max retries before giving up</li>
            </ul>

            <h3>Task Loop</h3>
            Main loop structure:
            <ol>
              <li>Restore state from RTC memory</li>
              <li>Read sensor (with retries)</li>
              <li>Connect Wi‑Fi (with timeout)</li>
              <li>Transmit data (with retries)</li>
              <li>Save state to RTC memory</li>
              <li>Enter deep sleep</li>
            </ol>

            <h3>Power Optimization</h3>
            Minimize active time:
            <ul>
              <li>Turn off Wi‑Fi when not needed</li>
              <li>Use fast I²C speed (400kHz)</li>
              <li>Batch operations (read all sensor data at once)</li>
              <li>Enter sleep immediately after transmit</li>
            </ul>
        "#,
        demo_explanation: r#"
            Firmware architecture is code structure, not visual. Review this before writing the capstone code.
        "#,
        key_takeaways: &[
            "Use a state machine to organize firmware logic",
            "Every operation can fail — implement error handling",
            "Error budget: max retries before giving up",
            "Minimize active time to save power",
        ],
        going_deeper:
            "For production firmware, use an RTOS (Real-Time Operating System) like FreeRTOS for task scheduling. \
             For simple projects, a state machine in the main loop is sufficient. Always implement watchdog timers \
             to recover from hangs. Use structured logging (not just print statements) for debugging.",
        math_details: r#"
Timing budget (5-minute cycle):
  Sensor read: ~100ms
  Wi‑Fi connect: ~2s (first time), ~500ms (reconnect)
  Transmit: ~200ms
  Sleep entry: ~50ms
  Total active: ~3s (worst case)

Error budget:
  Max retries per operation: 3
  Total max active time: ~10s (if all retries fail)
  Still acceptable for 5-minute cycle

Power impact:
  Normal cycle: 80mA × 3s = 240mAs
  Error cycle: 80mA × 10s = 800mAs
  Impact: ~3× power consumption (acceptable for rare errors)
        "#,
        implementation: r#"
<h4>LLM Prompt: Firmware State Machine</h4>
<pre>"Write Rust code for ESP32 firmware state machine.
States: Init, ReadSensor, ConnectWifi, Transmit, Sleep.
Include: error handling with retry limits, RTC memory for state,
watchdog timer, structured logging. Use esp-hal and esp-wifi crates."</pre>

<h4>Lab Exercise</h4>
<ol>
<li>Design state machine diagram (states, transitions, errors)</li>
<li>Implement state enum and transition logic</li>
<li>Add error handling with retry limits</li>
<li>Test each state transition — verify error recovery</li>
<li>Measure timing for each operation — verify power budget</li>
</ol>
        "#,
    },
    Lesson {
        id: 18,
        title: "Wi‑Fi Transmit Strategy",
        subtitle: "Batching, Retries, Timeouts, Security",
        icon: "📶",
        phase: "Capstone",
        demo_type: DemoType::Static,
        why_it_matters:
            "Wi‑Fi is <strong>power-hungry</strong> and <strong>unreliable</strong>. Efficient transmission strategy \
             is critical for battery life.",
        intuition: r#"
            <h3>Power Consumption</h3>
            Wi‑Fi is expensive:
            <ul>
              <li><strong>Idle</strong>: ~20mA</li>
              <li><strong>Connecting</strong>: ~80mA</li>
              <strong>Transmitting</strong>: ~170mA</li>
              <li><strong>Turn off when not needed</strong>!</li>
            </ul>

            <h3>Connection Strategy</h3>
            <ul>
              <li><strong>Connect once</strong> per cycle (not per reading)</li>
              <li><strong>Reuse connection</strong> if still valid</li>
              <li><strong>Timeout</strong>: Give up after 10 seconds</li>
              <li><strong>Turn off Wi‑Fi</strong> immediately after transmit</li>
            </ul>

            <h3>Transmission Protocol</h3>
            Options:
            <ul>
              <li><strong>HTTP POST</strong>: Simple, works with any server</li>
              <li><strong>MQTT</strong>: Efficient, designed for IoT</li>
              <li><strong>HTTPS</strong>: Secure but more power (TLS overhead)</li>
            </ul>

            <h3>Batching</h3>
            Send multiple readings at once:
            <ul>
              <li>Store readings in RTC memory</li>
              <li>Transmit batch when Wi‑Fi connects</li>
              <li>Reduces connection overhead</li>
            </ul>

            <h3>Retry Strategy</h3>
            <ul>
              <li><strong>Exponential backoff</strong>: Wait longer between retries</li>
              <li><strong>Max retries</strong>: 3 attempts, then give up</li>
              <li><strong>Error handling</strong>: Log failures, continue to sleep</li>
            </ul>
        "#,
        demo_explanation: r#"
            Wi‑Fi transmission is network communication, not visual. Review this before implementing data transmission.
        "#,
        key_takeaways: &[
            "Wi‑Fi is power-hungry — turn off when not needed",
            "Connect once per cycle, reuse connection if valid",
            "Use batching to reduce connection overhead",
            "Implement retry strategy with exponential backoff",
        ],
        going_deeper:
            "For production, use MQTT with QoS level 1 (at least once delivery). For simple projects, \
             HTTP POST to a webhook is sufficient. Always use HTTPS in production (but accept the power cost). \
             Consider using a message queue (like AWS IoT Core) for reliability.",
        math_details: r#"
Power consumption:
  Wi‑Fi idle: 20mA × 10s = 200mAs (wasteful!)
  Wi‑Fi connect: 80mA × 2s = 160mAs
  Wi‑Fi transmit: 170mA × 0.2s = 34mAs
  Total: ~194mAs (if Wi‑Fi turned off immediately)

Connection overhead:
  TCP handshake: ~100ms
  TLS handshake: ~500ms (HTTPS)
  HTTP request: ~50ms
  Total: ~650ms (HTTP) or ~1150ms (HTTPS)

Batching benefit:
  Single reading: 650ms overhead
  10 readings: 650ms + (10 × 50ms) = 1150ms
  Overhead per reading: 115ms (vs 650ms single)
        "#,
        implementation: r#"
<h4>LLM Prompt: Wi‑Fi Transmit with Retry</h4>
<pre>"Write Rust code for ESP32 Wi‑Fi HTTP POST with retry logic.
Include: connect Wi‑Fi (with timeout), HTTP POST request (with retry),
exponential backoff, turn off Wi‑Fi after transmit. Use esp-wifi crate.
Handle errors gracefully — log and continue to sleep."</pre>

<h4>Lab Exercise</h4>
<ol>
<li>Set up simple HTTP server (or use webhook service)</li>
<li>Implement Wi‑Fi connect with timeout (10s)</li>
<li>Implement HTTP POST with retry (3 attempts)</li>
<li>Measure power consumption: connect vs transmit</li>
<li>Test with poor signal — verify retry logic works</li>
</ol>
        "#,
    },
    Lesson {
        id: 19,
        title: "Power Budget + Measurement",
        subtitle: "Calculating Battery Lifetime",
        icon: "🔋",
        phase: "Capstone",
        demo_type: DemoType::Canvas,
        why_it_matters:
            "Power budget tells you if your design will work. Without it, you're guessing how long the battery will last.",
        intuition: r#"
            <h3>What Is a Power Budget?</h3>
            A power budget calculates total energy consumption:
            <ul>
              <li><strong>Active mode</strong>: Current × time</li>
              <li><strong>Sleep mode</strong>: Current × time</li>
              <li><strong>Total</strong>: Sum of all modes</li>
              <li><strong>Battery capacity</strong>: Total energy available</li>
            </ul>

            <h3>Typical Values</h3>
            ESP32 power consumption:
            <ul>
              <li><strong>Active (Wi‑Fi off)</strong>: ~40mA</li>
              <li><strong>Active (Wi‑Fi on)</strong>: ~80mA</li>
              <li><strong>Deep sleep</strong>: ~10µA</li>
              <li><strong>Wi‑Fi transmit</strong>: ~170mA</li>
            </ul>

            <h3>Calculation</h3>
            For 5-minute cycle:
            <ul>
              <li>Active: 80mA × 3s = 240mAs</li>
              <li>Sleep: 10µA × 297s = 2.97mAs</li>
              <li>Total: ~243mAs per cycle</li>
            </ul>

            <h3>Battery Lifetime</h3>
            For 2000mAh battery:
            <ul>
              <li>Capacity: 2000mAh × 3600s/h = 7,200,000mAs</li>
              <li>Cycles: 7,200,000 / 243 ≈ 29,600 cycles</li>
              <li>Lifetime: 29,600 × 5min ≈ 103 days</li>
            </ul>

            <h3>Measurement</h3>
            Use multimeter in current mode:
            <ul>
              <li>Break circuit (series measurement)</li>
              <li>Measure active current</li>
              <li>Measure sleep current (may need µA range)</li>
              <li>Verify calculations</li>
            </ul>
        "#,
        demo_explanation: r#"
            Adjust <strong>Active Current</strong>, <strong>Active Time</strong>, <strong>Sleep Current</strong>, and \
            <strong>Sleep Time</strong> to see how battery lifetime changes.
            <br><br>
            Watch for:
            <ul>
              <li><strong>Total energy per cycle</strong> — lower is better</li>
              <li><strong>Battery lifetime</strong> — days/months/years</li>
              <li><strong>Impact of active time</strong> — reducing active time dramatically improves lifetime</li>
            </ul>
        "#,
        key_takeaways: &[
            "Power budget = sum of (current × time) for all modes",
            "Deep sleep is essential — 10µA vs 80mA active",
            "Minimize active time to maximize battery lifetime",
            "Measure actual current consumption to verify calculations",
        ],
        going_deeper:
            "Real-world power consumption varies with temperature, battery age, and component tolerances. \
             Always add a 20–30% safety margin. For production, use a power profiler (like Nordic Power Profiler) \
             to measure actual consumption. Consider battery self-discharge (~5% per month for LiPo).",
        math_details: r#"
Power budget formula:
  E_cycle = (I_active × t_active) + (I_sleep × t_sleep)

  Where:
  E_cycle = energy per cycle (mAs)
  I_active = active current (mA)
  t_active = active time (s)
  I_sleep = sleep current (µA, convert to mA)
  t_sleep = sleep time (s)

Battery lifetime:
  Cycles = Battery_capacity (mAs) / E_cycle
  Lifetime = Cycles × Cycle_time

Example:
  I_active = 80mA, t_active = 3s → 240mAs
  I_sleep = 10µA = 0.01mA, t_sleep = 297s → 2.97mAs
  E_cycle = 243mAs

  Battery: 2000mAh = 7,200,000mAs
  Cycles = 7,200,000 / 243 = 29,600
  Lifetime = 29,600 × 5min = 148,000min = 103 days
        "#,
        implementation: r#"
<h4>LLM Prompt: Power Budget Calculator</h4>
<pre>"Write Rust function to calculate power budget and battery lifetime.
Input: active current, active time, sleep current, sleep time, battery capacity.
Output: energy per cycle, cycles, lifetime (days). Include validation:
warn if lifetime < 30 days (may need optimization)."</pre>

<h4>Lab Exercise</h4>
<ol>
<li>Measure active current with multimeter (series measurement)</li>
<li>Measure sleep current (may need µA range)</li>
<li>Time each operation (sensor read, Wi‑Fi connect, transmit)</li>
<li>Calculate power budget — verify < 300mAs per cycle</li>
<li>Calculate battery lifetime — verify > 90 days</li>
</ol>
        "#,
    },
    Lesson {
        id: 20,
        title: "Validation + Enclosure + Ship It",
        subtitle: "Final Checklist",
        icon: "🚀",
        phase: "Capstone",
        demo_type: DemoType::Static,
        why_it_matters:
            "A project isn't done until it <strong>works reliably</strong> in the real world. Validation and \
             proper enclosure are essential.",
        intuition: r#"
            <h3>Validation Checklist</h3>
            Before deploying:
            <ul>
              <li><strong>Power budget verified</strong> — measured, not calculated</li>
              <li><strong>Sensor accuracy</strong> — compare to reference</li>
              <li><strong>Wi‑Fi reliability</strong> — test in various locations</li>
              <li><strong>Deep sleep works</strong> — verify wake on timer</li>
              <li><strong>Error handling</strong> — test failure modes</li>
              <li><strong>Long-term test</strong> — run for 24+ hours</li>
            </ul>

            <h3>Enclosure Design</h3>
            Considerations:
            <ul>
              <li><strong>Ventilation</strong> — sensors need air flow</li>
              <li><strong>Antenna clearance</strong> — Wi‑Fi antenna needs space</li>
              <li><strong>Battery access</strong> — for replacement</li>
              <li><strong>Moisture protection</strong> — if outdoor use</li>
              <li><strong>Mounting</strong> — how will it be installed?</li>
            </ul>

            <h3>Deployment</h3>
            Steps:
            <ol>
              <li>Flash firmware to ESP32</li>
              <li>Configure Wi‑Fi credentials (or use WPS)</li>
              <li>Test in final location</li>
              <li>Monitor for 24 hours</li>
              <li>Document installation procedure</li>
            </ol>

            <h3>Monitoring</h3>
            Set up:
            <ul>
              <li><strong>Server dashboard</strong> — view sensor data</li>
              <li><strong>Alerts</strong> — notify on missing data</li>
              <li><strong>Battery monitoring</strong> — track voltage over time</li>
              <li><strong>Error logging</strong> — track failures</li>
            </ul>
        "#,
        demo_explanation: r#"
            Validation is testing, not visual. Review this checklist before deploying your project.
        "#,
        key_takeaways: &[
            "Validate power budget with actual measurements",
            "Test error handling and failure modes",
            "Design enclosure for sensor access and antenna clearance",
            "Monitor deployment for 24+ hours before considering it done",
        ],
        going_deeper:
            "For production, implement over-the-air (OTA) updates so you can fix bugs without physical access. \
             Use structured logging with timestamps. Implement watchdog timers to recover from hangs. \
             Consider adding a status LED for visual feedback.",
        math_details: r#"
Validation metrics:
  Sensor accuracy: ±2% RH, ±0.3°C (SHT31 spec)
  Power consumption: < 300mAs per cycle (measured)
  Wi‑Fi reliability: > 95% success rate
  Battery lifetime: > 90 days (calculated with 20% margin)

Deployment checklist:
  ✓ Firmware flashed and tested
  ✓ Wi‑Fi credentials configured
  ✓ Sensor calibrated (if needed)
  ✓ Power budget verified
  ✓ Enclosure sealed (if outdoor)
  ✓ Mounting secure
  ✓ Monitoring dashboard active
        "#,
        implementation: r#"
<h4>LLM Prompt: Validation Test Suite</h4>
<pre>"Write Rust test suite for ESP32 capstone project.
Include: sensor reading accuracy test, power consumption test,
Wi‑Fi connection reliability test, deep sleep wake test.
Run tests automatically and report results."</pre>

<h4>Final Checklist</h4>
<ol>
<li>✓ Power budget measured and verified</li>
<li>✓ Sensor accuracy validated</li>
<li>✓ Wi‑Fi reliability tested</li>
<li>✓ Deep sleep verified</li>
<li>✓ Error handling tested</li>
<li>✓ Long-term test (24+ hours) passed</li>
<li>✓ Enclosure designed and built</li>
<li>✓ Deployment procedure documented</li>
<li>✓ Monitoring dashboard set up</li>
<li>✓ Project complete!</li>
</ol>
        "#,
    },
];
