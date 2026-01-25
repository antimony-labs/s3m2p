---
title: "FCC Class B Certification: The Complete Guide for Robotics Companies"
slug: "fcc-class-b-robotics-guide"
date: "2026-01-24"
tags: [robotics, hardware, compliance, emc, fcc, certification]
summary: "A practical guide to FCC Part 15 Class B certification for robotics companies - from understanding emission limits to passing your first EMC test."
draft: false
---

*Everything you need to know to design, test, and certify your robot for the US market*

**Jump to:** [What is FCC Class B](#what-is-fcc-class-b) | [Part 15 Framework](#the-fcc-part-15-framework) | [Emission Limits](#emission-limits-the-numbers-that-matter) | [EMI Sources in Robots](#emi-sources-in-robots) | [Design for Compliance](#designing-for-compliance) | [Pre-Compliance Testing](#pre-compliance-testing-setup) | [Working with Test Labs](#working-with-test-labs) | [Certification Paths](#certification-paths-sdoc-vs-certification) | [Wireless Modules](#wireless-modules-part-15-subpart-c) | [Common Failures](#common-failures-and-fixes) | [Cost and Timeline](#cost-and-timeline) | [Checklist](#pre-submission-checklist)

## Introduction

<div style="text-align: center;">

![Test equipment and electronics workbench](/images/fcc-guide/test-equipment.jpg)

</div>

You've built a robot. It works. Your customers want it. But before you can legally sell it in the United States, you need to prove it won't interfere with your neighbor's radio, TV, or WiFi network.

That proof comes in the form of FCC Part 15 compliance - specifically, Class B certification for any device marketed for residential use. This includes home robots, educational robots, consumer drones, robot vacuums, lawn mowers, and any robot that might end up in someone's house.

Approximately 50% of consumer electronics products fail EMC testing on their first attempt ([industry estimates](https://incompliancemag.com/article/the-cost-of-emc-compliance/)). For robots - with their motors, switching power supplies, high-speed digital circuits, and often wireless connectivity - the failure rate can be even higher.

This guide covers everything a robotics company needs to know: the regulatory framework, the actual emission limits, common EMI sources in robots, design strategies for compliance, the testing process, and how to navigate certification. Whether you're designing a new robot from scratch or modifying an existing design for the US market, this is your roadmap.

## What is FCC Class B

The Federal Communications Commission (FCC) regulates electromagnetic emissions from electronic devices to protect the radio spectrum. [Part 15 of Title 47 of the Code of Federal Regulations](https://www.ecfr.gov/current/title-47/chapter-I/subchapter-A/part-15) (47 CFR Part 15) covers "Radio Frequency Devices" - equipment that can emit RF energy, whether intentionally or not.

Digital devices are classified into two categories:

**Class A**: Devices marketed for use in commercial, industrial, or business environments. Examples: industrial controllers, server equipment, manufacturing robots.

**Class B**: Devices marketed for use in residential environments, regardless of whether they're also used commercially. Examples: personal computers, consumer electronics, home robots, educational robots.

The distinction matters because Class B limits are approximately 10 dB stricter than Class A. A robot that passes Class A testing might fail Class B by a significant margin.

If your robot might reasonably be used in a home - and most robots marketed to consumers or educational institutions fall into this category - you need Class B compliance.

## The FCC Part 15 Framework

Part 15 covers three categories of devices:

<div class="mermaid">
flowchart TB
    P15[FCC Part 15] --> INT[Intentional Radiators]
    P15 --> UNINT[Unintentional Radiators]
    P15 --> INC[Incidental Radiators]
    INT --> W[WiFi Bluetooth Zigbee]
    UNINT --> D[MCUs Power Supplies]
    INC --> M[DC Motors Relays]
</div>

**Intentional radiators** (Subpart C): Devices designed to emit RF energy for communication. WiFi modules, Bluetooth radios, Zigbee transceivers. These require formal FCC Certification with an FCC ID.

**Unintentional radiators** (Subpart B): Digital devices that emit RF energy as a byproduct of operation. Microcontrollers, FPGAs, switching power supplies, high-speed buses. These can use Supplier's Declaration of Conformity (SDoC).

**Incidental radiators**: Devices not designed to generate RF but which may cause interference. DC motors, relays, mechanical switches. These have no specific limits but must not cause harmful interference.

Most robots combine all three: wireless communication (intentional), digital control circuits (unintentional), and motors (incidental). Each component type has different requirements.

### Key Regulatory Sections

| Section | Coverage |
|---------|----------|
| [15.107](https://www.ecfr.gov/current/title-47/section-15.107) | Conducted emission limits |
| [15.109](https://www.ecfr.gov/current/title-47/section-15.109) | Radiated emission limits |
| [15.31](https://www.ecfr.gov/current/title-47/section-15.31) | Measurement procedures (references [ANSI C63.4](https://webstore.ansi.org/standards/ieee/ansic632014)) |
| [15.247](https://www.ecfr.gov/current/title-47/section-15.247) | Spread spectrum devices (WiFi, Bluetooth) |
| [2.906](https://www.ecfr.gov/current/title-47/section-2.906) | Supplier's Declaration of Conformity |

## Emission Limits: The Numbers That Matter

These are the limits your robot must meet. Memorize them - or at least keep them handy during design reviews.

### Radiated Emission Limits (Class B at 3 meters)

| Frequency Range | Limit (dBuV/m) | Limit (uV/m) |
|-----------------|----------------|--------------|
| 30 - 88 MHz | 40 | 100 |
| 88 - 216 MHz | 43.5 | 150 |
| 216 - 960 MHz | 46 | 200 |
| 960 MHz - 1 GHz | 54 | 500 |
| Above 1 GHz | 54 avg / 74 peak | 500 avg |

At transition frequencies, the lower limit applies.

For comparison, Class A limits are 9-10 dB higher (roughly 3x the field strength). That margin matters - a device at 48 dBuV/m passes Class A but fails Class B by 8 dB.

### Conducted Emission Limits (Class B)

Applies to devices connected to AC power lines. Measured with a 50uH/50 ohm Line Impedance Stabilization Network (LISN).

| Frequency Range | Quasi-Peak (dBuV) | Average (dBuV) |
|-----------------|-------------------|----------------|
| 150 - 500 kHz | 66 to 56 (slope) | 56 to 46 (slope) |
| 500 kHz - 5 MHz | 56 | 46 |
| 5 - 30 MHz | 60 | 50 |

The 150-500 kHz range decreases linearly with log frequency at 20 dB/decade.

Battery-only devices that never connect to AC power are exempt from conducted emissions testing - a significant advantage for battery-powered robots.

### Understanding the Numbers

This section explains what the units mean and how measurements work. Understanding this helps you interpret test reports and pre-compliance measurements.

#### Units: dBuV and dBuV/m

**dBuV** = decibels relative to 1 microvolt

This is a logarithmic scale for measuring voltage. The reference point (0 dBuV) is 1 microvolt (1 uV = 0.000001 V).

| dBuV | Microvolts (uV) | Calculation |
|------|-----------------|-------------|
| 0 | 1 | Reference |
| 20 | 10 | 10x voltage |
| 40 | 100 | 100x voltage |
| 46 | 200 | Class B limit at 216-960 MHz |
| 60 | 1000 (1 mV) | 1000x voltage |

**Converting dBuV to microvolts:**
```
uV = 10^(dBuV / 20)

Example: 46 dBuV = 10^(46/20) = 10^2.3 = 200 uV
```

**dBuV/m** = decibels relative to 1 microvolt per meter

This measures electric field strength - how strong the electromagnetic field is at a given distance. For radiated emissions, the antenna converts field strength to voltage, and the measurement is expressed as dBuV/m.

The "/m" matters because field strength decreases with distance. FCC Class B measurements are taken at 3 meters. At 10 meters, the same source would read about 10 dB lower.

#### Why Decibels?

EMC measurements span enormous ranges. A quiet device might emit 20 dBuV/m while a noisy one emits 60 dBuV/m - that's a 100x difference in field strength. Decibels compress this range into manageable numbers.

**Quick dB math:**
- +3 dB = 1.4x (roughly double power, 1.4x voltage)
- +6 dB = 2x voltage
- +10 dB = 3.16x voltage
- +20 dB = 10x voltage
- -6 dB = 0.5x voltage (halved)

If you're 8 dB over the limit, you need to reduce emissions to about 40% of current level (roughly 1/2.5).

#### Detector Types: Peak, Quasi-Peak, and Average

When you measure RF emissions, the signal isn't constant - it fluctuates. Different detectors respond differently to these fluctuations.

<div class="mermaid">
flowchart LR
    subgraph SIG[Pulsed Signal]
        P1[Pulse] ~~~ G1[Gap] ~~~ P2[Pulse] ~~~ G2[Gap] ~~~ P3[Pulse]
    end
    SIG --> PEAK[Peak Detector]
    SIG --> QP[Quasi-Peak Detector]
    SIG --> AVG[Average Detector]
    PEAK --> R1[Highest: Catches every pulse]
    QP --> R2[Middle: Weighted by repetition rate]
    AVG --> R3[Lowest: RMS over time]
</div>

**Peak Detector**
- Captures the maximum amplitude of any signal
- Fastest response - catches brief transients
- Gives the highest reading
- Used in pre-compliance because it's quick and conservative
- NOT used for final FCC compliance (would be too strict)

**Quasi-Peak Detector**
- Defined by [CISPR 16-1-1](https://webstore.iec.ch/publication/12119) standard
- Has specific charge and discharge time constants
- Responds more to repetitive signals than to occasional pulses
- Mimics how interference affects AM radio reception
- Reading depends on pulse repetition rate:

| Pulse Rate | QP Reading vs Peak |
|------------|-------------------|
| Continuous (CW) | Equal to peak |
| 100 Hz repetition | ~10 dB below peak |
| 10 Hz repetition | ~20 dB below peak |
| Single pulse | Much below peak |

A motor with continuous PWM switching (high repetition rate) will have quasi-peak readings close to peak. A microcontroller that occasionally bursts data will have quasi-peak well below peak.

**Average Detector**
- True RMS measurement over the sweep time
- Lowest reading of the three
- Limits are typically 10 dB below quasi-peak limits
- Important for signals with low duty cycle

#### Practical Example

Your robot's motor driver switches at 20 kHz with occasional higher-frequency bursts:

```
Measurement at 150 MHz:
  Peak detector:       52 dBuV/m
  Quasi-peak detector: 45 dBuV/m
  Average detector:    38 dBuV/m

Class B limits at 150 MHz:
  Quasi-peak limit: 43.5 dBuV/m  ← FAIL by 1.5 dB
  Average limit: N/A (radiated doesn't have average limit)
```

In this case, you fail quasi-peak by 1.5 dB. That's close - within measurement uncertainty. You need to reduce emissions, but small fixes (adding a ferrite) might be enough.

#### Conducted vs. Radiated Units

**Conducted emissions** (power line noise):
- Measured in **dBuV** (voltage at LISN port)
- Both quasi-peak AND average limits apply
- Must pass BOTH limits

**Radiated emissions** (electromagnetic field):
- Measured in **dBuV/m** (field strength at antenna)
- Only quasi-peak limit applies below 1 GHz
- Above 1 GHz, both average and peak limits apply

#### Reading a Test Report

A typical test report shows:

```
Frequency   Measured    Limit     Margin    Det   Result
(MHz)       (dBuV/m)   (dBuV/m)   (dB)
─────────────────────────────────────────────────────────
 87.5        38.2       40.0      1.8       QP    PASS
152.3        41.8       43.5      1.7       QP    PASS
304.6        44.1       46.0      1.9       QP    PASS
456.9        47.2       46.0     -1.2       QP    FAIL ←
```

**Margin** = Limit - Measured
- Positive margin = PASS (you're under the limit)
- Negative margin = FAIL (you're over the limit)
- 6+ dB margin = comfortable
- 0-3 dB margin = marginal, could fail on another sample or different day
- Negative = must fix

#### Key Takeaways

1. **dBuV/m** is field strength - what the antenna sees at 3 meters
2. **Quasi-peak** is the primary limit for most FCC tests
3. **Peak** reading is always ≥ quasi-peak ≥ average
4. **Margin matters** - aim for 6 dB margin, not just barely passing
5. **Both limits** must pass for conducted emissions (QP and AVG)

When doing pre-compliance with a basic spectrum analyzer, you'll typically use peak detector (it's faster). Assume your quasi-peak reading will be 5-15 dB lower than peak, depending on signal characteristics. If your peak reading is within 10 dB of the limit, investigate further.

## EMI Sources in Robots

Robots are EMI nightmares. They combine multiple high-noise subsystems in close proximity.

### Motors

**Brushed DC motors**: The worst offenders. Mechanical commutation creates arcing at the brushes - broadband noise from DC to hundreds of MHz. Every brush contact generates a spark, and those sparks radiate.

**Brushless DC (BLDC) motors**: No brush arcing, but the electronic commutation introduces switching noise. The driver circuit switches high currents at kHz rates, creating harmonics throughout the spectrum.

**Servo motors**: Combine motor noise with encoder signals and high-speed PWM control loops.

**Stepper motors**: Chopping drives create significant conducted and radiated emissions from the current waveforms.

### Power Electronics

**Switching power supplies**: Switch-mode converters operate at 100 kHz to 2 MHz with fast edge rates. Primary EMI sources include:
- Switching frequency fundamentals and harmonics
- Common-mode currents through parasitic capacitances
- Differential-mode currents from ripple

**Motor drivers**: H-bridges and three-phase inverters switch high currents. Dead-time transients and reverse recovery of body diodes create high-frequency noise.

**Battery management**: Charging circuits with switching regulators add another noise source.

### Digital Circuits

**Microcontrollers**: Clock frequencies from 8 MHz to 400+ MHz generate harmonics. A 100 MHz clock has significant energy at 300 MHz, 500 MHz, 700 MHz...

**High-speed interfaces**: USB 2.0 (480 Mbps), Ethernet, HDMI, camera interfaces (MIPI CSI) - all radiate if traces aren't properly routed.

**FPGAs and SoCs**: Thousands of simultaneous switching outputs (SSO) create massive transient currents.

### Cables as Antennas

Every cable attached to your robot is a potential antenna. Power cables, sensor cables, motor cables - if they carry high-frequency noise, they radiate it.

Cable emissions often dominate radiated emissions tests. A device might have a well-shielded enclosure but fail because motor cables radiate switching noise.

## Designing for Compliance

<div style="text-align: center;">

![PCB circuit board closeup](/images/fcc-guide/pcb-closeup.jpg)

</div>

EMC must be designed in from the start. Fixing EMC problems after the PCB is manufactured is expensive - adding shielding, ferrites, and filter components as afterthoughts costs more and works worse than proper initial design.

### PCB Layer Stackup

For EMC, a 4-layer board beats a 2-layer board every time. Here are recommended stackups:

**4-Layer (Standard EMC Design):**
```
Layer 1: Signals (top)        - 1.0 oz copper
Prepreg: 7 mil FR4
Layer 2: Ground plane         - 1.0 oz copper
Core: 40 mil FR4
Layer 3: Power plane          - 1.0 oz copper
Prepreg: 7 mil FR4
Layer 4: Signals (bottom)     - 1.0 oz copper
```

Every signal has an adjacent reference plane, containing fields and reducing radiation.

**6-Layer (High-Speed/High-Power):**
```
Layer 1: Signals
Layer 2: Ground
Layer 3: Signals (inner, for clocks)
Layer 4: Power
Layer 5: Ground
Layer 6: Signals
```

Route clock signals on Layer 3 - sandwiched between ground planes for maximum shielding.

### Ground Plane Design

**Ground planes must be solid and unbroken.** The ground plane provides a low-impedance return path for high-frequency currents. When you cut the plane, return currents must flow around the cut, creating large loop areas that radiate.

<div class="mermaid">
flowchart LR
    subgraph BAD[Bad: Split Ground]
        S1[Signal] -.->|Long return path| G1[GND]
    end
    subgraph GOOD[Good: Solid Ground]
        S2[Signal] -->|Direct return| G2[GND Plane]
    end
</div>

**Rules for ground planes:**
- Never route signals across plane splits
- If you must split (analog/digital isolation), bridge with a common-mode choke, not a direct connection
- Use ground stitching vias every 1/20 wavelength of your highest frequency of concern
- For 1 GHz, that's every 15mm; for 100 MHz, every 150mm

### Power Distribution Network

**Decoupling capacitor placement:** Place capacitors as close as possible to IC power pins. The inductance of traces and vias matters.

**Recommended decoupling per IC:**
```
100 nF ceramic (0402 or 0603) - within 3mm of power pin
10 nF ceramic (0402) - within 5mm
1 uF ceramic (0805) - within 10mm
10 uF bulk (electrolytic or tantalum) - per power domain
```

**Power input filtering (for main power rails):**

```
          L1                     L2
VIN o----[===]----+----[===]----+---- VOUT
                  |             |
                C1 |           C2 | C3
                  |             |  |
GND o-------------+-------------+--+
```

Recommended values:
- C1: 100 uF electrolytic (bulk storage)
- L1: 10 uH ferrite bead ([Fair-Rite 2743019447](https://www.fair-rite.com/product/round-cable-emi-suppression-cores-2743019447/)) or inductor
- L2: Ferrite bead, 100 ohm @ 100 MHz ([Wurth 742792093](https://www.we-online.com/en/components/products/WE-CBF))
- C2: 10 uF ceramic X5R ([Murata GRM21BR61A106KE19](https://www.murata.com/en-us/products/productdetail?partno=GRM21BR61A106KE19))
- C3: 100 nF + 10 nF + 1 nF ceramics in parallel

### Motor Output Filtering

This is critical for robots. BLDC motor cables are often the dominant emission source.

**Basic motor filter (per phase):**

```
                  L1
Driver o----+----[===]----+---- Motor
            |             |
           C1|           C2|
            |             |
GND    o----+-------------+
```

Component values:
- L1: 4.7 uH - 22 uH power inductor ([Wurth 744373680022](https://www.we-online.com/en/components/products/WE-HCI), 22uH/8A)
- C1: 100 nF ceramic X7R 50V (at driver side)
- C2: 100 nF ceramic X7R 50V (at motor side)

**Enhanced filter with common-mode choke:**

```
           CMC             L1
     +---[====]---+---+---[===]---+--- Motor+
     |            |   |           |
IN --+            |  C1|         C2|
     |            |   |           |
     +---[====]---+---+-----------+--- Motor-
           CMC
```

Common-mode choke recommendations:
- [Wurth 744272102](https://www.we-online.com/en/components/products/WE-CMB) (1mH, 2A) - for small motors
- [Wurth 744273102](https://www.we-online.com/en/components/products/WE-CMB) (1mH, 4A) - for medium motors
- [Fair-Rite 2643102002](https://www.fair-rite.com/product/common-mode-chokes-2643102002/) - for high current applications

### Cable Shielding

**Shielded cables must be grounded correctly:**

For frequencies above 1 MHz (which includes most EMI concerns):
- Ground shield at BOTH ends
- Use 360-degree termination (connector backshell or clamp)
- Do NOT use pigtails (they add inductance and defeat shielding above 10 MHz)

**Good shield termination:**
```
Cable shield connects to conductive connector shell
Connector shell connects to chassis via metal-to-metal contact
```

**Bad shield termination:**
```
Shield twisted into pigtail, connected to ground via screw terminal
(This adds 10-20 nH inductance per cm of pigtail)
```

For motor cables, use shielded cables with the shield grounded to the motor housing (if conductive) and to chassis at the controller end.

### Enclosure Design

A conductive enclosure acts as a Faraday cage. Key requirements:

**Seams and joints:**
- Welded seams: best shielding
- Overlapping joints with conductive gasket: good
- Simple butt joints with screws: gaps radiate

**Apertures (openings):**
- Maximum aperture dimension should be < 1/20 wavelength at highest frequency of concern
- At 1 GHz, that's 15mm
- At 300 MHz, that's 50mm
- Ventilation slots: use multiple small slots instead of one large opening

**Cable penetrations:**
- Every cable entering the enclosure is a potential leak
- Use filtered connectors or bulkhead feedthrough filters
- Ground cable shields at the enclosure boundary

**Display/indicator windows:**
- Use conductive mesh behind plastic windows
- Or use ITO (Indium Tin Oxide) coated transparent plastic

## Pre-Compliance Testing Setup

<div style="text-align: center;">

![Electronics lab setup](/images/fcc-guide/electronics-lab.jpg)

</div>

Before spending $5,000-15,000 at a certified test lab, do pre-compliance testing in-house. This catches major issues when fixes are cheap.

### Essential Equipment

**Spectrum Analyzer Options:**

| Equipment | Price Range | Frequency Range | Notes |
|-----------|-------------|-----------------|-------|
| [tinySA Ultra](https://tinysa.org/wiki/pmwiki.php?n=Main.Ultra) | $120 | 100 kHz - 6 GHz | Entry level, good for finding gross problems |
| [Siglent SSA3021X](https://siglentna.com/spectrum-analyzers/ssa3000x-series-spectrum-analyzers/) | $1,500 | 9 kHz - 2.1 GHz | Solid mid-range, adequate for most pre-compliance |
| [Rigol DSA815-TG](https://www.rigolna.com/products/spectrum-analyzers/dsa800/) | $1,200 | 9 kHz - 1.5 GHz | Similar to Siglent, includes tracking generator |
| [Rigol RSA5065](https://www.rigolna.com/products/spectrum-analyzers/rsa5000/) | $4,000 | 9 kHz - 6.5 GHz | Real-time spectrum analyzer, catches transients |

For robotics pre-compliance, the Siglent SSA3021X or Rigol DSA815 are the sweet spot - accurate enough to predict lab results, affordable enough to justify.

**Near-Field Probe Set:**

Near-field probes let you locate emission sources on your PCB. Essential probes:

| Probe Type | What It Detects | Use Case |
|------------|-----------------|----------|
| H-field loop (small, 10mm) | Magnetic fields, current loops | Find noisy traces, identify loop areas |
| H-field loop (large, 25mm) | Lower frequency currents | Find power supply switching noise |
| E-field probe | Electric fields, voltage nodes | Find high-impedance noise sources |

Recommended sets:
- [Beehive Electronics 100D set](https://beehive-electronics.com/probes.html) (~$500) - includes H and E field probes
- [Com-Power PSA-1](https://www.com-power.com/near-field-probes.html) (~$900) - professional grade
- DIY option: Wind 3-5 turns of magnet wire on a small form, connect to SMA

**LISN (Line Impedance Stabilization Network):**

Required if your robot connects to AC power. The LISN provides a defined 50uH/50ohm impedance and filters out ambient noise from the power line.

| LISN | Price | Current Rating | Notes |
|------|-------|----------------|-------|
| [Tekbox TBLC08](https://www.tekbox.com/product/tblc08-lisn-line-impedance-stabilisation-network/) | $250 | 8A | Budget option, single-phase |
| [Com-Power LI-125A](https://www.com-power.com/lisn.html) | $1,200 | 25A | Professional grade |
| [Schwarzbeck NSLK 8127](https://schwarzbeck.de/en/lisn-line-impedance-stabilization-networks.html) | $3,000 | 16A | Lab-grade reference |

For most robots, the Tekbox TBLC08 is sufficient for pre-compliance.

### Setting Up Your Test Area

You don't need an anechoic chamber for pre-compliance. You need:

**Basic setup:**
1. Large table (1m x 2m minimum), non-conductive surface
2. Device Under Test (DUT) placed 80cm above floor/table
3. Ground reference plane (copper sheet or aluminum foil) under DUT
4. Spectrum analyzer at least 3 meters from DUT for radiated tests
5. All equipment powered from same outlet/power strip

**Reducing ambient noise:**
- Turn off WiFi routers, Bluetooth devices, phones
- Test during off-hours when RF environment is quieter
- Take background measurement with DUT off, then compare with DUT on
- Look for emissions that appear ONLY when DUT is on

### Pre-Compliance Measurement Procedure

**Radiated emissions scan:**

1. Connect antenna to spectrum analyzer
   - 30-300 MHz: biconical antenna or active rod antenna
   - 200 MHz - 1 GHz: log-periodic antenna
   - Cheapest acceptable: $200-400 for a basic biconical + log-periodic set

2. Set spectrum analyzer:
   - RBW (Resolution Bandwidth): 120 kHz (matches FCC quasi-peak measurement)
   - Sweep time: 1-2 seconds per sweep
   - Detector: Peak (quasi-peak is slower and not always available)
   - Start frequency: 30 MHz
   - Stop frequency: 1 GHz

3. Position antenna 3 meters from DUT (or 1 meter and add 10 dB to readings)

4. Run DUT in operating mode:
   - Motors running
   - Communication active
   - All subsystems powered

5. Rotate DUT (or antenna) to find worst-case orientation

6. Record peak emissions and compare to limits (with 6-10 dB margin for pre-compliance uncertainty)

**Conducted emissions scan:**

1. Connect DUT to LISN
2. Connect LISN measurement port to spectrum analyzer
3. Set spectrum analyzer:
   - RBW: 9 kHz (standard for conducted)
   - Sweep: 150 kHz - 30 MHz
   - Detector: Peak

4. Measure both Line and Neutral

5. Compare to limits (with 6 dB margin)

### Interpreting Pre-Compliance Results

**Calculating margin to limit:**

Your pre-compliance setup has uncertainty - typically 6-10 dB. If a certified lab uses a 3m measurement distance and you used 1m, add 10 dB to your readings (field strength drops with distance).

Rule of thumb:
- Pre-compliance reading + 6 dB < limit = likely pass
- Pre-compliance reading within 6 dB of limit = marginal, investigate
- Pre-compliance reading > limit = will fail, must fix

**Identifying emission sources:**

1. Note the frequency of problem emissions
2. Calculate what clock/switching frequency could produce that harmonic
   - 150 MHz emission = could be 50 MHz clock x3, or 30 MHz x5, or 25 MHz x6
3. Use near-field probe to scan PCB at that frequency
4. The probe signal will peak over the source

**Common frequency signatures:**

| Observed Frequency | Likely Source |
|-------------------|---------------|
| 30-50 MHz | Power supply harmonics, long cables |
| 48/96/144 MHz | USB 2.0 (12/48 MHz clock harmonics) |
| 100/200/300 MHz | 100 MHz processor clock |
| 50/100/150 MHz | 25 MHz Ethernet clock |
| PWM frequency x N | Motor driver switching |

## Working with Test Labs

### Finding a Test Lab

**Accreditation matters:**
- For SDoC, any competent lab works
- For Certification, you need an FCC-recognized lab (listed on [FCC website](https://apps.fcc.gov/oetcf/eas/reports/TestFirmSearch.cfm))
- [ISO/IEC 17025](https://www.iso.org/ISO-IEC-17025-testing-and-calibration-laboratories.html) accreditation indicates lab competence

**Finding labs:**
- [A2LA Directory](https://customer.a2la.org/index.cfm?event=directory.index) lists accredited labs by location
- [FCC Test Firm Search](https://apps.fcc.gov/oetcf/eas/reports/TestFirmSearch.cfm) lists recognized labs for certification
- Ask other hardware companies for recommendations

**Questions to ask:**
1. What is your availability? (Lead times can be 2-6 weeks)
2. Do you have experience with robots/motor-driven devices?
3. What is included in the quoted price?
4. What happens if we fail - is retest included or extra?
5. Can we be present during testing?
6. Do you offer pre-scan services (quick check before formal test)?
7. Can you provide engineering support to help diagnose failures?

### What Testing Costs

| Test | Typical Cost | What's Included |
|------|-------------|-----------------|
| FCC Part 15B pre-scan | $500-1,000 | Quick radiated + conducted scan, no report |
| FCC Part 15B full test | $2,500-5,000 | Full test per [ANSI C63.4](https://webstore.ansi.org/standards/ieee/ansic632014), test report |
| FCC Part 15B + 15C (WiFi/BT) | $8,000-15,000 | Full Part 15 + intentional radiator tests |
| Retest (same day) | Often included | If you fix issues on-site |
| Retest (return visit) | $1,000-2,500 | Come back after fixes |
| TCB certification fee | $1,000-3,000 | Required for intentional radiators |

**Negotiation tips:**
- Ask for package pricing if doing multiple tests (FCC + CE + Canada)
- Ask if pre-scan is included in full test price
- Confirm retest policy in writing before starting
- Some labs offer reduced rates for startups or first-time customers

### What to Bring

**Hardware:**
- 2-3 production-representative units (testing can damage samples)
- Final enclosure (not prototype - enclosure affects shielding)
- Final power supply
- All cables that ship with product
- Spare parts (fuses, connectors) in case something breaks

**Documentation:**
- Block diagram showing all major subsystems
- Schematic (lab may not need, but helpful for debugging)
- User manual (draft is fine)
- List of operating modes
- Photos showing internal layout
- Any previous test reports

**For wireless devices (add):**
- Module FCC ID and grant documentation
- Antenna specifications
- Maximum EIRP calculations

### What Testing Day Looks Like

**Setup (1-2 hours):**
- Lab technician reviews documentation
- DUT placed on turntable in chamber
- Cables arranged per test plan
- EUT configured for worst-case mode

**Radiated emissions (2-4 hours):**
- DUT rotated 360 degrees
- Antenna height varied 1-4 meters
- Measured in horizontal and vertical polarization
- Each frequency range scanned
- Peaks investigated in detail

**Conducted emissions (1-2 hours):**
- DUT connected to LISN
- Line and Neutral measured
- Multiple operating modes tested

**Results review:**
- Technician shows you preliminary results
- If marginal or failing, discuss possible causes
- May do diagnostic measurements with probes

### When You Fail

Most products fail first time. Don't panic.

**On-site fixes (if lab allows):**
- Add ferrite cores to cables
- Add shielding tape/foil to enclosure gaps
- Bypass filter components

Bring a kit:
- Assorted ferrite cores (snap-on type)
- Copper tape
- Ferrite beads
- Capacitors (100pF, 1nF, 10nF, 100nF)
- Conductive gasket material

**If you can't fix on-site:**
1. Get detailed failure data (frequency, amplitude, margin, orientation)
2. Ask lab to probe with near-field to identify source
3. Take photos of test setup
4. Go back to your lab to investigate and fix
5. Do pre-compliance verification before returning

**Common fixes by failure type:**

| Failure | Fix |
|---------|-----|
| Cable emissions 30-100 MHz | Add common-mode chokes at cable entry |
| Clock harmonic radiation | Add shield can over clock source, add series resistor |
| Power supply switching harmonics | Improve input filter, add ferrite on DC output |
| Motor cable emissions | Add output LC filter at driver, shield cables |

## Certification Paths: SDoC vs. Certification

### Supplier's Declaration of Conformity (SDoC)

For unintentional radiators (devices without intentional transmitters), you can self-declare compliance.

**Requirements:**
- Responsible party must be located in the United States (or use Certification instead)
- Testing must be performed (any competent lab, not necessarily accredited)
- Test report and documentation retained on file
- Compliance Information Statement included with product
- FCC logo optional on label

**Advantages:**
- No application fees
- No FCC database listing (faster to market)
- Can use any test lab

**Disadvantages:**
- Full liability on manufacturer
- Less credibility than formal Certification
- Still requires US responsible party

### Certification (via TCB)

Required for intentional radiators. Optional for unintentional radiators.

**Process:**
1. Test at FCC-recognized accredited lab ([ISO/IEC 17025](https://www.iso.org/ISO-IEC-17025-testing-and-calibration-laboratories.html))
2. Submit application to Telecommunications Certification Body ([TCB](https://apps.fcc.gov/oetcf/tcb/index.cfm))
3. TCB reviews test data and documentation
4. Grant of Equipment Authorization issued
5. FCC ID assigned and listed in FCC database

**Required for:**
- WiFi modules
- Bluetooth radios
- Any intentional transmitter

**Timeline**: 1-2 weeks after testing complete, assuming no issues.

### Using Pre-Certified Modules

If your robot includes WiFi or Bluetooth, consider using pre-certified modules.

**Advantages:**
- Module already has FCC ID
- Your product only needs Subpart B testing (unintentional emissions)
- Significant time and cost savings

**Requirements:**
- Must use module per manufacturer's integration guidelines
- Antenna must match certified configuration
- Module FCC ID must appear on your product label (or reference in manual)
- May still need limited testing for "host" emissions

**Cost tradeoff**: Pre-certified modules cost $10-50 per unit vs. $4-10 for custom RF design, but save $20,000-50,000+ in certification costs.

## Wireless Modules: Part 15 Subpart C

If your robot has WiFi, Bluetooth, Zigbee, or any other wireless communication, additional requirements apply under Part 15 Subpart C.

### Part 15.247 (Spread Spectrum)

Covers most WiFi and Bluetooth devices in ISM bands:
- 902-928 MHz
- 2.400-2.4835 GHz
- 5.725-5.875 GHz

**Requirements:**
- Power limits (typically 1W EIRP for point-to-multipoint)
- Antenna requirements
- Frequency hopping or spread spectrum modulation
- Out-of-band emission limits

### Certification vs. Modular Approval

**Full certification**: Test the entire device, including radio, as one unit. Required if:
- Using a non-modular radio chip
- Modifying antenna or RF path from pre-certified configuration
- Module doesn't have modular approval

**Modular approval**: Module certified independently. Your device inherits the module's certification. Requirements:
- Module has its own FCC ID with modular grant
- You follow module integration guidelines exactly
- Your product adds module's FCC ID to label or references it in documentation

Most robotics companies should use pre-certified modules unless RF expertise is core to the product.

## Common Failures and Fixes

### Failure: Radiated Emissions 30-100 MHz

**Typical cause**: Cables acting as antennas, driven by switching power supply harmonics or motor noise.

**Diagnosis:**
1. Note exact failure frequencies
2. Disconnect cables one at a time to identify which cable radiates
3. Use near-field probe on PCB to find source

**Fixes (in order of effectiveness):**

1. **Common-mode chokes at cable entry:**
   - [Wurth 744272102](https://www.we-online.com/en/components/products/WE-CMB) (1mH, 2A) for signal cables
   - [Wurth 744273102](https://www.we-online.com/en/components/products/WE-CMB) (1mH, 4A) for power cables
   - Place as close to connector as possible

2. **Ferrite cores on cables:**
   - [Fair-Rite 0431164281](https://www.fair-rite.com/product/round-cable-emi-suppression-cores-0431164281/) (snap-on, 100 ohm @ 25 MHz)
   - Multiple turns through core increase impedance
   - 3 turns = 9x impedance of 1 turn

3. **Improve cable shielding:**
   - Switch to shielded cables
   - Ground shields at both ends with 360-degree termination

4. **Add filtering at source:**
   - LC filter on power supply output
   - Pi filter on motor driver output

### Failure: Radiated Emissions at Clock Harmonics

**Typical cause**: High-speed clock signals radiating from PCB traces.

**Diagnosis:**
- Emission frequency matches clock frequency or harmonic
- Near-field probe peaks over clock oscillator or traces

**Fixes:**

1. **Route clocks on inner layers** (requires board respin)

2. **Add series resistor to slow edge rate:**
   - 22-47 ohm series resistor on clock output
   - Test timing to ensure still works

3. **Use spread-spectrum clock generator:**
   - Replaces fixed-frequency oscillator
   - Spreads energy over bandwidth, reducing peak
   - [Renesas/IDT 5V41066](https://www.renesas.com/us/en/products/clocks-timing/clock-generation/spread-spectrum-clocks) or similar
   - Reduces peaks by 10-15 dB

4. **Shield clock generator:**
   - Add shield can over oscillator
   - [Laird BMI-S-202](https://www.laird.com/products/electromagnetic-shielding/board-level-shielding) or similar

### Failure: Conducted Emissions 150-500 kHz

**Typical cause**: Switching power supply fundamental frequency.

**Diagnosis:**
- Emission at power supply switching frequency and harmonics
- Worse at higher loads

**Fixes:**

1. **Improve input filter:**
```
AC IN o--[CMC]--+--[L]--+-- DC stage
                |       |
               [C]     [C]
                |       |
GND    o--------+-------+
```
   - CMC: [Wurth 744272681](https://www.we-online.com/en/components/products/WE-CMB) (68mH common-mode choke)
   - L: 1-10 mH differential mode inductor
   - C: 100nF-1uF X2 safety capacitors

2. **Increase switching frequency:**
   - Move fundamental above 500 kHz where limits are flat
   - Requires power supply redesign

3. **Spread-spectrum modulation:**
   - Many modern PWM controllers support this
   - Spreads energy, reducing peak

### Failure: Motor Cable Emissions

**Typical cause**: Motor driver PWM harmonics coupling to motor cables.

**Diagnosis:**
- Emissions at PWM frequency harmonics
- Emissions disappear when motor unplugged
- Near-field probe shows motor cable as source

**Fixes:**

1. **Output LC filter at driver:**
```
         L1        L2
PWM+ o--[==]--+--[==]--+-- Motor+
              |        |
             C1|      C2|
              |        |
PWM- o--[==]--+--[==]--+-- Motor-
         L1        L2
```
   - L1/L2: 10-22 uH power inductors (matched for three-phase)
   - C1/C2: 100nF ceramic 50V or higher
   - This forms a second-order low-pass filter

2. **Shield motor cables:**
   - Use shielded cable ([Belden 29502](https://www.belden.com/products/cable/industrial-cable) or similar)
   - Ground shield at motor housing and controller chassis

3. **Reduce PWM frequency:**
   - Lower frequency = lower harmonic content at problem frequencies
   - May affect motor performance/audible noise

4. **Add ferrite cores:**
   - Snap-on cores on motor cables
   - [Fair-Rite 0431164281](https://www.fair-rite.com/product/round-cable-emi-suppression-cores-0431164281/) (round cable) or [0431167281](https://www.fair-rite.com/product/flat-cable-emi-suppression-cores-0431167281/) (flat cable)

### Failure: WiFi/Bluetooth Spurious Emissions

**Typical cause**: RF module harmonics or intermodulation with digital circuits.

**Fixes:**
- Improve RF ground plane (solid copper under antenna)
- Add filtering on RF module power supply (ferrite + 100nF + 10nF)
- Shield RF section from digital noise
- Review antenna placement (keep 10mm+ from noisy circuits)
- Verify matching manufacturer's reference design exactly

## Cost and Timeline

### Testing Costs (Typical Ranges)

| Test Type | Cost Range |
|-----------|------------|
| Pre-compliance equipment (own) | $2,000-6,000 one-time |
| Pre-compliance (lab, per day) | $500-1,500/day |
| FCC Part 15B (Subpart B only) | $2,500-5,000 |
| FCC Part 15C (intentional radiator) | $5,000-15,000 |
| Full certification (complex device) | $10,000-30,000+ |
| TCB review fees | $1,000-3,000 |

### Timeline

| Phase | Duration |
|-------|----------|
| Design for EMC | During development |
| Pre-compliance testing | 1-2 weeks |
| Remediation (if needed) | 2-8 weeks |
| Formal testing | 3-5 days |
| TCB review and grant | 1-2 weeks |
| **Total (best case)** | **4-6 weeks** |
| **Total (with remediation)** | **8-16 weeks** |

### Budget for Failure

First-time pass rates are around 50%. Budget for:
- At least one retest ($2,000-5,000)
- PCB respin ($5,000-20,000 depending on complexity)
- Mechanical modifications (shielding, gaskets)
- Schedule slip (weeks to months)

The cheapest fix is always design-stage mitigation.

## Pre-Submission Checklist

Use this checklist before sending your robot to the test lab.

### Design Review

- [ ] 4-layer (minimum) PCB with solid ground and power planes
- [ ] No signals routed over split planes
- [ ] High-speed signals routed on inner layers
- [ ] Decoupling: 100nF per power pin + bulk capacitors per rail
- [ ] Input filtering on all power rails (Pi or LC filter)
- [ ] Output filtering on motor drivers (LC filter)
- [ ] Common-mode chokes on all cable connections
- [ ] Motor cables filtered and/or shielded
- [ ] Clock signals have series termination or spread spectrum
- [ ] Enclosure provides continuous shielding (no gaps > 15mm)
- [ ] Cable penetrations have filtered connectors or ferrites

### Pre-Compliance Testing

- [ ] Radiated emissions scanned 30 MHz - 1 GHz
- [ ] Conducted emissions scanned 150 kHz - 30 MHz (if AC powered)
- [ ] Dominant emission frequencies identified
- [ ] Emission sources located with near-field probes
- [ ] Margin to limits: at least 6 dB after uncertainty adjustment
- [ ] All operating modes tested

### Documentation Prepared

- [ ] Block diagram showing all major subsystems
- [ ] Circuit schematic (for lab reference)
- [ ] PCB layout files (in case of board respin)
- [ ] Theory of operation (one page summary)
- [ ] User manual (draft acceptable)
- [ ] Photos of DUT (all sides, internal)
- [ ] List of all operating modes and configurations

### Test Samples Ready

- [ ] Production-representative units (final PCB revision)
- [ ] Final enclosure (not prototype)
- [ ] Final power supply
- [ ] All cables that ship with product
- [ ] Peripherals needed for operation
- [ ] 2-3 units (testing can damage samples)
- [ ] Spare parts kit (fuses, connectors)
- [ ] Fix kit (ferrites, copper tape, capacitors)

### For Wireless Devices (Additional)

- [ ] Module FCC ID and grant documentation obtained
- [ ] Antenna specifications documented
- [ ] Integration follows module manufacturer guidelines exactly
- [ ] Module FCC ID will appear on product label or in manual
- [ ] Maximum EIRP calculated and within limits

## Conclusion

FCC Class B certification is a hurdle, but a surmountable one. The keys:

1. **Design for EMC from the start** - Retrofitting EMC fixes is expensive and often ineffective.

2. **Understand your EMI sources** - Motors, switching power supplies, and high-speed digital are the usual suspects in robots.

3. **Do pre-compliance testing** - Catch problems when they're cheap to fix. Budget $2,000-6,000 for basic equipment.

4. **Use pre-certified wireless modules** - Don't reinvent RF compliance unless you have to.

5. **Budget for failure** - First-pass success rates are only 50%. Plan for at least one iteration.

The US market requires FCC compliance. The investment in EMC design pays dividends not just in certification, but in product reliability and customer satisfaction. A robot that doesn't interfere with WiFi is a robot that doesn't generate support calls.

Get it right, and you ship product. Get it wrong, and you're stuck in the lab while competitors reach customers first.

## References and Resources

### FCC Regulations

- [47 CFR Part 15 - Radio Frequency Devices](https://www.ecfr.gov/current/title-47/chapter-I/subchapter-A/part-15) - Complete Part 15 text
- [47 CFR 15.107 - Conducted limits](https://www.ecfr.gov/current/title-47/section-15.107) - Conducted emission limits table
- [47 CFR 15.109 - Radiated emission limits](https://www.ecfr.gov/current/title-47/section-15.109) - Radiated emission limits table
- [47 CFR 15.31 - Measurement procedures](https://www.ecfr.gov/current/title-47/section-15.31) - How measurements are made
- [47 CFR 2.906 - SDoC requirements](https://www.ecfr.gov/current/title-47/section-2.906) - Supplier's Declaration of Conformity
- [FCC Equipment Authorization](https://www.fcc.gov/engineering-technology/laboratory-division/general/equipment-authorization) - Overview of certification process
- [FCC Test Firm Search](https://apps.fcc.gov/oetcf/eas/reports/TestFirmSearch.cfm) - Find FCC-recognized test labs
- [FCC TCB List](https://apps.fcc.gov/oetcf/tcb/index.cfm) - Telecommunications Certification Bodies

### Measurement Standards

- [ANSI C63.4](https://webstore.ansi.org/standards/ieee/ansic632014) - Methods of Measurement for Radio-Noise Emissions (US)
- [CISPR 16-1-1](https://webstore.iec.ch/publication/12119) - EMC measuring apparatus - measuring receivers (defines detectors)
- [CISPR 32](https://webstore.iec.ch/publication/6013) - EMC of multimedia equipment - emission requirements
- [ISO/IEC 17025](https://www.iso.org/ISO-IEC-17025-testing-and-calibration-laboratories.html) - Laboratory accreditation standard

### Test Equipment

- [Siglent Spectrum Analyzers](https://siglentna.com/spectrum-analyzers/) - SSA3000X series for pre-compliance
- [Rigol Spectrum Analyzers](https://www.rigolna.com/products/spectrum-analyzers/) - DSA800/RSA5000 series
- [tinySA](https://tinysa.org/wiki/) - Ultra-budget spectrum analyzer
- [Tekbox EMC Products](https://www.tekbox.com/) - LISNs, probes, affordable antennas
- [Com-Power EMC Test Equipment](https://www.com-power.com/) - Professional LISNs, antennas, probes
- [Beehive Electronics](https://beehive-electronics.com/) - Near-field probe sets
- [Aaronia EMC Antennas](https://aaronia.com/antennas/emc-antennas/) - Biconical, log-periodic antennas

### Components

- [Wurth Elektronik EMC Components](https://www.we-online.com/en/components/products/emc_components) - Ferrite beads, CMCs, inductors
- [Fair-Rite Ferrite Products](https://www.fair-rite.com/) - Snap-on cores, cable cores, beads
- [Murata EMI Filters](https://www.murata.com/en-us/products/emc) - Chip ferrites, common-mode chokes
- [Laird Performance Materials](https://www.laird.com/products/electromagnetic-shielding) - Shield cans, gaskets, absorbers
- [Renesas Spread Spectrum Clocks](https://www.renesas.com/us/en/products/clocks-timing/clock-generation/spread-spectrum-clocks) - SSC oscillators for EMI reduction
- [Belden Industrial Cables](https://www.belden.com/products/cable/industrial-cable) - Shielded motor/sensor cables

### Design Resources

- [LearnEMC](https://learnemc.com/) - Free EMC education
- [Academy of EMC](https://www.academyofemc.com/) - Design guidelines
- [Henry Ott Consultants](https://www.hottconsultants.com/) - EMC design fundamentals

### Testing Labs (Examples)

- [TUV Rheinland](https://www.tuv.com/) - Global, multiple US locations
- [Element Materials Technology](https://www.element.com/) - Multiple US locations
- [Intertek](https://www.intertek.com/) - Global testing services
- [MET Laboratories](https://www.metlabs.com/) - US-based, robotics experience
- [UL Solutions](https://www.ul.com/) - Global, extensive consumer electronics experience

### Finding Accredited Labs

- [A2LA Directory](https://customer.a2la.org/index.cfm?event=directory.index) - Search by location and test type
- [FCC Test Firm Search](https://apps.fcc.gov/oetcf/eas/reports/TestFirmSearch.cfm) - FCC-recognized labs only
- [NVLAP Lab Search](https://www.nist.gov/nvlap) - NIST accredited labs
