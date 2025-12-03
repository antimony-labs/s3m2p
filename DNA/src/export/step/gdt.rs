//! Geometric Dimensioning and Tolerancing (GD&T) entities

use super::entities::*;
use std::io::{self, Write};

/// GEOMETRIC_TOLERANCE - simplified tolerance entity
pub struct GeometricTolerance {
    pub id: EntityId,
    pub name: Option<String>,
    pub value: f64,
}

impl StepEntity for GeometricTolerance {
    fn entity_name(&self) -> &'static str {
        "GEOMETRIC_TOLERANCE"
    }

    fn write_attributes(&self, w: &mut dyn Write) -> io::Result<()> {
        if let Some(name) = &self.name {
            write_step_string(name, w)?;
        } else {
            write!(w, "$")?;
        }
        write!(w, ",{:.6}", self.value)
    }
}