//! ═══════════════════════════════════════════════════════════════════════════════
//! FILE: mod.rs | learn_core/src/terminal/mod.rs
//! PURPOSE: Terminal configuration system for lesson-specific customization
//! MODIFIED: 2025-12-30
//! LAYER: LEARN → learn_core → terminal
//! ═══════════════════════════════════════════════════════════════════════════════

use crate::demos::fs_permissions::FsPermissionsDemo;

/// Configuration trait for lesson-specific terminal behavior
pub trait TerminalConfig: Send {
    /// Welcome message displayed when terminal starts
    fn welcome_message(&self) -> &str;

    /// Initialize the filesystem for this lesson
    fn init_filesystem(&self, demo: &mut FsPermissionsDemo);

    /// Commands available in this lesson
    fn allowed_commands(&self) -> &[&str];

    /// Terminal prompt format
    fn prompt_format(&self) -> &str {
        "user@ubuntu:~$ "
    }

    /// Hints displayed below the terminal
    fn hints(&self) -> &[&str] {
        &[]
    }

    /// Validate command and return error message if invalid
    fn validate_exercise(&self, _demo: &FsPermissionsDemo) -> Option<String> {
        None
    }
}

/// Default configuration (all commands available)
pub struct DefaultConfig;

impl TerminalConfig for DefaultConfig {
    fn welcome_message(&self) -> &str {
        "Ubuntu Linux Terminal - Type 'help' for available commands"
    }

    fn init_filesystem(&self, _demo: &mut FsPermissionsDemo) {
        // Use default filesystem from FsPermissionsDemo::init_fs()
    }

    fn allowed_commands(&self) -> &[&str] {
        &[
            "ls", "cd", "pwd", "cat", "chmod", "chown", "mkdir", "touch",
            "rm", "cp", "mv", "echo", "head", "tail", "grep", "clear",
            "whoami", "id", "su", "help",
        ]
    }

    fn hints(&self) -> &[&str] {
        &[
            "Try: ls -l",
            "Try: cat readme.txt",
            "Try: chmod 777 script.sh",
            "Try: su root",
        ]
    }
}
