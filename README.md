# Micro OS 🖥️

### A Minimal Operating System Kernel Developed Using Rust

Micro OS is an experimental, minimal operating system kernel developed using the Rust programming language. The project explores low-level system programming, kernel development, VGA text-mode output, and kernel logging.

The kernel is designed for the x86_64 architecture and is tested using QEMU emulation.

---

## 🚀 Project Overview

The primary objective of this project is to understand how an operating system kernel is developed from the ground up using Rust.

The project currently implements:

- A freestanding Rust kernel.
- Bootloader integration.
- x86_64 custom target configuration.
- VGA text-mode display.
- Coloured text output.
- A custom kernel logging system.
- QEMU-based kernel execution.

## 🛠️ Technologies Used

| Technology | Purpose |
|---|---|
| Rust | Kernel development |
| Cargo | Package management and building |
| Bootloader | Kernel boot process |
| QEMU | Hardware emulation |
| VGA Buffer | Text-mode display |
| Git & GitHub | Version control |

## ⚙️ Implemented Features

### Layer 1: Kernel Boot

- Configured the Rust nightly toolchain.
- Created a freestanding Rust kernel.
- Integrated the bootloader.
- Generated a bootable kernel image.
- Successfully booted the kernel using QEMU.

### Layer 2: VGA Console and Kernel Logger

- Implemented VGA text-mode output.
- Added VGA colour support.
- Created a custom kernel logger.
- Implemented four logging levels:

| Logging Level | Colour |
|---|---|
| INFO | Green |
| WARN | Yellow |
| ERROR | Red |
| DEBUG | Cyan |

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
│   └── logger.rs
│
├── Cargo.toml
├── Cargo.lock
├── rust-toolchain.toml
├── x86_64-micro_os.json
└── README.md
```

## 💻 Requirements

- Linux environment (Ubuntu)
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

## ▶️ Running the Kernel

Launch the kernel using QEMU:

```bash
qemu-system-x86_64 -drive format=raw,file=target/x86_64-micro_os/debug/bootimage-micro_os.bin
```

## 📌 Development Milestones

| Milestone | Description | Status |
|---|---|---|
| Layer 1 | Kernel boot in QEMU | Completed |
| Layer 2 | VGA text driver | Completed |
| Layer 2 | Coloured kernel logging | Completed |

## 🎯 Future Development

Potential future improvements include:

- Keyboard input handling.
- Interrupt handling.
- Memory management.
- Heap allocation.
- Basic shell implementation.
- Process management.

## 👨‍💻 Author

**Shreyas S**

Project: Micro OS  
Language: Rust  
Architecture: x86_64

---

*An educational project exploring operating system development and low-level programming using Rust.*
