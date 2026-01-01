//! ═══════════════════════════════════════════════════════════════════════════════
//! FILE: filesystem_tree.rs | learn_core/src/diagram/filesystem_tree.rs
//! PURPOSE: Directory tree visualization
//! MODIFIED: 2025-12-30
//! LAYER: LEARN → learn_core → diagram
//! ═══════════════════════════════════════════════════════════════════════════════

use super::{Diagram, DiagramRenderer, TextAlign};
use crate::demos::fs_permissions::FsPermissionsDemo;

#[derive(Clone, Debug)]
pub struct TreeNode {
    pub name: String,
    pub path: String,
    pub is_dir: bool,
    pub permissions: u16,
    pub depth: usize,
}

pub struct FilesystemTree {
    nodes: Vec<TreeNode>,
    width: f64,
    height: f64,
}

impl FilesystemTree {
    pub fn from_demo(demo: &FsPermissionsDemo, max_depth: usize) -> Self {
        let mut nodes = Vec::new();
        Self::walk_tree(demo, 0, 0, "", &mut nodes, max_depth);

        let height = (nodes.len() as f64 * 25.0) + 40.0;
        Self {
            nodes,
            width: 600.0,
            height,
        }
    }

    fn walk_tree(
        demo: &FsPermissionsDemo,
        idx: usize,
        depth: usize,
        parent_path: &str,
        nodes: &mut Vec<TreeNode>,
        max_depth: usize,
    ) {
        if depth > max_depth {
            return;
        }

        let inode = &demo.inodes[idx];
        let path = if parent_path.is_empty() {
            "/".to_string()
        } else if parent_path == "/" {
            format!("/{}", inode.name)
        } else {
            format!("{}/{}", parent_path, inode.name)
        };

        nodes.push(TreeNode {
            name: inode.name.clone(),
            path: path.clone(),
            is_dir: inode.is_dir,
            permissions: inode.permissions,
            depth,
        });

        if inode.is_dir {
            for &child_idx in &inode.children {
                Self::walk_tree(demo, child_idx, depth + 1, &path, nodes, max_depth);
            }
        }
    }
}

impl Diagram for FilesystemTree {
    fn width(&self) -> f64 {
        self.width
    }

    fn height(&self) -> f64 {
        self.height
    }

    fn render(&self, r: &mut dyn DiagramRenderer) {
        let mut y = 30.0;

        for node in &self.nodes {
            let x = 20.0 + (node.depth as f64 * 30.0);

            // Draw connecting line from parent (except root)
            if node.depth > 0 {
                r.draw_line(x - 15.0, y - 12.0, x - 5.0, y, "#666", 1.0);
            }

            // Icon
            let icon = if node.is_dir { "📁" } else { "📄" };
            r.draw_text(icon, x, y, "14px Arial", "#888", TextAlign::Left);

            // Name
            let color = if node.is_dir { "#64ffda" } else { "#e0e0e0" };
            r.draw_text(&node.name, x + 25.0, y, "14px JetBrains Mono", color, TextAlign::Left);

            // Permissions (octal)
            let perms = format!("{:04o}", node.permissions);
            r.draw_text(&perms, x + 250.0, y, "12px JetBrains Mono", "#888", TextAlign::Left);

            y += 25.0;
        }
    }
}
