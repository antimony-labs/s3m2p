//! ═══════════════════════════════════════════════════════════════════════════════
//! FILE: mod.rs | LEARN/learn_core/src/demos/mod.rs
//! PURPOSE: Demo implementations for all LEARN apps
//! MODIFIED: 2026-01-02
//! LAYER: LEARN → learn_core → demos
//! ═══════════════════════════════════════════════════════════════════════════════

// SLAM demos
pub mod complementary_filter;
pub mod ekf_slam;
pub mod graph_slam;
pub mod kalman_filter;
pub mod linear_regression;
pub mod particle_filter;

// ESP32 demos
pub mod adc_reading;
pub mod gpio_debounce;
pub mod i2c_bus;
pub mod ohms_law_power;
pub mod power_budget;
pub mod pwm_control;
pub mod rc_time_constant;

// Ubuntu demos
pub mod boids;
pub mod fs_permissions;
pub mod swarm_world;

// AI/ML demos
pub mod perceptron;

// Data Structures demos
pub mod array_demo;
pub mod balanced_tree_demo;
pub mod binary_tree_demo;
pub mod bst_demo;
pub mod graph_demo;
pub mod hash_table_demo;
pub mod heap_demo;
pub mod linked_list_demo;
pub mod pseudocode;
pub mod queue_demo;
pub mod stack_demo;

// Algorithm Problems (Practice section)
pub mod attention;
pub mod cnn_filter;
pub mod grid_world;
pub mod neural_network;
pub mod problems;

// SLAM exports
pub use complementary_filter::{ComplementaryFilterDemo, ImuReading, SensorHistory};
pub use ekf_slam::{EkfSlamDemo, SlamLandmark};
pub use graph_slam::{GraphEdge, GraphSlamDemo, PoseNode};
pub use kalman_filter::{KFPhase, KalmanFilterDemo, Mat2};
pub use linear_regression::LinearRegressionDemo;
pub use particle_filter::{Measurement, PFPhase, Particle, ParticleFilterDemo};

// ESP32 exports
pub use adc_reading::{AdcAttenuation, AdcReadingDemo};
pub use gpio_debounce::GpioDebounceDemo;
pub use i2c_bus::{I2cBusDemo, I2cPhase, I2cStage};
pub use ohms_law_power::OhmsLawPowerDemo;
pub use power_budget::PowerBudgetDemo;
pub use pwm_control::PwmControlDemo;
pub use rc_time_constant::RcTimeConstantDemo;

// Ubuntu exports
pub use boids::BoidsDemo;
pub use fs_permissions::FsPermissionsDemo;
pub use swarm_world::{Agent, Obstacle, SwarmWorld};

// AI/ML exports
pub use attention::{AttentionDemo, Sentence};
pub use cnn_filter::{CnnFilterDemo, FilterType, ImagePattern};
pub use grid_world::{Action, Cell, GridLayout, GridWorldDemo};
pub use neural_network::{Activation, NNDataPoint, NNDataset, NeuralNetworkDemo};
pub use perceptron::{DataPoint, Dataset, PerceptronDemo};

// Data Structures exports
pub use array_demo::{ArrayAnimation, ArrayDemo};
pub use balanced_tree_demo::{
    AvlAnimation, AvlNode, BalancedTreeDemo, HighlightType, RotationType,
};
pub use binary_tree_demo::{BinaryTreeDemo, TraversalOrder, TreeAnimation, TreeNode};
pub use bst_demo::{BstAnimation, BstDemo, BstNode, HighlightState};
pub use graph_demo::{Edge, GraphAnimation, GraphDemo, TraversalAlgorithm, Vertex, VertexState};
pub use hash_table_demo::{CollisionStrategy, HashAnimation, HashEntry, HashTableDemo};
pub use heap_demo::{HeapAnimation, HeapDemo, HeapType};
pub use linked_list_demo::{LinkedListDemo, ListAnimation, ListNode};
pub use pseudocode::{CodeLine, Pseudocode};
pub use queue_demo::{QueueAnimation, QueueDemo};
pub use stack_demo::{StackAnimation, StackDemo};

// Algorithm Problems exports
pub use problems::{BinarySearchDemo, BinarySearchVariant};
pub use problems::{Difficulty, Pattern, Problem, ALL_PATTERNS, PROBLEMS};
pub use problems::{SlidingWindowDemo, SlidingWindowVariant};
pub use problems::{StackItem, StackProblemVariant, StackProblemsDemo};
pub use problems::{TwoPointerVariant, TwoPointersDemo};
