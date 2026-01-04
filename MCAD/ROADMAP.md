# MCAD Roadmap

Professional CAD platform comparable to Siemens NX and Onshape.

## Completed

### Phase 1: DNA Foundation
- `DNA/src/cad/sketch.rs` - Sketch data structures, SketchPlane, Point2, entities
- `DNA/src/cad/constraints.rs` - Geometric & dimensional constraints with evaluate/gradient
- `DNA/src/cad/solver.rs` - Newton-Raphson constraint solver
- `DNA/src/cad/extrude.rs` - Sketch → Solid extrusion with B-Rep topology

### Phase 2: Basic UI
- Mode toggle (3D View ↔ Sketch2D)
- Drawing tools: Line, Circle, Point
- 2D renderer with grid
- Extrude button
- STEP/STL export

### Phase 3: Interactive Sketching
- Entity selection (click to select, yellow highlight)
- Constraint application (H/V/⊥ buttons)
- Solver integration ("Solve Constraints" button)
- Snapping (50mm grid + existing points)
- Constraint visualization (H/V icons)
- Rubber-band preview while drawing
- Keyboard shortcuts (L, C, P, S, Esc, Delete)
- Pan/zoom in sketch mode

---

## Phase 4: Professional Features

### 4.1 Arc Tool (3-Point Arc)

**Files:** `DNA/src/cad/sketch.rs`, `MCAD/src/lib.rs`

```rust
SketchEntity::Arc {
    id: SketchEntityId,
    start: SketchPointId,
    end: SketchPointId,
    center: SketchPointId,
    radius: f32,
}
```

**Workflow:**
1. Click start point
2. Click midpoint (on arc)
3. Click end point
4. Calculate center using circle-through-3-points formula

**Implementation:**
- Add `SketchTool::Arc` handling in `handle_sketch_click`
- Use 3 temp_points to collect clicks
- Compute center from 3 points: solve circumcenter equations
- Draw arc in `draw_sketch_2d` using canvas `arc()` with start/end angles

### 4.2 Rectangle Tool

**Files:** `MCAD/src/lib.rs`

Sugar for 4 lines + coincident constraints:
1. Click corner 1
2. Click corner 2 (opposite diagonal)
3. Generate 4 lines forming rectangle
4. Auto-add horizontal/vertical constraints

### 4.3 Revolve Operation

**Files:** `DNA/src/cad/revolve.rs` (new), `MCAD/src/lib.rs`

```rust
pub struct RevolveParams {
    pub angle: f32,       // Degrees (360 for full revolution)
    pub axis: RevolveAxis,
    pub segments: u32,    // Tessellation for curved surfaces
}

pub enum RevolveAxis {
    X,
    Y,
    Custom { point: Point2, direction: Vector2 },
}

pub fn revolve_sketch(sketch: &Sketch, params: &RevolveParams) -> Result<Solid, RevolveError>
```

**Algorithm:**
1. Find closed profile in sketch
2. For each profile point, rotate around axis at angular increments
3. Generate vertices at each rotation step
4. Create faces connecting adjacent rotation slices
5. Cap ends if angle < 360°

### 4.4 Linear Pattern

**Files:** `DNA/src/cad/pattern.rs` (new)

```rust
pub fn linear_pattern(
    solid: &Solid,
    direction: Vector3,
    count: u32,
    spacing: f32,
) -> Vec<Solid>
```

Creates `count` copies of solid, each offset by `spacing` along `direction`.

### 4.5 Circular Pattern

```rust
pub fn circular_pattern(
    solid: &Solid,
    axis: Vector3,
    center: Point3,
    count: u32,
) -> Vec<Solid>
```

Creates `count` copies rotated around `axis` at equal angular intervals.

### 4.6 UI Updates

- Add "Arc" button to sketch tools
- Add "Rectangle" button
- Add "Revolve" operation with axis selector
- Add pattern dialog with count/spacing inputs

---

## Phase 5: Advanced Features

### 5.1 Feature Tree (Parametric History)

**Files:** `MCAD/src/lib.rs` (new structs)

