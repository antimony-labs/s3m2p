//! ═══════════════════════════════════════════════════════════════════════════════
//! FILE: permission_matrix.rs | learn_core/src/diagram/permission_matrix.rs
//! PURPOSE: Permission table visualization with rwx grid
//! MODIFIED: 2025-12-30
//! LAYER: LEARN → learn_core → diagram
//! ═══════════════════════════════════════════════════════════════════════════════

use super::{Diagram, DiagramRenderer, TextAlign};
use crate::demos::fs_permissions::FsPermissionsDemo;

#[derive(Clone, Debug)]
pub struct FilePerms {
    pub name: String,
    pub owner: String,
    pub group: String,
    pub permissions: u16,
}

pub struct PermissionMatrix {
    files: Vec<FilePerms>,
    width: f64,
    height: f64,
}

impl PermissionMatrix {
    pub fn from_demo(demo: &FsPermissionsDemo) -> Self {
        let mut files = Vec::new();

        // Get files from current directory
        let cwd = &demo.inodes[demo.cwd];
        for &child_idx in &cwd.children {
            let child = &demo.inodes[child_idx];
            files.push(FilePerms {
                name: child.name.clone(),
                owner: child.owner.clone(),
                group: child.group.clone(),
                permissions: child.permissions,
            });
        }

        let height = (files.len() as f64 * 30.0) + 80.0;
        Self {
            files,
            width: 600.0,
            height,
        }
    }
}

impl Diagram for PermissionMatrix {
    fn width(&self) -> f64 {
        self.width
    }

    fn height(&self) -> f64 {
        self.height
    }

    fn render(&self, r: &mut dyn DiagramRenderer) {
        // Draw header
        r.draw_text("File", 20.0, 25.0, "bold 14px Rajdhani", "#64ffda", TextAlign::Left);
        r.draw_text("Owner", 180.0, 25.0, "bold 14px Rajdhani", "#64ffda", TextAlign::Left);
        r.draw_text("Group", 280.0, 25.0, "bold 14px Rajdhani", "#64ffda", TextAlign::Left);

        // Permission headers
        let perm_x = 380.0;
        r.draw_text("r", perm_x, 25.0, "bold 12px JetBrains Mono", "#64ffda", TextAlign::Center);
        r.draw_text("w", perm_x + 30.0, 25.0, "bold 12px JetBrains Mono", "#64ffda", TextAlign::Center);
        r.draw_text("x", perm_x + 60.0, 25.0, "bold 12px JetBrains Mono", "#64ffda", TextAlign::Center);

        r.draw_text("r", perm_x + 100.0, 25.0, "bold 12px JetBrains Mono", "#64ffda", TextAlign::Center);
        r.draw_text("w", perm_x + 130.0, 25.0, "bold 12px JetBrains Mono", "#64ffda", TextAlign::Center);
        r.draw_text("x", perm_x + 160.0, 25.0, "bold 12px JetBrains Mono", "#64ffda", TextAlign::Center);

        // Header line
        r.draw_line(10.0, 35.0, 590.0, 35.0, "#333", 1.0);

        let mut y = 60.0;
        for file in &self.files {
            // File name
            let icon = if file.name.ends_with('/') { "📁" } else { "📄" };
            r.draw_text(icon, 20.0, y, "14px Arial", "#888", TextAlign::Left);
            r.draw_text(&file.name, 45.0, y, "12px JetBrains Mono", "#e0e0e0", TextAlign::Left);

            // Owner and group
            r.draw_text(&file.owner, 180.0, y, "12px Inter", "#888", TextAlign::Left);
            r.draw_text(&file.group, 280.0, y, "12px Inter", "#888", TextAlign::Left);

            // Permission bits (owner)
            self.draw_perm_bit(r, perm_x - 10.0, y - 12.0, (file.permissions >> 8) & 1 == 1);
            self.draw_perm_bit(r, perm_x + 20.0, y - 12.0, (file.permissions >> 7) & 1 == 1);
            self.draw_perm_bit(r, perm_x + 50.0, y - 12.0, (file.permissions >> 6) & 1 == 1);

            // Permission bits (group)
            self.draw_perm_bit(r, perm_x + 90.0, y - 12.0, (file.permissions >> 5) & 1 == 1);
            self.draw_perm_bit(r, perm_x + 120.0, y - 12.0, (file.permissions >> 4) & 1 == 1);
            self.draw_perm_bit(r, perm_x + 150.0, y - 12.0, (file.permissions >> 3) & 1 == 1);

            // Permission bits (other)
            self.draw_perm_bit(r, perm_x + 190.0, y - 12.0, (file.permissions >> 2) & 1 == 1);
            self.draw_perm_bit(r, perm_x + 220.0, y - 12.0, (file.permissions >> 1) & 1 == 1);
            self.draw_perm_bit(r, perm_x + 250.0, y - 12.0, (file.permissions >> 0) & 1 == 1);

            y += 30.0;
        }
    }
}

impl PermissionMatrix {
    fn draw_perm_bit(&self, r: &mut dyn DiagramRenderer, x: f64, y: f64, enabled: bool) {
        let (fill, stroke) = if enabled {
            ("#44ff88", Some("#44ff88"))
        } else {
            ("transparent", Some("#444"))
        };
        r.draw_rect(x, y, 18.0, 18.0, fill, stroke);
    }
}
