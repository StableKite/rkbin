# AArch64 PCIe EP loader

Rust source project replacing the committed proprietary artifact `bin/rk35/rk3588_pcie_v3.20.bin`.

Latest upstream reference in the imported rkbin snapshot:
- size: `6560` bytes
- SHA-256: `a63ade015a1969f7f50be362e0809b0b13d160e4e8d0582fdda4a2d1126aa0ce`
- observed entry/mapping note: `raw offset 0`

Status: **Stage 7 pending; no drop-in firmware is emitted yet.** This directory is deliberately source-only. The original binary is removed from Git and is used only as reverse-engineering evidence outside the repository.

The project participates in the root Cargo workspace. As reconstruction progresses, functions must be added from documented address-level evidence, with host tests where possible and a bare-metal target only after the entry ABI/link map is established.
