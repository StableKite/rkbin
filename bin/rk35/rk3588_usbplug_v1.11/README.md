# RK3588 USB plug helper

Rust source project replacing the committed proprietary artifact `bin/rk35/rk3588_usbplug_v1.11.bin`.

Latest upstream reference in the imported rkbin snapshot:
- size: `90268` bytes
- SHA-256: `fa38c130f3dd9160895543f0ef83833612963cc428c11f38ec9ecd66ae64d753`
- observed entry/mapping note: `container/header at offset 0`

Status: **Stage 7 pending; no drop-in firmware is emitted yet.** This directory is deliberately source-only. The original binary is removed from Git and is used only as reverse-engineering evidence outside the repository.

The project participates in the root Cargo workspace. As reconstruction progresses, functions must be added from documented address-level evidence, with host tests where possible and a bare-metal target only after the entry ABI/link map is established.
