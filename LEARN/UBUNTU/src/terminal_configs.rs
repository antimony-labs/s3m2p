//! ═══════════════════════════════════════════════════════════════════════════════
//! FILE: terminal_configs.rs | UBUNTU/src/terminal_configs.rs
//! PURPOSE: Lesson-specific terminal configurations
//! MODIFIED: 2025-12-30
//! LAYER: LEARN → UBUNTU
//! ═══════════════════════════════════════════════════════════════════════════════

use learn_core::{demos::fs_permissions::FsPermissionsDemo, TerminalConfig};

/// Lesson 5: The Terminal - Absolute beginner
pub struct Lesson5Config;

impl TerminalConfig for Lesson5Config {
    fn welcome_message(&self) -> &str {
        "Welcome to the Linux terminal! 🐧\n\
         This is where you control your computer with text commands.\n\
         Start by typing 'ls' to see what files are here."
    }

    fn init_filesystem(&self, demo: &mut FsPermissionsDemo) {
        // Simple filesystem - just one file
        let user_home = demo.cwd;
        let readme_idx = demo.create_file(user_home, "readme.txt", "user", "user", 0o644);
        demo.inodes[readme_idx].content = "Welcome to Ubuntu Linux!\n\
            This is a beginner-friendly operating system.\n\
            Try these commands: ls, pwd, cat readme.txt"
            .to_string();
    }

    fn allowed_commands(&self) -> &[&str] {
        &["ls", "pwd", "cat", "echo", "clear", "help"]
    }

    fn hints(&self) -> &[&str] {
        &[
            "Try: ls",
            "Try: pwd",
            "Try: cat readme.txt",
            "Try: echo Hello World",
        ]
    }
}

/// Lesson 7: Directory Navigation
pub struct Lesson7Config;

impl TerminalConfig for Lesson7Config {
    fn welcome_message(&self) -> &str {
        "Directory Navigation Lab 📁\n\
         Learn to move around the filesystem.\n\
         Goal: Navigate to /etc and list its contents."
    }

    fn init_filesystem(&self, demo: &mut FsPermissionsDemo) {
        let user_home = demo.cwd;

        // Create nested directory structure
        let documents = demo.create_dir(user_home, "documents", "user", "user", 0o755);
        let projects = demo.create_dir(user_home, "projects", "user", "user", 0o755);

        demo.create_file(documents, "notes.txt", "user", "user", 0o644);
        demo.create_file(projects, "app.py", "user", "user", 0o755);

        // Create /etc with some files
        let etc = demo.create_dir(0, "etc", "root", "root", 0o755);
        let hosts = demo.create_file(etc, "hosts", "root", "root", 0o644);
        demo.inodes[hosts].content = "127.0.0.1 localhost".to_string();

        let hostname = demo.create_file(etc, "hostname", "root", "root", 0o644);
        demo.inodes[hostname].content = "ubuntu".to_string();

        // Create /var for practice
        let var = demo.create_dir(0, "var", "root", "root", 0o755);
        let log = demo.create_dir(var, "log", "root", "root", 0o755);
        demo.create_file(log, "syslog", "root", "root", 0o644);
    }

    fn allowed_commands(&self) -> &[&str] {
        &["ls", "cd", "pwd", "cat", "clear", "help"]
    }

    fn hints(&self) -> &[&str] {
        &[
            "Try: cd documents",
            "Try: cd ..",
            "Try: cd /etc",
            "Try: pwd",
        ]
    }
}

/// Lesson 8: File Permissions Deep Dive
pub struct Lesson8Config;

impl TerminalConfig for Lesson8Config {
    fn welcome_message(&self) -> &str {
        "File Permissions Lab 🔐\n\
         Master Unix permissions with chmod and chown.\n\
         Goal: Make script.sh executable (chmod 755 script.sh)"
    }

    fn init_filesystem(&self, demo: &mut FsPermissionsDemo) {
        let user_home = demo.cwd;

        // Create files with varying permissions
        let readme = demo.create_file(user_home, "readme.txt", "user", "user", 0o644);
        demo.inodes[readme].content = "Public file - everyone can read".to_string();

        let secret = demo.create_file(user_home, "secret.txt", "user", "user", 0o600);
        demo.inodes[secret].content = "Private file - only owner can read".to_string();

        let script = demo.create_file(user_home, "script.sh", "user", "user", 0o644);
        demo.inodes[script].content = "#!/bin/bash\necho 'Hello from script'".to_string();

        let shared = demo.create_file(user_home, "shared.txt", "user", "user", 0o664);
        demo.inodes[shared].content = "Group-writable file".to_string();

        // Create a file owned by root
        let system = demo.create_file(user_home, "system.conf", "root", "root", 0o644);
        demo.inodes[system].content = "# System configuration (owned by root)".to_string();
    }

    fn allowed_commands(&self) -> &[&str] {
        &[
            "ls", "cat", "chmod", "chown", "pwd", "su", "whoami", "clear", "help",
        ]
    }

    fn hints(&self) -> &[&str] {
        &[
            "Try: ls -l",
            "Try: chmod 755 script.sh",
            "Try: su root",
            "Try: chown root:root readme.txt",
        ]
    }
}

