//! ═══════════════════════════════════════════════════════════════════════════════
//! FILE: selection3d.rs | MCAD/src/selection3d.rs
//! PURPOSE: 3D selection state for face/edge/vertex picking
//! MODIFIED: 2026-01-07
//! LAYER: MCAD (application)
//! ═══════════════════════════════════════════════════════════════════════════════

use cad_engine::{EdgeId, FaceId, VertexId};

/// Selection mode for 3D picking
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum SelectionMode3D {
    #[default]
    Face,
    Edge,
    Vertex,
}

impl SelectionMode3D {
    pub fn label(&self) -> &'static str {
        match self {
            SelectionMode3D::Face => "Face",
            SelectionMode3D::Edge => "Edge",
            SelectionMode3D::Vertex => "Vertex",
        }
    }
}

/// 3D selection state
#[derive(Default, Clone, Debug)]
pub struct Selection3D {
    /// Current selection mode
    pub mode: SelectionMode3D,

    /// Selected faces (multi-select with Ctrl)
    pub selected_faces: Vec<FaceId>,

    /// Selected edges (multi-select with Ctrl)
    pub selected_edges: Vec<EdgeId>,

    /// Selected vertices (multi-select with Ctrl)
    pub selected_vertices: Vec<VertexId>,

    /// Face currently under cursor (hover highlight)
    pub hover_face: Option<FaceId>,

    /// Edge currently under cursor
    pub hover_edge: Option<EdgeId>,

    /// Vertex currently under cursor
    pub hover_vertex: Option<VertexId>,
}

impl Selection3D {
    pub fn new() -> Self {
        Self::default()
    }

    /// Set selection mode
    pub fn set_mode(&mut self, mode: SelectionMode3D) {
        self.mode = mode;
        // Clear hover when changing mode
        self.hover_face = None;
        self.hover_edge = None;
        self.hover_vertex = None;
    }

    /// Clear all selections and hovers
    pub fn clear(&mut self) {
        self.selected_faces.clear();
        self.selected_edges.clear();
        self.selected_vertices.clear();
        self.hover_face = None;
        self.hover_edge = None;
        self.hover_vertex = None;
    }

    /// Clear only selections (keep hover)
    pub fn clear_selection(&mut self) {
        self.selected_faces.clear();
        self.selected_edges.clear();
        self.selected_vertices.clear();
    }

    // ─────────────────────────────────────────────────────────────────────────
    // Face selection
    // ─────────────────────────────────────────────────────────────────────────

    /// Handle face click (with Ctrl for multi-select)
    pub fn click_face(&mut self, face_id: FaceId, ctrl: bool) {
        if ctrl {
            // Toggle selection
            if let Some(pos) = self.selected_faces.iter().position(|&f| f == face_id) {
                self.selected_faces.remove(pos);
            } else {
                self.selected_faces.push(face_id);
            }
        } else {
            // Replace selection
            self.selected_faces.clear();
            self.selected_faces.push(face_id);
        }
    }

    /// Check if a face is selected
    pub fn is_face_selected(&self, face_id: FaceId) -> bool {
        self.selected_faces.contains(&face_id)
    }

    /// Get the primary selected face (first in list)
    pub fn primary_face(&self) -> Option<FaceId> {
        self.selected_faces.first().copied()
    }

    // ─────────────────────────────────────────────────────────────────────────
    // Edge selection
    // ─────────────────────────────────────────────────────────────────────────

    /// Handle edge click (with Ctrl for multi-select)
    pub fn click_edge(&mut self, edge_id: EdgeId, ctrl: bool) {
        if ctrl {
            if let Some(pos) = self.selected_edges.iter().position(|&e| e == edge_id) {
                self.selected_edges.remove(pos);
            } else {
                self.selected_edges.push(edge_id);
            }
        } else {
            self.selected_edges.clear();
            self.selected_edges.push(edge_id);
        }
    }

    /// Check if an edge is selected
    pub fn is_edge_selected(&self, edge_id: EdgeId) -> bool {
        self.selected_edges.contains(&edge_id)
    }

    // ─────────────────────────────────────────────────────────────────────────
    // Vertex selection
    // ─────────────────────────────────────────────────────────────────────────

    /// Handle vertex click (with Ctrl for multi-select)
    pub fn click_vertex(&mut self, vertex_id: VertexId, ctrl: bool) {
        if ctrl {
            if let Some(pos) = self.selected_vertices.iter().position(|&v| v == vertex_id) {
                self.selected_vertices.remove(pos);
            } else {
                self.selected_vertices.push(vertex_id);
            }
        } else {
            self.selected_vertices.clear();
            self.selected_vertices.push(vertex_id);
        }
    }

    /// Check if a vertex is selected
    pub fn is_vertex_selected(&self, vertex_id: VertexId) -> bool {
        self.selected_vertices.contains(&vertex_id)
    }

    // ─────────────────────────────────────────────────────────────────────────
    // Status
    // ─────────────────────────────────────────────────────────────────────────

    /// Get selection count for current mode
    pub fn selection_count(&self) -> usize {
        match self.mode {
            SelectionMode3D::Face => self.selected_faces.len(),
            SelectionMode3D::Edge => self.selected_edges.len(),
            SelectionMode3D::Vertex => self.selected_vertices.len(),
        }
    }

    /// Get status text for UI
    pub fn status_text(&self) -> String {
        let count = self.selection_count();
        let mode = self.mode.label();
        if count == 0 {
            format!("{} mode", mode)
        } else if count == 1 {
            format!("1 {} selected", mode.to_lowercase())
        } else {
            format!("{} {}s selected", count, mode.to_lowercase())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_face_selection() {
        let mut sel = Selection3D::new();

        // Single click
        sel.click_face(FaceId(1), false);
        assert_eq!(sel.selected_faces.len(), 1);
        assert!(sel.is_face_selected(FaceId(1)));

        // Ctrl+click to add
        sel.click_face(FaceId(2), true);
        assert_eq!(sel.selected_faces.len(), 2);

        // Ctrl+click to remove
        sel.click_face(FaceId(1), true);
        assert_eq!(sel.selected_faces.len(), 1);
        assert!(!sel.is_face_selected(FaceId(1)));

        // Single click replaces
        sel.click_face(FaceId(3), false);
        assert_eq!(sel.selected_faces.len(), 1);
        assert!(sel.is_face_selected(FaceId(3)));
    }

    #[test]
    fn test_mode_switch() {
        let mut sel = Selection3D::new();
        sel.hover_face = Some(FaceId(1));

        sel.set_mode(SelectionMode3D::Edge);
        assert_eq!(sel.mode, SelectionMode3D::Edge);
        assert!(sel.hover_face.is_none());
    }

    #[test]
    fn test_status_text() {
        let mut sel = Selection3D::new();
        assert_eq!(sel.status_text(), "Face mode");

        sel.click_face(FaceId(1), false);
        assert_eq!(sel.status_text(), "1 face selected");

        sel.click_face(FaceId(2), true);
        assert_eq!(sel.status_text(), "2 faces selected");
    }
}
