---
title: "ISO 13482: Safety Requirements for Personal Care Robots"
slug: "iso-13482-personal-care-robots"
date: "2026-01-25"
tags: [robotics, safety, compliance, iso, certification]
summary: "A practical guide to ISO 13482 compliance for personal care robots - from risk assessment to certification."
draft: false
---

*Everything you need to know to design, test, and certify your personal care robot for safety compliance*

**Jump to:** [What is ISO 13482](#what-is-iso-13482) | [Robot Categories](#robot-categories) | [Risk Assessment](#risk-assessment-methodology) | [Safety Requirements](#safety-requirements) | [Protective Measures](#protective-measures) | [Testing](#testing-and-verification) | [Certification](#certification-paths) | [Common Hazards](#common-hazards-in-personal-care-robots) | [Documentation](#documentation-requirements) | [Checklist](#pre-certification-checklist)

## Introduction

You've built a robot that works alongside people in their homes. It moves autonomously, manipulates objects, and operates in unstructured environments where children, elderly, and pets share the space.

Before you can sell it, you need to prove it won't hurt anyone.

[ISO 13482:2014](https://www.iso.org/standard/53820.html) - "Robots and robotic devices - Safety requirements for personal care robots" - is the international standard that defines safety requirements for robots that physically interact with people in personal care applications. Unlike industrial robots that operate in caged environments, personal care robots share space with untrained users in unpredictable settings.

This guide covers the complete ISO 13482 framework: robot classifications, risk assessment methodology, specific safety requirements, protective measures, testing procedures, and the path to certification. Whether you're designing a mobile manipulator, an assistive robot, or a person carrier, this is your roadmap to compliance.

## What is ISO 13482

ISO 13482 is part of the broader ISO robotics safety framework:

<div class="mermaid">
flowchart TB
    ISO[ISO Robotics Safety Standards]
    ISO --> IND[ISO 10218-1/2<br>Industrial Robots]
    ISO --> PCR[ISO 13482<br>Personal Care Robots]
    ISO --> MED[IEC 80601-2-77<br>Medical Robots]
    ISO --> SRV[ISO 23482-1/2<br>Service Robots]
    PCR --> MS[Mobile Servant]
    PCR --> PA[Physical Assistant]
    PCR --> PC[Person Carrier]
</div>

**Key characteristics of ISO 13482:**

- Applies to robots intended for personal care of people (non-medical, non-industrial)
- Covers robots operating in close proximity to humans in unstructured environments
- Addresses physical human-robot interaction and autonomous navigation
- Requires comprehensive risk assessment specific to personal care scenarios
- Published in 2014, with technical reports [ISO/TR 23482-1](https://www.iso.org/standard/71584.html) and [ISO/TR 23482-2](https://www.iso.org/standard/71624.html) providing application guidelines

**What ISO 13482 is NOT:**

- Not for industrial robots (use [ISO 10218](https://www.iso.org/standard/51330.html))
- Not for medical robots (use [IEC 80601-2-77](https://webstore.iec.ch/publication/34586))
- Not for toys (use [EN 71](https://www.en-71.com/) toy safety)
- Not for military or emergency response robots

## Robot Categories

ISO 13482 defines three categories of personal care robots. Your robot may fall into one or more categories.

### Mobile Servant Robot

A robot that moves autonomously to perform tasks for people. This includes:

- Mobile manipulators (robots with arms that move around)
- Fetch and carry robots
- Cleaning robots (floor, window, pool)
- Delivery robots (indoor/outdoor)
- Companion robots that move

**Key safety concerns:**
- Autonomous navigation around people
- Collision avoidance and mitigation
- Manipulation near humans
- Operating in unstructured environments

### Physical Assistant Robot

A robot that provides physical support or augmentation to a person. This includes:

- Exoskeletons
- Walking assist devices
- Limb support robots
- Lifting assist robots

**Key safety concerns:**
- Direct physical attachment to human
- Force and torque limits on human body
- Mechanical failure modes
- User control and override

### Person Carrier Robot

A robot that transports people. This includes:

- Autonomous wheelchairs
- Personal mobility devices
- People movers

**Key safety concerns:**
- Stability and tip-over prevention
- Emergency stop and egress
- Speed limits and collision protection
- User restraint systems

### Multi-Category Robots

Many robots span categories. A mobile manipulator that can also carry a person touches all three. When this happens, you must meet the requirements of ALL applicable categories.

## Risk Assessment Methodology

Risk assessment is the foundation of ISO 13482 compliance. You cannot skip this - the entire safety design flows from it.

### The Risk Assessment Process

<div class="mermaid">
flowchart LR
    ID[Identify Hazards] --> EST[Estimate Risk]
    EST --> EVAL[Evaluate Risk]
    EVAL --> RED[Reduce Risk]
    RED --> DOC[Document]
    DOC --> ID
</div>

This is an iterative process. Every design change triggers re-evaluation.

### Step 1: Identify Hazards

Systematically identify all hazards associated with your robot. ISO 13482 Annex A provides a comprehensive hazard checklist, organized by:

**Mechanical hazards:**
- Crushing (body parts caught between robot and environment)
- Shearing (body parts caught in moving joints)
- Cutting/severing (sharp edges, pinch points)
- Entanglement (hair, clothing, cables caught in moving parts)
- Impact (robot collides with person)
- Stabbing/puncture (pointed elements)
- Tripping (over robot or cables)

**Electrical hazards:**
- Electric shock (exposed conductors, fault conditions)
- Electrostatic discharge
- Battery thermal events

**Thermal hazards:**
- Burns from hot surfaces (motors, electronics)
- Cold contact (cryogenic systems, if any)

**Noise and vibration:**
- Hearing damage from prolonged exposure
- Startle responses leading to secondary injuries

**Radiation hazards:**
- Laser (LiDAR, depth sensors)
- RF emissions (WiFi, Bluetooth)
- UV (if used for disinfection)

**Material hazards:**
- Contact with hazardous materials (lubricants, battery chemicals)
- Allergenic materials in contact surfaces

**Ergonomic hazards:**
- Incorrect posture during interaction
- Repetitive strain
- Psychological stress (fear, anxiety around robot)

**Environmental hazards:**
- Robot operating on stairs, ramps, uneven surfaces
- Outdoor operation (weather, lighting)
- Obstacles, clutter, pets, children

### Step 2: Estimate Risk

For each hazard, estimate the risk using:

**Risk = Severity x Probability of Occurrence**

**Severity levels (S):**

| Level | Description | Examples |
|-------|-------------|----------|
| S1 | Slight injury | Bruise, minor scratch |
| S2 | Moderate injury | Cut requiring stitches, sprain |
| S3 | Serious injury | Fracture, concussion, deep laceration |
| S4 | Severe/fatal | Amputation, permanent disability, death |

**Probability components:**

Probability considers:
- **Exposure frequency** - How often is someone near the hazard?
- **Probability of hazardous event** - Given exposure, how likely is the event?
- **Possibility of avoidance** - Can the person avoid or limit harm?

Use a risk matrix to combine these:

| | S1 (Slight) | S2 (Moderate) | S3 (Serious) | S4 (Severe) |
|---|-------------|---------------|--------------|-------------|
| **High probability** | Medium | High | Very High | Very High |
| **Medium probability** | Low | Medium | High | Very High |
| **Low probability** | Negligible | Low | Medium | High |
| **Very low probability** | Negligible | Negligible | Low | Medium |

### Step 3: Evaluate Risk

Determine if each risk is acceptable:

- **Negligible/Low**: Acceptable as-is
- **Medium**: Acceptable with additional protective measures
- **High/Very High**: Not acceptable - must redesign or add safeguards

Document the rationale for each decision.

### Step 4: Reduce Risk

Apply risk reduction in this order (the "3-step method" from [ISO 12100](https://www.iso.org/standard/51528.html)):

1. **Inherently safe design** - Eliminate the hazard or reduce risk through design choices
2. **Safeguarding and protective measures** - Add guards, sensors, limits
3. **Information for use** - Warnings, instructions, training

Higher-priority measures are always preferred. You cannot substitute warnings for guards, or guards for inherently safe design.

### Step 5: Document

Document everything:
- Hazard identification methodology
- Risk estimation for each hazard
- Risk evaluation decisions
- Risk reduction measures applied
- Residual risk assessment
- Verification of effectiveness

This documentation is required for certification and liability protection.

## Safety Requirements

ISO 13482 specifies safety requirements organized by robot function. Here are the key requirements with practical implementation guidance.

### General Safety Requirements (All Categories)

#### Emergency Stop

**Requirement:** Robot shall have an emergency stop function that:
- Immediately stops all hazardous motion
- Does not create additional hazards when activated
- Requires deliberate action to reset
- Is accessible to the user

**Implementation:**
```
Emergency Stop Categories (per ISO 13850):
- Category 0: Immediate power removal to actuators
- Category 1: Controlled stop, then power removal
- Category 2: Controlled stop, power maintained for braking
```

For personal care robots, Category 1 is typically preferred - it allows controlled deceleration before power removal, reducing tip-over and impact risks.

**Specific requirements:**
- E-stop button within reach of user (if applicable)
- Wireless/remote E-stop for autonomous operation
- E-stop activation time < 500ms for most applications
- Visual/audible indication of E-stop state

**Component recommendations:**
- [IDEC HW1B-V4F02-R](https://us.idec.com/idec-us/en/USD/Operator-Interfaces/Emergency-Stop-Switches) - Panel mount E-stop
- [Banner Engineering](https://www.bannerengineering.com/us/en/products/safety/safety-controllers.html) - Safety controllers
- [SICK](https://www.sick.com/us/en/safety/safety-controllers/c/g285051) - Safety controllers and E-stop integration

#### Protective Stop

**Requirement:** Robot shall have protective stop function that:
- Stops motion when safety-related input is triggered (proximity sensor, force limit)
- Can be automatic (no button press required)
- Resumes operation when condition clears (if safe to do so)

**Implementation:**
- Speed/separation monitoring triggers protective stop when person detected
- Force/torque limits trigger protective stop when contact detected
- Safe limited speed mode for operation near people

#### Safe State

**Requirement:** Robot shall enter a safe state when:
- Emergency stop activated
- Safety system failure detected
- Power loss occurs
- Communication loss (for networked robots)

**Implementation:**
- Brakes engage on all joints (fail-safe brakes that engage on power loss)
- No uncontrolled motion
- Robot remains stable (no tip-over)
- Safe state maintained until deliberate reset

#### Speed and Force Limits

**Requirement:** Robot shall limit speed and force to values that do not cause injury during contact with humans.

**Reference values from ISO/TS 15066** (collaborative robots - applicable to personal care):

| Body Region | Maximum Pressure (N/cm²) | Maximum Force (N) |
|-------------|--------------------------|-------------------|
| Skull/forehead | 130 | 130 |
| Face | 65 | 65 |
| Neck (front) | 35 | 35 |
| Chest/abdomen | 110 | 110 |
| Upper arm | 150 | 150 |
| Hand/finger | 180 | 140 |
| Thigh/knee | 220 | 220 |

These are quasi-static (clamping) limits. Transient (impact) limits are 2x higher.

**Implementation:**
- Force/torque sensors on end effector and/or joints
- Compliant actuators (series elastic, quasi-direct drive)
- Soft covers on contact surfaces
- Speed limiting based on proximity to humans

**Component recommendations:**
- [ATI Force/Torque Sensors](https://www.ati-ia.com/products/ft/ft_models.aspx) - Industrial grade
- [OnRobot HEX](https://onrobot.com/en/products/hex-6-axis-force-torque-sensor) - Collaborative robot F/T
- [Robotiq FT 300](https://robotiq.com/products/ft-300-force-torque-sensor) - Affordable option
- [Hebi Series Elastic Actuators](https://www.hebirobotics.com/) - Compliant joints

### Mobile Servant Robot Requirements

#### Autonomous Navigation Safety

**Requirement:** Robot shall detect obstacles and either stop or navigate around them safely.

**Implementation:**

**Obstacle detection:**
- Minimum 2 independent sensing modalities (e.g., LiDAR + depth camera)
- Detection range appropriate for stopping distance at maximum speed
- Coverage of all directions robot can move (front, back, sides for omnidirectional)

**Stopping distance calculation:**
```
d_stop = v × t_reaction + v² / (2 × a_decel)

Where:
- v = robot velocity
- t_reaction = perception + processing delay (typ. 100-300ms)
- a_decel = deceleration rate (limited by friction, stability)

Example at 1 m/s, 200ms reaction, 2 m/s² decel:
d_stop = 1.0 × 0.2 + 1.0² / (2 × 2.0) = 0.2 + 0.25 = 0.45m
```

Add safety margin: detection range should be > 2× stopping distance.

**Speed limiting:**
- Maximum speed in presence of detected people
- Speed limits based on environment (cluttered vs. open)
- Speed reduction near obstacles

**Sensor recommendations:**
- [SICK TiM](https://www.sick.com/us/en/lidar-sensors/2d-lidar-sensors/tim/c/g292755) - 2D LiDAR for navigation
- [Ouster OS1](https://ouster.com/products/hardware/os1-lidar-sensor) - 3D LiDAR
- [Intel RealSense D455](https://www.intelrealsense.com/depth-camera-d455/) - Depth camera
- [Stereolabs ZED 2](https://www.stereolabs.com/zed-2/) - Stereo depth camera

#### Collision Detection and Response

**Requirement:** Robot shall detect collisions and respond appropriately.

**Implementation:**
- Motor current monitoring for unexpected resistance
- Accelerometer/IMU for impact detection
- Bumper switches or force-sensing skin
- Immediate stop or reversal on collision detection

**Response hierarchy:**
1. Immediate motor stop (within 100ms of detection)
2. Slight reversal if safe (to release any clamping)
3. Transition to protective stop state
4. Alert user/operator

#### Stability

**Requirement:** Robot shall maintain stability under all operating conditions.

**Analysis required:**
- Static stability (center of gravity within support polygon)
- Dynamic stability during motion (acceleration, deceleration, turning)
- Stability on slopes (specify maximum slope angle)
- Stability when manipulating objects at reach limits
- Stability when contacted by person (push test)

**Testing:**
- Tilt table testing to verify tip-over angles
- Dynamic testing on specified slopes
- Worst-case loading scenarios (max reach, max payload, max speed turn)

### Physical Assistant Robot Requirements

#### Force and Torque Limits on Human Body

**Requirement:** Forces and torques applied to user's body shall not exceed safe limits.

**Implementation:**
- Series elastic elements or backdrivable joints
- Real-time force/torque monitoring at attachment points
- Active compliance control
- Hard mechanical limits as backup to electronic limits

**Attachment point design:**
- Distribute forces over large area (no point loads)
- Allow natural joint movement (don't constrain improperly)
- Quick-release mechanisms for emergency doffing
- Padding at all contact points

#### User Control

**Requirement:** User shall be able to override robot action at any time.

**Implementation:**
- Minimal resistance when user moves against robot
- User-accessible disable switch
- "Zero torque" mode for manual manipulation
- Audible/tactile feedback of robot state

### Person Carrier Robot Requirements

#### Stability During Transport

**Requirement:** Robot shall not tip over with person on board under any operating condition.

**Implementation:**
- Low center of gravity
- Wide wheelbase
- Active stabilization if needed
- Speed limiting on slopes
- Detection and avoidance of stairs/edges

**Testing:**
- Stability testing per [ISO 7176](https://www.iso.org/standard/75002.html) (wheelchairs) as reference
- Static tip-over angle > 15° in all directions
- Dynamic stability under maximum braking/acceleration

#### User Restraint

**Requirement:** If required for safety, robot shall have user restraint system.

**Implementation:**
- Seat belt or harness where tip-over risk exists
- Restraint warning system (alert if not fastened)
- Restraint shall not impede emergency egress

#### Speed Limits

**Requirement:** Speed shall be limited appropriate to environment and user.

**Reference values:**
- Indoor, level: typically 6-10 km/h maximum
- Outdoor, level: typically 10-15 km/h maximum
- Slopes: reduced based on gradient
- Crowded areas: walking speed (4-5 km/h)

## Protective Measures

When inherently safe design isn't sufficient, add protective measures.

### Safeguarding by Design

#### Soft Covers and Padding

Cover all contact surfaces with compliant materials:

**Material recommendations:**
- EVA foam (20-40 Shore A) for general padding
- Silicone rubber for durable contact surfaces
- [PORON](https://www.rogerscorp.com/poron-industrial) urethane foams for impact absorption

**Design guidelines:**
- Minimum 10mm padding on surfaces likely to contact people
- No exposed sharp edges or pinch points
- Rounded corners (minimum radius 5mm)
- Smooth seams (no gaps > 4mm where fingers could enter)

#### Speed and Force Limiting Actuators

Consider actuator technologies that are inherently safer:

| Actuator Type | Advantages | Disadvantages |
|---------------|------------|---------------|
| Series Elastic Actuators (SEA) | Built-in compliance, force sensing | Reduced bandwidth, added complexity |
| Quasi-Direct Drive | Backdrivable, low reflected inertia | Lower torque density |
| Pneumatic | Naturally compliant, low inertia | Requires air supply, less precise |
| Cable-driven | Low inertia at end effector | Complex routing, cable wear |

**Suppliers:**
- [Hebi Robotics](https://www.hebirobotics.com/) - SEA modules
- [Genesis Robotics LiveDrive](https://www.genesis-robotics.com/) - Quasi-direct drive
- [Festo](https://www.festo.com/us/en/e/automation-digitalisation/pneumatic-muscles-id_1244941/) - Pneumatic muscles

### Safety-Related Control Systems

#### Safety Integrity Levels

For safety functions that depend on control systems, determine required Safety Integrity Level (SIL) per [IEC 62061](https://webstore.iec.ch/publication/30293) or Performance Level (PL) per [ISO 13849-1](https://www.iso.org/standard/69883.html).

**Typical requirements for personal care robots:**

| Safety Function | Typical PL/SIL |
|-----------------|----------------|
| Emergency stop | PL d / SIL 2 |
| Speed limiting | PL c-d / SIL 1-2 |
| Force limiting | PL c-d / SIL 1-2 |
| Protective stop | PL c / SIL 1 |
| Safe limited speed | PL c / SIL 1 |

**Implementation:**
- Dual-channel architecture for higher SIL/PL
- Diagnostic coverage through cross-checking
- Safe state on detected failure
- Proof test intervals and component reliability data

**Safety PLC/controllers:**
- [Pilz PNOZ](https://www.pilz.com/en-US/products/controllers) - Safety controllers
- [SICK Flexi Soft](https://www.sick.com/us/en/safety/safety-controllers/flexi-soft/c/g249174) - Modular safety
- [Rockwell GuardLogix](https://www.rockwellautomation.com/en-us/products/hardware/allen-bradley/safety-products/safety-controllers.html) - Safety PLC

#### Sensor Redundancy

For safety-critical sensing, use redundant sensors:

**Approaches:**
- Dual sensors with cross-checking (e.g., two LiDARs, compare readings)
- Diverse sensors (e.g., LiDAR + depth camera - different failure modes)
- Sensor self-diagnostics (detect sensor failure)

**Failure response:**
- If sensors disagree: reduce speed, increase caution
- If sensor fails: enter safe limited mode or stop
- Alert operator to degraded state

### Information for Use

When risks remain after design and safeguarding, provide information:

#### User Manual Requirements

**Content:**
- Intended use and foreseeable misuse
- User capabilities required (physical, cognitive)
- Operating environment limits
- Safety warnings and precautions
- Emergency procedures
- Maintenance requirements
- Residual risks user must be aware of

**Warnings must be:**
- Clear and understandable by intended users
- In appropriate languages for target markets
- Visible during normal operation (not hidden inside covers)

#### Training Requirements

For some personal care robots, user training is required:
- Document minimum training requirements
- Provide training materials
- Consider certification of users for complex robots

## Testing and Verification

Testing verifies that your safety design actually works.

### Required Tests

#### Stability Tests

**Tilt table test:**
- Place robot on tilt platform
- Increase angle until tip-over (or to maximum specified operating slope)
- Test in all directions (forward, back, left, right)
- With maximum payload at maximum reach (worst case)

**Dynamic stability:**
- Maximum braking from top speed
- Maximum acceleration
- Turning at maximum speed
- On specified maximum slope

**Push test:**
- Apply force to robot at various heights
- Force representing typical human push (50-100N)
- Robot shall not tip

#### Collision Tests

**Impact force measurement:**
- Measure impact forces at representative collision speeds
- Compare to biomechanical limits (ISO/TS 15066 or equivalent)
- Test with representative contact geometries

**Test setup:**
- Calibrated force measurement device (load cell or instrumented dummy)
- High-speed video to verify test conditions
- Multiple impact locations (contact surfaces, edges, corners)

#### Emergency Stop Tests

**Verification:**
- Activation time (from button press to motion stop)
- Final state (brakes engaged, no uncontrolled motion)
- No additional hazards created (no tip-over, no pinching)
- Reset requires deliberate action
- Works from any operating state

#### Speed and Force Limit Tests

**Speed limiting:**
- Measure actual maximum speed
- Verify speed is below limits under all conditions
- Test with and without nearby obstacles/people

**Force limiting:**
- Measure forces during contact scenarios
- Verify forces below biomechanical limits
- Test at various speeds and contact geometries

### Test Documentation

Document for each test:
- Test procedure
- Test equipment (calibrated, traceable)
- Test conditions (environment, robot configuration)
- Pass/fail criteria
- Results and evidence (data, photos, video)
- Tester identification and date

## Certification Paths

### Self-Declaration vs. Third-Party Certification

**Self-declaration:**
- Company declares conformity to ISO 13482
- Prepare technical file demonstrating compliance
- Accept full liability
- No external verification

**Third-party certification:**
- Accredited certification body reviews evidence
- Issues certificate of conformity
- Periodic audits
- Shared liability
- Required by some markets/customers

### Finding a Certification Body

Look for bodies accredited to test against ISO 13482:

- [TÜV SÜD](https://www.tuvsud.com/en/industries/manufacturing/robotics) - Global, robotics expertise
- [TÜV Rheinland](https://www.tuv.com/world/en/robot-safety.html) - Global, robotics expertise
- [UL Solutions](https://www.ul.com/services/robotics-testing-and-certification) - Global, strong in North America
- [Bureau Veritas](https://group.bureauveritas.com/) - Global
- [SGS](https://www.sgs.com/) - Global

### Certification Process

1. **Pre-assessment** (optional): Certification body reviews design, identifies gaps
2. **Documentation review**: Review of technical file, risk assessment, test reports
3. **Witness testing**: Certification body observes or conducts tests
4. **Audit**: Review of quality management system, design controls
5. **Certificate issuance**: Upon successful completion
6. **Surveillance**: Periodic audits to maintain certification

### Timeline and Costs

| Phase | Typical Duration | Typical Cost |
|-------|------------------|--------------|
| Risk assessment | 2-4 weeks | Internal |
| Design for safety | Ongoing | Internal |
| Testing (internal) | 4-8 weeks | $10,000-50,000 (equipment, labor) |
| Pre-assessment | 1-2 weeks | $5,000-15,000 |
| Certification audit | 2-4 weeks | $15,000-40,000 |
| Certificate issuance | 2-4 weeks | Included |
| Annual surveillance | 1-2 days | $5,000-10,000/year |

Costs vary significantly based on robot complexity and certification body.

## Common Hazards in Personal Care Robots

### Pinch Points and Entanglement

**Common locations:**
- Between arm links at joints
- Between wheels and body
- In cable routing
- In gripper mechanisms

**Solutions:**
- Minimum 25mm gap (adult fingers) or < 4mm (prevent entry)
- Cable management (internal routing, conduit)
- Guards over exposed mechanisms
- Torque limits on joints

### Tip-Over

**Causes:**
- High center of gravity
- Narrow wheelbase
- High acceleration/deceleration
- Manipulation at reach limits
- External push
- Slopes and uneven surfaces

**Solutions:**
- Design for low CG, wide base
- Speed/acceleration limits
- Payload limits with reach-dependent derating
- Slope detection and speed limiting
- Anti-tip casters or outriggers

### Uncontrolled Motion

**Causes:**
- Software errors
- Communication loss
- Sensor failure
- Actuator failure

**Solutions:**
- Fail-safe brakes (engage on power loss)
- Watchdog timers
- Safe state on communication loss
- Redundant sensors for critical functions
- Motor current limiting

### Collision Injury

**Causes:**
- Navigation errors
- Sensor blind spots
- Unexpected obstacles
- Person moving into robot path

**Solutions:**
- Soft covers on contact surfaces
- Speed limiting in presence of people
- Force limiting through compliant actuators
- Multiple sensing modalities
- Collision detection and response

### Battery Hazards

**Risks:**
- Thermal runaway (fire)
- Electric shock
- Chemical exposure (if breached)

**Solutions:**
- Cell-level protection (BMS with temperature, voltage, current monitoring)
- Physical protection of battery pack
- Fusing and isolation
- UL 2271 or equivalent battery certification
- Thermal management

## Documentation Requirements

Documentation is where most companies underestimate the effort. A complete technical file for ISO 13482 typically runs 500-2000+ pages. This section provides exact requirements, templates, and example language.

### Technical File Structure

Organize your technical file with this structure:

```
Technical File: [Product Name] [Model Number]
├── 00_Cover_and_Index/
│   ├── TF-000 Technical File Cover Sheet
│   ├── TF-001 Document Index and Revision Log
│   └── TF-002 Applicable Standards Matrix
├── 01_General_Description/
│   ├── TF-010 Product Description and Intended Use
│   ├── TF-011 System Architecture Overview
│   ├── TF-012 Specifications and Performance Data
│   └── TF-013 Photos and General Arrangement Drawings
├── 02_Risk_Assessment/
│   ├── TF-020 Risk Assessment Procedure
│   ├── TF-021 Hazard Identification Worksheets
│   ├── TF-022 Risk Estimation and Evaluation
│   ├── TF-023 Risk Reduction Measures
│   └── TF-024 Residual Risk Summary
├── 03_Design_Documentation/
│   ├── TF-030 Mechanical Design Package
│   ├── TF-031 Electrical Design Package
│   ├── TF-032 Software Design Package
│   ├── TF-033 Safety System Design
│   └── TF-034 Bill of Materials
├── 04_Test_Reports/
│   ├── TF-040 Test Plan and Procedures
│   ├── TF-041 Stability Test Report
│   ├── TF-042 Collision and Impact Test Report
│   ├── TF-043 Emergency Stop Test Report
│   ├── TF-044 Speed and Force Limit Verification
│   ├── TF-045 Electrical Safety Test Report
│   ├── TF-046 EMC Test Report
│   └── TF-047 Functional Safety Verification
├── 05_User_Documentation/
│   ├── TF-050 User Manual
│   ├── TF-051 Quick Start Guide
│   ├── TF-052 Maintenance Instructions
│   └── TF-053 Training Materials
├── 06_Quality_and_Manufacturing/
│   ├── TF-060 Quality Management System Summary
│   ├── TF-061 Manufacturing Process Controls
│   └── TF-062 Production Test Procedures
└── 07_Declarations/
    ├── TF-070 Declaration of Conformity
    └── TF-071 Supporting Certificates
```

### TF-010: Product Description and Intended Use

This document defines what your robot is and how it should be used. Certification bodies scrutinize this carefully.

**Required content:**

```
PRODUCT DESCRIPTION AND INTENDED USE
Document: TF-010 | Rev: A | Date: 2026-01-25

1. PRODUCT IDENTIFICATION
   Product Name: [Your Robot Name]
   Model Number: [Model]
   Manufacturer: [Company Name]
   Address: [Full Address]

2. PRODUCT DESCRIPTION
   [Your Robot] is a mobile servant robot intended to assist users
   with activities of daily living in residential environments. The
   robot is capable of autonomous navigation, object manipulation,
   and human-robot interaction through voice and touch interfaces.

   Physical characteristics:
   - Dimensions: [L x W x H] mm
   - Weight: [X] kg (without payload)
   - Maximum payload: [X] kg
   - Maximum speed: [X] m/s
   - Battery: [Type], [Capacity] Wh
   - Operating time: [X] hours typical

3. INTENDED USE
   3.1 Intended Users
       - Adults (18+ years) in residential settings
       - Users shall be capable of understanding and following
         safety instructions in the user manual
       - No specific training required for basic operation
       - [Specific user populations, if applicable]

   3.2 Intended Environment
       - Indoor residential environments
       - Level floors with maximum slope of [X] degrees
       - Temperature range: [X] to [X] °C
       - Humidity: [X] to [X]% RH non-condensing
       - Lighting conditions: [X] to [X] lux

   3.3 Intended Tasks
       - Fetching and carrying household objects up to [X] kg
       - [List specific intended tasks]
       - [Be specific - vague descriptions cause problems]

4. REASONABLY FORESEEABLE MISUSE
   The following uses are foreseeable but not intended. The risk
   assessment addresses hazards arising from these scenarios:
   - Children climbing on or riding the robot
   - Operation on stairs or steep ramps
   - Outdoor operation
   - Carrying loads exceeding specified maximum
   - Attempting to lift persons
   - Operation by persons who have not read the manual
   - Modification of safety systems

5. LIMITATIONS OF USE
   The robot is NOT intended for:
   - Medical or therapeutic applications
   - Transportation of persons
   - Operation in commercial or industrial settings
   - Outdoor use
   - Use by children without adult supervision
   - [Other exclusions]

6. ROBOT CATEGORY PER ISO 13482
   ☒ Mobile Servant Robot (Clause 5.2)
   ☐ Physical Assistant Robot (Clause 5.3)
   ☐ Person Carrier Robot (Clause 5.4)
```

### TF-021: Hazard Identification Worksheet

Use a systematic format for each identified hazard:

```
HAZARD IDENTIFICATION WORKSHEET
Document: TF-021 | Rev: A | Date: 2026-01-25 | Page: 1 of [N]

HAZARD ID: HAZ-001
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Hazard Type: Mechanical - Impact

Hazard Description:
Robot base collides with person's lower legs during autonomous
navigation due to obstacle detection failure or delayed response.

Life Cycle Phase: Normal operation

Hazardous Situation:
Person standing or walking in robot's path while robot is moving
at maximum speed (1.0 m/s). Person may be unaware of robot
approach. Robot sensors fail to detect person or detection
occurs too late to stop before contact.

Hazardous Event:
Robot base (mass 45 kg) impacts person's lower leg (tibia/fibula
region) at up to 1.0 m/s.

Harm:
Bruising, contusion, potential fracture of tibia/fibula in
vulnerable populations (elderly, osteoporosis).

Affected Persons:
- Primary user
- Bystanders (family members, visitors)
- Vulnerable populations (elderly, children)

Applicable Standards/Clauses:
- ISO 13482:2014 Clause 5.2.4 (Collision avoidance)
- ISO/TS 15066:2016 Table A.2 (Biomechanical limits)

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

HAZARD ID: HAZ-002
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Hazard Type: Mechanical - Crushing

Hazard Description:
Person's fingers crushed between robot arm links during arm
movement.

[Continue for each hazard...]
```

### TF-022: Risk Estimation and Evaluation

For each hazard, document the risk estimation:

```
RISK ESTIMATION AND EVALUATION
Document: TF-022 | Rev: A | Date: 2026-01-25

HAZARD: HAZ-001 - Robot base collision with person's legs

INITIAL RISK ESTIMATION (before risk reduction measures)
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Severity of Harm (S):
  S1 ☐ Slight (reversible, minor first aid)
  S2 ☒ Moderate (reversible, medical treatment needed)
  S3 ☐ Serious (irreversible, significant impairment)
  S4 ☐ Severe (life-threatening or fatal)

  Rationale: Impact at 1.0 m/s with 45 kg robot can cause
  contusions requiring medical evaluation. Fracture possible
  but unlikely in healthy adults. S3 possible for vulnerable
  populations but S2 selected as typical case.

Probability of Occurrence of Harm:
  Exposure (E):
    E1 ☐ Rare (less than once per month)
    E2 ☐ Occasional (monthly to weekly)
    E3 ☒ Frequent (daily)
    E4 ☐ Continuous

    Rationale: Robot operates daily in shared living space.
    Users regularly in proximity during operation.

  Probability of Hazardous Event (P):
    P1 ☐ Negligible (unlikely to occur)
    P2 ☒ Low (may occur during robot lifetime)
    P3 ☐ Medium (likely to occur)
    P4 ☐ High (expected to occur)

    Rationale: Sensors provide detection in most cases but
    edge cases exist (transparent obstacles, rapid movement
    into path, sensor degradation).

  Avoidability (A):
    A1 ☐ Likely (ample warning, slow hazard development)
    A2 ☒ Possible (warning present, fast hazard development)
    A3 ☐ Unlikely (no warning or very fast)

    Rationale: Robot produces audible indication while moving.
    User may not always attend to warning. Collision occurs
    rapidly once path intersection occurs.

INITIAL RISK LEVEL: HIGH
(Severity S2 × Probability Medium = High per risk matrix)

RISK EVALUATION:
Is this risk acceptable? NO
Risk reduction required: YES

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

RISK REDUCTION MEASURES APPLIED
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Measure 1: Inherently Safe Design
  Description: Maximum operating speed limited to 0.5 m/s
  when people detected within 2m, 0.3 m/s within 1m.

  Verification: Speed limit test report TF-044, Section 4.2

  Effect on risk: Reduces kinetic energy at impact by 75%.
  Reduces severity from S2 to S1.

Measure 2: Safeguarding
  Description: Dual-sensor obstacle detection (LiDAR + depth
  camera) with safety-rated processing. Protective stop
  triggered when obstacle detected within stopping distance.

  Verification: Functional safety analysis TF-047.
  Performance Level: PL c per ISO 13849-1.

  Effect on risk: Reduces probability of hazardous event
  from P2 to P1.

Measure 3: Safeguarding
  Description: Compliant bumper with contact detection.
  Motor current monitoring detects unexpected resistance.
  Immediate stop and slight reversal on contact detection.

  Verification: Collision test report TF-042.

  Effect on risk: Reduces severity by limiting contact
  duration and force.

Measure 4: Information for Use
  Description: User manual Section 3.2 warns users to
  maintain awareness of robot position. Section 4.1
  instructs on emergency stop usage.

  Effect on risk: Improves avoidability from A2 to A1.

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

RESIDUAL RISK ESTIMATION (after risk reduction measures)
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Severity: S1 (reduced from S2 due to speed limiting)
Probability: Low (E3 × P1 × A1)

RESIDUAL RISK LEVEL: LOW

RISK EVALUATION:
Is residual risk acceptable? YES

Rationale: Residual risk of minor bruising with low probability
is comparable to hazards in everyday life (bumping into
furniture). Further risk reduction not practicable without
significantly degrading robot utility.

Residual risk communicated in: User Manual Section 2.3

Assessed by: [Name] | Date: [Date] | Signature: ___________
Reviewed by: [Name] | Date: [Date] | Signature: ___________
```

### TF-042: Collision and Impact Test Report

Test reports must be detailed and traceable:

```
COLLISION AND IMPACT TEST REPORT
Document: TF-042 | Rev: A | Date: 2026-01-25

1. PURPOSE
   This report documents collision and impact testing performed
   to verify compliance with ISO 13482:2014 Clause 5.2.4 and
   biomechanical limits per ISO/TS 15066:2016.

2. REFERENCE DOCUMENTS
   - ISO 13482:2014 Robots and robotic devices - Safety
     requirements for personal care robots
   - ISO/TS 15066:2016 Robots and robotic devices - Collaborative
     robots
   - TF-040 Test Plan and Procedures, Section 6

3. TEST EQUIPMENT
   ┌─────────────────────────────────────────────────────────────┐
   │ Equipment          │ Manufacturer  │ Model    │ Cal. Due   │
   ├─────────────────────────────────────────────────────────────┤
   │ Force transducer   │ PCB Piezo.    │ 208C03   │ 2026-06-15 │
   │ DAQ system         │ NI            │ USB-6341 │ 2026-08-20 │
   │ High-speed camera  │ Photron       │ SA-X2    │ N/A        │
   │ Pressure film      │ Fujifilm      │ Prescale │ N/A        │
   │ Test dummy (leg)   │ [Mfr]         │ [Model]  │ N/A        │
   └─────────────────────────────────────────────────────────────┘

   Calibration certificates attached as Appendix A.

4. TEST SETUP
   [Diagram showing test arrangement]

   - Robot positioned on level floor
   - Instrumented test dummy (lower leg surrogate) positioned
     in robot path at 0°, 45°, 90° approach angles
   - Force transducer mounted behind contact surface
   - High-speed camera recording at 1000 fps
   - Room temperature: 22°C, Humidity: 45% RH

5. TEST PROCEDURE
   5.1 Robot commanded to move toward dummy at specified speed
   5.2 Impact forces recorded from first contact until stop
   5.3 Peak force, contact duration, and pressure extracted
   5.4 Three repetitions per condition
   5.5 Robot speed varied: 0.3, 0.5, 0.8, 1.0 m/s
   5.6 Payload varied: 0 kg, 2 kg (maximum), at full reach

6. ACCEPTANCE CRITERIA
   Per ISO/TS 15066:2016 Table A.2 for lower leg (tibia):
   - Maximum transient force: 440 N (2× quasi-static limit)
   - Maximum quasi-static force: 220 N
   - Maximum pressure: 220 N/cm²

7. TEST RESULTS

   7.1 Collision at 0.5 m/s (normal operating speed with person)
   ┌─────────────────────────────────────────────────────────────┐
   │ Trial │ Peak Force │ Duration │ Pressure  │ Pass/Fail     │
   │       │ (N)        │ (ms)     │ (N/cm²)   │               │
   ├─────────────────────────────────────────────────────────────┤
   │   1   │    127     │   45     │    42     │ PASS          │
   │   2   │    134     │   48     │    45     │ PASS          │
   │   3   │    131     │   44     │    43     │ PASS          │
   ├─────────────────────────────────────────────────────────────┤
   │ Mean  │    131     │   46     │    43     │               │
   │ Limit │    440     │   N/A    │   220     │               │
   │Margin │    70%     │          │    80%    │               │
   └─────────────────────────────────────────────────────────────┘

   7.2 Collision at 1.0 m/s (maximum speed, no person detected)
   [Similar table - expect higher forces]

   7.3 Collision with maximum payload at reach limit
   [Similar table - worst case condition]

8. OBSERVATIONS
   - Compliant bumper deformation absorbed significant energy
   - Contact detection triggered motor stop within 35ms average
   - Slight reversal (15mm) occurred within 100ms of detection
   - No tip-over observed in any test condition
   - High-speed video confirms clean contact without secondary
     impacts

9. CONCLUSIONS
   All measured impact forces and pressures are below the
   limits specified in ISO/TS 15066:2016 Table A.2 with
   significant margin (>50% in all cases).

   The robot PASSES collision and impact requirements.

10. ATTACHMENTS
    - Appendix A: Calibration certificates
    - Appendix B: Raw data files
    - Appendix C: High-speed video clips (USB drive)
    - Appendix D: Pressure film samples

Tested by: [Name] | Date: [Date] | Signature: _______________
Reviewed by: [Name] | Date: [Date] | Signature: _______________
```

### TF-050: User Manual Requirements

The user manual is a safety document, not marketing material. Required sections per ISO 13482:

```
USER MANUAL STRUCTURE (per ISO 13482 Clause 7)

1. SAFETY INFORMATION (must come first)
   1.1 Safety Symbols Used in This Manual
   1.2 General Safety Precautions
   1.3 Emergency Stop Procedures
   1.4 Warnings and Cautions

2. PRODUCT DESCRIPTION
   2.1 Intended Use
   2.2 User Requirements and Limitations
   2.3 Residual Risks (what hazards remain)
   2.4 Environments Suitable for Use
   2.5 Environments NOT Suitable for Use

3. SPECIFICATIONS
   3.1 Physical Specifications
   3.2 Performance Specifications
   3.3 Environmental Specifications
   3.4 Electrical Specifications

4. SETUP AND INSTALLATION
   4.1 Unpacking
   4.2 Initial Setup
   4.3 Charging
   4.4 WiFi/Network Configuration
   4.5 Environment Preparation

5. OPERATION
   5.1 Starting the Robot
   5.2 Normal Operation
   5.3 Control Interfaces
   5.4 Operating Modes
   5.5 Shutting Down

6. EMERGENCY PROCEDURES
   6.1 Emergency Stop
   6.2 Power Failure Behavior
   6.3 Robot Recovery After Emergency
   6.4 What To Do If Robot Behaves Unexpectedly

7. MAINTENANCE
   7.1 Routine Maintenance Schedule
   7.2 Cleaning
   7.3 Battery Care
   7.4 Software Updates
   7.5 When to Contact Service

8. TROUBLESHOOTING
   8.1 Error Messages and Meanings
   8.2 Common Issues and Solutions
   8.3 Contacting Support

9. TECHNICAL DATA
   9.1 Complete Specifications
   9.2 Compliance Information
   9.3 Warranty Information

10. APPENDICES
    A. Quick Reference Card
    B. Declaration of Conformity
```

**Example safety section language:**

```
1. SAFETY INFORMATION

1.1 SAFETY SYMBOLS

  ⚠️ WARNING: Indicates a hazardous situation which, if not
  avoided, could result in serious injury.

  ⚡ ELECTRICAL HAZARD: Risk of electric shock.

  🔥 FIRE HAZARD: Risk of fire or burn.

1.2 GENERAL SAFETY PRECAUTIONS

  ⚠️ WARNING: Read this entire manual before operating the
  robot. Failure to follow safety instructions may result
  in serious injury.

  • Do not allow children under 12 to operate the robot
    without adult supervision.

  • Do not modify, disable, or bypass any safety features.

  • Do not operate the robot on stairs, ramps steeper than
    [X] degrees, or uneven surfaces.

  • Keep fingers, hair, clothing, and jewelry away from
    moving parts.

  • Stop operation immediately if the robot makes unusual
    sounds or movements.

  • Do not attempt to lift or carry persons with the robot
    arm under any circumstances.

1.3 EMERGENCY STOP

  In case of emergency, press the red EMERGENCY STOP button
  located on [location]. This immediately stops all robot
  motion.

  To resume operation after emergency stop:
  1. Ensure the hazard has been resolved
  2. Release the emergency stop button by rotating clockwise
  3. Press the RESET button on the control panel
  4. Confirm robot status on the display
  5. Resume operation

2.3 RESIDUAL RISKS

  Even with all safety features operating correctly, the
  following risks remain. Users should be aware of these
  hazards:

  • COLLISION: The robot may contact persons during movement.
    Contact forces are limited but minor bruising is possible.
    Maintain awareness of robot position during operation.

  • PINCH POINTS: Fingers may be pinched between arm segments
    if inserted during arm movement. Keep hands clear of arm
    joints during operation.

  • TRIP HAZARD: The robot or its charging cable may present
    a trip hazard. Ensure adequate lighting and keep charging
    cables clear of walking paths.

  • BATTERY: In case of battery damage, do not touch leaking
    material. Ventilate area and contact service.
```

### TF-070: Declaration of Conformity

Required format for self-declaration:

```
DECLARATION OF CONFORMITY

We, the undersigned,

Manufacturer:
  [Company Legal Name]
  [Street Address]
  [City, State/Province, Postal Code]
  [Country]

declare under our sole responsibility that the product:

Product Name:     [Robot Name]
Model Number:     [Model Number]
Serial Numbers:   [From] to [To] (or "All units manufactured
                  from [Date]")

to which this declaration relates, is in conformity with the
following standards and specifications:

Safety:
  ISO 13482:2014    Robots and robotic devices - Safety
                    requirements for personal care robots

  ISO 12100:2010    Safety of machinery - General principles
                    for design - Risk assessment and risk
                    reduction

Electrical Safety:
  IEC 62368-1:2020  Audio/video, information and communication
                    technology equipment - Part 1: Safety
                    requirements

Functional Safety:
  ISO 13849-1:2023  Safety of machinery - Safety-related parts
                    of control systems - Part 1: General
                    principles for design

Electromagnetic Compatibility:
  CISPR 32:2015     Electromagnetic compatibility of multimedia
                    equipment - Emission requirements

  IEC 61000-6-1:2016 Electromagnetic compatibility - Generic
                    standards - Immunity standard for
                    residential, commercial and light-
                    industrial environments

Radio Equipment (if applicable):
  [FCC Part 15 / ETSI EN 300 328 / etc.]

The technical documentation required by these standards is
maintained at:

  [Company Name]
  [Address where technical file is kept]

and is available for inspection by relevant authorities.

This declaration is issued under the sole responsibility of
the manufacturer.

Signed for and on behalf of: [Company Name]

_________________________________
[Name of Authorized Person]
[Title]
[Place], [Date]
```

### Supporting Standards Compliance

Your technical file must demonstrate compliance with supporting standards:

**Electrical safety (IEC 62368-1 or IEC 60204-1):**
- Insulation and dielectric strength test reports
- Grounding continuity measurements
- Overcurrent protection verification
- Temperature rise tests
- Marking and labeling review

**Functional safety (ISO 13849-1):**
- Safety function list with required PL
- Block diagrams of safety-related circuits
- MTTFD, DC, CCF calculations
- PL achieved for each safety function
- Validation test reports

**EMC ([CISPR 32](https://webstore.iec.ch/publication/6013), [IEC 61000-6-1](https://webstore.iec.ch/publication/4233)):**
- Radiated emissions test report
- Conducted emissions test report (if AC powered)
- Immunity test reports (ESD, RF, surge, etc.)
- See [FCC Class B guide](/post/fcc-class-b-robotics-guide) for details

**Wireless (if applicable):**
- FCC/CE certification for radio modules
- Antenna specifications
- RF exposure assessment (if applicable)

### Document Control Requirements

Maintain document control per [ISO 9001](https://www.iso.org/standard/62085.html) or equivalent:

**Revision control:**
```
REVISION HISTORY

Rev │ Date       │ Description              │ Author  │ Approved
────┼────────────┼──────────────────────────┼─────────┼──────────
A   │ 2026-01-15 │ Initial release          │ [Name]  │ [Name]
B   │ 2026-02-20 │ Updated speed limits     │ [Name]  │ [Name]
C   │ 2026-03-10 │ Added bumper test data   │ [Name]  │ [Name]
```

**Change assessment:**
For any design change, document:
1. Description of change
2. Affected hazards (reference HAZ-XXX numbers)
3. Impact on risk assessment
4. Re-verification required (Y/N)
5. Documents requiring update

### Document Retention

Retain technical file for:
- **Minimum 10 years** after last unit manufactured (EU requirement)
- **Product lifetime plus 10 years** (recommended for liability protection)
- **Indefinitely** for critical safety documentation

Store in:
- Secure, climate-controlled location
- Backup copies at separate location
- Electronic copies with verified integrity (checksums)

### Effort Estimate

Realistic effort for complete documentation:

| Document Set | First Product | Updates |
|--------------|---------------|---------|
| Risk assessment (complete) | 80-200 hours | 20-40 hours |
| Design documentation | 40-80 hours | As-needed |
| Test reports | 60-120 hours | Re-test scope |
| User manual | 40-80 hours | 8-16 hours |
| Declaration and admin | 8-16 hours | 4-8 hours |
| **Total** | **230-500 hours** | **Varies** |

This is engineering time, not calendar time. Plan for 3-6 months of documentation effort alongside design work.

### Maintaining Documentation

**Version control:**
- Track all document revisions with unique identifiers
- Link documents to specific product versions/serial numbers
- Archive superseded versions (never delete)
- Use document management system for traceability

**Change management:**
- Assess safety impact of ALL design changes before implementation
- Update risk assessment for any change affecting hazards
- Re-verify affected safety functions
- Update technical file before shipping changed product
- Maintain change log linking changes to document updates

## Pre-Certification Checklist

Use this checklist before pursuing certification.

### Risk Assessment Complete

- [ ] All hazards identified (mechanical, electrical, thermal, etc.)
- [ ] Risk estimation completed for each hazard
- [ ] Risk evaluation documented with acceptance rationale
- [ ] Risk reduction measures implemented and verified
- [ ] Residual risks documented and communicated

### Safety Functions Verified

- [ ] Emergency stop tested and documented
- [ ] Protective stop functions verified
- [ ] Speed limits verified under all conditions
- [ ] Force limits verified against biomechanical limits
- [ ] Safe state verified on power loss, E-stop, failures

### Physical Safety Verified

- [ ] Stability tested (static and dynamic)
- [ ] No accessible pinch points > 4mm and < 25mm
- [ ] No sharp edges or corners
- [ ] Soft covers on contact surfaces
- [ ] Collision detection and response verified

### Electrical Safety Verified

- [ ] Insulation and grounding verified
- [ ] Overcurrent protection tested
- [ ] Battery safety verified (BMS, thermal, physical protection)
- [ ] EMC compliance verified

### Functional Safety Verified

- [ ] Safety functions achieve required PL/SIL
- [ ] Dual-channel where required
- [ ] Diagnostic coverage verified
- [ ] Proof test intervals defined

### Documentation Complete

- [ ] Technical file assembled
- [ ] Risk assessment documented
- [ ] Test reports available
- [ ] User manual prepared
- [ ] Training requirements defined (if applicable)

### Software Safety

- [ ] Safety-related software identified
- [ ] Software developed per IEC 62443 or equivalent
- [ ] Software verification and validation complete
- [ ] Software change management in place

## Conclusion

ISO 13482 compliance is comprehensive but achievable. The keys:

1. **Start with risk assessment** - This drives all safety design decisions. Do it early and iterate throughout development.

2. **Design for inherent safety** - Eliminate hazards through design before adding safeguards. Low forces, compliant actuators, and stable geometry are better than complex safety systems.

3. **Layer protective measures** - Use redundant sensors, dual-channel safety circuits, and defense in depth.

4. **Test thoroughly** - Verify safety functions work under realistic and worst-case conditions. Document everything.

5. **Maintain compliance** - Design changes trigger re-assessment. Keep documentation current.

Personal care robots share space with vulnerable people in unpredictable environments. The investment in safety engineering protects users and protects your company from liability.

Get it right, and you build trust with users. Get it wrong, and a single incident can end your company.

## References and Resources

### ISO Standards

- [ISO 13482:2014](https://www.iso.org/standard/53820.html) - Safety requirements for personal care robots
- [ISO/TR 23482-1:2020](https://www.iso.org/standard/71584.html) - Application of ISO 13482, Part 1
- [ISO/TR 23482-2:2019](https://www.iso.org/standard/71624.html) - Application of ISO 13482, Part 2
- [ISO 12100:2010](https://www.iso.org/standard/51528.html) - General principles for risk assessment
- [ISO 13849-1:2023](https://www.iso.org/standard/69883.html) - Safety-related parts of control systems
- [ISO 10218-1/2](https://www.iso.org/standard/51330.html) - Industrial robot safety (reference)
- [ISO/TS 15066:2016](https://www.iso.org/standard/62996.html) - Collaborative robots (force limits reference)

### IEC Standards

- [IEC 62061](https://webstore.iec.ch/publication/30293) - Functional safety of machinery control systems
- [IEC 60204-1](https://webstore.iec.ch/publication/1016) - Electrical equipment of machines
- [IEC 62368-1](https://webstore.iec.ch/publication/59640) - Audio/video and IT equipment safety
- [IEC 61508](https://webstore.iec.ch/publication/22273) - Functional safety of E/E/PE systems

### Design Resources

- [RIA Robotics Safety Standards](https://www.robotics.org/robotic-content/safety-standards) - Standards overview
- [OSHA Technical Manual on Robotics](https://www.osha.gov/otm/section-4-safety-hazards/chapter-4) - Safety guidance
- [EU Machinery Directive 2006/42/EC](https://eur-lex.europa.eu/legal-content/EN/TXT/?uri=CELEX%3A32006L0042) - European requirements

### Safety Component Suppliers

- [Pilz](https://www.pilz.com/) - Safety controllers, sensors, services
- [SICK](https://www.sick.com/) - Safety sensors, controllers
- [Banner Engineering](https://www.bannerengineering.com/) - Safety sensors
- [ATI Industrial Automation](https://www.ati-ia.com/) - Force/torque sensors
- [Schunk](https://schunk.com/) - Grippers with integrated safety

### Testing and Certification

- [TÜV SÜD Product Service](https://www.tuvsud.com/en/industries/manufacturing/robotics)
- [TÜV Rheinland Robot Safety](https://www.tuv.com/world/en/robot-safety.html)
- [UL Solutions Robotics](https://www.ul.com/services/robotics-testing-and-certification)
- [Intertek](https://www.intertek.com/electrical/product-testing/) - Electrical and product safety testing
