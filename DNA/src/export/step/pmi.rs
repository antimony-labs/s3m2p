//! Product Manufacturing Information (PMI) entities

use super::entities::*;
use std::io::{self, Write};

/// DIMENSIONAL_LOCATION - linear distance between two features
pub struct DimensionalLocation {
    pub id: EntityId,
    pub name: Option<String>,
    pub value: f64,
}

impl StepEntity for DimensionalLocation {
    fn entity_name(&self) -> &'static str {
        "DIMENSIONAL_LOCATION"
    }

    fn write_attributes(&self, w: &mut dyn Write) -> io::Result<()> {
        if let Some(name) = &self.name {
            write_step_string(name, w)?;
        } else {
            write!(w, "$")?;
        }
        // Simplified: normally references shape aspects, here just value
        write!(w, ",{:.6}", self.value)
    }
}