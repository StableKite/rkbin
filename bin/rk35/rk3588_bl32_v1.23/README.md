# AArch64 BL32

Rust source project replacing the committed proprietary artifact `bin/rk35/rk3588_bl32_v1.23.bin`.

Latest upstream reference in the imported rkbin snapshot:
- size: `530920` bytes
- SHA-256: `cb4ec2d0d8986b22cc55dbcf57c499f10fcca124bcd1e9a5d62cbf9ce0f40636`
- observed entry/mapping note: `unknown`

Status: **Stage 7 pending; no drop-in firmware is emitted yet.** This directory is deliberately source-only. The original binary is removed from Git and is used only as reverse-engineering evidence outside the repository.

The project participates in the root Cargo workspace. As reconstruction progresses, functions must be added from documented address-level evidence, with host tests where possible and a bare-metal target only after the entry ABI/link map is established.
