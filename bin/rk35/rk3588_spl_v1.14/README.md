# AArch64 SPL

Rust source project replacing the committed proprietary artifact `bin/rk35/rk3588_spl_v1.14.bin`.

Latest upstream reference in the imported rkbin snapshot:
- size: `318282` bytes
- SHA-256: `b811b90b6847dc251838e82036c328f00eaacefe94659432265a0ccb0fc8f4e5`
- observed entry/mapping note: `raw offset 0`

Status: **Stage 7 pending; no drop-in firmware is emitted yet.** This directory is deliberately source-only. The original binary is removed from Git and is used only as reverse-engineering evidence outside the repository.

The project participates in the root Cargo workspace. As reconstruction progresses, functions must be added from documented address-level evidence, with host tests where possible and a bare-metal target only after the entry ABI/link map is established.
