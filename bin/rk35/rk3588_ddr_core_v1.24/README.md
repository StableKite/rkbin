# RK3588 DDR core v1.24 — Rust reconstruction

Source reconstruction shared by the RK3588/RK3588S DDR v1.24 profiles.

Reference firmware: `rk3588_ddr_lp4_2112MHz_lp5_2400MHz_v1.24.bin`, upstream build id `a4ba73facc`, SHA-256 `2853a0da7ab895af43d50615af73eccf0694115dd0202483ee1c93c9071a42c3`.

Status: **recovered, incomplete, not hardware-tested**. The crate currently contains behavior and layouts established through the reverse-engineering stages; it is not yet a drop-in DDR initializer. New behavior must be tied to address-level evidence before being marked recovered.

Build/test from repository root: `cargo test --workspace`. Bare-metal emission is intentionally gated until the recovered entry path is complete.
