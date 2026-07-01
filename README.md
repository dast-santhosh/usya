<div align="center">
  <img src="./src/assets/logo.bmp" alt="Usya (Dhrishti) Logo" width="300"/>

  # Usya (Dhrishti) Operating System

  *A completely custom, bare-metal operating system kernel in Rust.*

  [![Rust](https://img.shields.io/badge/Rust-Nightly-orange.svg)](#)
  [![Target](https://img.shields.io/badge/Target-x86__64--unknown--none-blue.svg)](#)
  [![License](https://img.shields.io/badge/License-MIT-green.svg)](#)

</div>

---

## 🚀 About The Project

**Usya** (conceptually named *Dhrishti*) is an experimental, bare-metal operating system built entirely in Rust. It bypasses legacy technologies like VGA text mode to provide a modern, pure Framebuffer UI from the very first boot sequence.

### ✨ Features
- **Strictly Bare-Metal:** `#![no_std]` environment running directly on the hardware.
- **Pure Framebuffer UI:** Initializes a clean, high-resolution visual interface immediately.
- **Custom Graphics Engine:**
  - Pure white background.
  - Transparent *GRAY* logo watermark centered on the screen.
  - Custom text rendering engine supporting dark blue, green, and multi-grey text.

---

## 🛠️ Getting Started

### Prerequisites

You need the Rust nightly toolchain and the `bootimage` runner to build the OS.

```bash
rustup override set nightly
rustup component add rust-src llvm-tools-preview
cargo install bootimage
```

### Building & Running

Build the codebase for the custom target:
```bash
cargo build --target x86_64-usya.json
```

Run the OS using QEMU via the `bootimage` runner:
```bash
cargo run
```

---

## 🗺️ Roadmap

- [x] Phase 1: Pure Framebuffer UI (White background, Watermark Logo)
- [ ] Phase 2: Custom Text Rendering Engine
- [ ] Phase 3: Memory Management (Heap Allocator)
- [ ] Phase 4: Hardware Interrupts & IDT

<br />
<div align="center">
  <sub>Built with 🦀 and ❤️ in Rust</sub>
</div>
