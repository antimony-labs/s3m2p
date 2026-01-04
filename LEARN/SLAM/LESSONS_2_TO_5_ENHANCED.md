# Enhanced Lessons 2-5: Math & Implementation Guide

## LESSON 2: Kalman Filter

### Math Details (LaTeX):
```
<h4>Predict Step</h4>
$$\hat{x}_k^- = F \hat{x}_{k-1} + B u_k$$
$$P_k^- = F P_{k-1} F^T + Q$$

<h4>Update Step</h4>
$$K_k = P_k^- H^T (H P_k^- H^T + R)^{-1}$$
$$\hat{x}_k = \hat{x}_k^- + K_k (z_k - H \hat{x}_k^-)$$
$$P_k = (I - K_k H) P_k^-$$

**Where:**
- $\hat{x}_k$ = State estimate (position, velocity)
- $P_k$ = Covariance matrix (uncertainty)
- $K_k$ = Kalman Gain (trust factor, calculated automatically)
- $Q$ = Process noise (how much uncertainty motion adds)
- $R$ = Measurement noise (how noisy is GPS)
- $F$ = State transition (motion model)
- $H$ = Measurement matrix (how sensor sees state)

**Error in Demo:**
- Error = $|\vec{x}_{true} - \hat{x}|$
- Ellipse size = $\sqrt{\lambda_{max}(P)}$ (largest eigenvalue of covariance)
```

### Implementation:
```
<h4>1D Kalman Filter from Scratch</h4>
<pre>
struct KalmanFilter1D {
    x: f32,      // State estimate
    p: f32,      // Variance
    q: f32,      // Process noise
    r: f32,      // Measurement noise
}

impl KalmanFilter1D {
    fn predict(&mut self, u: f32) {
        self.x += u;              // State transition
        self.p += self.q;         // Uncertainty grows
    }

    fn update(&mut self, z: f32) {
        let k = self.p / (self.p + self.r);  // Kalman gain
        self.x += k * (z - self.x);           // Correct estimate
        self.p *= (1.0 - k);                  // Shrink uncertainty
    }
}
</pre>

<h4>LLM Prompt: GPS + IMU Fusion</h4>
<pre>"Implement 2D Kalman Filter in Rust that fuses:
- GPS measurements (x, y) at 1Hz with R=[[10,0],[0,10]]
- IMU velocity (vx, vy) at 100Hz with Q=[[0.1,0],[0,0.1]]
State vector: [x, y, vx, vy]
Target: Raspberry Pi with rppal crate for GPIO"</pre>

<h4>Hardware: GPS Modules</h4>
- **u-blox NEO-M9N** - Multi-band, 1.5m accuracy, UART
- **Adafruit Ultimate GPS** - $40, 3m accuracy, good for learning

<h4>LLM Prompt: Parse NMEA</h4>
<pre>"Parse NMEA $GPGGA sentences from UART to extract:
lat/lon, altitude, fix quality, HDOP.
Convert to local XY coordinates using UTM projection."</pre>
```

---

## LESSON 3: Particle Filter

### Math Details:
```
<h4>Particle Representation</h4>
$$p(x_t | z_{1:t}) \approx \sum_{i=1}^N w_t^{(i)} \delta(x_t - x_t^{(i)})$$

**Where:**
- $x_t^{(i)}$ = $i$-th particle (hypothesis about state)
- $w_t^{(i)}$ = Weight (how likely this particle is correct)
- $N$ = Number of particles (more = better but slower)

<h4>Motion Model (Prediction)</h4>
$$x_t^{(i)} = f(x_{t-1}^{(i)}, u_t) + \mathcal{N}(0, Q)$$

Each particle moves according to motion + random noise.

<h4>Measurement Model (Update)</h4>
$$w_t^{(i)} \propto p(z_t | x_t^{(i)}) \cdot w_{t-1}^{(i)}$$

Particles near sensor reading get high weight.

<h4>Resampling</h4>
Draw new particles proportional to weights: $P(select \ particle \ i) = w_i$

**Error:** Weighted average position vs ground truth.
```

### Implementation:
```
<h4>Particle Filter from Scratch</h4>
<pre>
struct Particle {
    x: f32,
    weight: f32,
}

struct ParticleFilter {
    particles: Vec<Particle>,
    motion_noise: f32,
    sensor_noise: f32,
}

impl ParticleFilter {
    fn predict(&mut self, u: f32) {
        for p in &mut self.particles {
            p.x += u + rand::normal(0.0, self.motion_noise);
        }
    }

    fn update(&mut self, z: f32) {
        for p in &mut self.particles {
            let error = z - p.x;
            p.weight *= (-0.5 * error.powi(2) / self.sensor_noise.powi(2)).exp();
        }
        self.normalize_weights();
    }

    fn resample(&mut self) {
        // Systematic resampling
        let new_particles = self.particles.weighted_sample(self.particles.len());
        self.particles = new_particles;
    }
}
</pre>

<h4>When to Use Particle Filters</h4>
- Multi-modal distributions (robot could be in multiple rooms)
- Non-Gaussian noise (sensor sometimes gives wild readings)
- Non-linear motion (robot turning, drone flipping)

**Kalman fails here!** It assumes single Gaussian.
```

---

## LESSON 4: EKF SLAM with LiDAR

