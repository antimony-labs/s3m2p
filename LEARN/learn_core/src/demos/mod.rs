//! ═══════════════════════════════════════════════════════════════════════════════
//! FILE: mod.rs | LEARN/learn_core/src/demos/mod.rs
//! PURPOSE: Demo implementations for all LEARN apps
//! MODIFIED: 2025-12-12
//! LAYER: LEARN → learn_core → demos
//! ═══════════════════════════════════════════════════════════════════════════════

pub mod linear_regression;
pub mod complementary_filter;
pub mod kalman_filter;
pub mod particle_filter;
pub mod ekf_slam;
pub mod graph_slam;
pub mod gpio_debounce;
pub mod pwm_control;
pub mod adc_reading;
pub mod i2c_bus;
pub mod fs_permissions;
pub mod ohms_law_power;
pub mod rc_time_constant;
pub mod power_budget;

pub use linear_regression::LinearRegressionDemo;
pub use complementary_filter::{ComplementaryFilterDemo, ImuReading, SensorHistory};
pub use kalman_filter::{KalmanFilterDemo, KFPhase, Mat2};
pub use particle_filter::{ParticleFilterDemo, PFPhase, Particle, Measurement};
pub use ekf_slam::{EkfSlamDemo, SlamLandmark};
pub use graph_slam::{GraphSlamDemo, PoseNode, GraphEdge};
pub use gpio_debounce::GpioDebounceDemo;
pub use pwm_control::PwmControlDemo;
pub use adc_reading::{AdcReadingDemo, AdcAttenuation};
pub use i2c_bus::{I2cBusDemo, I2cPhase, I2cStage};
pub use fs_permissions::FsPermissionsDemo;
pub use ohms_law_power::OhmsLawPowerDemo;
pub use rc_time_constant::RcTimeConstantDemo;
pub use power_budget::PowerBudgetDemo;
