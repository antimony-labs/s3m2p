//! Power law network effects simulation module
//!
//! Provides high-performance simulations of power law phenomena including:
//! - Preferential attachment networks (Barabási-Albert model)
//! - Cascade dynamics (SIR, threshold models)
//! - Self-organized criticality (sandpile, forest fire)
//! - Resource distribution (Pareto, Zipf)
//!
//! ## Traceability
//! - Used by: WASM visualizations, scientific simulations
//! - Tests: tests/powerlaw_tests.rs
//!
//! ## Design Principles
//! - Zero-allocation hot paths (pre-sized arrays)
//! - SoA layout for cache locality
//! - Generational indices for safety
//! - O(1) or O(log n) operations where possible

pub mod arena;
pub mod attachment;
pub mod cascade;
pub mod criticality;
pub mod distribution;
pub mod edges;
pub mod metrics;
pub mod sampling;

pub use arena::{NetworkArena, NodeHandle, NodeProperties};
pub use attachment::BarabasiAlbert;
pub use cascade::{CascadeArena, CascadeState};
pub use criticality::{CellState, ForestFire, Sandpile};
pub use distribution::{Pareto, Zipf};
pub use edges::EdgeArena;
pub use metrics::NetworkMetrics;
pub use sampling::{AliasTable, PowerLawSampler};
