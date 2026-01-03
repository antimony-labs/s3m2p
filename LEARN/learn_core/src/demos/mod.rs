//! ═══════════════════════════════════════════════════════════════════════════════
//! FILE: mod.rs | LEARN/learn_core/src/demos/mod.rs
//! PURPOSE: Demo implementations for all LEARN apps
//! MODIFIED: 2026-01-02
//! LAYER: LEARN → learn_core → demos
//! ═══════════════════════════════════════════════════════════════════════════════

// SLAM demos
pub mod linear_regression;
pub mod complementary_filter;
pub mod kalman_filter;
pub mod particle_filter;
pub mod ekf_slam;
pub mod graph_slam;

// ESP32 demos
pub mod gpio_debounce;
pub mod pwm_control;
pub mod adc_reading;
pub mod i2c_bus;

// Ubuntu demos
pub mod fs_permissions;
pub mod swarm_world;
pub mod boids;

// AI/ML demos
pub mod perceptron;
pub mod neural_network;
pub mod cnn_filter;
pub mod attention;
pub mod grid_world;

// SLAM exports
pub use linear_regression::LinearRegressionDemo;
pub use complementary_filter::{ComplementaryFilterDemo, ImuReading, SensorHistory};
pub use kalman_filter::{KalmanFilterDemo, KFPhase, Mat2};
pub use particle_filter::{ParticleFilterDemo, PFPhase, Particle, Measurement};
pub use ekf_slam::{EkfSlamDemo, SlamLandmark};
pub use graph_slam::{GraphSlamDemo, PoseNode, GraphEdge};

// ESP32 exports
pub use gpio_debounce::GpioDebounceDemo;
pub use pwm_control::PwmControlDemo;
pub use adc_reading::{AdcReadingDemo, AdcAttenuation};
pub use i2c_bus::{I2cBusDemo, I2cPhase, I2cStage};

// Ubuntu exports
pub use fs_permissions::FsPermissionsDemo;
pub use swarm_world::{SwarmWorld, Agent, Obstacle};
pub use boids::BoidsDemo;

// AI/ML exports
pub use perceptron::{PerceptronDemo, Dataset, DataPoint};
pub use neural_network::{NeuralNetworkDemo, Activation, NNDataset, NNDataPoint};
pub use cnn_filter::{CnnFilterDemo, FilterType, ImagePattern};
pub use attention::{AttentionDemo, Sentence};
pub use grid_world::{GridWorldDemo, Cell, Action, GridLayout};
