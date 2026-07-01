# Usya (Dhrishti) Operating System

A completely custom, bare-metal operating system kernel in Rust named Usya (the OS itself is conceptually named Dhrishti). It is built for the `x86_64-unknown-none` target architecture.

## Phase 1 Goals
- Initialize a pure Framebuffer UI, bypassing legacy VGA text mode entirely.
- Implement a pure white background.
- Implement a custom text rendering engine that supports dark blue, green, and multi-grey text.
- Feature a transparent 'GRAY' logo watermark in the center.

## Build Requirements
- Rust nightly toolchain
- `bootimage` tool

This codebase is strictly bare-metal Rust (`#![no_std]`).
