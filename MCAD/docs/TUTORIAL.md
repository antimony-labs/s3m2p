# MCAD Tutorial: Flanged Bushing

This tutorial guides you through creating a complete mechanical part in MCAD, covering all major features: sketching, constraints, 3D operations, and export.

## Part Overview

The Flanged Bushing is a common mechanical component used for shaft support and alignment. Our design includes:

- Cylindrical body (created via Revolve)
- Flange at one end (created via Extrude)
- Central bore (created via Boolean Difference)
- Four mounting holes (created via Circular Pattern)

**Final dimensions:**
- Body diameter: 40mm
- Body height: 30mm
- Flange diameter: 60mm
- Flange thickness: 5mm
- Bore diameter: 20mm
- Mounting holes: 4x M6 on 50mm PCD

---

## Part 1: Base Cylinder (Revolve)

We'll create the main body using a Revolve operation around a profile sketch.

### Step 1.1: Create New Sketch

1. Click **Sketch** in the toolbar
2. Select the **XY Plane**
3. The view automatically orients to show the sketch plane

### Step 1.2: Draw the Revolve Profile

We'll draw a half-section of the bushing that will be revolved 360 degrees.

1. Press `L` to activate the Line tool
2. Draw the profile (click points in order):
   - Start at origin (0, 0) - this will be the center axis
   - Draw right to (20, 0) - half the body diameter
   - Draw up to (20, 30) - body height
   - Draw left to (30, 30) - flange outer radius
   - Draw up to (30, 35) - flange top
   - Draw left to (0, 35) - back to axis
   - Draw down to (0, 0) - close the profile

3. Press `Escape` to finish drawing

### Step 1.3: Add Construction Axis

The Y-axis line (from origin upward) will serve as the revolve axis.

1. Press `S` to switch to Select tool
2. Click the leftmost vertical line (from (0,0) to (0,35))
3. Press `X` to mark it as construction geometry
4. The line now displays as dashed

### Step 1.4: Apply Constraints

Make the profile parametric with constraints:

**Horizontal constraints:**
1. Select the bottom horizontal line
2. Press `H` to apply Horizontal constraint
3. Repeat for the top horizontal line

**Vertical constraints:**
1. Select the right vertical line (body wall)
2. Press `V` to apply Vertical constraint
3. Repeat for the axis line

**Dimensional constraints:**
1. Select the bottom horizontal line
2. Press `D` and enter `20` (body radius)
3. Select the right vertical line
4. Press `D` and enter `30` (body height)
5. Select the top-right horizontal line
6. Press `D` and enter `10` (flange extension = 30-20)
7. Select the flange vertical line
8. Press `D` and enter `5` (flange thickness)

**Check status:** The Status Bar should show "Fully constrained" or a low DOF count.

### Step 1.5: Complete the Revolve

1. Click **Finish Sketch** to exit sketch mode
2. Click the **Revolve** button in the toolbar
3. Enter angle: `360` degrees
4. Click **Apply**

**Result:** A solid cylindrical bushing with integrated flange.

---

## Part 2: Center Hole (Boolean Difference)

We'll create the center bore using a cylinder primitive and Boolean subtraction.

### Step 2.1: Create Bore Cylinder

1. Click **Cylinder** in the primitives section
2. Enter parameters:
   - Radius: `10` (for 20mm bore diameter)
   - Height: `40` (slightly taller than part)
3. Click **Create**

### Step 2.2: Position the Cylinder

The cylinder is created at the origin, aligned with our part. If needed:

1. Use the transform tools to center it
2. Ensure it passes completely through the part

### Step 2.3: Apply Boolean Difference

1. Select the main bushing body (click on it)
2. Hold `Ctrl` and click the bore cylinder
3. Click **Difference** in the Boolean operations
4. Click **Apply**

**Result:** The bore cylinder is subtracted, leaving a hollow bushing.

---

## Part 3: Mounting Holes (Circular Pattern)

We'll create one mounting hole and pattern it around the flange.

### Step 3.1: Create Sketch on Flange Face

1. Click **Sketch** in the toolbar
2. Click the top face of the flange to select it as the sketch plane
3. The view orients to look down at the flange

### Step 3.2: Draw Hole Profile

1. Press `C` to activate Circle tool
2. Click at position (25, 0) to place center - this is on the 50mm PCD
3. Move cursor outward and click when radius indicator shows approximately `3` (for M6 clearance)

### Step 3.3: Constrain the Hole

1. Select the circle
2. Apply a Radius constraint with value `3.2` (M6 clearance = 6.4mm diameter)

3. Constrain the center position:
   - Select the circle center and the origin
   - Apply Distance constraint: `25` (radius of 50mm PCD)

### Step 3.4: Exit and Extrude Cut

1. Click **Finish Sketch**
2. Click **Extrude**
3. Enter distance: `10` (through the flange)
4. Select **Cut** mode (subtracts material)
5. Click **Apply**

### Step 3.5: Apply Circular Pattern

1. Select the hole feature (click on a hole edge)
2. Click **Circular Pattern**
3. Set parameters:
   - Axis: Y (vertical)
   - Count: `4`
   - Angle: `360` degrees (full circle)
4. Click **Apply**

**Result:** Four evenly-spaced mounting holes on the flange.

---

## Part 4: Verification

Before export, verify the model:

### Step 4.1: Visual Inspection

1. Orbit the view to inspect all features
2. Check that the bore passes completely through
3. Verify all four mounting holes are present

### Step 4.2: Manifold Check

MCAD automatically checks for manifold (watertight) geometry:

- Look for the status message
- A valid manifold is required for export

### Step 4.3: Measurements

Use the measurement tools to verify:
- Body diameter: 40mm
- Flange diameter: 60mm
- Bore diameter: 20mm
- Overall height: 35mm

---

## Part 5: Export

### Step 5.1: STEP Export

For sharing with other CAD systems:

1. Click **Export STEP**
2. Save the file as `flanged_bushing.step`
3. Verify by importing into another CAD system

### Step 5.2: STL Export

For 3D printing:

1. Click **Export STL**
2. Save the file as `flanged_bushing.stl`
3. Import into your slicer software

**Print settings recommendation:**
- Layer height: 0.2mm
- Infill: 50% or higher for structural parts
- Supports: Not required for this geometry

---

## Summary

In this tutorial, you learned:

| Operation | Feature | Tools Used |
|-----------|---------|------------|
| **Revolve** | Main body | Sketch, Line, Constraints |
| **Boolean** | Center bore | Cylinder primitive, Difference |
| **Extrude Cut** | Mounting hole | Sketch on face, Circle, Cut |
| **Pattern** | Hole copies | Circular Pattern |
| **Export** | Final part | STEP, STL |

## Next Steps

Try modifying the design:

1. Change the bore diameter to 25mm
2. Add a chamfer to the flange edge
3. Create a counterbore for the mounting holes
4. Add a keyway to the bore

---

## Appendix: Complete Dimension Summary

| Feature | Dimension | Value |
|---------|-----------|-------|
| Body | Outer diameter | 40mm |
| Body | Height | 30mm |
| Flange | Outer diameter | 60mm |
| Flange | Thickness | 5mm |
| Bore | Diameter | 20mm |
| Bore | Depth | Through |
| Mounting holes | Diameter | 6.4mm (M6 clearance) |
| Mounting holes | PCD | 50mm |
| Mounting holes | Count | 4 |
| Mounting holes | Depth | Through flange |

---

*This tutorial is part of the MCAD documentation at too.foo/mcad*
