# AArch64 BL31

Rust source project replacing the committed proprietary artifact `bin/rk35/rk3588_bl31_v1.56.elf`.

Latest upstream reference in the imported rkbin snapshot:
- size: `365344` bytes
- SHA-256: `2864c2fc36333e03e7e3fe6459225d320d310850b4ac7620d1f3601efb7d4a9b`
- observed entry/mapping note: `0x60000`

Status: **Stage 7 pending; no drop-in firmware is emitted yet.** This directory is deliberately source-only. The original binary is removed from Git and is used only as reverse-engineering evidence outside the repository.

The project participates in the root Cargo workspace. As reconstruction progresses, functions must be added from documented address-level evidence, with host tests where possible and a bare-metal target only after the entry ABI/link map is established.
