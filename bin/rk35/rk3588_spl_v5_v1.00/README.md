# AArch64 SPL v5

Rust source project replacing the committed proprietary artifact `bin/rk35/rk3588_spl_v5_v1.00.bin`.

Latest upstream reference in the imported rkbin snapshot:
- size: `288981` bytes
- SHA-256: `9adef27f338b202dd8affefd0a3f86d13123b470e9a0e98c04fc87c5b23438dc`
- observed entry/mapping note: `raw offset 0`

Status: **Stage 7 pending; no drop-in firmware is emitted yet.** This directory is deliberately source-only. The original binary is removed from Git and is used only as reverse-engineering evidence outside the repository.

The project participates in the root Cargo workspace. As reconstruction progresses, functions must be added from documented address-level evidence, with host tests where possible and a bare-metal target only after the entry ABI/link map is established.
