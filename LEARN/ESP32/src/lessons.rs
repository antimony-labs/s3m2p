//! ═══════════════════════════════════════════════════════════════════════════════
//! FILE: lessons.rs | ESP32/src/lessons.rs
//! PURPOSE: ESP32 (ESP‑WROOM‑32) lessons - structured + demo-driven
//! MODIFIED: 2025-12-14
//! LAYER: LEARN → ESP32
//! ═══════════════════════════════════════════════════════════════════════════════

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
];

/// A single ESP32 lesson
pub struct Lesson {
    pub id: usize,
    pub title: &'static str,
    pub subtitle: &'static str,
    pub icon: &'static str,
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
}

/// All ESP32 lessons (ESP‑WROOM‑32) - ordered from fundamentals to buses
pub static LESSONS: &[Lesson] = &[
    Lesson {
        id: 0,
        title: "GPIO Debounce",
        subtitle: "Reliable Button Inputs",
        icon: "🔘",
        why_it_matters:
            "A real button press should be <strong>one</strong> event. Without debouncing you get phantom presses, \
             double-clicks, and flaky menus — even with perfect code everywhere else.",
        intuition: r#"
            <h3>Goal</h3>
            Turn a noisy mechanical switch into a clean digital signal you can trust.

            <h3>What’s actually happening</h3>
            A switch is two pieces of metal touching. When they first touch, they bounce for a few milliseconds:
            HIGH/LOW/HIGH/LOW… then finally settle. Your CPU is fast enough to see all those transitions.

            <div class="mermaid">
            flowchart LR
                Raw[RawGPIO] --> Debounce[WaitForStability]
                Debounce --> Clean[CleanState]
            </div>

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
            increase it. If the button feels “laggy”, decrease it.
        "#,
        demo_explanation: r#"
            The top timeline is the <strong>raw</strong> GPIO signal (with bounce). The bottom is the <strong>debounced</strong> output.
            <br><br>
            Use:
            <ul>
              <li><strong>Bounce Severity</strong>: how “messy” the button is</li>
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
    },
    Lesson {
        id: 1,
        title: "PWM Control",
        subtitle: "LEDC: Duty, Frequency, Resolution",
        icon: "📶",
        why_it_matters:
            "PWM is the workhorse for controlling <strong>brightness</strong>, <strong>motor speed</strong>, and <strong>power</strong> using only digital pins.",
        intuition: r#"
            <h3>The core idea</h3>
            PWM rapidly switches a pin HIGH/LOW. The load (LED, motor, filter capacitor) averages it.
            The <strong>duty cycle</strong> sets the average output.

            <div class="mermaid">
            flowchart LR
                Counter[TimerCounter] --> Cmp{Counter&lt;Duty}
                Cmp -->|yes| Hi[GPIO_HIGH]
                Cmp -->|no| Lo[GPIO_LOW]
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
            The top plot is the digital PWM output. The bottom plot shows a smoothed “average” output.
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
    },
    Lesson {
        id: 2,
        title: "ADC Reading",
        subtitle: "Quantization, Noise, Averaging",
        icon: "📊",
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
    },
    Lesson {
        id: 3,
        title: "I2C Communication",
        subtitle: "Addressing, ACK/NAK, Clock Stretching",
        icon: "🔗",
        why_it_matters:
            "I²C lets you connect many peripherals (IMUs, OLEDs, environmental sensors) using only two wires.",
        intuition: r#"
            <h3>The core idea</h3>
            Two wires, many devices:
            <ul>
              <li>SDA = data</li>
              <li>SCL = clock</li>
            </ul>
            Both are <strong>open-drain</strong> and need pull-ups.

            <div class="mermaid">
            sequenceDiagram
                participant Master
                participant Slave
                Master->>Slave: START + Address + W
                Slave-->>Master: ACK
                Master->>Slave: DataByte
                Slave-->>Master: ACK
                Master->>Slave: STOP
            </div>

            <h3>ESP‑WROOM‑32 notes</h3>
            Many devkits default to GPIO21 (SDA) and GPIO22 (SCL), but ESP32 can route I²C to other pins too.
            Keep everything at 3.3V logic unless you add proper level shifting.

            <h3>Mini-lab</h3>
            Increase NAK chance and see how a transaction aborts. Add clock stretching and watch SCL LOW extend.
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
            "I²C lines are open-drain and require pull-up resistors",
            "Address is 7-bit; the R/W bit is appended on the wire",
            "ACK is SDA LOW on the 9th clock; NACK is SDA HIGH",
            "Clock stretching holds SCL LOW longer to delay the next edge",
        ],
        going_deeper:
            "If you see unreliable I²C, check pull-up value, wiring length, and bus speed. \
             A common first step is an address scan. If a device NACKs, verify its address and power. \
             For multiple devices, avoid address conflicts (some sensors have address-select pins).",
        math_details: r#"
I²C frame (write, 7-bit address):
  START
  [A6 A5 A4 A3 A2 A1 A0 W]
  ACK
  [D7 D6 D5 D4 D3 D2 D1 D0]
  ACK
  STOP
        "#,
    },
];
