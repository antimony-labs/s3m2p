# MCAD User Manual

**Version 0.1.0**

A parametric solid modeler for mechanical CAD with constraint-based sketching, feature-based modeling, and STEP/STL export.

---

## Table of Contents

1. [Getting Started](#1-getting-started)
2. [Sketching Fundamentals](#2-sketching-fundamentals)
3. [Geometric Constraints](#3-geometric-constraints)
4. [Dimensional Constraints](#4-dimensional-constraints)
5. [Smart Snap System](#5-smart-snap-system)
6. [3D Feature Operations](#6-3d-feature-operations)
7. [Selection & Editing](#7-selection--editing)
8. [Export](#8-export)
9. [Keyboard Shortcuts](#9-keyboard-shortcuts)

---

## 1. Getting Started

### 1.1 Interface Overview

MCAD uses a desktop-optimized interface with no scrolling required. The layout consists of:

| Element | Location | Description |
|---------|----------|-------------|
| **Toolbar** | Top | Mode switching, primitive creation, view controls |
| **Canvas** | Center | 3D viewport and 2D sketch workspace |
| **Status Bar** | Bottom | Current mode, snap feedback, constraint status |

### 1.2 Navigation

MCAD supports standard CAD navigation controls:

| Action | Control | Description |
|--------|---------|-------------|
| **Orbit** | Left-drag | Rotate the view around the model |
| **Pan** | Middle-drag | Move the view parallel to screen |
| **Zoom** | Scroll wheel | Zoom in/out toward cursor |

### 1.3 View Presets

Switch to standard orthographic views using the view buttons or keyboard shortcuts:

| View | Shortcut | Camera Position |
|------|----------|-----------------|
| Front | `1` | Looking along -Z axis |
| Back | - | Looking along +Z axis |
| Top | `7` | Looking along -Y axis |
| Bottom | - | Looking along +Y axis |
| Left | `3` | Looking along +X axis |
| Right | - | Looking along -X axis |
| Isometric | `0` | 45-degree oblique view |

---

## 2. Sketching Fundamentals

### 2.1 Creating a New Sketch

Sketches are 2D workspaces where you define profiles for 3D features.

**To create a sketch:**
1. Click **Sketch** in the toolbar
2. Select a plane (XY, XZ, YZ) or planar face
3. The view automatically orients to the sketch plane
4. Begin drawing geometry

### 2.2 Sketch Plane Selection

Available sketch planes:

| Plane | Description | Keyboard |
|-------|-------------|----------|
| XY | Horizontal (floor) plane | Default |
| XZ | Vertical (front) plane | - |
| YZ | Vertical (side) plane | - |
| Face | Any planar solid face | Click face |

### 2.3 Drawing Tools

#### Line Tool

Creates straight line segments between two points.

**To draw a line:**
1. Press `L` or click the Line tool
2. Click to place the **start point**
3. Move cursor and click to place the **end point**
4. Continue clicking to chain connected segments
5. Press `Escape` to finish

**Automatic behavior:**
- Lines near horizontal/vertical auto-constrain
- Endpoints near existing geometry receive coincident constraints

#### Arc Tool

Creates circular arc segments.

**Three-point arc:**
1. Press `A` or click the Arc tool
2. Click to place the **start point**
3. Click to place the **end point**
4. Move cursor to define the **radius/bulge**
5. Click to complete

#### Rectangle Tool

Creates rectangular profiles from two corner points.

**To draw a rectangle:**
1. Press `R` or click the Rectangle tool
2. Click to place the **first corner**
3. Click to place the **opposite corner**
4. Four lines with H/V constraints are created automatically

#### Circle Tool

Creates circles defined by center and radius.

**To draw a circle:**
1. Press `C` or click the Circle tool
2. Click to place the **center point**
3. Move cursor to define **radius**
4. Click to complete

#### Point Tool

Creates reference points for construction.

**To place a point:**
1. Press `P` or click the Point tool
2. Click to place the point
3. Points snap to existing geometry

### 2.4 Construction Geometry

Construction geometry is used as references during sketching but does not appear in the final profile.

**To toggle construction mode:**
1. Select one or more sketch entities
2. Press `X` to toggle construction status
3. Construction geometry displays as dashed lines

### 2.5 Exiting Sketch Mode

**To exit the sketch:**
- Press `Escape` to cancel and discard
- Click **Finish Sketch** to apply and return to 3D

---

## 3. Geometric Constraints

Geometric constraints define relationships between sketch entities without specifying numeric values.

### 3.1 Horizontal Constraint

Forces a line segment to be parallel to the X-axis.

**Application:**
- Select a line
- Apply Horizontal constraint

**Symbol:** `H` displayed near constrained entity

### 3.2 Vertical Constraint

Forces a line segment to be parallel to the Y-axis.

**Application:**
- Select a line
- Apply Vertical constraint

**Symbol:** `V` displayed near constrained entity

### 3.3 Coincident Constraint

Forces two points to share the same location.

**Application:**
- Select two points (or a point and a line/arc)
- Apply Coincident constraint

**Automatic:** Created when snapping endpoint to existing geometry

### 3.4 Parallel Constraint

Forces two lines to maintain the same angle.

**Application:**
- Select two lines
- Apply Parallel constraint

**Symbol:** `//` displayed between entities

### 3.5 Perpendicular Constraint

Forces two lines to meet at 90 degrees.

**Application:**
- Select two lines
- Apply Perpendicular constraint

**Symbol:** Perpendicular marks displayed at intersection

### 3.6 Tangent Constraint

Forces a line to meet a curve at a smooth junction.

**Application:**
- Select a line and an arc/circle
- Apply Tangent constraint

### 3.7 Concentric Constraint

Forces two circles/arcs to share the same center.

**Application:**
- Select two circles or arcs
- Apply Concentric constraint

### 3.8 Constraint Status

The Status Bar displays the current constraint state:

| Status | Meaning | Action Required |
|--------|---------|-----------------|
| **Under-constrained** | Sketch has free DOF | Add more constraints |
| **Fully constrained** | All geometry fixed | Ready for features |
| **Over-constrained** | Redundant constraints | Remove constraints |

The DOF (Degrees of Freedom) count shows remaining freedom: `DOF: 3`

---

## 4. Dimensional Constraints

Dimensional constraints specify numeric values for geometric relationships.

### 4.1 Distance Constraint

Defines the distance between two points.

**To add distance:**
1. Select two points
2. Press `D` or use Constraint menu
3. Enter the distance value
4. Press Enter to apply

### 4.2 Horizontal Distance

Defines the X-axis distance between two points.

**Application:**
- Select two points
- Apply Horizontal Distance
- Enter value

### 4.3 Vertical Distance

Defines the Y-axis distance between two points.

**Application:**
- Select two points
- Apply Vertical Distance
- Enter value

### 4.4 Angle Constraint

Defines the angle between two lines.

**To constrain angle:**
1. Select two lines
2. Apply Angle constraint
3. Enter angle in degrees
4. Press Enter

### 4.5 Radius Constraint

Defines the radius of a circle or arc.

**To constrain radius:**
1. Select a circle or arc
2. Apply Radius constraint
3. Enter radius value
4. Press Enter

### 4.6 Diameter Constraint

Defines the diameter of a circle.

**To constrain diameter:**
1. Select a circle
2. Apply Diameter constraint
3. Enter diameter value
4. Press Enter

### 4.7 Driving vs. Reference Dimensions

| Type | Behavior |
|------|----------|
| **Driving** | Actively controls geometry (can be edited) |
| **Reference** | Displays measured value (read-only) |

---

## 5. Smart Snap System

MCAD's snap system intelligently detects geometric relationships while sketching.

### 5.1 Snap Priority

Snaps are evaluated in priority order:

| Priority | Snap Type | Visual Indicator |
|----------|-----------|------------------|
| 1 | **Point** | Circle on endpoint |
| 2 | **Midpoint** | Triangle on segment |
| 3 | **Center** | Cross at center |
| 4 | **Intersection** | X at crossing |
| 5 | **Perpendicular** | Perpendicular symbol |
| 6 | **Grid** | Crosshair on grid |

### 5.2 Endpoint Snap

Snaps cursor to existing sketch points.

**Activation:** Move cursor within 15px of a point
**Feedback:** Circle highlight appears on point

### 5.3 Midpoint Snap

Snaps to the midpoint of line segments and arcs.

**Activation:** Move cursor near midpoint
**Feedback:** Triangle symbol appears

### 5.4 Center Snap

Snaps to circle and arc centers.

**Activation:** Move cursor near center point
**Feedback:** Cross marker appears

### 5.5 Intersection Snap

Snaps to line-line intersection points.

**Activation:** Move cursor near intersection
**Feedback:** X marker appears at intersection

### 5.6 Perpendicular Snap

Snaps to the perpendicular foot on a line from current cursor.

**Activation:** Move cursor near perpendicular position
**Feedback:** Perpendicular symbol appears

### 5.7 Grid Snap

Snaps to the visible grid when no other snap is active.

**Grid size:** 50mm default
**Activation:** Always active as fallback

---

## 6. 3D Feature Operations

### 6.1 Extrude

Creates a 3D solid by extruding a 2D sketch profile along its normal.

**To extrude:**
1. Exit sketch mode with completed profile
2. Click **Extrude** button
3. Enter extrusion distance (mm)
4. Click Apply

**Parameters:**
| Parameter | Description | Default |
|-----------|-------------|---------|
| Distance | Extrusion length | 50mm |
| Direction | Positive/Negative normal | Positive |

### 6.2 Revolve

Creates a 3D solid by revolving a 2D profile around an axis.

**To revolve:**
1. Create sketch with profile and axis line
2. Mark axis as construction geometry
3. Click **Revolve** button
4. Enter angle (degrees)
5. Click Apply

**Parameters:**
| Parameter | Description | Default |
|-----------|-------------|---------|
| Axis | Revolution center line | Auto-detect |
| Angle | Sweep angle | 360 degrees |

### 6.3 Boolean Operations

Combine solids using Boolean operations:

| Operation | Result |
|-----------|--------|
| **Union** | Combines volumes |
| **Difference** | Subtracts second from first |
| **Intersection** | Keeps only overlapping volume |

**To apply Boolean:**
1. Create two overlapping primitives/features
2. Select operation type
3. Result replaces both operands

### 6.4 Primitives

Create standard shapes directly:

| Primitive | Parameters |
|-----------|------------|
| **Box** | Width, Height, Depth |
| **Cylinder** | Radius, Height |
| **Sphere** | Radius |
| **Cone** | Base Radius, Height |

### 6.5 Patterns

#### Linear Pattern

Copies a feature along a linear path.

**Parameters:**
- Direction vector
- Count
- Spacing

#### Circular Pattern

Copies a feature around a central axis.

**Parameters:**
- Axis
- Count
- Total angle

---

## 7. Selection & Editing

### 7.1 Selection Modes

In 3D view, selection modes determine what geometry can be picked:

| Mode | Selects | Use Case |
|------|---------|----------|
| **Face** | Solid faces | Sketch plane, Boolean |
| **Edge** | Solid edges | Fillet, chamfer |
| **Vertex** | Solid vertices | Measurements |

Press `Tab` to cycle through selection modes.

### 7.2 Multi-Selection

Select multiple entities:
- **Add to selection:** Ctrl+Click
- **Remove from selection:** Ctrl+Click again
- **Clear selection:** Click empty space

### 7.3 Undo/Redo

MCAD maintains a command history:

| Action | Shortcut |
|--------|----------|
| **Undo** | `Ctrl+Z` |
| **Redo** | `Ctrl+Y` |

History stores up to 100 operations.

### 7.4 Delete Geometry

**In sketch mode:**
1. Select entity/entities
2. Press `Delete` or `Backspace`

**In 3D mode:**
- Features cannot be deleted individually
- Use Undo to revert operations

---

## 8. Export

### 8.1 STEP Format (ISO 10303-21)

Industry-standard format for CAD data exchange.

**To export STEP:**
1. Ensure solid is valid (manifold check)
2. Click **Export STEP** button
3. File downloads as `model.step`

**Compatibility:** SolidWorks, NX, Fusion 360, FreeCAD

### 8.2 STL Format

Triangulated mesh format for 3D printing.

**To export STL:**
1. Click **Export STL** button
2. File downloads as `model.stl`

**Note:** STL is unitless; MCAD exports in millimeters

---

## 9. Keyboard Shortcuts

### Global Shortcuts

| Key | Action |
|-----|--------|
| `Escape` | Cancel current operation / Exit sketch |
| `Ctrl+Z` | Undo |
| `Ctrl+Y` | Redo |
| `Delete` | Delete selected geometry |
| `Tab` | Cycle selection mode |

### View Shortcuts

| Key | Action |
|-----|--------|
| `1` | Front view |
| `3` | Right view |
| `7` | Top view |
| `0` | Isometric view |
| `5` | Toggle orthographic/perspective |

### Sketch Tool Shortcuts

| Key | Tool |
|-----|------|
| `L` | Line |
| `A` | Arc |
| `R` | Rectangle |
| `C` | Circle |
| `P` | Point |
| `X` | Toggle construction |
| `S` | Select tool |

### Constraint Shortcuts

| Key | Action |
|-----|--------|
| `H` | Horizontal constraint |
| `V` | Vertical constraint |
| `D` | Distance/Dimension |

---

## Appendix A: Constraint Solver

MCAD uses a Newton-Raphson iterative solver to satisfy sketch constraints.

**Convergence criteria:**
- Maximum iterations: 100
- Tolerance: 1e-6 mm

**Solver status messages:**

| Status | Meaning |
|--------|---------|
| `Solved (N iterations)` | Converged successfully |
| `No constraints` | Nothing to solve |
| `Failed to converge` | System may be over-constrained |

## Appendix B: Supported Units

MCAD works internally in millimeters (mm).

| Measurement | Unit |
|-------------|------|
| Linear | mm |
| Angular | degrees |
| Area | mm^2 |
| Volume | mm^3 |

---

*MCAD is part of the antimony-labs suite at too.foo*