/// Lesson 9: File Operations
pub struct Lesson9Config;

impl TerminalConfig for Lesson9Config {
    fn welcome_message(&self) -> &str {
        "File Operations Lab 📋\n\
         Learn to create, copy, move, and delete files.\n\
         Goal: Create a 'backup' directory and copy readme.txt into it."
    }

    fn init_filesystem(&self, demo: &mut FsPermissionsDemo) {
        let user_home = demo.cwd;

        let readme = demo.create_file(user_home, "readme.txt", "user", "user", 0o644);
        demo.inodes[readme].content = "Important data to backup!".to_string();

        let temp = demo.create_file(user_home, "temp.txt", "user", "user", 0o644);
        demo.inodes[temp].content = "Temporary file".to_string();

        let old_data = demo.create_file(user_home, "old_data.txt", "user", "user", 0o644);
        demo.inodes[old_data].content = "Outdated information".to_string();
    }

    fn allowed_commands(&self) -> &[&str] {
        &[
            "ls", "pwd", "cat", "mkdir", "touch", "rm", "cp", "mv", "echo", "clear", "help",
        ]
    }

    fn hints(&self) -> &[&str] {
        &[
            "Try: mkdir backup",
            "Try: cp readme.txt backup/",
            "Try: mv temp.txt backup/",
            "Try: rm old_data.txt",
        ]
    }
}

/// Lesson 10: User Management
pub struct Lesson10Config;

impl TerminalConfig for Lesson10Config {
    fn welcome_message(&self) -> &str {
        "User Management Lab 👥\n\
         Understand users, groups, and privileges.\n\
         Goal: Switch to root and explore system files."
    }

    fn init_filesystem(&self, demo: &mut FsPermissionsDemo) {
        let user_home = demo.cwd;

        let note = demo.create_file(user_home, "user-file.txt", "user", "user", 0o644);
        demo.inodes[note].content = "File owned by user".to_string();

        // Create /etc with important files
        let etc = demo.create_dir(0, "etc", "root", "root", 0o755);

        let passwd = demo.create_file(etc, "passwd", "root", "root", 0o644);
        demo.inodes[passwd].content = "root:x:0:0:root:/root:/bin/bash\n\
            user:x:1000:1000:User:/home/user:/bin/bash\n\
            daemon:x:1:1:daemon:/usr/sbin:/usr/sbin/nologin"
            .to_string();

        let shadow = demo.create_file(etc, "shadow", "root", "shadow", 0o640);
        demo.inodes[shadow].content = "root:$6$encrypted$...:18000:0:99999:7:::\n\
            user:$6$encrypted$...:18000:0:99999:7:::"
            .to_string();

        let group = demo.create_file(etc, "group", "root", "root", 0o644);
        demo.inodes[group].content = "root:x:0:\n\
            user:x:1000:\n\
            sudo:x:27:user"
            .to_string();
    }

    fn allowed_commands(&self) -> &[&str] {
        &["ls", "cat", "pwd", "whoami", "id", "su", "clear", "help"]
    }

    fn hints(&self) -> &[&str] {
        &[
            "Try: whoami",
            "Try: id",
            "Try: cat /etc/shadow (will fail)",
            "Try: su root",
            "Try: cat /etc/shadow (now works)",
        ]
    }
}

/// Lesson 11: Package Management (Simulated)
pub struct Lesson11Config;

impl TerminalConfig for Lesson11Config {
    fn welcome_message(&self) -> &str {
        "Package Management Lab 📦\n\
         Learn apt commands (simulated - read-only).\n\
         Note: Commands are simulated for learning."
    }

    fn init_filesystem(&self, demo: &mut FsPermissionsDemo) {
        let user_home = demo.cwd;
        let note = demo.create_file(user_home, "readme.txt", "user", "user", 0o644);
        demo.inodes[note].content = "Package management tutorial.\n\
            Commands are simulated for learning purposes."
            .to_string();
    }

    fn allowed_commands(&self) -> &[&str] {
        &["ls", "cat", "pwd", "echo", "clear", "help"]
    }

    fn hints(&self) -> &[&str] {
        &[
            "Note: apt commands are simulated",
            "Try: echo 'sudo apt update'",
            "Try: echo 'sudo apt install vim'",
        ]
    }
}

/// Lesson 12-20: Default Config (minimal commands)
pub struct DefaultLessonConfig;

impl TerminalConfig for DefaultLessonConfig {
    fn welcome_message(&self) -> &str {
        "Linux Terminal\n\
         Type 'help' for available commands."
    }

    fn init_filesystem(&self, demo: &mut FsPermissionsDemo) {
        let user_home = demo.cwd;
        let readme = demo.create_file(user_home, "readme.txt", "user", "user", 0o644);
        demo.inodes[readme].content = "Welcome to this lesson!".to_string();
    }

    fn allowed_commands(&self) -> &[&str] {
        &["ls", "cat", "pwd", "echo", "clear", "help"]
    }

    fn hints(&self) -> &[&str] {
        &["Try: ls", "Try: cat readme.txt"]
    }
}