```rust
pub struct FeatureId(pub u32);

pub enum Feature {
    Sketch { id: FeatureId, sketch: Sketch },
    Extrude { id: FeatureId, sketch_ref: FeatureId, params: ExtrudeParams },
    Revolve { id: FeatureId, sketch_ref: FeatureId, params: RevolveParams },
    Boolean { id: FeatureId, op: BooleanOp, a: FeatureId, b: FeatureId },
    Pattern { id: FeatureId, source: FeatureId, pattern_type: PatternType },
}

pub struct FeatureTree {
    features: Vec<Feature>,
    current_solid: Option<Solid>,
}

impl FeatureTree {
    pub fn add_feature(&mut self, feature: Feature) -> FeatureId;
    pub fn regenerate(&mut self);  // Rebuild from feature list
    pub fn edit_feature(&mut self, id: FeatureId, new_params: FeatureParams);
    pub fn rollback(&mut self, id: FeatureId);  // Show state at feature
    pub fn delete_feature(&mut self, id: FeatureId);
}
```

**UI:**
- Left panel showing feature tree
- Click feature to select/highlight
- Double-click to edit parameters
- Right-click context menu (edit, delete, rollback)
- Drag to reorder (with dependency validation)

### 5.2 Assembly Modeling

```rust
pub struct PartId(pub u32);

pub struct Assembly {
    parts: Vec<(PartId, String, Transform3)>,  // ID, name, position/rotation
    mates: Vec<Mate>,
}

pub enum Mate {
    Coincident { face1: (PartId, FaceId), face2: (PartId, FaceId) },
    Concentric { cyl1: (PartId, FaceId), cyl2: (PartId, FaceId) },
    Distance { face1: (PartId, FaceId), face2: (PartId, FaceId), value: f32 },
    Angle { face1: (PartId, FaceId), face2: (PartId, FaceId), value: f32 },
}
```

**Workflow:**
1. Create parts as separate solids
2. Add parts to assembly
3. Define mates between faces
4. Solve mate constraints to position parts

### 5.3 IGES Export

**Files:** `DNA/src/export/iges.rs`

IGES (Initial Graphics Exchange Specification) format:
- Simpler than STEP
- Entity types: points, lines, arcs, surfaces, solids
- ASCII format with 80-column records

### 5.4 Fillet/Chamfer

**Files:** `DNA/src/cad/fillet.rs`

```rust
pub fn fillet_edge(solid: &mut Solid, edge: EdgeId, radius: f32) -> Result<(), FilletError>;
pub fn chamfer_edge(solid: &mut Solid, edge: EdgeId, distance: f32) -> Result<(), ChamferError>;
```

**Algorithm (Fillet):**
1. Find faces adjacent to edge
2. Compute offset surfaces
3. Create rolling ball blend surface
4. Split original faces at blend boundaries
5. Replace edge region with blend surface

### 5.5 Sketch on Face

Allow creating sketches on existing solid faces:
1. Select face
2. Enter sketch mode with face as sketch plane
3. Project face edges as construction geometry
4. Draw new entities
5. Extrude cut or boss from face

---

## Success Criteria

### Phase 4 Complete When:
- [ ] Arc tool works (3-click arc creation)
- [ ] Rectangle tool creates 4 constrained lines
- [ ] Revolve generates solid of revolution
- [ ] Linear pattern creates N copies
- [ ] Circular pattern creates rotated copies

### Phase 5 Complete When:
- [ ] Feature tree displays model history
- [ ] Can edit feature parameters and regenerate
- [ ] Basic assembly with 2+ parts
- [ ] Mates constrain part positions
- [ ] Fillet rounds selected edges
- [ ] IGES export works

---

## Technical Notes

### Circumcenter Formula (Arc Tool)
Given 3 points P1, P2, P3:
```
D = 2(P1x(P2y - P3y) + P2x(P3y - P1y) + P3x(P1y - P2y))
Cx = ((P1x² + P1y²)(P2y - P3y) + (P2x² + P2y²)(P3y - P1y) + (P3x² + P3y²)(P1y - P2y)) / D
Cy = ((P1x² + P1y²)(P3x - P2x) + (P2x² + P2y²)(P1x - P3x) + (P3x² + P3y²)(P2x - P1x)) / D
```

### Revolve Surface Generation
For profile point P at angle θ around Y-axis:
```
x' = Px * cos(θ)
y' = Py
z' = Px * sin(θ)
```

### Fillet Geometry
Rolling ball radius R creates blend surface where:
- Distance from original edge = R
- Tangent to both adjacent faces
- Cross-section is circular arc of radius R