### Math Details:
```
<h4>State Vector (Robot + Landmarks)</h4>
$$\mathbf{x} = [x_r, y_r, \theta_r, x_1, y_1, x_2, y_2, ..., x_n, y_n]^T$$

- $(x_r, y_r, \theta_r)$ = Robot pose
- $(x_i, y_i)$ = Landmark $i$ position

<h4>EKF Prediction (Robot Moves)</h4>
$$\hat{x}_{k|k-1} = f(\hat{x}_{k-1}, u_k)$$
$$P_{k|k-1} = F_k P_{k-1} F_k^T + Q_k$$

Where $F_k = \frac{\partial f}{\partial x}$ (Jacobian, linearizes motion)

<h4>EKF Update (See Landmark)</h4>
$$K = P H^T (H P H^T + R)^{-1}$$
$$\hat{x}_k = \hat{x}_{k|k-1} + K (z - h(\hat{x}_{k|k-1}))$$

Where $h$ = measurement model (range/bearing to landmark)

**Jacobian for range/bearing:**
$$H = \begin{bmatrix}
\frac{-\Delta x}{d} & \frac{-\Delta y}{d} & 0 & \frac{\Delta x}{d} & \frac{\Delta y}{d} \\
\frac{\Delta y}{d^2} & \frac{-\Delta x}{d^2} & -1 & \frac{-\Delta y}{d^2} & \frac{\Delta x}{d^2}
\end{bmatrix}$$
```

### Implementation with Ouster LiDAR:
```
<h4>Hardware: Ouster OS1-64</h4>
- 64 channels, 360° horizontal, ±22.5° vertical
- Up to 120m range, 10-20Hz
- Ethernet interface (UDP packets)
- $18,000 new, ~$3,000 used

<h4>LLM Prompt: Parse Ouster Packets</h4>
<pre>"Write Rust UDP server that:
1. Listens on port 7502 for Ouster OS1 lidar data
2. Parses binary packets to extract point cloud (x,y,z,intensity)
3. Filters points: remove >50m, intensity <threshold
4. Publish to ROS2 topic sensor_msgs::PointCloud2
Use ouster_sdk or parse manually"</pre>

<h4>Landmark Extraction</h4>
<pre>
// LLM Prompt:
"Implement RANSAC line fitting on 2D lidar scan to extract:
- Wall segments (>1m long)
- Corner features (intersection of 2 walls)
Return landmark positions in robot frame (range, bearing)"
</pre>

<h4>Data Association (Which Landmark?)</h4>
<pre>
// Mahalanobis distance - accounts for uncertainty
fn mahalanobis_dist(z: Vec2, landmark: Vec2, S: Mat2) -> f32 {
    let innovation = z - landmark;
    (innovation.transpose() * S.inverse() * innovation).sqrt()
}

// Match to nearest landmark within 3-sigma
if mahal_dist < 3.0 { /* associate */ }
else { /* new landmark */ }
</pre>

<h4>Complete EKF SLAM Loop</h4>
<pre>"Create ROS2 node that:
1. Subscribes to /odom (wheel encoders) at 50Hz
2. Subscribes to /scan (lidar) at 10Hz
3. Runs EKF SLAM with state [x,y,θ, landmarks...]
4. Publishes /map (landmark positions)
5. Publishes /pose_corrected (robot pose)
Target: Rust + rclrs + nalgebra for matrices"</pre>
```

---

## LESSON 5: Graph SLAM

### Math Details:
```
<h4>Pose Graph</h4>
Nodes: $\mathbf{x} = [x_1, y_1, \theta_1, ..., x_T, y_T, \theta_T]$
Edges: Constraints between poses

<h4>Error Function</h4>
$$F(\mathbf{x}) = \sum_{(i,j) \in \mathcal{E}} \mathbf{e}_{ij}^T \Omega_{ij} \mathbf{e}_{ij}$$

Where $\mathbf{e}_{ij} = \mathbf{z}_{ij} - h(x_i, x_j)$ (measurement residual)

<h4>Optimization (Gauss-Newton)</h4>
$$H \Delta \mathbf{x} = -\mathbf{b}$$
$$\mathbf{x}^{k+1} = \mathbf{x}^k + \Delta \mathbf{x}$$

**Loop Closure:** When you revisit a place, add edge between current pose and old pose.
This "pulls" the whole graph into consistency.
```

### Implementation:
```
<h4>LLM Prompt: Graph SLAM with g2o</h4>
<pre>"Use g2o Rust bindings to:
1. Create SE(2) pose graph (x,y,θ nodes)
2. Add odometry edges between sequential poses
3. Detect loop closures using ICP on lidar scans
4. Add loop closure edges with computed transform
5. Optimize graph using Levenberg-Marquardt
6. Export optimized trajectory to CSV"</pre>

<h4>Loop Closure Detection</h4>
<pre>// LLM Prompt:
"Implement scan matching with ICP:
1. Take current lidar scan
2. Compare to scans from >30s ago
3. If ICP converges with <0.2m error, it's a loop closure
4. Return relative transform (Δx, Δy, Δθ)"</pre>

<h4>Real Dataset</h4>
Use Intel Research Lab dataset or:
<pre>"Download TUM RGB-D SLAM dataset, extract:
- /odom topic → odometry edges
- /camera/rgb + /camera/depth → loop closures via ORB-SLAM
Build pose graph, optimize, compare to ground truth trajectory"</pre>
```
