//! ═══════════════════════════════════════════════════════════════════════════════
//! FILE: lessons.rs | OPENCV/src/lessons.rs
//! PURPOSE: OpenCV/Computer Vision lesson definitions and curriculum structure
//! MODIFIED: 2026-01-02
//! LAYER: LEARN → OPENCV
//! ═══════════════════════════════════════════════════════════════════════════════

/// Demo type for a lesson
#[derive(Clone, Copy, Debug, PartialEq)]
#[allow(dead_code)]
pub enum DemoType {
    /// No interactive demo - theory content only
    Static,
    /// Canvas-based interactive demo (no camera)
    Canvas,
    /// Camera input with processed output side-by-side
    Camera,
    /// Side-by-side comparison with sliders
    SideBySide,
}

/// A single OpenCV/Computer Vision lesson
pub struct Lesson {
    pub id: usize,
    pub title: &'static str,
    pub subtitle: &'static str,
    pub icon: &'static str,
    pub phase: &'static str,
    pub demo_type: DemoType,
    pub description: &'static str,
    pub content: &'static str,
    pub key_concepts: &'static [&'static str],
    pub concept_definitions: &'static [(&'static str, &'static str)],
}

/// Phase names for grouping lessons
pub static PHASES: &[&str] = &[
    "The Big Picture",
    "Filtering & Enhancement",
    "Feature Detection",
    "Geometric Transforms",
    "Real-World Applications",
];

