//! ═══════════════════════════════════════════════════════════════════════════════
//! FILE: lessons.rs | UBUNTU/src/lessons.rs
//! PURPOSE: Ubuntu/Linux lesson definitions and curriculum structure
//! MODIFIED: 2025-12-30
//! LAYER: LEARN → UBUNTU
//! ═══════════════════════════════════════════════════════════════════════════════

/// Demo type for a lesson
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DemoType {
    /// No interactive demo - story/theory content only
    Static,
    /// Interactive terminal simulator
    Terminal,
    /// Static content with interactive calculator widget
    Calculator,
    /// Terminal with visual diagram (split layout)
    TerminalDiagram,
}

/// A single Ubuntu/Linux lesson
pub struct Lesson {
    pub id: usize,
    pub title: &'static str,
    pub subtitle: &'static str,
    pub icon: &'static str,
    pub phase: &'static str,
    pub demo_type: DemoType,
    pub description: &'static str,
    pub content: &'static str,
    pub key_concepts: &'static [&'static str],
    pub concept_definitions: &'static [(&'static str, &'static str)],
}

/// All Ubuntu lessons organized in phases
pub static LESSONS: &[Lesson] = &[
    // ═══════════════════════════════════════════════════════════════════════════
    // PHASE 0: THE STORY OF LINUX (Theory)
    // ═══════════════════════════════════════════════════════════════════════════
    Lesson {
        id: 0,
        title: "The Birth of Linux",
        subtitle: "A Story of Freedom",
        icon: "🐧",
        phase: "The Story of Linux",
        demo_type: DemoType::Static,
        description: "In 1991, a Finnish student posted a humble message that would change computing forever. This is the story of Linux.",
        content: r#"
## "Just a Hobby"

> "I'm doing a (free) operating system (just a hobby, won't be big and professional like gnu) for 386(486) AT clones."
>
> — Linus Torvalds, August 25, 1991

With these modest words, posted to a Usenet newsgroup, a 21-year-old Finnish student named **Linus Torvalds** announced what would become one of the most important software projects in history.

---

## The Grandfather: Unix (1969)

Our story begins at **Bell Labs** in New Jersey. Ken Thompson, Dennis Ritchie, and their colleagues created **Unix** — an elegant, multi-user operating system written in C. Unix introduced ideas we still use today:

- **Everything is a file** (even devices and processes)
- **Small programs that do one thing well**
- **Pipes to connect programs together**
- **Hierarchical filesystem** (folders within folders)

Unix was brilliant, but it was proprietary. You needed a license. Universities could use it for teaching, but you couldn't freely share or modify it.

---

## The Rebel: Richard Stallman (1983)

**Richard Stallman**, a programmer at MIT, believed software should be free — not free as in "free beer," but free as in **freedom**:

- Freedom to **run** the program
- Freedom to **study** how it works
- Freedom to **share** copies
- Freedom to **improve** and share improvements

In 1983, he launched the **GNU Project** (GNU's Not Unix) to create a completely free Unix-like operating system. By 1991, GNU had most of the pieces: compilers, editors, utilities. But it was missing the heart — the **kernel**.

### What is GNU?

**GNU** is a recursive acronym: **GNU's Not Unix**. It's programmer humor — the definition contains itself, like an infinite loop.

Think of GNU as the **revolutionary cookbook**:
- Unix was like a restaurant with secret recipes
- GNU said "here are all the recipes, cook whatever you want"
- You can modify them, share them, sell dishes made from them
- The only rule: if you improve a recipe, you must share the improvement

GNU created free versions of essential Unix tools:
- **GCC** (compiler) - turns code into programs
- **Bash** (shell) - command interpreter
- **Make** (build tool) - automates compilation
- **Emacs** (editor) - Stallman's legendary text editor
- **coreutils** (ls, cp, mv, cat) - basic commands you'll use every day

> **The Four Freedoms:**
> - **Freedom 0**: Run the program for any purpose
> - **Freedom 1**: Study how it works (source code access)
> - **Freedom 2**: Share copies to help others
> - **Freedom 3**: Improve and share your improvements
>
> These aren't just technical permissions — they're a philosophy about how knowledge should work.

---

## The Student: Linus Torvalds (1991)

At the University of Helsinki, Linus was frustrated. He wanted to learn about operating systems, but **MINIX** (a teaching OS) had restrictive licensing. So he decided to write his own kernel.

Working on his Intel 386 PC, Linus started from scratch. He posted updates online, and something magical happened: **people started contributing**. A developer in Australia fixed a bug. Someone in Germany added a feature. The internet enabled collaboration at a scale never seen before.

The name? Originally "Freax" (free + freak + x from Unix). But the FTP server admin thought that was silly and named the folder "Linux" instead. It stuck.

---

## The Penguin: Tux

Why a penguin? Linus was once bitten by a penguin at a zoo in Australia. He found them amusing — "contstrappedently sitting around and digesting fish."

In 1996, a logo contest was held. Larry Ewing created **Tux** — a chubby, satisfied penguin who has become one of the most recognized mascots in computing.

---

## The Revolution

Linux + GNU tools = a complete, free operating system. The **GPL license** (copyleft) ensured that improvements must be shared back with the community.

Today, Linux powers:
- **90%+ of cloud servers**
- **100% of the top 500 supercomputers**
- **Android phones** (Linux kernel)
- **Smart TVs, routers, cars**
- **NASA's Mars helicopters**

What started as "just a hobby" became the backbone of the modern internet.

---

## The Philosophy

Linux represents a different way of thinking about software:

| Traditional | Open Source |
|-------------|-------------|
| Code is secret | Code is shared |
| Company controls | Community governs |
| Pay for license | Free to use |
| Trust the vendor | Verify yourself |

You're not just learning an operating system. You're joining a movement that believes **knowledge should be free**.
"#,
        key_concepts: &["Unix", "GNU", "Linus Torvalds", "Open Source", "GPL License"],
        concept_definitions: &[
            ("Unix", "Original operating system created at Bell Labs in 1969, grandfather of Linux"),
            ("GNU", "GNU's Not Unix - recursive acronym for project creating free Unix-like tools"),
            ("Linus Torvalds", "Finnish student who created the Linux kernel in 1991 as a hobby project"),
            ("Open Source", "Software with publicly available source code that anyone can study and modify"),
            ("GPL License", "GNU General Public License - ensures software remains free and improvements are shared"),
        ],
    },

    Lesson {
        id: 1,
        title: "What is an Operating System?",
        subtitle: "Kernel, Userspace & Hardware",
        icon: "🧠",
        phase: "The Story of Linux",
        demo_type: DemoType::Static,
        description: "Before we use Linux, let's understand what an operating system actually does. It's the translator between you and the machine.",
        content: r#"
## The Big Picture

An operating system is the **middleman** between you and the raw hardware. Without it, you'd have to manually manage memory addresses, CPU cycles, and disk sectors. The OS abstracts all that complexity away.

---

## The Kernel: Heart of the System

The **kernel** is the core of any operating system. It's the only software that talks directly to hardware.

### What the Kernel Does:
- **Process Management**: Runs multiple programs "simultaneously" (time-slicing)
- **Memory Management**: Gives each program its own memory space
- **Device Drivers**: Translates for hardware (keyboard, disk, GPU)
- **Filesystem**: Organizes data on storage devices
- **Networking**: Handles TCP/IP, sockets, packets

### Linux Kernel Architecture:
```
┌─────────────────────────────────────────────┐
│              USER SPACE                      │
│  ┌─────────┐ ┌─────────┐ ┌─────────┐        │
│  │ Firefox │ │ VS Code │ │ Terminal│  ...   │
│  └────┬────┘ └────┬────┘ └────┬────┘        │
│       │           │           │              │
│  ═════╧═══════════╧═══════════╧═════════════│
│           System Call Interface              │
│  ═══════════════════════════════════════════│
│              KERNEL SPACE                    │
│  ┌──────────┬──────────┬──────────┐         │
│  │ Process  │  Memory  │   VFS    │         │
│  │ Scheduler│ Manager  │(Filesys) │         │
│  └──────────┴──────────┴──────────┘         │
│  ┌──────────────────────────────────┐       │
│  │         Device Drivers           │       │
│  │   (disk, network, GPU, USB...)   │       │
│  └──────────────────────────────────┘       │
└─────────────────────────────────────────────┘
                    │
            ┌───────┴───────┐
            │   HARDWARE    │
            │ CPU RAM DISK  │
            └───────────────┘
```

---

## User Space: Where You Live

Everything above the kernel is **user space**:

- **Shell** (bash, zsh): The command interpreter
- **Desktop Environment** (GNOME, KDE): The graphical interface
- **Applications**: Firefox, LibreOffice, VS Code
- **System Libraries** (glibc): Shared code used by programs

User programs can't touch hardware directly. They make **system calls** to ask the kernel nicely.

---

## Monolithic vs Microkernel

Linux uses a **monolithic kernel**: drivers run inside kernel space for performance.

**Microkernels** (like MINIX, Mach) run drivers in user space — more stable but slower.

```
MONOLITHIC (Linux)          MICROKERNEL (Mach)
┌────────────────┐          ┌────────────────┐
│   User Space   │          │   User Space   │
├────────────────┤          │  ┌──────────┐  │
│                │          │  │ Drivers  │  │
│     Kernel     │          │  └────┬─────┘  │
│   + Drivers    │          ├───────┼────────┤
│                │          │  Microkernel   │
└────────────────┘          └────────────────┘
     Faster                    More Stable
```

---

## Systems Engineering Perspective

Think of your computer as a **bustling city**, and the OS as its **government**:

### The OS as Resource Manager

**CPU Scheduling** → Air Traffic Controller
- Many processes want to run, but there's only a few CPU cores
- The scheduler gives each process tiny time slices (milliseconds)
- You never notice because it switches thousands of times per second
- Priority system: critical processes get more time

**Memory Management** → Librarian
- Limited shelf space (RAM) for many books (programs)
- Virtual memory: each program thinks it has the whole library to itself
- Swap space: store rarely-used books in the basement (disk)
- When a program crashes, the librarian keeps other programs' books safe

**Device Drivers** → Translators
- Your mouse speaks "USB HID protocol"
- Your disk speaks "SATA commands"
- The kernel translates these into a common language programs understand
- Analogy: Like a UN translator converting between languages

### Abstraction Layers

Each layer hides complexity from the layer above:

```
┌──────────────────────────────┐
│   Applications               │ → "Save this file"
├──────────────────────────────┤
│   System Libraries (glibc)   │ → "Write bytes to fd 3"
├──────────────────────────────┤
│   System Calls               │ → write(3, buffer, size)
├──────────────────────────────┤
│   Kernel (VFS Layer)         │ → Translate to filesystem ops
├──────────────────────────────┤
│   Filesystem Driver (ext4)   │ → Translate to block writes
├──────────────────────────────┤
│   Block Device Driver        │ → Send SATA commands
├──────────────────────────────┤
│   Hardware Controller        │ → Physical disk write
└──────────────────────────────┘
```

Each layer knows nothing about the layers below — that's **abstraction**.

### Trade-offs Everywhere

**Performance vs Safety**
- Kernel mode: Fast but dangerous (one bug crashes everything)
- User mode: Slower but safe (crashes isolated to one program)
- Why sudo is powerful: temporarily grants kernel-level access

**Flexibility vs Complexity**
- More features = more code = more bugs
- Linux kernel: ~30 million lines of code
- Every new feature is a security surface

**Isolation vs Efficiency**
- Processes: Completely isolated (safe) but slow to create
- Threads: Share memory (fast) but can interfere with each other

> **The OS is basically a very sophisticated referee** that keeps your programs from fighting over resources like toddlers over toys. It enforces fair sharing, prevents bullying, and sends troublemakers to timeout (kills crashed processes).

---

## "Everything is a File"

This is Linux's most powerful idea. Almost everything is represented as a file:

| Path | What It Represents |
|------|-------------------|
| `/dev/sda` | First hard disk |
| `/dev/null` | Black hole (discards data) |
| `/dev/random` | Random number generator |
| `/proc/cpuinfo` | CPU information |
| `/sys/class/net/` | Network interfaces |

You can read CPU temperature, control LED brightness, or check battery status — all by reading/writing files.

---

## Why This Matters

Understanding the OS architecture helps you:
- **Debug problems**: Know where to look (kernel? driver? application?)
- **Optimize performance**: Understand what's actually happening
- **Appreciate the design**: 50+ years of Unix wisdom
"#,
        key_concepts: &["Kernel", "User Space", "System Calls", "Device Drivers", "Monolithic"],
        concept_definitions: &[
            ("Kernel", "Core of the OS that manages hardware and provides services to programs"),
            ("User Space", "Protected area where applications run without direct hardware access"),
            ("System Calls", "API that programs use to request kernel services (open, read, write, etc.)"),
            ("Device Drivers", "Kernel modules that translate between hardware and OS abstractions"),
            ("Monolithic", "Kernel architecture where drivers run in kernel space for better performance"),
        ],
    },

    Lesson {
        id: 2,
        title: "Linux vs Windows vs macOS",
        subtitle: "Philosophy & Architecture",
        icon: "⚖️",
        phase: "The Story of Linux",
        demo_type: DemoType::Static,
        description: "Three operating systems, three philosophies. Understanding the differences helps you appreciate what makes Linux unique.",
        content: r#"
## The Three Philosophies

| | Linux | Windows | macOS |
|-|-------|---------|-------|
| **Philosophy** | Freedom & Transparency | Convenience & Compatibility | Design & Ecosystem |
| **Motto** | "You own your computer" | "It just works (for most people)" | "It just works (beautifully)" |

---

## Technical Comparison

| Aspect | Linux | Windows | macOS |
|--------|-------|---------|-------|
| **Kernel** | Linux (monolithic, open) | NT (hybrid, closed) | XNU (hybrid, partially open) |
| **Source Code** | Fully open (GPL) | Proprietary | Darwin core is open |
| **Cost** | Free | $100-200 | Free (with Mac hardware) |
| **Default Shell** | Bash/Zsh | PowerShell/CMD | Zsh |
| **Package Manager** | apt, dnf, pacman | Windows Store, winget | Homebrew, App Store |
| **Filesystem** | ext4, btrfs, xfs | NTFS | APFS |
| **Root Access** | Full control | Limited (Admin) | Limited (requires SIP disable) |
| **Updates** | User controlled | Forced/scheduled | Suggested, not forced |
| **Telemetry** | None by default | Extensive | Moderate |

---

## The Kernel Wars

### Linux Kernel
- **Monolithic**: Drivers in kernel space (fast)
- **Open development**: Anyone can read/contribute
- **Modular**: Load/unload drivers dynamically
- **Portable**: Runs on everything from Raspberry Pi to supercomputers

### Windows NT Kernel
- **Hybrid**: Mix of monolithic and microkernel ideas
- **Closed**: Only Microsoft sees the code
- **Backward compatible**: Can run software from decades ago
- **Hardware support**: Great driver availability

### XNU (macOS)
- **Hybrid**: Mach microkernel + BSD Unix layer
- **Darwin**: Core is open-source, but iOS/macOS additions are not
- **Tight integration**: Optimized for Apple hardware only

---

## What Linux Does Differently

### 1. Package Management
No hunting for installers. One command installs software:
```bash
sudo apt install firefox    # Debian/Ubuntu
sudo dnf install firefox    # Fedora
sudo pacman -S firefox      # Arch
```

Thousands of packages in curated repositories. Dependencies handled automatically.

### 2. Customization
Everything is changeable:
- Window manager (GNOME, KDE, i3, bspwm...)
- Shell (bash, zsh, fish...)
- Init system (systemd, OpenRC...)
- Literally any component

Windows and macOS give you themes. Linux lets you rebuild the car.

### 3. Transparency
- Every config is a text file you can edit
- Every log is readable
- Every process is inspectable
- No mystery "something went wrong" errors

### 4. Security Model
- Root access is explicit (sudo)
- No silent background installations
- Open code means bugs are found and fixed publicly
- No telemetry by default

---

## When to Choose Each

| Choose **Linux** if you... | Choose **Windows** if you... | Choose **macOS** if you... |
|---------------------------|-----------------------------|-----------------------------|
| Want full control | Need specific Windows software | Want Apple ecosystem |
| Value privacy | Are a gamer (DirectX) | Do creative work (Final Cut) |
| Are learning computing | Need enterprise compatibility | Prefer curated experience |
| Run servers | Use Microsoft Office heavily | Have money to spare |
| Like customization | Want "it just works" | Want Unix + polish |

---

## The Linux Advantage

Why are we teaching you Linux?

1. **Servers**: 90%+ of cloud runs Linux. Learning it is career-valuable.
2. **Understanding**: See how computers really work.
3. **Freedom**: No licensing fees, no vendor lock-in.
4. **Community**: Decades of documentation, forums, and help.
5. **Future-proof**: Skills transfer across decades of technology.

You're not just learning an OS. You're learning the foundation of modern computing.
"#,
        key_concepts: &["GPL vs Proprietary", "Package Managers", "Customization", "Transparency", "Root Access"],
        concept_definitions: &[
            ("GPL vs Proprietary", "GPL = open source, free to modify; Proprietary = closed, controlled by company"),
            ("Package Managers", "Tools for installing software: apt (Ubuntu), brew (macOS), Windows Store"),
            ("Customization", "Linux lets you change everything; macOS/Windows limit modifications"),
            ("Transparency", "Linux shows exactly what it's doing; others hide system internals"),
            ("Root Access", "Linux gives full system control; macOS/Windows restrict for security"),
        ],
    },

    // ═══════════════════════════════════════════════════════════════════════════
    // PHASE 1: GETTING STARTED
    // ═══════════════════════════════════════════════════════════════════════════
    Lesson {
        id: 3,
        title: "Dual-Boot Installation",
        subtitle: "Ubuntu + Windows",
        icon: "💿",
        phase: "Getting Started",
        demo_type: DemoType::Calculator,
        description: "Install Ubuntu alongside Windows. Keep both operating systems and choose at boot time.",
        content: r#"
## Why Dual-Boot?

Dual-booting lets you keep Windows for gaming/specific software while learning Linux. At startup, you'll see a menu to choose which OS to boot.

---

## Prerequisites Checklist

Before you begin, complete these steps:

- [ ] **Backup your data** — Partitioning can go wrong. External drive or cloud backup.
- [ ] **Disable Windows Fast Startup** — Settings → Power Options → "Choose what power buttons do" → Uncheck "Turn on fast startup"
- [ ] **Check BitLocker** — If enabled, you'll need your recovery key. Disable it temporarily.
- [ ] **Download Ubuntu ISO** — Get the latest LTS from ubuntu.com (~5GB)
- [ ] **Create bootable USB** — Use Rufus (Windows) or balenaEtcher (any OS)
- [ ] **Free up disk space** — Shrink Windows partition (at least 50GB for Ubuntu)

---

## Creating Bootable USB with Rufus (Windows)

**Rufus** is a free, portable tool that creates bootable USB drives. No installation needed.

### Step-by-Step:

1. **Download Rufus** from rufus.ie (portable version, ~1MB)
2. **Insert USB drive** (8GB minimum, will be completely erased)
3. **Launch Rufus** — it auto-detects your USB drive
4. **Configure**:
   - Device: Your USB drive should be auto-selected
   - Boot selection: Click "SELECT" and choose your Ubuntu ISO
   - Partition scheme: **GPT** (for UEFI) or **MBR** (for Legacy BIOS)
     - Modern PCs (2012+): Use GPT
     - Older PCs: Use MBR
   - File system: **FAT32**
   - Volume label: "UBUNTU" (optional)
5. **START** — Rufus warns everything will be erased
6. **Wait 5-10 minutes** — progress bar shows write status
7. **Done** — Safe to eject when Rufus says "READY"

⚠️ **WARNING**: This will ERASE everything on the USB drive. Back up any files first!

> **Fun fact**: Rufus is named after a dog, but it's arguably more reliable than most pets at creating boot drives. It's been creating bootable USBs since 2011.

---

## Understanding Boot Keys

Getting into the boot menu can feel like a scavenger hunt. Here's a cheat sheet:

### Common Boot Keys by Manufacturer:

| Brand | Boot Menu | BIOS Setup | Notes |
|-------|-----------|------------|-------|
| Dell | F12 | F2 | Spam key during Dell logo |
| HP | F9 or Esc | F10 | Some models use Esc |
| Lenovo | F12 | F1 or F2 | ThinkPads may use Enter first |
| ASUS | F8 or Esc | Del or F2 | ROG models often use F8 |
| Acer | F12 | F2 | May need enabling in BIOS |
| MSI | F11 | Del | Gaming laptops |
| Gigabyte | F12 | Del | Motherboards |

**Tip**: When in doubt, **spam all these keys repeatedly** during the logo screen. Timing is finicky — too early doesn't work, too late and you miss it.

### Boot Menu vs BIOS Setup

**Boot Menu** (F12, F9, etc.)
- One-time boot device selection
- Faster — no permanent changes
- Shows list: Windows, USB, Network, etc.
- Recommended for installation

**BIOS Setup** (F2, Del, etc.)
- Permanent configuration
- Can disable Secure Boot, change boot order
- More powerful but requires saving changes

### Typical Boot Menu:

```
┌──────────────────────────────────┐
│  Boot Device Selection            │
│                                   │
│  [1] Windows Boot Manager         │
│  [2] USB: SanDisk Cruzer 8GB   ← │ Select this
│  [3] UEFI: Built-in EFI Shell    │
│  [4] Enter BIOS Setup             │
│                                   │
│  Use ↑↓ to select, Enter to boot │
└──────────────────────────────────┘
```

> Finding your boot key is like a scavenger hunt where the prize is installing Linux. Each manufacturer decided to be unique, naturally.

---

## Shrinking Windows Partition

1. Press `Win + X` → Disk Management
2. Right-click your main partition (usually C:)
3. Select "Shrink Volume"
4. Enter amount to shrink (50000 MB = 50GB minimum)
5. Click "Shrink" — this creates "Unallocated" space

---

## Boot from USB

1. Insert your Ubuntu USB drive
2. Restart and enter BIOS/UEFI (usually F2, F12, Del, or Esc during boot)
3. Disable Secure Boot (optional, but reduces issues)
4. Set USB as first boot device
5. Save and exit — Ubuntu installer should start

---

## Partition Layout

When you select "Install Ubuntu alongside Windows," the installer handles most of this. For manual partitioning:

### Use the partition calculator below to plan your layout:

| Partition | Mount Point | Size | Type | Purpose |
|-----------|-------------|------|------|---------|
| EFI | /boot/efi | 512 MB | FAT32 | Bootloader (may already exist) |
| Root | / | 25-50 GB | ext4 | Operating system and programs |
| Swap | [swap] | RAM size | swap | Virtual memory, hibernation |
| Home | /home | Remaining | ext4 | Your files (documents, config) |

**Tip**: If you have 16GB+ RAM, you can skip swap or make it smaller (4-8GB).

---

## The Bootloader: GRUB

**GRUB** (GRand Unified Bootloader) is what shows you the OS selection menu at startup.

```
┌─────────────────────────────────────┐
│           GNU GRUB                   │
│                                      │
│   Ubuntu                             │
│   Advanced options for Ubuntu        │
│   Windows Boot Manager               │
│   System setup                       │
│                                      │
│  Use ↑↓ to select, Enter to boot    │
└─────────────────────────────────────┘
```

GRUB lives on the EFI partition and is installed automatically.

---

## Post-Installation

After Ubuntu boots for the first time:

1. **Update the system**:
   ```bash
   sudo apt update && sudo apt upgrade
   ```

2. **Install drivers** (if needed):
   Settings → Additional Drivers

3. **Adjust boot order** (optional):
   Default is Ubuntu. To change:
   ```bash
   sudo nano /etc/default/grub
   # Change GRUB_DEFAULT=0 to GRUB_DEFAULT=2 for Windows
   sudo update-grub
   ```

---

## Troubleshooting

### Windows not showing in GRUB?
```bash
sudo update-grub
```

### Black screen after install?
Boot with `nomodeset` kernel parameter. In GRUB, press `e`, add `nomodeset` after `quiet splash`, press F10.

### Clock is wrong in Windows?
Linux uses UTC by default. Fix:
```bash
timedatectl set-local-rtc 1
```

---

## Partition Calculator

Enter your total disk space to get recommended partition sizes:
"#,
        key_concepts: &["GRUB", "EFI Partition", "ext4", "Swap", "Shrink Partition"],
        concept_definitions: &[
            ("GRUB", "GRand Unified Bootloader - shows OS selection menu at startup"),
            ("EFI Partition", "512MB FAT32 partition that stores bootloader for UEFI systems"),
            ("ext4", "Fourth Extended Filesystem - default Linux filesystem, journaled and reliable"),
            ("Swap", "Disk space used as virtual RAM when physical memory is full"),
            ("Shrink Partition", "Reduce Windows partition size to create free space for Ubuntu"),
        ],
    },

    Lesson {
        id: 4,
        title: "First Boot",
        subtitle: "Desktop Tour",
        icon: "🖥️",
        phase: "Getting Started",
        demo_type: DemoType::Static,
        description: "Your first look at Ubuntu. Navigate the desktop, find applications, and open your first terminal.",
        content: r#"
## Welcome to Ubuntu

When Ubuntu boots, you'll see the **GNOME** desktop environment. It's clean, modern, and designed for productivity.

---

## The Desktop Layout

```
┌─────────────────────────────────────────────────┐
│ Activities    [Clock/Date]    [System Tray] ▼  │
├─────────────────────────────────────────────────┤
│                                                 │
│                                                 │
│              Desktop Area                       │
│                                                 │
│                                                 │
│ ┌───┐                                          │
│ │ 📁 │ Files                                   │
│ ├───┤                                          │
│ │ 🦊 │ Firefox                                 │
│ ├───┤                                          │
│ │ ⚙️ │ Settings                                │
│ ├───┤                                          │
│ │ 🛒 │ Software                                │
│ └───┘                                          │
│        [Dock/Favorites Bar]                    │
└─────────────────────────────────────────────────┘
```

---

## Key Areas

### 1. Activities (Top Left)
Click "Activities" or press the **Super key** (Windows key) to:
- See all open windows
- Search for applications
- Switch between workspaces

### 2. System Tray (Top Right)
- **Network**: WiFi and wired connections
- **Sound**: Volume control
- **Power**: Shutdown, restart, suspend
- **Settings**: Quick access to system settings

### 3. Dock (Left Side)
Your favorite applications. Right-click any app icon to add it here.

### 4. File Manager (Nautilus)
Click the Files icon. This is your graphical file browser.

---

## Opening the Terminal

The terminal is your most powerful tool. Three ways to open it:

1. **Keyboard shortcut**: `Ctrl + Alt + T`
2. **Activities search**: Press Super, type "Terminal"
3. **Right-click desktop**: "Open in Terminal" (in some setups)

### Your First Commands

Try these in the terminal:

```bash
# Who am I?
whoami

# Where am I?
pwd

# What's here?
ls

# What's the date?
date

# System information
uname -a
```

---

## Essential Keyboard Shortcuts

| Shortcut | Action |
|----------|--------|
| `Super` | Open Activities / Search |
| `Ctrl + Alt + T` | Open Terminal |
| `Super + L` | Lock screen |
| `Super + D` | Show desktop |
| `Alt + Tab` | Switch windows |
| `Super + Arrow` | Snap window to side |
| `Ctrl + Q` | Close application |
| `Ctrl + Alt + Delete` | Log out menu |

---

## Customization Basics

### Change Wallpaper
Right-click desktop → "Change Background"

### Appearance Settings
Settings → Appearance
- Light/Dark mode
- Accent colors
- Dock position and size

### Install GNOME Tweaks (for more options)
```bash
sudo apt install gnome-tweaks
```

---

## Your First Update

Keep your system secure and up-to-date:

```bash
# Update package lists
sudo apt update

# Upgrade installed packages
sudo apt upgrade

# Or do both in one line
sudo apt update && sudo apt upgrade -y
```

You'll be asked for your password. Type it (nothing will appear — this is normal) and press Enter.

---

## What's Next?

You've booted Ubuntu, explored the desktop, and opened a terminal. In the next lesson, we'll dive deep into the terminal — your window into the true power of Linux.
"#,
        key_concepts: &["GNOME", "Activities", "Terminal", "Super Key", "apt update"],
        concept_definitions: &[
            ("GNOME", "Default Ubuntu desktop environment - clean, modern graphical interface"),
            ("Activities", "Top-left corner button to search apps, files, and switch windows"),
            ("Terminal", "Command-line interface where you type text commands"),
            ("Super Key", "Windows key on keyboard - opens Activities overview"),
            ("apt update", "Command to refresh package list from repositories"),
        ],
    },

    Lesson {
        id: 5,
        title: "The Terminal",
        subtitle: "Your Command Center",
        icon: "⌨️",
        phase: "Getting Started",
        demo_type: DemoType::Terminal,
        description: "The terminal is where Linux power users live. Learn the basics of command-line interaction.",
        content: r#"
## Why the Terminal?

The graphical interface is nice, but the terminal is:
- **Faster**: Type commands instead of clicking through menus
- **Scriptable**: Automate repetitive tasks
- **Consistent**: Works the same across all Linux systems
- **Powerful**: Access features GUIs don't expose
- **Remote**: Works over SSH on servers

---

## Anatomy of a Command

```
┌──────────────────────────────────────────┐
│  user@ubuntu:~$ ls -la /home             │
│  └──┬───────┘ └┘ └─┬─┘ └──┬──┘           │
│     │         │    │      │              │
│   prompt   command flags  argument       │
└──────────────────────────────────────────┘
```

- **Prompt**: Shows user, hostname, current directory
- **Command**: The program to run
- **Flags/Options**: Modify behavior (usually start with -)
- **Arguments**: What the command operates on

---

## Essential Commands

### Navigation & Information
```bash
pwd                 # Print working directory (where am I?)
ls                  # List files and directories
ls -l               # Long format (permissions, size, date)
ls -la              # Include hidden files (starting with .)
cd /path/to/dir     # Change directory
cd ~                # Go to home directory
cd ..               # Go up one level
cd -                # Go to previous directory
```

### Getting Help
```bash
man ls              # Manual page for 'ls'
ls --help           # Quick help for 'ls'
whatis ls           # One-line description
```

### Terminal Control
```bash
clear               # Clear the screen (or Ctrl + L)
history             # Show command history
exit                # Close terminal
```

---

## Keyboard Shortcuts

| Shortcut | Action |
|----------|--------|
| `Tab` | Autocomplete command/path |
| `Tab Tab` | Show all possible completions |
| `↑ / ↓` | Navigate command history |
| `Ctrl + C` | Cancel current command |
| `Ctrl + L` | Clear screen |
| `Ctrl + R` | Search command history |
| `Ctrl + A` | Move cursor to start of line |
| `Ctrl + E` | Move cursor to end of line |
| `Ctrl + U` | Delete from cursor to start |
| `Ctrl + K` | Delete from cursor to end |

---

## The Manual (man pages)

Every command has a manual. Press `q` to exit.

```bash
man ls       # How to use 'ls'
man man      # Manual for the manual!
```

### Sections of a man page:
- **NAME**: What the command is called
- **SYNOPSIS**: How to use it
- **DESCRIPTION**: What it does in detail
- **OPTIONS**: Available flags
- **EXAMPLES**: Usage examples (not always present)

---

## Try It!

Use the terminal below to practice basic commands:

**Suggested commands:**
- `pwd` — see where you are
- `ls` — list files
- `ls -la` — show hidden files with details
- `cd /etc` — change to /etc directory
- `cat /etc/hostname` — display your hostname
- `history` — see what you've typed
- `help` — see available commands
"#,
        key_concepts: &["Prompt", "Commands", "Flags", "man pages", "Tab Completion"],
        concept_definitions: &[
            ("Prompt", "Command interpreter showing username, hostname, and current directory"),
            ("Commands", "Instructions you type to tell the computer what to do"),
            ("Flags", "Options that modify command behavior, like -l in 'ls -l'"),
            ("man pages", "Manual pages - documentation for every Linux command"),
            ("Tab Completion", "Press Tab to autocomplete file/command names - huge time saver"),
        ],
    },

    // ═══════════════════════════════════════════════════════════════════════════
    // PHASE 2: FILESYSTEM FUNDAMENTALS
    // ═══════════════════════════════════════════════════════════════════════════
    Lesson {
        id: 6,
        title: "The Filesystem Hierarchy",
        subtitle: "Everything is a File",
        icon: "🌳",
        phase: "Filesystem Fundamentals",
        demo_type: DemoType::Static,
        description: "Linux organizes files differently than Windows. Learn the standard directory structure used by all Linux distributions.",
        content: r#"
## The Root of Everything

Unlike Windows (C:, D:, E:), Linux has a single **root** directory: `/`

Everything — files, directories, devices, processes — lives under this one tree.

---

## The Filesystem Hierarchy Standard (FHS)

```
/                           # Root — the top of the tree
├── bin/                    # Essential binaries (ls, cp, cat)
├── boot/                   # Bootloader and kernel
├── dev/                    # Device files (disks, terminals)
├── etc/                    # Configuration files
├── home/                   # User home directories
│   └── username/           # Your files live here
├── lib/                    # Shared libraries
├── media/                  # Mounted removable media (USB drives)
├── mnt/                    # Temporary mount points
├── opt/                    # Optional/third-party software
├── proc/                   # Virtual filesystem (process info)
├── root/                   # Root user's home directory
├── run/                    # Runtime data (PIDs, sockets)
├── sbin/                   # System binaries (admin commands)
├── srv/                    # Service data (web server files)
├── sys/                    # Virtual filesystem (hardware info)
├── tmp/                    # Temporary files (cleared on reboot)
├── usr/                    # User programs and data
│   ├── bin/                # User binaries
│   ├── lib/                # User libraries
│   └── share/              # Shared data (docs, icons)
└── var/                    # Variable data (logs, databases)
    └── log/                # System logs
```

---

## Key Directories Explained

### `/home` — Your Home
Your personal space. Configuration, documents, downloads all go here.
- `/home/alice` — Alice's home
- `~` is shorthand for your home directory

### `/etc` — Configuration
System-wide settings. All plain text files.
- `/etc/passwd` — User accounts
- `/etc/hostname` — Computer name
- `/etc/apt/sources.list` — Package repositories

### `/var` — Variable Data
Data that changes during operation.
- `/var/log` — System logs
- `/var/www` — Web server files
- `/var/cache` — Cached data

### `/dev` — Devices
Hardware represented as files.
- `/dev/sda` — First hard disk
- `/dev/sda1` — First partition
- `/dev/null` — Black hole
- `/dev/zero` — Infinite zeros
- `/dev/random` — Random data

### `/proc` and `/sys` — Virtual Filesystems
Not real files! The kernel generates these on-the-fly.
```bash
cat /proc/cpuinfo      # CPU information
cat /proc/meminfo      # Memory information
cat /sys/class/power_supply/BAT0/capacity  # Battery level
```

---

## "Everything is a File"

This is Linux's most powerful abstraction:

| Thing | File Path |
|-------|-----------|
| Hard disk | `/dev/sda` |
| USB drive | `/dev/sdb1` |
| Terminal | `/dev/tty1` |
| Random numbers | `/dev/random` |
| Null (discard) | `/dev/null` |
| Process info | `/proc/[pid]/` |
| CPU temp | `/sys/class/thermal/` |

This means you can use the same tools (cat, echo, read) for everything!

---

## Absolute vs Relative Paths

**Absolute path**: Starts from root (/)
```bash
cd /home/user/Documents
```

**Relative path**: Starts from current location
```bash
cd Documents        # If you're in /home/user
cd ../Downloads     # Go up one, then into Downloads
```

### Special Directories
- `.` — Current directory
- `..` — Parent directory
- `~` — Home directory
- `-` — Previous directory (for cd)

---

## Navigating the Tree

```bash
# Where am I?
pwd

# What's here?
ls

# Go to root
cd /

# Explore!
ls /etc
ls /var/log
cat /etc/os-release

# Go home
cd ~
# or just
cd
```
"#,
        key_concepts: &["Root (/)", "FHS", "/home", "/etc", "/dev", "/var"],
        concept_definitions: &[
            ("Root (/)", "Top of filesystem hierarchy - everything starts here"),
            ("FHS", "Filesystem Hierarchy Standard - consistent layout across Linux distributions"),
            ("/home", "User home directories - your personal files live here"),
            ("/etc", "Configuration files for system and applications"),
            ("/dev", "Device files representing hardware (disks, terminals, etc.)"),
            ("/var", "Variable data - logs, caches, temporary files"),
        ],
    },

    Lesson {
        id: 7,
        title: "Directory Navigation",
        subtitle: "pwd, cd, ls",
        icon: "📁",
        phase: "Filesystem Fundamentals",
        demo_type: DemoType::TerminalDiagram,
        description: "Master the essential commands for moving around the filesystem.",
        content: r#"
## The Big Three

Three commands will get you anywhere:
- `pwd` — **P**rint **W**orking **D**irectory (where am I?)
- `cd` — **C**hange **D**irectory (go somewhere)
- `ls` — **L**i**S**t (what's here?)

---

## pwd — Where Am I?

```bash
pwd
# /home/user/Documents
```

The output is your **absolute path** — your exact location in the filesystem tree.

---

## cd — Change Directory

### Basic Usage
```bash
cd /etc              # Go to /etc (absolute path)
cd Documents         # Go to Documents folder (relative path)
cd ..                # Go up one level
cd ../..             # Go up two levels
cd ~                 # Go to home directory
cd                   # Also goes home (shortcut)
cd -                 # Go to previous directory
```

### Path Building
```bash
# If you're in /home/user
cd Documents/Work/Projects
# Now you're in /home/user/Documents/Work/Projects

cd ../../Personal
# Now you're in /home/user/Documents/Personal
```

---

## ls — List Contents

### Basic Usage
```bash
ls                   # List files and directories
ls /etc              # List specific directory
ls -l                # Long format (details)
ls -a                # Show hidden files (.*)
ls -la               # Long format + hidden files
ls -lh               # Human-readable sizes (KB, MB, GB)
ls -lt               # Sort by time (newest first)
ls -lS               # Sort by size (largest first)
ls -R                # Recursive (all subdirectories)
```

### Understanding Long Format
```
-rw-r--r-- 1 user group  4096 Dec 30 10:30 document.txt
└────┬────┘ │ └──┘ └───┘ └──┘ └────┬─────┘ └─────┬─────┘
     │      │   │    │     │       │             │
  perms   links owner group size   date        name
```

### Hidden Files
Files starting with `.` are hidden by default:
```bash
ls -a
.bashrc  .config  Documents  Downloads  .ssh
```

---

## Practical Navigation Patterns

### Jumping Around
```bash
# Save location, do something, return
pushd /var/log      # Save current dir, go to /var/log
# do stuff...
popd                # Return to saved location
```

### Tab Completion
```bash
cd /ho[TAB]         # Completes to /home/
cd /home/u[TAB]     # Completes to /home/user/
ls /etc/pas[TAB]    # Completes to /etc/passwd
```

### Wildcards
```bash
ls *.txt            # All .txt files
ls *.{jpg,png}      # All .jpg and .png files
ls file?            # file1, file2, fileA, etc.
ls [abc]*           # Files starting with a, b, or c
```

---

## tree — Visual Directory Structure

Not installed by default, but very useful:

```bash
sudo apt install tree

tree                # Current directory tree
tree -L 2           # Only 2 levels deep
tree -d             # Directories only
tree /etc -L 1      # /etc, 1 level
```

Output:
```
.
├── Documents
│   ├── report.pdf
│   └── notes.txt
├── Downloads
└── Pictures
    ├── vacation
    └── family
```

---

## Try It!

Use the terminal below to practice navigation:

**Exercises:**
1. Find out where you are (`pwd`)
2. List everything including hidden files (`ls -la`)
3. Navigate to `/etc` and back home
4. Use `cd -` to toggle between two directories
5. Try tab completion!
"#,
        key_concepts: &["pwd", "cd", "ls -la", "Absolute Path", "Relative Path"],
        concept_definitions: &[
            ("pwd", "Print Working Directory - shows your current location in filesystem"),
            ("cd", "Change Directory - move to different folder"),
            ("ls -la", "List all files including hidden ones with detailed information"),
            ("Absolute Path", "Full path from root: /home/user/file.txt"),
            ("Relative Path", "Path from current directory: ../file.txt or ./folder/file.txt"),
        ],
    },

    Lesson {
        id: 8,
        title: "File Permissions",
        subtitle: "The Unix Security Model",
        icon: "🔐",
        phase: "Filesystem Fundamentals",
        demo_type: DemoType::TerminalDiagram,
        description: "Every file has an owner and permissions. Understanding this system is fundamental to Linux security.",
        content: r#"
## The Big Idea

Linux is a multi-user system. Permissions answer three questions:
1. **WHO** can access this file?
2. **WHAT** can they do with it?
3. **HOW** strict are the rules?

---

## The Three Permission Types

| Letter | Number | Meaning | For Files | For Directories |
|--------|--------|---------|-----------|-----------------|
| r | 4 | Read | View contents | List contents |
| w | 2 | Write | Modify contents | Add/delete files |
| x | 1 | Execute | Run as program | Enter directory |

---

## The Three User Classes

| Class | Symbol | Who |
|-------|--------|-----|
| Owner | u | The user who owns the file |
| Group | g | Users in the file's group |
| Others | o | Everyone else |

---

## Reading Permissions

```bash
ls -l myfile.txt
-rw-r--r-- 1 alice developers 1024 Dec 30 10:00 myfile.txt
```

Breaking down `-rw-r--r--`:
```
 -    rw-    r--    r--
 │    └┬┘    └┬┘    └┬┘
 │     │      │      │
type  owner  group  others

- = regular file
d = directory
l = symbolic link
```

This means:
- **Owner (alice)**: can read and write
- **Group (developers)**: can only read
- **Others**: can only read

---

## Octal Notation

Each permission class is a number from 0-7:

```
 r   w   x
 4 + 2 + 1 = 7 (full access)
 4 + 0 + 0 = 4 (read only)
 4 + 2 + 0 = 6 (read + write)
```

### Common Permission Patterns

| Octal | Symbolic | Meaning |
|-------|----------|---------|
| 755 | rwxr-xr-x | Executable program |
| 644 | rw-r--r-- | Normal document |
| 600 | rw------- | Private file |
| 700 | rwx------ | Private executable |
| 777 | rwxrwxrwx | DANGEROUS! Everyone can do anything |

---

## chmod — Change Permissions

### Octal Method
```bash
chmod 755 script.sh      # rwxr-xr-x
chmod 644 document.txt   # rw-r--r--
chmod 600 secret.key     # rw-------
```

### Symbolic Method
```bash
chmod u+x script.sh      # Add execute for owner
chmod g-w file.txt       # Remove write for group
chmod o-rwx private.txt  # Remove all for others
chmod a+r public.txt     # Add read for all
chmod u=rwx,go=rx file   # Set exact permissions
```

Symbols:
- `+` add permission
- `-` remove permission
- `=` set exactly
- `u` owner, `g` group, `o` others, `a` all

---

## chown — Change Ownership

Only root can change ownership:
```bash
sudo chown alice file.txt              # Change owner
sudo chown alice:developers file.txt   # Change owner and group
sudo chown :developers file.txt        # Change group only
sudo chown -R alice:alice directory/   # Recursive
```

---

## Special Permissions

| Octal | Name | Effect |
|-------|------|--------|
| 4000 | SUID | Run as file owner |
| 2000 | SGID | Run as file group |
| 1000 | Sticky | Only owner can delete |

The `/tmp` directory has the sticky bit:
```bash
ls -ld /tmp
drwxrwxrwt 10 root root 4096 Dec 30 10:00 /tmp
       └── 't' means sticky bit
```

---

## Security Best Practices

1. **Never use 777** — Anyone can modify your files
2. **Private keys should be 600** — SSH will refuse insecure keys
3. **Scripts should be 755** — Readable but owner-modifiable
4. **Home directory should be 755 or 700**

---

## Try It!

Use the terminal to practice permissions:

**Exercises:**
1. `ls -l` to see current permissions
2. Create a file: `touch myfile.txt`
3. Check its default permissions
4. Make it private: `chmod 600 myfile.txt`
5. Try to read `/etc/shadow` (you'll get "Permission denied")
6. Switch to root: `su root` and try again
"#,
        key_concepts: &["rwx", "chmod", "chown", "Octal Notation", "Owner/Group/Others"],
        concept_definitions: &[
            ("rwx", "Read, Write, Execute permissions - the three actions you can control"),
            ("chmod", "Change mode - command to modify file permissions using octal notation"),
            ("chown", "Change owner - command to change file ownership (requires root)"),
            ("Octal Notation", "Base-8 number system for permissions: r=4, w=2, x=1. Example: 755 = rwxr-xr-x"),
            ("Owner/Group/Others", "Three permission classes: file owner, group members, everyone else"),
        ],
    },

    Lesson {
        id: 9,
        title: "File Operations",
        subtitle: "touch, mkdir, rm, cp, mv",
        icon: "📝",
        phase: "Filesystem Fundamentals",
        demo_type: DemoType::Terminal,
        description: "Create, copy, move, and delete files and directories from the command line.",
        content: r#"
## Creating Files and Directories

### touch — Create Empty File (or Update Timestamp)
```bash
touch newfile.txt           # Create empty file
touch file1.txt file2.txt   # Create multiple files
touch existing.txt          # Update modification time
```

### mkdir — Make Directory
```bash
mkdir mydir                 # Create directory
mkdir -p a/b/c/d            # Create parent directories too
mkdir dir1 dir2 dir3        # Create multiple directories
```

---

## Copying

### cp — Copy Files
```bash
cp source.txt dest.txt      # Copy file
cp file.txt /path/to/       # Copy to directory
cp -r mydir/ backup/        # Copy directory recursively
cp -i file.txt backup/      # Ask before overwriting
cp -v *.txt backup/         # Verbose (show what's copied)
```

### Important Flags
- `-r` or `-R` — Recursive (for directories)
- `-i` — Interactive (confirm overwrites)
- `-v` — Verbose
- `-p` — Preserve permissions and timestamps

---

## Moving and Renaming

### mv — Move or Rename
```bash
mv oldname.txt newname.txt  # Rename file
mv file.txt /path/to/       # Move to directory
mv -i source.txt dest.txt   # Ask before overwriting
mv *.txt Documents/         # Move multiple files
mv dir1/ dir2/              # Rename directory
```

Moving and renaming are the same operation in Linux!

---

## Deleting

### rm — Remove Files
```bash
rm file.txt                 # Delete file
rm -r directory/            # Delete directory and contents
rm -f file.txt              # Force (no confirmation)
rm -rf directory/           # Force delete directory (DANGEROUS)
rm -i *.txt                 # Ask for each file
```

### rmdir — Remove Empty Directory
```bash
rmdir empty_dir             # Only works if empty
```

### Warning: rm is Permanent!
There's no trash can. Files are gone forever.
```bash
# NEVER do this:
rm -rf /                    # Deletes everything
rm -rf ~                    # Deletes your home
rm -rf *                    # Deletes current directory contents
```

---

## Viewing File Contents

### cat — Display Entire File
```bash
cat file.txt                # Print file contents
cat file1.txt file2.txt     # Concatenate files
```

### head / tail — View Start or End
```bash
head file.txt               # First 10 lines
head -n 20 file.txt         # First 20 lines
tail file.txt               # Last 10 lines
tail -n 20 file.txt         # Last 20 lines
tail -f logfile             # Follow (watch for new lines)
```

### less — Scrollable Viewer
```bash
less file.txt               # Scroll with arrows
# Press 'q' to quit
# Press '/' to search
# Press 'n' for next match
```

---

## Symbolic Links

### ln — Create Links
```bash
ln -s /path/to/original link_name   # Create symlink
ls -l link_name                     # Shows -> pointing to original
```

Symlinks are like shortcuts. If you delete the original, the link breaks.

---

## File Information

### file — Determine File Type
```bash
file document.pdf           # PDF document
file script.sh              # Bourne-Again shell script
file /bin/ls                # ELF executable
```

### stat — Detailed File Info
```bash
stat file.txt               # Size, permissions, timestamps, inode
```

### du — Disk Usage
```bash
du -h file.txt              # Size of file
du -sh directory/           # Size of directory
du -sh */                   # Size of all subdirectories
```

---

## Try It!

Practice file operations in the terminal:

**Exercises:**
1. Create a directory: `mkdir practice`
2. Create files: `touch practice/file1.txt practice/file2.txt`
3. Copy a file: `cp practice/file1.txt practice/backup.txt`
4. Rename: `mv practice/backup.txt practice/copy.txt`
5. Delete the practice directory: `rm -r practice`
6. View system files: `cat /etc/hostname`
"#,
        key_concepts: &["touch", "mkdir", "cp -r", "mv", "rm -rf", "ln -s"],
        concept_definitions: &[
            ("touch", "Create empty file or update timestamp of existing file"),
            ("mkdir", "Make directory - create new folders in filesystem"),
            ("cp -r", "Copy recursively - copies directories and all their contents"),
            ("mv", "Move or rename files - changes location or name"),
            ("rm -rf", "Remove recursively and force - deletes directories and contents (dangerous!)"),
            ("ln -s", "Create symbolic link - like a shortcut to another file"),
        ],
    },

    // ═══════════════════════════════════════════════════════════════════════════
    // PHASE 3: SYSTEM ADMINISTRATION
    // ═══════════════════════════════════════════════════════════════════════════
    Lesson {
        id: 10,
        title: "User Management",
        subtitle: "Users, Groups & sudo",
        icon: "👥",
        phase: "System Administration",
        demo_type: DemoType::Terminal,
        description: "Linux is a multi-user system. Learn to manage user accounts and understand privilege escalation.",
        content: r#"
## Users in Linux

Every process runs as a user. Every file is owned by a user.

### Key Concepts
- **UID**: User ID (numeric identifier)
- **GID**: Group ID
- **root**: The superuser (UID 0) — can do anything
- **Regular users**: Limited to their own files and permitted actions

---

## User Information

### Who Am I?
```bash
whoami                      # Current username
id                          # UID, GID, and groups
id alice                    # Info about another user
```

### User Database
```bash
cat /etc/passwd             # All user accounts
# format: username:x:UID:GID:comment:home:shell

getent passwd alice         # Specific user info
```

Example line:
```
alice:x:1000:1000:Alice Smith:/home/alice:/bin/bash
└──┬─┘ │ └─┬─┘└─┬─┘└────┬────┘└────┬────┘└───┬───┘
 name  │  UID  GID    comment    home      shell
       └── password in /etc/shadow
```

---

## Groups

Groups allow shared permissions across multiple users.

```bash
groups                      # Your groups
groups alice                # Another user's groups
cat /etc/group              # All groups

# Add user to group (requires sudo)
sudo usermod -aG docker alice    # Add alice to docker group
```

Important groups:
- `sudo` or `wheel` — Can use sudo
- `docker` — Can run Docker without sudo
- `www-data` — Web server user

---

## Managing Users

### Creating Users
```bash
sudo useradd -m bob         # Create user with home directory
sudo useradd -m -s /bin/bash bob  # Also set shell to bash
sudo passwd bob             # Set password
```

### Modifying Users
```bash
sudo usermod -aG sudo bob   # Add bob to sudo group
sudo usermod -l newname oldname   # Rename user
sudo usermod -d /home/newdir bob  # Change home directory
```

### Deleting Users
```bash
sudo userdel bob            # Delete user (keep home)
sudo userdel -r bob         # Delete user AND home directory
```

---

## The sudo Command

`sudo` = "superuser do" — Run one command as root.

```bash
sudo apt update             # Run apt update as root
sudo -i                     # Start a root shell
sudo -u alice command       # Run as user alice
```

### sudoers File
Who can use sudo is defined in `/etc/sudoers`:
```bash
sudo visudo                 # Safe way to edit sudoers
```

Never edit `/etc/sudoers` directly!

---

## Switching Users

### su — Switch User
```bash
su alice                    # Switch to alice (need her password)
su -                        # Switch to root (need root password)
su - alice                  # Switch to alice with fresh environment
```

### sudo su
```bash
sudo su                     # Become root (using YOUR password)
sudo su - alice             # Become alice (no password needed)
```

---

## Password Management

```bash
passwd                      # Change your password
sudo passwd bob             # Change bob's password (as root)
sudo passwd -l bob          # Lock account
sudo passwd -u bob          # Unlock account
```

Password hashes are stored in `/etc/shadow` (readable only by root).

---

## Try It!

Practice user commands in the terminal:

**Exercises:**
1. Check your user info: `whoami` and `id`
2. List all groups you belong to: `groups`
3. View the passwd file: `cat /etc/passwd`
4. Switch to root: `su root` (password in demo)
5. Check you're root: `whoami`
6. Exit root: `exit`
"#,
        key_concepts: &["UID/GID", "sudo", "useradd", "usermod", "/etc/passwd"],
        concept_definitions: &[
            ("UID/GID", "User ID and Group ID - numerical identifiers for users and groups"),
            ("sudo", "SuperUser DO - temporarily run command as root with elevated privileges"),
            ("useradd", "Command to create new user accounts on the system"),
            ("usermod", "Modify existing user accounts - change groups, home directory, shell"),
            ("/etc/passwd", "File containing user account information (not actual passwords)"),
        ],
    },

    Lesson {
        id: 11,
        title: "Package Management",
        subtitle: "apt & Software Installation",
        icon: "📦",
        phase: "System Administration",
        demo_type: DemoType::Terminal,
        description: "Install, update, and remove software using Ubuntu's package manager.",
        content: r#"
## What is a Package Manager?

A package manager is like an app store for the command line:
- **Downloads** software from trusted repositories
- **Installs** it in the right locations
- **Handles dependencies** (libraries it needs)
- **Updates** everything with one command
- **Removes** software cleanly

Ubuntu uses **APT** (Advanced Package Tool).

---

## The apt Command

### Updating Package Lists
```bash
sudo apt update             # Refresh available packages
# This doesn't install anything — just updates the catalog
```

### Upgrading Installed Packages
```bash
sudo apt upgrade            # Upgrade all packages
sudo apt full-upgrade       # Upgrade, including removals if needed
```

### Installing Software
```bash
sudo apt install firefox    # Install Firefox
sudo apt install vim git    # Install multiple packages
sudo apt install ./package.deb  # Install local .deb file
```

### Removing Software
```bash
sudo apt remove firefox     # Remove package (keep config)
sudo apt purge firefox      # Remove package AND config files
sudo apt autoremove         # Remove unused dependencies
```

---

## Finding Packages

### Search
```bash
apt search "text editor"    # Search by keywords
apt search ^vim             # Packages starting with 'vim'
```

### Package Information
```bash
apt show vim                # Details about a package
apt list --installed        # All installed packages
apt list --upgradable       # Packages with updates available
```

### Check if Installed
```bash
dpkg -l | grep vim          # Search installed packages
which vim                   # Find where it's installed
```

---

## Repository Management

Packages come from repositories (repos). Ubuntu has:
- **Main**: Officially supported open source
- **Universe**: Community-maintained open source
- **Restricted**: Proprietary drivers
- **Multiverse**: Software with restrictions

### Adding Repositories
```bash
# Add a PPA (Personal Package Archive)
sudo add-apt-repository ppa:user/repo
sudo apt update

# Add third-party repo manually
sudo add-apt-repository "deb http://example.com/repo stable main"
```

### Repository Configuration
```bash
cat /etc/apt/sources.list              # Main sources
ls /etc/apt/sources.list.d/            # Additional sources
```

---

## Common Tasks

### Update Everything
```bash
sudo apt update && sudo apt upgrade -y
```

### Clean Up
```bash
sudo apt autoremove                    # Remove orphaned packages
sudo apt clean                         # Clear package cache
```

### Fix Broken Packages
```bash
sudo apt --fix-broken install
sudo dpkg --configure -a
```

---

## dpkg — Low-Level Tool

APT uses dpkg under the hood:

```bash
dpkg -l                     # List all installed packages
dpkg -i package.deb         # Install a .deb file
dpkg -r package             # Remove a package
dpkg -L vim                 # List files installed by vim
dpkg -S /usr/bin/vim        # Which package owns this file?
```

---

## Other Package Formats

### Snap
```bash
snap find vlc               # Search
snap install vlc            # Install
snap list                   # List installed snaps
snap remove vlc             # Remove
```

### Flatpak
```bash
flatpak search gimp         # Search
flatpak install flathub org.gimp.GIMP   # Install
flatpak list                # List installed
flatpak uninstall org.gimp.GIMP         # Remove
```

---

## Try It!

Practice package management in the terminal:

**Exercises:**
1. Update package lists: `apt update` (simulated)
2. Search for packages: `apt search editor`
3. Show package info: `apt show vim`
4. Install a package: `apt install htop`
5. List installed: `apt list --installed`
"#,
        key_concepts: &["apt update", "apt install", "apt remove", "dpkg", "Repositories"],
        concept_definitions: &[
            ("apt update", "Refresh package list from repositories - run before installing"),
            ("apt install", "Download and install software packages"),
            ("apt remove", "Uninstall packages but keep configuration files"),
            ("dpkg", "Low-level package manager - apt uses this underneath"),
            ("Repositories", "Servers hosting software packages - Ubuntu's app store"),
        ],
    },

    Lesson {
        id: 12,
        title: "System Services",
        subtitle: "systemctl & Daemons",
        icon: "⚙️",
        phase: "System Administration",
        demo_type: DemoType::Terminal,
        description: "Background services (daemons) keep Linux running. Learn to manage them with systemctl.",
        content: r#"
## What is a Service?

A **service** (or daemon) is a program that runs in the background:
- **nginx**: Web server
- **ssh**: Remote access
- **cron**: Scheduled tasks
- **docker**: Container runtime
- **NetworkManager**: Network connections

They start automatically at boot and run without user interaction.

---

## systemd & systemctl

**systemd** is the init system on modern Linux. It manages:
- Services (daemons)
- Boot process
- Logging
- Mounts
- Timers

**systemctl** is the command to control systemd.

---

## Service Management

### Check Status
```bash
systemctl status nginx              # Service status
systemctl status ssh                # Is SSH running?
systemctl is-active nginx           # Just "active" or "inactive"
systemctl is-enabled nginx          # Starts at boot?
```

### Start / Stop / Restart
```bash
sudo systemctl start nginx          # Start service
sudo systemctl stop nginx           # Stop service
sudo systemctl restart nginx        # Stop + Start
sudo systemctl reload nginx         # Reload config (no downtime)
```

### Enable / Disable (Boot Behavior)
```bash
sudo systemctl enable nginx         # Start at boot
sudo systemctl disable nginx        # Don't start at boot
sudo systemctl enable --now nginx   # Enable AND start now
```

---

## Understanding Status Output

```bash
systemctl status ssh
```

```
● ssh.service - OpenBSD Secure Shell server
     Loaded: loaded (/lib/systemd/system/ssh.service; enabled)
     Active: active (running) since Mon 2024-12-30 10:00:00 UTC
   Main PID: 1234 (sshd)
      Tasks: 1 (limit: 4915)
     Memory: 2.1M
        CPU: 32ms
     CGroup: /system.slice/ssh.service
             └─1234 sshd: /usr/sbin/sshd -D

Dec 30 10:00:00 ubuntu sshd[1234]: Server listening on 0.0.0.0 port 22
```

Key information:
- **Loaded**: Where the service file is, enabled/disabled
- **Active**: Running/stopped and since when
- **Main PID**: Process ID
- **CGroup**: Process hierarchy

---

## Listing Services

```bash
systemctl list-units --type=service              # Running services
systemctl list-units --type=service --all        # All services
systemctl list-unit-files --type=service         # All service files
systemctl list-unit-files --state=enabled        # Services enabled at boot
```

---

## Viewing Logs

**journalctl** shows systemd logs:

```bash
journalctl                          # All logs
journalctl -u ssh                   # Logs for SSH service
journalctl -u nginx -f              # Follow nginx logs (live)
journalctl -u ssh --since "1 hour ago"   # Recent logs
journalctl -u ssh -n 50             # Last 50 lines
journalctl -b                       # Logs since boot
journalctl -p err                   # Only errors
```

---

## Service Files

Services are defined in `.service` files:

```bash
cat /lib/systemd/system/ssh.service
```

```ini
[Unit]
Description=OpenBSD Secure Shell server
After=network.target

[Service]
ExecStart=/usr/sbin/sshd -D
Restart=on-failure

[Install]
WantedBy=multi-user.target
```

Creating custom services is beyond this lesson, but now you understand the structure.

---

## Common Services

| Service | Purpose |
|---------|---------|
| sshd | SSH server (remote login) |
| nginx / apache2 | Web server |
| docker | Container runtime |
| cron | Scheduled tasks |
| cups | Printing |
| NetworkManager | Network management |
| ufw | Firewall |

---

## Try It!

Practice service management in the terminal:

**Exercises:**
1. Check SSH status: `systemctl status ssh`
2. List running services: `systemctl list-units --type=service`
3. Check if a service is enabled: `systemctl is-enabled ssh`
4. View logs: `journalctl -u ssh -n 10`
"#,
        key_concepts: &["systemctl", "daemon", "start/stop/restart", "enable/disable", "journalctl"],
        concept_definitions: &[
            ("systemctl", "System control - command to manage services on systemd-based Linux"),
            ("daemon", "Background service that runs without user interaction (like sshd, nginx)"),
            ("start/stop/restart", "Control service state - start begins, stop ends, restart does both"),
            ("enable/disable", "Control boot behavior - enable starts service at boot, disable prevents it"),
            ("journalctl", "Query systemd journal - view logs for services and system events"),
        ],
    },

    Lesson {
        id: 13,
        title: "Process Management",
        subtitle: "ps, top, kill",
        icon: "🔄",
        phase: "System Administration",
        demo_type: DemoType::Terminal,
        description: "Every running program is a process. Learn to view, monitor, and control them.",
        content: r#"
## What is a Process?

A **process** is a running instance of a program:
- Has a unique **PID** (Process ID)
- Runs as a specific **user**
- Uses **CPU**, **memory**, and other resources
- Can spawn child processes

---

## Viewing Processes

### ps — Process Snapshot
```bash
ps                          # Your current terminal's processes
ps aux                      # All processes, detailed
ps -ef                      # All processes, different format
ps aux | grep firefox       # Find specific process
```

### Understanding ps aux Output
```
USER       PID %CPU %MEM    VSZ   RSS TTY      STAT START   TIME COMMAND
root         1  0.0  0.1 169584 11148 ?        Ss   10:00   0:01 /sbin/init
alice     1234  2.0  3.5 3458920 286844 ?      Sl   10:05   1:20 /usr/bin/firefox
```

- **USER**: Who owns the process
- **PID**: Process ID
- **%CPU / %MEM**: Resource usage
- **STAT**: State (S=sleeping, R=running, Z=zombie)
- **COMMAND**: What's running

### Process States
- **R**: Running
- **S**: Sleeping (waiting for something)
- **D**: Uninterruptible sleep (usually I/O)
- **Z**: Zombie (finished but not cleaned up)
- **T**: Stopped

---

## top — Live Process Monitor

```bash
top                         # Interactive process viewer
```

### top Keyboard Controls
| Key | Action |
|-----|--------|
| `q` | Quit |
| `k` | Kill a process (enter PID) |
| `M` | Sort by memory |
| `P` | Sort by CPU |
| `u` | Filter by user |
| `h` | Help |

### htop — Better Alternative
```bash
sudo apt install htop
htop
```
More colorful and user-friendly!

---

## Killing Processes

### kill — Send Signal to Process
```bash
kill 1234                   # Send SIGTERM (gentle stop)
kill -9 1234                # Send SIGKILL (force kill)
kill -STOP 1234             # Pause process
kill -CONT 1234             # Resume process
```

### Common Signals
| Signal | Number | Effect |
|--------|--------|--------|
| SIGTERM | 15 | Ask nicely to terminate |
| SIGKILL | 9 | Force kill (cannot be caught) |
| SIGSTOP | 19 | Pause |
| SIGCONT | 18 | Resume |
| SIGHUP | 1 | Hangup (often: reload config) |

### killall — Kill by Name
```bash
killall firefox             # Kill all Firefox processes
killall -9 firefox          # Force kill all Firefox
```

### pkill — Kill by Pattern
```bash
pkill -f "python script.py" # Kill process matching pattern
```

---

## Background & Foreground

### Running in Background
```bash
command &                   # Start in background
./long_script.sh &          # Run script in background
```

### Job Control
```bash
jobs                        # List background jobs
fg                          # Bring last job to foreground
fg %1                       # Bring job #1 to foreground
bg                          # Resume stopped job in background
```

### Ctrl Shortcuts
- `Ctrl + C`: Kill foreground process
- `Ctrl + Z`: Suspend (pause) foreground process
- `Ctrl + D`: End input (EOF)

---

## pgrep — Find Process IDs

```bash
pgrep firefox               # PIDs of Firefox processes
pgrep -u alice              # All processes by alice
pgrep -l ssh                # PIDs and names
```

---

## nice & renice — Priority

Lower nice = higher priority (range: -20 to 19)

```bash
nice -n 10 ./cpu_heavy.sh   # Start with lower priority
sudo renice -n -5 1234      # Increase priority of running process
```

---

## Try It!

Practice process management in the terminal:

**Exercises:**
1. View all processes: `ps aux`
2. Find a specific process: `ps aux | grep bash`
3. See your PID: `echo $$`
4. Check process tree: `pstree`
"#,
        key_concepts: &["PID", "ps aux", "top/htop", "kill -9", "Background Jobs"],
        concept_definitions: &[
            ("PID", "Process ID - unique number identifying each running program"),
            ("ps aux", "List all processes with detailed information"),
            ("top/htop", "Real-time process viewer - see CPU/memory usage live"),
            ("kill -9", "Force kill process - SIGKILL signal, cannot be ignored"),
            ("Background Jobs", "Processes running in background with & - terminal remains usable"),
        ],
    },

    // ═══════════════════════════════════════════════════════════════════════════
    // PHASE 4: NETWORKING
    // ═══════════════════════════════════════════════════════════════════════════
    Lesson {
        id: 14,
        title: "Network Basics",
        subtitle: "ip, ping, DNS",
        icon: "🌐",
        phase: "Networking",
        demo_type: DemoType::Terminal,
        description: "Understand how your Linux system connects to networks and the internet.",
        content: r#"
## Network Fundamentals

### Key Concepts
- **IP Address**: Your computer's address on the network
- **Subnet**: A group of IP addresses in the same network
- **Gateway**: The router that connects you to other networks
- **DNS**: Translates domain names to IP addresses
- **Port**: A "door" for specific services (SSH=22, HTTP=80, HTTPS=443)

---

## Checking Your Network

### ip — Modern Network Tool
```bash
ip addr                     # Show all interfaces and IPs
ip addr show eth0           # Show specific interface
ip link                     # Show interface state (up/down)
ip route                    # Show routing table
```

### Understanding ip addr Output
```
2: eth0: <BROADCAST,MULTICAST,UP,LOWER_UP> mtu 1500
    inet 192.168.1.100/24 brd 192.168.1.255 scope global eth0
    inet6 fe80::1/64 scope link
```
- **eth0**: Interface name
- **UP**: Interface is active
- **inet 192.168.1.100/24**: IPv4 address (CIDR notation)
- **inet6**: IPv6 address

### Legacy Commands
```bash
ifconfig                    # Old way (deprecated but common)
route -n                    # Old routing table
```

---

## Testing Connectivity

### ping — Check if Host is Reachable
```bash
ping google.com             # Ping continuously
ping -c 4 google.com        # Ping 4 times and stop
ping 8.8.8.8                # Ping Google's DNS directly
```

### traceroute — Path to Destination
```bash
traceroute google.com       # Show hops to destination
tracepath google.com        # Alternative (no root needed)
```

### mtr — Combines ping + traceroute
```bash
mtr google.com              # Interactive continuous trace
```

---

## DNS Lookup

### dig — Query DNS
```bash
dig google.com              # Full DNS query
dig google.com A            # Just A record (IPv4)
dig google.com +short       # Just the IP
dig @8.8.8.8 google.com     # Use specific DNS server
```

### nslookup — Interactive DNS Tool
```bash
nslookup google.com
```

### host — Simple Lookup
```bash
host google.com             # Forward lookup
host 8.8.8.8                # Reverse lookup
```

---

## DNS Configuration

### /etc/resolv.conf
```bash
cat /etc/resolv.conf
# nameserver 8.8.8.8
# nameserver 8.8.4.4
```

### /etc/hosts
Local hostname resolution (checked before DNS):
```bash
cat /etc/hosts
# 127.0.0.1       localhost
# 192.168.1.50    myserver
```

---

## Understanding Network Ports

### What is a Port?

Think of it this way: If an **IP address** is like a building's street address, a **port** is like an apartment number within that building.

- `192.168.1.100:80` means "building 192.168.1.100, apartment 80"
- The IP gets you to the right computer
- The port gets you to the right service on that computer

Every network connection uses both: `IP:PORT`

### Port Analogy

Imagine a hotel:
- **IP address (192.168.1.100)** → The hotel's street address
- **Port 80 (HTTP)** → Front lobby for web visitors (public)
- **Port 443 (HTTPS)** → Secure lobby with encryption
- **Port 22 (SSH)** → Back door for admins only
- **Port 25 (SMTP)** → Mailroom for outgoing letters
- **Port 3306 (MySQL)** → Database vault in the basement

### Common Ports Reference

| Port | Service | Description | Analogy |
|------|---------|-------------|---------|
| 20/21 | FTP | File transfer | Loading dock |
| 22 | SSH | Secure remote access | Admin's secret entrance |
| 25 | SMTP | Send email | Outgoing mail slot |
| 53 | DNS | Domain name resolution | Reception desk (directory) |
| 80 | HTTP | Web traffic (unencrypted) | Public lobby |
| 443 | HTTPS | Secure web traffic | Secure lobby with guards |
| 110 | POP3 | Receive email | Mailbox pickup |
| 143 | IMAP | Email (modern) | Mail sorting room |
| 3306 | MySQL | Database | Vault |
| 5432 | PostgreSQL | Database | Another vault |
| 6379 | Redis | Cache | Quick-access safe |
| 8080 | HTTP-alt | Development web | Side door developers use |

### Port Ranges

Ports are numbered **0-65535** and divided into categories:

**0-1023: Well-Known Ports**
- Require root/admin to bind
- Reserved for standard services (HTTP, SSH, FTP)
- "The official apartments — can't just move in"

**1024-49151: Registered Ports**
- Used by common applications (MySQL, PostgreSQL)
- Don't require root, but conventionally assigned
- "Regular apartments anyone can rent"

**49152-65535: Dynamic/Private Ports**
- Temporary ports for client connections
- Randomly assigned by OS
- "Hotel rooms for temporary guests"

### Checking Open Ports

**See what's listening on your machine:**

```bash
# Modern way (ss = socket statistics)
sudo ss -tulpn

# Legacy way (netstat)
sudo netstat -tulpn

# Output explains:
# -t = TCP connections
# -u = UDP connections
# -l = Listening ports
# -p = Process name
# -n = Numeric (don't resolve names)
```

Example output:
```
tcp   LISTEN  0  128  0.0.0.0:22    *:*    users:(("sshd",pid=1234))
tcp   LISTEN  0  128  0.0.0.0:80    *:*    users:(("nginx",pid=5678))
```

This shows SSH on port 22 and a web server on port 80.

### Firewall and Ports

```bash
# See which ports are allowed through firewall
sudo ufw status

# Allow a specific port
sudo ufw allow 8080/tcp

# Block a port
sudo ufw deny 23/tcp
```

> **Humor**: Port 80 is like the front door — everyone knows where it is and walks right in. Port 8080 is the side door developers use when port 80 is locked (because someone else's web server is already using it).

---

## Common Network Tools

### wget — Download Files
```bash
wget https://example.com/file.tar.gz
wget -O output.html https://example.com/page
```

### curl — Transfer Data
```bash
curl https://example.com                  # Get page content
curl -O https://example.com/file.tar.gz   # Download file
curl -I https://example.com               # Headers only
curl -X POST -d "data=value" https://api.example.com
```

### netstat / ss — Network Connections
```bash
ss -tuln                    # Listening TCP/UDP ports
ss -tunp                    # With process info
netstat -tuln               # Old way (still common)
```

---

## Network Configuration Files

| File | Purpose |
|------|---------|
| `/etc/hostname` | Machine name |
| `/etc/hosts` | Local DNS |
| `/etc/resolv.conf` | DNS servers |
| `/etc/network/interfaces` | Network config (Debian) |
| `/etc/netplan/*.yaml` | Network config (Ubuntu 18.04+) |

---

## Try It!

Practice network commands in the terminal:

**Exercises:**
1. Check your IP: `ip addr`
2. Test connectivity: `ping -c 3 localhost`
3. Look up DNS: `dig localhost`
4. View routing: `ip route`
"#,
        key_concepts: &["ip addr", "ping", "DNS", "dig", "curl/wget"],
        concept_definitions: &[
            ("ip addr", "Modern command to show network interfaces and IP addresses"),
            ("ping", "Test network connectivity by sending packets to a host"),
            ("DNS", "Domain Name System - converts names like google.com to IP addresses"),
            ("dig", "DNS lookup tool - query nameservers for domain information"),
            ("curl/wget", "Command-line tools for downloading files from the internet"),
        ],
    },

    Lesson {
        id: 15,
        title: "SSH & Remote Access",
        subtitle: "Secure Shell",
        icon: "🔑",
        phase: "Networking",
        demo_type: DemoType::Terminal,
        description: "SSH is how you securely access remote Linux servers. Essential for cloud and server administration.",
        content: r#"
## What is SSH?

**SSH** (Secure Shell) provides encrypted remote access to Linux systems:
- Run commands on remote servers
- Transfer files securely
- Create encrypted tunnels
- No passwords visible on the network

---

## Basic SSH Connection

```bash
ssh user@hostname           # Connect to remote server
ssh alice@192.168.1.100     # Connect with username and IP
ssh alice@server.example.com  # Connect with domain name
ssh -p 2222 user@host       # Non-standard port
```

First connection, you'll see:
```
The authenticity of host 'server (192.168.1.100)' can't be established.
ED25519 key fingerprint is SHA256:abcd1234...
Are you sure you want to continue connecting (yes/no)?
```
Type `yes` to add the server to known hosts.

---

## SSH Key Authentication

Passwords are weak. Keys are better:
1. **Private key**: Stays on your computer (NEVER share!)
2. **Public key**: Goes on servers you want to access

### Generate SSH Key Pair
```bash
ssh-keygen -t ed25519 -C "your_email@example.com"
# or for older systems:
ssh-keygen -t rsa -b 4096 -C "your_email@example.com"
```

Follow prompts:
- Location: `~/.ssh/id_ed25519` (default)
- Passphrase: Optional but recommended

### Copy Public Key to Server
```bash
ssh-copy-id user@server     # Easiest method
# or manually:
cat ~/.ssh/id_ed25519.pub | ssh user@server "mkdir -p ~/.ssh && cat >> ~/.ssh/authorized_keys"
```

Now you can log in without a password!

---

## SSH Config File

Create `~/.ssh/config` for shortcuts:

```bash
Host myserver
    HostName 192.168.1.100
    User alice
    Port 22
    IdentityFile ~/.ssh/id_ed25519

Host work
    HostName work.example.com
    User alice
    ProxyJump jump.example.com
```

Now just:
```bash
ssh myserver                # Uses config settings
ssh work                    # Jumps through proxy
```

---

## SSH Key Agent

The agent remembers your key passphrase:

```bash
# Start agent
eval "$(ssh-agent -s)"

# Add key
ssh-add ~/.ssh/id_ed25519

# List loaded keys
ssh-add -l

# Remove all keys
ssh-add -D
```

Add to `~/.bashrc` to auto-start:
```bash
eval "$(ssh-agent -s)" > /dev/null
ssh-add ~/.ssh/id_ed25519 2> /dev/null
```

---

## Copying Files with SCP/SFTP

### scp — Secure Copy
```bash
scp file.txt user@server:/path/to/destination/
scp user@server:/remote/file.txt /local/path/
scp -r directory/ user@server:/path/    # Copy directory
```

### sftp — Interactive File Transfer
```bash
sftp user@server
# Then use: get, put, ls, cd, lcd, mkdir
```

### rsync — Efficient Sync
```bash
rsync -avz local/ user@server:/remote/   # Sync directories
rsync -avz --delete local/ user@server:/remote/  # Mirror (delete extra)
```

---

## Security Best Practices

### 1. Disable Password Authentication
On server, edit `/etc/ssh/sshd_config`:
```
PasswordAuthentication no
```

### 2. Use Non-Standard Port
```
Port 2222
```

### 3. Limit Users
```
AllowUsers alice bob
```

### 4. Key Permissions
```bash
chmod 700 ~/.ssh
chmod 600 ~/.ssh/id_ed25519
chmod 644 ~/.ssh/id_ed25519.pub
chmod 600 ~/.ssh/authorized_keys
```

---

## Try It!

Practice SSH in the terminal:

**Exercises:**
1. Generate a key pair: `ssh-keygen -t ed25519`
2. View your public key: `cat ~/.ssh/id_ed25519.pub`
3. Check key permissions: `ls -la ~/.ssh/`
"#,
        key_concepts: &["ssh", "ssh-keygen", "ssh-agent", "scp", "~/.ssh/config"],
        concept_definitions: &[
            ("ssh", "Secure Shell - encrypted remote access to another computer"),
            ("ssh-keygen", "Generate SSH key pairs for passwordless authentication"),
            ("ssh-agent", "Holds private keys in memory so you don't re-enter passphrase"),
            ("scp", "Secure Copy - copy files between computers over SSH"),
            ("~/.ssh/config", "Configuration file for SSH connection shortcuts"),
        ],
    },

    Lesson {
        id: 16,
        title: "Firewall Basics",
        subtitle: "ufw & iptables",
        icon: "🛡️",
        phase: "Networking",
        demo_type: DemoType::Terminal,
        description: "Control what network traffic is allowed in and out of your system.",
        content: r#"
## What is a Firewall?

A firewall filters network traffic based on rules:
- **Allow**: Let traffic through
- **Deny**: Block silently
- **Reject**: Block and notify sender

Linux has a powerful firewall (iptables/nftables) built into the kernel. **ufw** (Uncomplicated Firewall) makes it easier to use.

---

## ufw — Simple Firewall

### Check Status
```bash
sudo ufw status             # Status and rules
sudo ufw status verbose     # More details
sudo ufw status numbered    # Rules with numbers
```

### Enable/Disable
```bash
sudo ufw enable             # Turn on firewall
sudo ufw disable            # Turn off firewall
sudo ufw reset              # Reset to defaults
```

---

## Basic Rules

### Allow Traffic
```bash
sudo ufw allow 22           # Allow SSH (port 22)
sudo ufw allow ssh          # Same thing (by service name)
sudo ufw allow 80/tcp       # Allow HTTP (TCP only)
sudo ufw allow 443          # Allow HTTPS
sudo ufw allow 6000:6007/tcp  # Allow port range
```

### Deny Traffic
```bash
sudo ufw deny 23            # Block telnet
sudo ufw deny from 10.0.0.1 # Block specific IP
```

### Delete Rules
```bash
sudo ufw status numbered    # See rule numbers
sudo ufw delete 2           # Delete rule #2
sudo ufw delete allow 80    # Delete by specification
```

---

## Common Scenarios

### Web Server
```bash
sudo ufw allow ssh
sudo ufw allow 80/tcp
sudo ufw allow 443/tcp
sudo ufw enable
```

### Database Server (Internal Only)
```bash
sudo ufw allow from 192.168.1.0/24 to any port 3306
sudo ufw deny 3306
```

### Development Machine
```bash
sudo ufw default deny incoming
sudo ufw default allow outgoing
sudo ufw allow ssh
sudo ufw enable
```

---

## Advanced Rules

### Limit Connections (Brute-Force Protection)
```bash
sudo ufw limit ssh          # Max 6 connections per 30 sec
```

### Specific Interface
```bash
sudo ufw allow in on eth0 to any port 80
```

### Specific IP Range
```bash
sudo ufw allow from 192.168.1.0/24 to any port 22
```

### Logging
```bash
sudo ufw logging on         # Enable logging
sudo ufw logging medium     # Set log level
tail -f /var/log/ufw.log    # View logs
```

---

## Default Policies

```bash
sudo ufw default deny incoming   # Block all incoming
sudo ufw default allow outgoing  # Allow all outgoing
sudo ufw default deny routed     # Block forwarded traffic
```

**Recommended defaults:**
- Deny incoming (whitelist what you need)
- Allow outgoing (you initiated it)

---

## iptables (Advanced)

ufw is a frontend for iptables. Direct iptables is more powerful but complex:

```bash
# View current rules
sudo iptables -L -n -v

# Allow SSH
sudo iptables -A INPUT -p tcp --dport 22 -j ACCEPT

# Block IP
sudo iptables -A INPUT -s 10.0.0.1 -j DROP

# Save rules
sudo iptables-save > /etc/iptables/rules.v4
```

Modern systems are moving to **nftables**, which replaces iptables.

---

## Application Profiles

ufw knows about common applications:
```bash
sudo ufw app list           # See available profiles
sudo ufw app info OpenSSH   # Details about profile
sudo ufw allow "Nginx Full" # Allow Nginx HTTP+HTTPS
```

---

## Try It!

Practice firewall commands in the terminal:

**Exercises:**
1. Check status: `ufw status`
2. List app profiles: `ufw app list`
3. See what SSH rule looks like: `ufw show added`
"#,
        key_concepts: &["ufw", "allow/deny", "iptables", "Port Filtering", "Default Policy"],
        concept_definitions: &[
            ("ufw", "Uncomplicated Firewall - simple interface for managing firewall rules"),
            ("allow/deny", "Firewall actions: allow permits traffic, deny blocks it"),
            ("iptables", "Low-level Linux firewall - ufw is a frontend for this"),
            ("Port Filtering", "Block or allow network traffic on specific ports"),
            ("Default Policy", "What happens to traffic if no rule matches - usually deny"),
        ],
    },

    // ═══════════════════════════════════════════════════════════════════════════
    // PHASE 5: DEVELOPER WORKFLOW
    // ═══════════════════════════════════════════════════════════════════════════
    Lesson {
        id: 17,
        title: "Environment Variables",
        subtitle: "PATH, export, .bashrc",
        icon: "🌿",
        phase: "Developer Workflow",
        demo_type: DemoType::Terminal,
        description: "Environment variables configure how your shell and programs behave.",
        content: r#"
## What are Environment Variables?

Environment variables are **key-value pairs** that affect process behavior:
- Tell programs where to find things
- Configure program behavior
- Pass information between processes

---

## Viewing Variables

```bash
printenv                    # All environment variables
printenv PATH               # Specific variable
echo $PATH                  # Also works
echo $HOME
echo $USER
env                         # Same as printenv
```

### Important Variables

| Variable | Purpose | Example |
|----------|---------|---------|
| `PATH` | Where to find commands | `/usr/bin:/bin` |
| `HOME` | Your home directory | `/home/alice` |
| `USER` | Your username | `alice` |
| `SHELL` | Your shell | `/bin/bash` |
| `EDITOR` | Default text editor | `vim` or `nano` |
| `LANG` | Language/locale | `en_US.UTF-8` |
| `PWD` | Current directory | `/home/alice/project` |
| `TERM` | Terminal type | `xterm-256color` |

---

## Setting Variables

### For Current Session
```bash
MY_VAR="hello"              # Set (not exported)
export MY_VAR="hello"       # Set and export to children
export PATH="$PATH:/new/path"   # Add to existing PATH
```

### Exported vs Non-Exported
```bash
PRIVATE="secret"            # Only this shell sees it
export PUBLIC="visible"     # Child processes see it too

# Test:
echo $PRIVATE               # Works in current shell
bash -c 'echo $PRIVATE'     # Empty (not exported)
bash -c 'echo $PUBLIC'      # "visible" (exported)
```

---

## Making Variables Permanent

### ~/.bashrc — Interactive Non-Login Shells
```bash
# Add to ~/.bashrc
export MY_VAR="permanent"
export PATH="$PATH:$HOME/bin"
alias ll='ls -la'
```

### ~/.profile or ~/.bash_profile — Login Shells
```bash
# Add to ~/.profile
export EDITOR=vim
```

### Reload Configuration
```bash
source ~/.bashrc            # Reload bashrc
. ~/.profile                # Shorthand for source
```

---

## The PATH Variable

`PATH` tells the shell where to find commands:

```bash
echo $PATH
# /usr/local/bin:/usr/bin:/bin:/usr/local/sbin:/usr/sbin

which python                # Shows /usr/bin/python
which ls                    # Shows /usr/bin/ls
```

### Adding to PATH
```bash
# Temporarily
export PATH="$PATH:/opt/myapp/bin"

# Permanently (add to ~/.bashrc)
export PATH="$HOME/.local/bin:$PATH"   # Prepend (higher priority)
export PATH="$PATH:/opt/tools"          # Append
```

### Custom Scripts
```bash
mkdir -p ~/bin
# Put scripts there, add to PATH:
export PATH="$HOME/bin:$PATH"
```

---

## Unsetting Variables

```bash
unset MY_VAR                # Remove variable
```

---

## Startup File Order

When you log in:
1. `/etc/profile` (system-wide)
2. `~/.bash_profile` OR `~/.bash_login` OR `~/.profile`

When you open a terminal:
1. `/etc/bash.bashrc` (system-wide)
2. `~/.bashrc`

### Best Practice
Put everything in `~/.bashrc`, and have `~/.profile` source it:
```bash
# In ~/.profile
if [ -f ~/.bashrc ]; then
    . ~/.bashrc
fi
```

---

## Common Customizations

Add to `~/.bashrc`:
```bash
# Aliases
alias ll='ls -la'
alias la='ls -A'
alias ..='cd ..'
alias ...='cd ../..'
alias grep='grep --color=auto'

# Custom prompt
export PS1='\u@\h:\w\$ '

# Editor
export EDITOR=vim
export VISUAL=vim

# History settings
export HISTSIZE=10000
export HISTFILESIZE=20000
export HISTCONTROL=ignoredups:erasedups
```

---

## Try It!

Practice environment variables in the terminal:

**Exercises:**
1. View your PATH: `echo $PATH`
2. Create a variable: `export MY_NAME="Ubuntu User"`
3. Use it: `echo "Hello, $MY_NAME"`
4. View all variables: `printenv | head -20`
"#,
        key_concepts: &["PATH", "export", ".bashrc", "printenv", "source"],
        concept_definitions: &[
            ("PATH", "Environment variable listing directories where executable programs are found"),
            ("export", "Make variable available to child processes - creates environment variable"),
            (".bashrc", "Script that runs every time you open a terminal - customize your environment"),
            ("printenv", "Print all environment variables - see what's configured"),
            ("source", "Execute script in current shell - apply .bashrc changes without restart"),
        ],
    },

    Lesson {
        id: 18,
        title: "Shell Scripting",
        subtitle: "Bash Basics",
        icon: "📜",
        phase: "Developer Workflow",
        demo_type: DemoType::Terminal,
        description: "Automate tasks by writing shell scripts. Combine commands into reusable programs.",
        content: r#"
## What is a Shell Script?

A shell script is a text file containing commands that run sequentially. Automate anything you do in the terminal!

---

## Your First Script

Create a file `hello.sh`:
```bash
#!/bin/bash
echo "Hello, World!"
```

The `#!` (shebang) tells the system which interpreter to use.

### Make it Executable
```bash
chmod +x hello.sh
./hello.sh               # Run it!
```

Or run directly with bash:
```bash
bash hello.sh
```

---

## Variables

```bash
#!/bin/bash
NAME="Alice"
echo "Hello, $NAME"
echo "Hello, ${NAME}!"   # Braces for clarity

# Command substitution
TODAY=$(date +%Y-%m-%d)
FILES=$(ls | wc -l)
echo "Today is $TODAY, we have $FILES files"
```

### Special Variables
| Variable | Meaning |
|----------|---------|
| `$0` | Script name |
| `$1, $2...` | Arguments |
| `$#` | Number of arguments |
| `$@` | All arguments |
| `$?` | Exit status of last command |
| `$$` | Current process ID |

---

## User Input

```bash
#!/bin/bash
echo "What's your name?"
read NAME
echo "Hello, $NAME!"

# One-liner with prompt
read -p "Enter your age: " AGE
echo "You are $AGE years old"
```

---

## Conditionals

### if/then/else
```bash
#!/bin/bash
if [ -f /etc/passwd ]; then
    echo "File exists"
else
    echo "File not found"
fi
```

### Test Operators

**File tests:**
| Test | True if... |
|------|------------|
| `-f file` | File exists (regular file) |
| `-d dir` | Directory exists |
| `-e path` | Path exists |
| `-r file` | File is readable |
| `-w file` | File is writable |
| `-x file` | File is executable |

**String tests:**
| Test | True if... |
|------|------------|
| `-z "$str"` | String is empty |
| `-n "$str"` | String is not empty |
| `"$a" = "$b"` | Strings are equal |
| `"$a" != "$b"` | Strings differ |

**Numeric tests:**
| Test | True if... |
|------|------------|
| `$a -eq $b` | Equal |
| `$a -ne $b` | Not equal |
| `$a -lt $b` | Less than |
| `$a -gt $b` | Greater than |
| `$a -le $b` | Less or equal |
| `$a -ge $b` | Greater or equal |

---

## Loops

### for Loop
```bash
#!/bin/bash
for i in 1 2 3 4 5; do
    echo "Number: $i"
done

# Range
for i in {1..10}; do
    echo "Count: $i"
done

# Files
for file in *.txt; do
    echo "Processing $file"
done
```

### while Loop
```bash
#!/bin/bash
COUNT=0
while [ $COUNT -lt 5 ]; do
    echo "Count: $COUNT"
    COUNT=$((COUNT + 1))
done
```

### Reading Lines
```bash
#!/bin/bash
while read line; do
    echo "Line: $line"
done < /etc/passwd
```

---

## Functions

```bash
#!/bin/bash
greet() {
    echo "Hello, $1!"
}

greet "Alice"
greet "Bob"

# With return value
is_even() {
    if [ $(($1 % 2)) -eq 0 ]; then
        return 0   # True
    else
        return 1   # False
    fi
}

if is_even 4; then
    echo "4 is even"
fi
```

---

## Practical Examples

### Backup Script
```bash
#!/bin/bash
DATE=$(date +%Y%m%d)
tar -czf backup_$DATE.tar.gz /home/$USER/Documents
echo "Backup created: backup_$DATE.tar.gz"
```

### System Info
```bash
#!/bin/bash
echo "=== System Info ==="
echo "Hostname: $(hostname)"
echo "Uptime: $(uptime -p)"
echo "Disk Usage: $(df -h / | tail -1 | awk '{print $5}')"
echo "Memory: $(free -h | grep Mem | awk '{print $3"/"$2}')"
```

---

## Try It!

Practice scripting in the terminal:

**Exercises:**
1. Create a simple script: `echo '#!/bin/bash' > test.sh`
2. Add a command: `echo 'echo "Hello"' >> test.sh`
3. Make executable: `chmod +x test.sh`
4. Run it: `./test.sh`
"#,
        key_concepts: &["Shebang", "Variables", "if/then", "for/while", "Functions"],
        concept_definitions: &[
            ("Shebang", "#!/bin/bash at top of script - tells system which interpreter to use"),
            ("Variables", "Named storage for values - no spaces around = sign"),
            ("if/then", "Conditional execution - run code only if condition is true"),
            ("for/while", "Loops - repeat commands multiple times with different values"),
            ("Functions", "Reusable blocks of code with parameters - like mini-programs"),
        ],
    },

    // ═══════════════════════════════════════════════════════════════════════════
    // PHASE 6: MAINTENANCE
    // ═══════════════════════════════════════════════════════════════════════════
    Lesson {
        id: 19,
        title: "Troubleshooting & Logs",
        subtitle: "journalctl, dmesg, /var/log",
        icon: "🔍",
        phase: "Maintenance",
        demo_type: DemoType::Terminal,
        description: "When things go wrong, logs tell you why. Learn to find and read system logs.",
        content: r#"
## Where to Look When Things Break

Linux logs everything. When something fails, the answer is usually in a log file.

---

## journalctl — Systemd Logs

Modern logging system for systemd:

```bash
journalctl                      # All logs
journalctl -b                   # Since last boot
journalctl -b -1                # Previous boot
journalctl --since "1 hour ago" # Recent logs
journalctl --since today        # Today's logs
journalctl -p err               # Only errors
journalctl -f                   # Follow (live)
```

### Service-Specific Logs
```bash
journalctl -u ssh               # SSH service logs
journalctl -u nginx -f          # Follow nginx logs
journalctl -u docker --since "10 min ago"
```

### Priority Levels
| Level | Name | Meaning |
|-------|------|---------|
| 0 | emerg | System is unusable |
| 1 | alert | Immediate action needed |
| 2 | crit | Critical conditions |
| 3 | err | Error conditions |
| 4 | warning | Warning conditions |
| 5 | notice | Normal but significant |
| 6 | info | Informational |
| 7 | debug | Debug messages |

```bash
journalctl -p warning           # Warnings and above
journalctl -p 0..4              # Emergency through error
```

---

## dmesg — Kernel Messages

The kernel logs hardware events, driver messages, and boot info:

```bash
dmesg                           # All kernel messages
dmesg | tail -20                # Last 20 lines
dmesg -T                        # Human-readable timestamps
dmesg -T | grep -i error        # Find errors
dmesg -T | grep -i usb          # USB events
dmesg -w                        # Follow (watch for new)
```

### Common Uses
- **Hardware issues**: Disk errors, USB problems
- **Driver loading**: Module load failures
- **Boot problems**: What happened during startup

---

## /var/log — Traditional Logs

Classic log files in `/var/log`:

| File | Contents |
|------|----------|
| `syslog` | General system messages |
| `auth.log` | Authentication (logins, sudo) |
| `kern.log` | Kernel messages |
| `dpkg.log` | Package installations |
| `apt/history.log` | APT operations |
| `boot.log` | Boot messages |
| `nginx/` | Web server logs |
| `mysql/` | Database logs |

### Reading Logs
```bash
cat /var/log/syslog             # Full file
tail -20 /var/log/syslog        # Last 20 lines
tail -f /var/log/syslog         # Follow live
less /var/log/syslog            # Scrollable viewer
grep error /var/log/syslog      # Find errors
```

### Authentication Logs
```bash
# Who logged in?
tail /var/log/auth.log

# Failed login attempts
grep "Failed password" /var/log/auth.log

# sudo usage
grep sudo /var/log/auth.log
```

---

## Common Troubleshooting Patterns

### 1. Service Won't Start
```bash
sudo systemctl status myservice
journalctl -u myservice -n 50
# Look for error messages
```

### 2. Application Crashes
```bash
dmesg | tail -20                # Kernel messages
journalctl -b | grep -i "myapp"
# Check app's own logs if it has them
```

### 3. Disk Problems
```bash
dmesg | grep -i "error\|fail"
journalctl -p err               # All errors
df -h                           # Check disk space
```

### 4. Network Issues
```bash
journalctl -u NetworkManager
ping 8.8.8.8                    # Can we reach internet?
ip addr                         # Do we have an IP?
```

### 5. Login Problems
```bash
tail /var/log/auth.log
journalctl -u ssh
```

---

## Disk Space for Logs

Logs can fill up your disk:

```bash
du -sh /var/log                 # Total log size
ls -lhS /var/log/*.log | head   # Largest log files

# Clean old journals
sudo journalctl --vacuum-time=7d   # Keep 7 days
sudo journalctl --vacuum-size=500M # Keep 500MB max
```

---

## Useful Tools

### less Navigation
| Key | Action |
|-----|--------|
| `q` | Quit |
| `/pattern` | Search forward |
| `?pattern` | Search backward |
| `n` | Next match |
| `N` | Previous match |
| `g` | Go to start |
| `G` | Go to end |
| `F` | Follow mode |

### grep Patterns
```bash
grep -i error file.log          # Case insensitive
grep -C 3 error file.log        # 3 lines context
grep -v DEBUG file.log          # Exclude DEBUG lines
grep -E "error|fail" file.log   # Multiple patterns
zgrep error file.log.gz         # Search compressed
```

---

## Try It!

Practice log investigation in the terminal:

**Exercises:**
1. View kernel messages: `dmesg | head -20`
2. Check for errors: `dmesg | grep -i error`
3. View systemd journal: `journalctl -n 20`
4. Check auth log: `cat /var/log/auth.log 2>/dev/null || echo "Permission denied"`
"#,
        key_concepts: &["journalctl", "dmesg", "/var/log", "tail -f", "Log Priorities"],
        concept_definitions: &[
            ("journalctl", "Query systemd journal - centralized logging system for services"),
            ("dmesg", "Display kernel ring buffer - hardware and driver messages"),
            ("/var/log", "Directory containing log files for system and applications"),
            ("tail -f", "Follow file in real-time - shows new lines as they're written"),
            ("Log Priorities", "Severity levels: debug, info, notice, warning, error, critical"),
        ],
    },
];

/// Get a lesson by its ID
pub fn get_lesson(id: usize) -> Option<&'static Lesson> {
    LESSONS.iter().find(|l| l.id == id)
}

/// Get all lessons in a phase
pub fn get_lessons_by_phase(phase: &str) -> Vec<&'static Lesson> {
    LESSONS.iter().filter(|l| l.phase == phase).collect()
}

/// All unique phase names in order
pub static PHASES: &[&str] = &[
    "The Story of Linux",
    "Getting Started",
    "Filesystem Fundamentals",
    "System Administration",
    "Networking",
    "Developer Workflow",
    "Maintenance",
];
