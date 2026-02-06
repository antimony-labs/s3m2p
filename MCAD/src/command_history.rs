//! ═══════════════════════════════════════════════════════════════════════════════
//! FILE: command_history.rs | MCAD/src/command_history.rs
//! PURPOSE: Undo/redo command history system for sketch operations
//! MODIFIED: 2026-01-08
//! LAYER: MCAD (L1 Bubble)
//! ═══════════════════════════════════════════════════════════════════════════════

use cad_engine::{SketchEntityId, SketchPointId};

/// Commands for undo/redo system
#[derive(Clone, Debug)]
pub enum SketchCommand {
    /// Added geometry (points and entities)
    AddGeometry {
        point_ids: Vec<SketchPointId>,
        entity_ids: Vec<SketchEntityId>,
    },
    /// Added a constraint
    AddConstraint { index: usize },
    /// Toggled construction mode on points
    ToggleConstruction {
        point_ids: Vec<SketchPointId>,
        prev_states: Vec<bool>,
    },
}

/// Command history for undo/redo operations
///
/// Maintains two stacks: one for undo operations and one for redo.
/// When a new command is pushed, the redo stack is cleared.
/// Has a configurable maximum size to prevent memory issues.
#[derive(Default)]
pub struct CommandHistory {
    undo_stack: Vec<SketchCommand>,
    redo_stack: Vec<SketchCommand>,
    max_size: usize,
}

impl CommandHistory {
    /// Create a new command history with default max size of 100
    pub fn new() -> Self {
        Self {
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
            max_size: 100,
        }
    }

    /// Create a command history with a custom max size
    pub fn with_max_size(max_size: usize) -> Self {
        Self {
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
            max_size,
        }
    }

    /// Push a new command to the undo stack
    /// This clears the redo stack (can't redo after new action)
    pub fn push(&mut self, cmd: SketchCommand) {
        self.undo_stack.push(cmd);
        self.redo_stack.clear(); // Clear redo stack on new action
        if self.undo_stack.len() > self.max_size {
            self.undo_stack.remove(0);
        }
    }

    /// Check if undo is available
    pub fn can_undo(&self) -> bool {
        !self.undo_stack.is_empty()
    }

    /// Check if redo is available
    pub fn can_redo(&self) -> bool {
        !self.redo_stack.is_empty()
    }

    /// Pop the most recent command from the undo stack
    pub fn pop_undo(&mut self) -> Option<SketchCommand> {
        self.undo_stack.pop()
    }

    /// Push a command to the redo stack (called after undoing)
    pub fn push_redo(&mut self, cmd: SketchCommand) {
        self.redo_stack.push(cmd);
    }

    /// Pop the most recent command from the redo stack
    pub fn pop_redo(&mut self) -> Option<SketchCommand> {
        self.redo_stack.pop()
    }

    /// Push a command to the undo stack (called after redoing)
    pub fn push_undo(&mut self, cmd: SketchCommand) {
        self.undo_stack.push(cmd);
    }

    /// Get the number of commands in the undo stack
    pub fn undo_count(&self) -> usize {
        self.undo_stack.len()
    }

    /// Get the number of commands in the redo stack
    pub fn redo_count(&self) -> usize {
        self.redo_stack.len()
    }

