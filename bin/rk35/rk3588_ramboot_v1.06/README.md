# RK3588 RAM boot helper

Rust source project replacing the committed proprietary artifact `bin/rk35/rk3588_ramboot_v1.06.bin`.

Latest upstream reference in the imported rkbin snapshot:
- size: `49276` bytes
- SHA-256: `10bbe6e80740cb0b9511cc84a9347cb90dd1915744b6511336d4fb75029c3691`
- observed entry/mapping note: `container/header at offset 0`

Status: **Stage 7 pending; no drop-in firmware is emitted yet.** This directory is deliberately source-only. The original binary is removed from Git and is used only as reverse-engineering evidence outside the repository.

The project participates in the root Cargo workspace. As reconstruction progresses, functions must be added from documented address-level evidence, with host tests where possible and a bare-metal target only after the entry ABI/link map is established.
