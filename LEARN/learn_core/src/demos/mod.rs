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
pub mod ohms_law_power;
pub mod rc_time_constant;
pub mod power_budget;

// Ubuntu demos
pub mod fs_permissions;
pub mod swarm_world;
pub mod boids;

// AI/ML demos
pub mod perceptron;

// Data Structures demos
pub mod pseudocode;
pub mod array_demo;
pub mod linked_list_demo;
pub mod stack_demo;
pub mod queue_demo;
pub mod binary_tree_demo;
pub mod bst_demo;
pub mod heap_demo;
pub mod hash_table_demo;
pub mod graph_demo;
pub mod balanced_tree_demo;

// Algorithm Problems (Practice section)
pub mod problems;
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
pub use ohms_law_power::OhmsLawPowerDemo;
pub use rc_time_constant::RcTimeConstantDemo;
pub use power_budget::PowerBudgetDemo;

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

// Data Structures exports
pub use pseudocode::{Pseudocode, CodeLine};
pub use array_demo::{ArrayDemo, ArrayAnimation};
pub use linked_list_demo::{LinkedListDemo, ListNode, ListAnimation};
pub use stack_demo::{StackDemo, StackAnimation};
pub use queue_demo::{QueueDemo, QueueAnimation};
pub use binary_tree_demo::{BinaryTreeDemo, TreeNode, TreeAnimation, TraversalOrder};
pub use bst_demo::{BstDemo, BstNode, BstAnimation, HighlightState};
pub use heap_demo::{HeapDemo, HeapAnimation, HeapType};
pub use hash_table_demo::{HashTableDemo, HashEntry, HashAnimation, CollisionStrategy};
pub use graph_demo::{GraphDemo, Vertex, Edge, VertexState, GraphAnimation, TraversalAlgorithm};
pub use balanced_tree_demo::{BalancedTreeDemo, AvlNode, AvlAnimation, RotationType, HighlightType};

// Algorithm Problems exports
pub use problems::{Problem, Pattern, Difficulty, PROBLEMS, ALL_PATTERNS};