    /// Clear all history
    pub fn clear(&mut self) {
        self.undo_stack.clear();
        self.redo_stack.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_add_geometry_cmd(point_count: u32) -> SketchCommand {
        let point_ids: Vec<_> = (0..point_count).map(|i| SketchPointId(i)).collect();
        SketchCommand::AddGeometry {
            point_ids,
            entity_ids: vec![],
        }
    }

    #[test]
    fn test_new_history_empty() {
        let history = CommandHistory::new();
        assert!(!history.can_undo());
        assert!(!history.can_redo());
        assert_eq!(history.undo_count(), 0);
        assert_eq!(history.redo_count(), 0);
    }

    #[test]
    fn test_push_enables_undo() {
        let mut history = CommandHistory::new();
        history.push(make_add_geometry_cmd(1));

        assert!(history.can_undo());
        assert!(!history.can_redo());
        assert_eq!(history.undo_count(), 1);
    }

    #[test]
    fn test_pop_undo_returns_command() {
        let mut history = CommandHistory::new();
        history.push(make_add_geometry_cmd(1));
        history.push(make_add_geometry_cmd(2));

        let cmd = history.pop_undo().unwrap();
        match cmd {
            SketchCommand::AddGeometry { point_ids, .. } => {
                assert_eq!(point_ids.len(), 2);
            }
            _ => panic!("Wrong command type"),
        }

        assert_eq!(history.undo_count(), 1);
    }

    #[test]
    fn test_undo_empty_returns_none() {
        let mut history = CommandHistory::new();
        assert!(history.pop_undo().is_none());
    }

    #[test]
    fn test_push_clears_redo() {
        let mut history = CommandHistory::new();
        history.push(make_add_geometry_cmd(1));
        history.pop_undo();
        history.push_redo(make_add_geometry_cmd(1));

        assert!(history.can_redo());

        // New action should clear redo
        history.push(make_add_geometry_cmd(2));
        assert!(!history.can_redo());
    }

    #[test]
    fn test_redo_after_undo() {
        let mut history = CommandHistory::new();
        history.push(make_add_geometry_cmd(1));

        // Simulate undo
        let cmd = history.pop_undo().unwrap();
        history.push_redo(cmd);

        assert!(history.can_redo());
        assert!(!history.can_undo());

        // Simulate redo
        let cmd = history.pop_redo().unwrap();
        history.push_undo(cmd);

        assert!(history.can_undo());
        assert!(!history.can_redo());
    }

    #[test]
    fn test_max_size_limit() {
        let mut history = CommandHistory::with_max_size(3);

        history.push(make_add_geometry_cmd(1));
        history.push(make_add_geometry_cmd(2));
        history.push(make_add_geometry_cmd(3));
        history.push(make_add_geometry_cmd(4)); // Should remove first

        assert_eq!(history.undo_count(), 3);

        // First command should be the one with 2 points (1 was removed)
        history.pop_undo(); // 4
        history.pop_undo(); // 3
        let cmd = history.pop_undo().unwrap(); // 2

        match cmd {
            SketchCommand::AddGeometry { point_ids, .. } => {
                assert_eq!(point_ids.len(), 2);
            }
            _ => panic!("Wrong command type"),
        }
    }

    #[test]
    fn test_clear_removes_all() {
        let mut history = CommandHistory::new();
        history.push(make_add_geometry_cmd(1));
        history.push(make_add_geometry_cmd(2));
        let cmd = history.pop_undo().unwrap();
        history.push_redo(cmd);

        history.clear();

        assert!(!history.can_undo());
        assert!(!history.can_redo());
        assert_eq!(history.undo_count(), 0);
        assert_eq!(history.redo_count(), 0);
    }

    #[test]
    fn test_add_constraint_command() {
        let mut history = CommandHistory::new();
        history.push(SketchCommand::AddConstraint { index: 5 });

        let cmd = history.pop_undo().unwrap();
        match cmd {
            SketchCommand::AddConstraint { index } => {
                assert_eq!(index, 5);
            }
            _ => panic!("Wrong command type"),
        }
    }

    #[test]
    fn test_toggle_construction_command() {
        let mut history = CommandHistory::new();
        history.push(SketchCommand::ToggleConstruction {
            point_ids: vec![SketchPointId(0), SketchPointId(1)],
            prev_states: vec![false, true],
        });

        let cmd = history.pop_undo().unwrap();
        match cmd {
            SketchCommand::ToggleConstruction {
                point_ids,
                prev_states,
            } => {
                assert_eq!(point_ids.len(), 2);
                assert_eq!(prev_states, vec![false, true]);
            }
            _ => panic!("Wrong command type"),
        }
    }

    #[test]
    fn test_multiple_undo_redo_sequence() {
        let mut history = CommandHistory::new();

        // Push 3 commands
        history.push(make_add_geometry_cmd(1));
        history.push(make_add_geometry_cmd(2));
        history.push(make_add_geometry_cmd(3));

        // Undo 2
        let cmd3 = history.pop_undo().unwrap();
        history.push_redo(cmd3);
        let cmd2 = history.pop_undo().unwrap();
        history.push_redo(cmd2);

        assert_eq!(history.undo_count(), 1);
        assert_eq!(history.redo_count(), 2);

        // Redo 1
        let cmd = history.pop_redo().unwrap();
        history.push_undo(cmd);

        assert_eq!(history.undo_count(), 2);
        assert_eq!(history.redo_count(), 1);
    }
}
