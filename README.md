# Micro-OS 🚀

### A Minimal Operating System Kernel Developed Using Rust

Micro-OS is an experimental, minimal operating system kernel developed using the Rust programming language. The project explores low-level system programming, kernel development, memory management, keyboard input and command-line interaction.

The kernel is designed for the x86_64 architecture and tested using QEMU emulation.

---

## 🎯 Project Overview

The primary objective is to understand how an operating system kernel can be developed from the ground up using Rust.

## ⚙️ Technologies Used

| Technology | Purpose |
|---|---|
| Rust | Kernel development |
| Cargo | Package management |
| Bootloader | Kernel boot process |
| QEMU | Hardware emulation |
| VGA Buffer | Text-mode display |
| Git & GitHub | Version control |

## ✨ Implemented Features

### Layer 1: Kernel Boot
- Configured Rust nightly toolchain.
- Created a freestanding Rust kernel.
- Integrated the bootloader.
- Generated a bootable kernel image.
- Successfully booted using QEMU.

### Layer 2: VGA Console and Kernel Logger
- Implemented VGA text-mode output.
- Added coloured text support.
- Implemented kernel logging.
- Four logging levels:
  - INFO – Green
  - WARN – Yellow
  - ERROR – Red
  - DEBUG – Cyan

### Layer 3: Keyboard Driver
- Implemented keyboard input using hardware ports.
- Scancode-to-ASCII conversion.
- Shift key support.
- Enter key handling.
- Backspace support.

### Layer 4: Command-Line Interface
- Developed an interactive Micro-OS shell.
- Implemented character input and command processing.
- Added basic commands.

Available commands:

| Command | Function |
|---|---|
| help | Display available commands |
| about | Display OS information |
| version | Display OS version |
| echo | Display custom messages |

### Layer 5: Memory Management
- Implemented a fixed-size 64 KB memory pool.
- Added memory allocation.
- Tracks allocated memory.
- Displays available memory.
- Prevents allocation beyond pool capacity.

Available commands:

| Command | Function |
|---|---|
| mem | Display memory statistics |
| alloc N | Allocate N bytes |

### Layer 6: Round Robin Scheduler Demonstration
- Implemented a basic Round Robin scheduling simulation.
- Created three demonstration tasks.
- Demonstrated sequential task scheduling.
- Displayed task execution rounds.

Available command:

| Command | Function |
|---|---|
| tasks | Run scheduler demonstration |

**Note:** This is a scheduling simulation, not a complete multitasking implementation with CPU context switching.

---

## 📁 Project Structure

```text
micro_os/
│
├── .cargo/
│   └── config.toml
│
├── src/
│   ├── main.rs
│   ├── vga_buffer.rs
│   ├── logger.rs
│   ├── keyboard.rs
│   ├── shell.rs
│   ├── memory.rs
│   └── scheduler.rs
│
├── Cargo.toml
├── Cargo.lock
├── rust-toolchain.toml
├── x86_64-micro_os.json
└── README.md
```

## 💻 Requirements

- Ubuntu Linux environment
- Rust nightly toolchain
- Cargo
- QEMU x86_64 emulator
- Bootimage utility

## 🔧 Build Instructions

Clone the repository:

```bash
git clone https://github.com/Shreyas-krato13/rust-micro-kernel.git
```

Navigate into the project:

```bash
cd rust-micro-kernel
```

Build the kernel:

```bash
cargo build
```

Generate the bootable image:

```bash
cargo bootimage
```

## ▶️ Running Micro-OS

Launch using QEMU:

```bash
qemu-system-x86_64 -drive format=raw,file=target/x86_64-micro_os/debug