/// All OpenCV lessons organized in phases
pub static LESSONS: &[Lesson] = &[
    // ═══════════════════════════════════════════════════════════════════════════
    // PHASE 1: THE BIG PICTURE (Theory)
    // ═══════════════════════════════════════════════════════════════════════════
    Lesson {
        id: 0,
        title: "What is Computer Vision?",
        subtitle: "Teaching Machines to See",
        icon: "👁️",
        phase: "The Big Picture",
        demo_type: DemoType::Static,
        description: "Computer vision bridges the gap between pixels and understanding. Learn how machines interpret the visual world.",
        content: r#"
## The Vision Problem

Humans process visual information effortlessly. We recognize faces, read signs, catch balls, and navigate crowded streets without conscious effort. But for computers, **seeing is incredibly hard**.

A digital image is just a grid of numbers. Each pixel is a value (or three values for color). The challenge is extracting **meaning** from this sea of numbers.

---

## What is Computer Vision?

**Computer Vision** is the field of AI that enables machines to interpret and understand visual information from the world.

| What Humans See | What Computers See |
|-----------------|-------------------|
| A cat on a couch | 1920×1080 grid of RGB values |
| A stop sign | Red hexagon shape, white text |
| A friend's face | Facial feature coordinates |

---

## The Image Processing Pipeline

Most computer vision systems follow this flow:

1. **Acquisition** → Camera captures light as pixels
2. **Pre-processing** → Noise reduction, normalization
3. **Feature Extraction** → Edges, corners, textures
4. **Analysis** → Pattern recognition, classification
5. **Decision** → Object detected, action taken

---

## Why It Matters

Computer vision powers:
- **Self-driving cars** → Lane detection, obstacle avoidance
- **Medical imaging** → Tumor detection, X-ray analysis
- **Manufacturing** → Quality control, defect detection
- **Security** → Face recognition, motion detection
- **Augmented reality** → Object tracking, scene understanding

---

## The OpenCV Library

**OpenCV** (Open Source Computer Vision Library) is the most widely used computer vision library. Created by Intel in 1999, it provides:

- 2500+ optimized algorithms
- Cross-platform (Windows, Linux, macOS, iOS, Android)
- Bindings for Python, Java, C++
- Real-time processing capabilities

In this course, we'll implement key algorithms from scratch in Rust to understand them deeply, then apply them to real-time camera feeds.
"#,
        key_concepts: &["Pixels", "Feature Extraction", "Real-time Processing", "OpenCV"],
        concept_definitions: &[
            ("Pixels", "The smallest unit of a digital image, containing color/intensity values"),
            ("Feature Extraction", "The process of identifying meaningful patterns in image data"),
            ("Real-time Processing", "Processing video frames fast enough for live interaction (typically 30+ fps)"),
            ("OpenCV", "Open Source Computer Vision Library - the most popular CV library"),
        ],
    },
    Lesson {
        id: 1,
        title: "Pixels & Color Spaces",
        subtitle: "The Digital Image",
        icon: "🎨",
        phase: "The Big Picture",
        demo_type: DemoType::Canvas,
        description: "Understand how images are represented digitally. Explore RGB, grayscale, and HSV color spaces.",
        content: r#"
## What is a Pixel?

The word **pixel** comes from "picture element." It's the smallest addressable unit of a digital image.

Each pixel stores color information:
- **Grayscale**: Single value (0-255), 0=black, 255=white
- **RGB**: Three values (Red, Green, Blue), each 0-255
- **RGBA**: RGB + Alpha channel for transparency

---

## Image as a Matrix

A digital image is a 2D array (matrix) of pixels:

```
Image[height][width] = pixel value
```

For a 1920×1080 RGB image:
- **Width**: 1920 pixels
- **Height**: 1080 pixels
- **Total pixels**: 2,073,600
- **Bytes**: 6,220,800 (3 bytes per pixel)

---

## RGB Color Model

Colors are created by mixing Red, Green, and Blue light:

| Color | R | G | B |
|-------|---|---|---|
| Red | 255 | 0 | 0 |
| Green | 0 | 255 | 0 |
| Blue | 0 | 0 | 255 |
| White | 255 | 255 | 255 |
| Black | 0 | 0 | 0 |
| Yellow | 255 | 255 | 0 |

---

## Grayscale Conversion

Converting RGB to grayscale uses weighted averaging:

```
Gray = 0.299*R + 0.587*G + 0.114*B
```

Why these weights? Human eyes are most sensitive to green, less to red, and least to blue.

---

## HSV Color Space

**HSV** (Hue, Saturation, Value) is often better for computer vision:

- **Hue**: The color type (0-360°, like a color wheel)
- **Saturation**: Color intensity (0-100%)
- **Value**: Brightness (0-100%)

HSV separates color from brightness, making it easier to detect objects by color regardless of lighting conditions.

---

## Why Color Spaces Matter

Different tasks need different representations:
- **Edge detection** → Grayscale (simpler, faster)
- **Color tracking** → HSV (robust to lighting)
- **Display** → RGB (how screens work)
- **Skin detection** → YCbCr (separates luminance)
"#,
        key_concepts: &["Pixel", "RGB", "Grayscale", "HSV", "Color Space"],
        concept_definitions: &[
            ("Pixel", "Picture element - the smallest unit of a digital image"),
            ("RGB", "Red-Green-Blue color model used by displays"),
            ("Grayscale", "Single-channel image representing intensity only"),
            ("HSV", "Hue-Saturation-Value color space, useful for color detection"),
            ("Color Space", "A system for representing colors mathematically"),
        ],
    },

    // ═══════════════════════════════════════════════════════════════════════════
    // PHASE 2: FILTERING & ENHANCEMENT
    // ═══════════════════════════════════════════════════════════════════════════
    Lesson {
        id: 2,
        title: "Convolution",
        subtitle: "The Foundation of Filtering",
        icon: "🔲",
        phase: "Filtering & Enhancement",
        demo_type: DemoType::Canvas,
        description: "Learn how convolution kernels transform images. The mathematical foundation of all image filtering.",
        content: r#"
## What is Convolution?

**Convolution** is the fundamental operation in image processing. It slides a small matrix (called a **kernel**) across an image, computing weighted sums at each position.

Think of it as a "moving average" with weights.

---

## The Kernel (Filter)

A kernel is a small matrix of numbers, typically 3×3 or 5×5:

```
Identity kernel (no change):
[0  0  0]
[0  1  0]
[0  0  0]

Blur kernel (average neighbors):
[1/9  1/9  1/9]
[1/9  1/9  1/9]
[1/9  1/9  1/9]
```

---

## How Convolution Works

For each pixel in the image:
1. Center the kernel on the pixel
2. Multiply each kernel value by the overlapping pixel value
3. Sum all products
4. The result is the new pixel value

```
Output[y][x] = Σ Σ Kernel[j][i] × Image[y+j][x+i]
```

---

## Common Kernels

### Blur (Box Filter)
Averages neighboring pixels, reducing noise:
```
[1  1  1]
[1  1  1] × (1/9)
[1  1  1]
```

### Sharpen
Enhances edges by emphasizing differences:
```
[ 0  -1   0]
[-1   5  -1]
[ 0  -1   0]
```

### Edge Detection (Sobel X)
Detects vertical edges:
```
[-1  0  1]
[-2  0  2]
[-1  0  1]
```

---

## Boundary Handling

What happens at image edges? Common strategies:
- **Zero padding**: Assume pixels outside are black
- **Replicate**: Copy edge pixels outward
- **Reflect**: Mirror the image at boundaries
- **Wrap**: Treat image as toroidal (tileable)

---

## Separable Kernels

Some kernels can be decomposed into two 1D passes (horizontal then vertical). This reduces operations from O(k²) to O(2k):

```
2D Gaussian = [1D horizontal] × [1D vertical]
```

A 5×5 kernel: 25 operations → 10 operations per pixel!
"#,
        key_concepts: &["Convolution", "Kernel", "Filter", "Blur", "Sharpen"],
        concept_definitions: &[
            ("Convolution", "Mathematical operation of sliding a kernel across an image"),
            ("Kernel", "A small matrix of weights used in convolution"),
            ("Filter", "An operation that modifies image pixels based on their neighbors"),
            ("Blur", "Smoothing effect that reduces detail and noise"),
            ("Sharpen", "Enhancement that increases edge contrast"),
        ],
    },
    Lesson {
        id: 3,
        title: "Edge Detection",
        subtitle: "Finding Boundaries",
        icon: "📐",
        phase: "Filtering & Enhancement",
        demo_type: DemoType::Camera,
        description: "Detect edges in real-time using the Canny algorithm. See boundaries emerge from your camera feed.",
        content: r#"
## What Are Edges?

**Edges** are rapid changes in image intensity. They mark boundaries between objects, textures, or regions.

Edges carry most of the semantic information in an image. A simple line drawing (just edges) is often recognizable, while a blurred photo isn't.

---

## Gradient-Based Detection

Edges are detected by finding **gradients** (rates of change):

```
Gx = ∂I/∂x (horizontal change)
Gy = ∂I/∂y (vertical change)

Magnitude = √(Gx² + Gy²)
Direction = arctan(Gy / Gx)
```

---

## Sobel Operator

The Sobel operator approximates gradients using convolution:

**Horizontal edges (Gx):**
```
[-1  0  1]
[-2  0  2]
[-1  0  1]
```

**Vertical edges (Gy):**
```
[-1  -2  -1]
[ 0   0   0]
[ 1   2   1]
```

---

## The Canny Edge Detector

The **Canny algorithm** (1986) is the gold standard for edge detection:

1. **Gaussian blur** → Reduce noise
2. **Sobel gradients** → Find intensity changes
3. **Non-maximum suppression** → Thin edges to 1 pixel
4. **Double threshold** → Classify strong/weak edges
5. **Hysteresis** → Connect weak edges to strong ones

---

## Threshold Parameters

- **Low threshold**: Edges weaker than this are discarded
- **High threshold**: Edges stronger than this are kept
- **Weak edges**: Between thresholds, kept only if connected to strong edges

Try adjusting the sliders in the demo to see how thresholds affect detection!

---

## Applications

- **Object detection** → Outline shapes
- **Lane detection** → Find road markings
- **Document scanning** → Detect paper edges
- **Medical imaging** → Tumor boundaries
"#,
        key_concepts: &["Edge", "Gradient", "Sobel", "Canny", "Threshold"],
        concept_definitions: &[
            ("Edge", "A rapid change in image intensity marking a boundary"),
            ("Gradient", "The rate of change of intensity in a direction"),
            ("Sobel", "A gradient-approximation operator using 3×3 kernels"),
            ("Canny", "Multi-stage edge detection algorithm considered the gold standard"),
            ("Threshold", "A cutoff value for classifying pixels as edge or non-edge"),
        ],
    },
    Lesson {
        id: 4,
        title: "Noise Reduction",
        subtitle: "Cleaning Up Images",
        icon: "✨",
        phase: "Filtering & Enhancement",
        demo_type: DemoType::Camera,
        description: "Compare blur techniques in real-time. Gaussian, median, and bilateral filtering.",
        content: r#"
## Why Reduce Noise?

Real camera images always contain **noise** — random variations in pixel values caused by sensor limitations, low light, or compression.

Noise can confuse edge detectors, feature extractors, and classifiers. We need to smooth it out while preserving important details.

---

## Types of Noise

- **Gaussian noise**: Random variations following a normal distribution
- **Salt-and-pepper**: Random black and white pixels
- **Speckle**: Multiplicative noise in radar/ultrasound images

---

## Gaussian Blur

Uses a Gaussian (bell curve) kernel. Pixels closer to the center have more influence:

```
Kernel (5×5 approximation):
[ 1   4   7   4  1]
[ 4  16  26  16  4]
[ 7  26  41  26  7] × (1/273)
[ 4  16  26  16  4]
[ 1   4   7   4  1]
```

**Pros**: Smooth, natural-looking blur
**Cons**: Blurs edges along with noise

---

## Median Filter

Replaces each pixel with the **median** of its neighbors (not average).

For a 3×3 neighborhood:
1. Collect all 9 pixel values
2. Sort them
3. Take the middle value

**Pros**: Excellent for salt-and-pepper noise
**Cons**: Can lose fine details

---

## Bilateral Filter

The smart filter: **preserves edges** while smoothing.

Uses two Gaussian functions:
1. **Spatial**: Nearby pixels have more weight
2. **Range**: Similar-intensity pixels have more weight

This means edges (where intensity changes sharply) are preserved, while flat regions are smoothed.

**Pros**: Best edge preservation
**Cons**: Slower than Gaussian (not separable)

---

## Comparison

| Filter | Speed | Edge Preservation | Salt-Pepper |
|--------|-------|-------------------|-------------|
| Gaussian | Fast | Poor | Poor |
| Median | Medium | Good | Excellent |
| Bilateral | Slow | Excellent | Good |
"#,
        key_concepts: &["Noise", "Gaussian Blur", "Median Filter", "Bilateral Filter"],
        concept_definitions: &[
            ("Noise", "Random variations in pixel values due to sensor/transmission imperfections"),
            ("Gaussian Blur", "Smoothing using a bell-curve weighted average of neighbors"),
            ("Median Filter", "Noise reduction by replacing pixels with the median of neighbors"),
            ("Bilateral Filter", "Edge-preserving smoothing using spatial and intensity similarity"),
        ],
    },

    // ═══════════════════════════════════════════════════════════════════════════
    // PHASE 3: FEATURE DETECTION
    // ═══════════════════════════════════════════════════════════════════════════
    Lesson {
        id: 5,
        title: "Corner Detection",
        subtitle: "Finding Interest Points",
        icon: "🎯",
        phase: "Feature Detection",
        demo_type: DemoType::Camera,
        description: "Detect corners in real-time with Harris corner detection. Find stable tracking points.",
        content: r#"
## What Are Corners?

**Corners** are points where two edges meet. They're important because:
- Unique and easily identifiable
- Stable across viewpoints
- Good for tracking and matching

---

## The Aperture Problem

Edges are ambiguous — you can slide along them without noticing movement. But corners are unambiguous in all directions.

This makes corners ideal **feature points** for:
- Object tracking
- Image stitching (panoramas)
- 3D reconstruction
- Motion estimation

---

## Harris Corner Detector

The Harris algorithm (1988) detects corners by analyzing how the image changes when we shift a small window:

1. Compute gradients Ix and Iy (using Sobel)
2. Build the structure tensor M for each pixel:
   ```
   M = [Ix²    Ix·Iy]
       [Ix·Iy  Iy²  ]
   ```
3. Compute corner response:
   ```
   R = det(M) - k·trace(M)²
   ```
   where k ≈ 0.04-0.06

4. Threshold R to find corners

---

## Interpreting the Response

- **R > threshold** → Corner (both eigenvalues large)
- **R < 0** → Edge (one eigenvalue large)
- **R ≈ 0** → Flat region (both eigenvalues small)

---

## Shi-Tomasi Improvement

The Shi-Tomasi method (1994) uses a simpler corner measure:

```
R = min(λ₁, λ₂)
```

This is more stable and is the default in OpenCV's `goodFeaturesToTrack()`.

---

## Non-Maximum Suppression

To avoid detecting the same corner multiple times:
1. Find local maxima of R
2. Keep only the strongest response in each neighborhood

This gives clean, well-separated corner points.
"#,
        key_concepts: &["Corner", "Harris", "Feature Point", "Structure Tensor"],
        concept_definitions: &[
            ("Corner", "A point where two edges meet, detectable from all directions"),
            ("Harris", "Classic corner detection algorithm using eigenvalue analysis"),
            ("Feature Point", "A distinctive, trackable location in an image"),
            ("Structure Tensor", "A matrix summarizing local gradient structure"),
        ],
    },
    Lesson {
        id: 6,
        title: "Blob Detection",
        subtitle: "Finding Regions",
        icon: "⭕",
        phase: "Feature Detection",
        demo_type: DemoType::Camera,
        description: "Detect blob-like regions in images. Find circular objects and bright/dark spots.",
        content: r#"
## What Are Blobs?

**Blobs** are regions that differ from their surroundings in brightness or color. They often represent:
- Objects (balls, heads, buttons)
- Defects in manufacturing
- Cells in microscopy
- Stars in astronomy

---

## Laplacian of Gaussian (LoG)

The LoG filter detects blobs by finding regions where intensity changes rapidly in all directions:

```
LoG = ∇²G = ∂²G/∂x² + ∂²G/∂y²
```

Where G is a Gaussian. The LoG responds strongly to blob-like structures whose size matches the Gaussian's scale (σ).

---

## Difference of Gaussians (DoG)

A fast approximation to LoG:

```
DoG = G(σ₁) - G(σ₂)
```

Subtract two blurred versions of the image. Regions where they differ are blobs.

This is what SIFT uses for scale-space blob detection.

---

## Simple Blob Detection

A practical approach:
1. **Threshold** the image to binary
2. **Find connected components** (groups of connected pixels)
3. **Filter by properties**:
   - Area (size)
   - Circularity (how round)
   - Convexity (how convex)

---

## Circularity

How round is a blob?

```
Circularity = 4π × Area / Perimeter²
```

- Circle: Circularity = 1
- Square: Circularity ≈ 0.785
- Long thin shape: Circularity → 0

---

## Applications

- **Particle tracking** in physics
- **Cell counting** in biology
- **Defect detection** in manufacturing
- **Eye tracking** (finding pupils)
- **Star detection** in astronomy
"#,
        key_concepts: &["Blob", "Laplacian", "DoG", "Connected Components"],
        concept_definitions: &[
            ("Blob", "A region differing from its surroundings in brightness or color"),
            ("Laplacian", "Second derivative operator that responds to intensity extrema"),
            ("DoG", "Difference of Gaussians - fast approximation to blob detection"),
            ("Connected Components", "Groups of pixels that are touching (8-connectivity or 4-connectivity)"),
        ],
    },
    Lesson {
        id: 7,
        title: "Thresholding",
        subtitle: "Binary Decisions",
        icon: "🔳",
        phase: "Feature Detection",
        demo_type: DemoType::Camera,
        description: "Convert grayscale to binary. Explore global, adaptive, and Otsu's thresholding.",
        content: r#"
## What is Thresholding?

**Thresholding** converts a grayscale image to binary (black and white):

```
if pixel > threshold:
    output = white (255)
else:
    output = black (0)
```

This simplifies images for further processing like contour detection.

---

## Global Thresholding

Use one threshold for the entire image:

```
Binary[y][x] = 255 if Gray[y][x] > T else 0
```

**Problem**: Doesn't work well with uneven lighting.

---

## Adaptive Thresholding

Use different thresholds for different regions:

1. For each pixel, compute local average (or Gaussian-weighted average)
2. Threshold = local_average - C (where C is a constant)

This handles shadows and lighting gradients.

---

## Otsu's Method

Automatically finds the optimal global threshold by:
1. Computing histogram of pixel intensities
2. Finding threshold that minimizes within-class variance
3. Equivalently: maximizes between-class variance

The idea: separate foreground and background into two distinct groups.

---

## Choosing the Right Method

| Method | Use When |
|--------|----------|
| Global | Uniform lighting, high contrast |
| Adaptive | Shadows, gradients, uneven lighting |
| Otsu | Bimodal histogram (two distinct peaks) |

---

## Common Issues

- **Noise**: Apply blur before thresholding
- **Thin lines**: May break apart at threshold boundaries
- **Gradual transitions**: No clear threshold exists

---

## Applications

- **Document scanning** → Black text on white paper
- **Cell segmentation** → Separate cells from background
- **Object isolation** → Silhouette extraction
- **QR code reading** → Binary pattern detection
"#,
        key_concepts: &["Threshold", "Binary Image", "Adaptive", "Otsu"],
        concept_definitions: &[
            ("Threshold", "A cutoff value dividing pixels into two categories"),
            ("Binary Image", "An image with only two values: black (0) and white (255)"),
            ("Adaptive", "Using locally-computed thresholds instead of a global value"),
            ("Otsu", "Automatic threshold selection minimizing within-class variance"),
        ],
    },

    // ═══════════════════════════════════════════════════════════════════════════
    // PHASE 4: GEOMETRIC TRANSFORMS
    // ═══════════════════════════════════════════════════════════════════════════
    Lesson {
        id: 8,
        title: "Image Transformations",
        subtitle: "Rotate, Scale, Warp",
        icon: "🔄",
        phase: "Geometric Transforms",
        demo_type: DemoType::Canvas,
        description: "Apply geometric transformations to images. Affine and perspective warping.",
        content: r#"
## Geometric Transformations

Transformations change the **spatial arrangement** of pixels:
- **Translation**: Shift the image
- **Rotation**: Spin around a point
- **Scaling**: Resize (zoom in/out)
- **Shear**: Slant the image
- **Perspective**: 3D-like distortion

---

## Affine Transformations

Affine transforms preserve parallel lines. They can be represented as a 2×3 matrix:

```
[x']   [a  b  tx] [x]
[y'] = [c  d  ty] [y]
                  [1]
```

Combines rotation, scaling, shear, and translation.

---

## Common Affine Operations

**Translation (shift by tx, ty):**
```
[1  0  tx]
[0  1  ty]
```

**Rotation (by angle θ):**
```
[cos(θ)  -sin(θ)  0]
[sin(θ)   cos(θ)  0]
```

**Scaling (by sx, sy):**
```
[sx  0   0]
[0   sy  0]
```

---

## Perspective Transform

Perspective (homography) transforms don't preserve parallel lines — they simulate 3D viewpoint changes.

Used for:
- **Document scanning** → Straighten photographed pages
- **Panorama stitching** → Align overlapping images
- **Augmented reality** → Place virtual objects

Represented as a 3×3 matrix:

```
[x']   [h11  h12  h13] [x]
[y'] = [h21  h22  h23] [y]
[w']   [h31  h32  h33] [1]

x_out = x'/w',  y_out = y'/w'
```

---

## Interpolation

When transforming, output pixels may land between input pixels. Interpolation fills in values:

- **Nearest neighbor**: Use closest pixel (fast, blocky)
- **Bilinear**: Weighted average of 4 neighbors (smooth)
- **Bicubic**: Weighted average of 16 neighbors (sharper)

---

## Applications

- **Image registration** → Align images from different sources
- **Video stabilization** → Compensate for camera shake
- **OCR preprocessing** → Deskew scanned documents
- **Face alignment** → Normalize face positions
"#,
        key_concepts: &["Affine", "Perspective", "Homography", "Interpolation"],
        concept_definitions: &[
            ("Affine", "Linear transformation preserving parallel lines"),
            ("Perspective", "Transformation simulating 3D viewpoint change"),
            ("Homography", "A 3×3 matrix mapping points between two planes"),
            ("Interpolation", "Estimating pixel values between known samples"),
        ],
    },
    Lesson {
        id: 9,
        title: "Contour Detection",
        subtitle: "Finding Shapes",
        icon: "📊",
        phase: "Geometric Transforms",
        demo_type: DemoType::Camera,
        description: "Find and analyze contours in real-time. Detect shapes and compute their properties.",
        content: r#"
## What Are Contours?

**Contours** are curves joining continuous points along a boundary with the same color or intensity. They're the outlines of objects.

Unlike edges (which are local), contours form closed loops representing complete object boundaries.

---

## Finding Contours

The basic algorithm:
1. **Threshold** image to binary
2. **Trace boundaries** of white regions
3. **Store as point sequences**

OpenCV uses the Suzuki-Abe algorithm (1985) which efficiently finds contours and their hierarchy.

---

## Contour Hierarchy

Contours can be nested:
- **External contour**: Outer boundary
- **Internal contours**: Holes inside the object

The hierarchy tracks parent-child relationships:
```
  ┌──────────────┐
  │   Object     │
  │  ┌────────┐  │
  │  │  Hole  │  │
  │  └────────┘  │
  └──────────────┘
```

---

## Contour Properties

For each contour, we can compute:
- **Area**: Number of pixels inside
- **Perimeter**: Length of the boundary
- **Bounding box**: Smallest enclosing rectangle
- **Centroid**: Center of mass
- **Moments**: Shape statistics

---

## Shape Analysis

**Convex Hull**: The smallest convex polygon containing the contour (like a rubber band around it).

**Convexity Defects**: Points where the contour deviates from its convex hull — useful for gesture recognition (fingers).

**Approximate Polygon**: Simplify contour to fewer points using Douglas-Peucker algorithm.

---

## Shape Matching

Compare shapes using **Hu Moments** — 7 values that are:
- Translation invariant
- Scale invariant
- Rotation invariant

Two shapes with similar Hu moments are geometrically similar.

---

## Applications

- **Object counting** → Count contoured regions
- **Gesture recognition** → Analyze hand shapes
- **Logo detection** → Match contour signatures
- **OCR** → Character segmentation
"#,
        key_concepts: &["Contour", "Convex Hull", "Moments", "Shape Matching"],
        concept_definitions: &[
            ("Contour", "A curve joining continuous boundary points of same intensity"),
            ("Convex Hull", "Smallest convex polygon enclosing a shape"),
            ("Moments", "Statistical measures describing shape distribution"),
            ("Shape Matching", "Comparing shapes using invariant descriptors"),
        ],
    },

    // ═══════════════════════════════════════════════════════════════════════════
    // PHASE 5: REAL-WORLD APPLICATIONS
    // ═══════════════════════════════════════════════════════════════════════════
    Lesson {
        id: 10,
        title: "Color Tracking",
        subtitle: "Following Objects",
        icon: "🎯",
        phase: "Real-World Applications",
        demo_type: DemoType::Camera,
        description: "Track colored objects in real-time. Use HSV color space for robust detection.",
        content: r#"
## Why Track by Color?

Color tracking is simple but effective. When an object has a distinctive color (red ball, blue marker, yellow tennis ball), we can track it reliably.

Key advantage: Works even when shape changes or object rotates.

---

## HSV for Color Detection

**HSV** (Hue, Saturation, Value) separates color from brightness:

- **Hue**: The color (0-180 in OpenCV, 0-360° conceptually)
- **Saturation**: Color purity (0 = gray, 255 = pure color)
- **Value**: Brightness (0 = black, 255 = bright)

This makes color detection robust to lighting changes.

---

## The Tracking Pipeline

1. **Convert** frame from RGB to HSV
2. **Threshold** to create mask (pixels in color range = white)
3. **Clean up** with morphological operations
4. **Find contours** in the mask
5. **Track** the largest contour (your object)

---

## Choosing Color Ranges

For a red object:
```
Lower: H=0, S=100, V=100
Upper: H=10, S=255, V=255
```

Red wraps around in HSV, so you may need two ranges:
- 0-10 (red on one side)
- 170-180 (red on other side)

---

## Morphological Cleanup

Remove noise from the mask:

**Erosion**: Shrink white regions (removes small noise)
**Dilation**: Expand white regions (fills small holes)
**Opening**: Erosion then dilation (removes noise, preserves shape)
**Closing**: Dilation then erosion (fills holes, preserves shape)

---

## Tracking Output

From the detected contour:
- **Centroid**: (x, y) position of object center
- **Bounding box**: Rectangle around object
- **Area**: Size of object (for depth estimation)

Draw crosshairs or boxes to visualize tracking.

---

## Limitations

- **Multiple objects** of same color confuse tracking
- **Changing lighting** shifts apparent hue
- **Occlusion** loses the object temporarily
- **Similar backgrounds** cause false positives
"#,
        key_concepts: &["HSV", "Color Range", "Morphology", "Object Tracking"],
        concept_definitions: &[
            ("HSV", "Hue-Saturation-Value color space, separating color from brightness"),
            ("Color Range", "Upper and lower bounds defining a target color in HSV"),
            ("Morphology", "Operations like erosion/dilation that modify shape based on structure"),
            ("Object Tracking", "Following an object's position across video frames"),
        ],
    },
    Lesson {
        id: 11,
        title: "Face Detection",
        subtitle: "Finding Faces",
        icon: "👤",
        phase: "Real-World Applications",
        demo_type: DemoType::Camera,
        description: "Detect faces in real-time. Understand the classic Haar cascade approach.",
        content: r#"
## The Face Detection Challenge

Faces are complex objects that vary in:
- Size and position
- Orientation and pose
- Lighting conditions
- Expression and appearance

Yet humans detect faces instantly. How can computers do it?

---

## Viola-Jones Framework

The classic face detection algorithm (2001) uses:

1. **Haar-like features**: Simple patterns of light and dark regions
2. **Integral image**: Fast feature computation
3. **AdaBoost**: Learning which features matter
4. **Cascade classifier**: Fast rejection of non-faces

This was revolutionary for real-time face detection.

---

## Haar-like Features

Simple rectangular patterns:
```
Edge features:    Line features:    Four-rectangle:
█████░░░░░        █████░░░░░        █████░░░░░
█████░░░░░        ░░░░░█████        ░░░░░█████
█████░░░░░        █████░░░░░        ░░░░░█████
█████░░░░░        ░░░░░█████        █████░░░░░
```

The value is (sum of black pixels) - (sum of white pixels).

---

## Why Haar Features Work

Faces have consistent patterns:
- Eyes are darker than cheeks
- Bridge of nose is lighter than eyes
- Eyebrows are darker than forehead

Haar features capture these relationships.

---

## The Cascade

A cascade is a sequence of classifiers:
- **Stage 1**: Very simple, rejects 50% of windows quickly
- **Stage 2**: More complex, rejects more non-faces
- **...more stages...**
- **Final stage**: Detailed check, confirms face

Non-faces are rejected early, making scanning fast.

---

## Detection Process

1. Scan image with sliding window at multiple scales
2. At each position, run the cascade
3. If all stages pass → face detected
4. Apply non-maximum suppression to remove duplicates

---

## Modern Approaches

Today, deep learning dominates:
- **CNNs**: More accurate than Haar cascades
- **MTCNN**: Multi-task cascade for face + landmarks
- **RetinaFace**: State-of-the-art accuracy
- **BlazeFace**: Optimized for mobile devices

But Haar cascades remain useful for:
- Low-power devices
- Embedded systems
- Educational understanding
"#,
        key_concepts: &["Haar Cascade", "Viola-Jones", "Integral Image", "Face Detection"],
        concept_definitions: &[
            ("Haar Cascade", "A sequence of classifiers using Haar-like rectangular features"),
            ("Viola-Jones", "The 2001 framework enabling real-time face detection"),
            ("Integral Image", "A data structure allowing O(1) computation of rectangular sums"),
            ("Face Detection", "Locating faces (bounding boxes) in images or video"),
        ],
    },
];
