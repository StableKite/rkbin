# RK3588 / RK3588S source reconstruction

This tree preserves upstream `rkbin` content for other SoCs unchanged. RK3588-family proprietary prebuilt firmware artifacts are removed from the working tree and replaced by Rust source projects located beside the paths they replace.

## Policy

- Do not commit generated RK3588 `.bin`/`.elf` artifacts.
- Reconstruct against the latest upstream artifact for each current component; older versions are used only as differential evidence when needed.
- Keep existing `RKBOOT`/`RKTRUST` path conventions. When a component becomes complete, `rk3588-xtask` emits the expected artifact name at the original path.
- Each recovered behavior must have provenance/evidence in its component README or RE notes.
- `Recovered` does not mean `HardwareTested`. Firmware emission remains gated until the relevant entry ABI and hardware behavior are sufficiently reconstructed.

## Current state

DDR v1.24 contains the Stage-6 Rust reconstruction and host tests. The two RAMBOOT null blobs are exact source-generated replacements. BL31 v1.56, BL32 v1.23, SPL v1.14, SPL-v5 v1.00, USBPlug v1.11, RAMBOOT v1.06 and PCIe v3.20 have source projects and reference provenance but still require deep reverse engineering before firmware emission.

Run `cargo test --workspace` for host-side tests and `cargo run -p rk3588-xtask -- status` for the current gate state.

## Licensing

The newly added RK3588/RK3588S Rust reconstruction files are licensed under the MIT license in `LICENSES/MIT-RK3588-RUST.txt`. Existing upstream files retain their existing licensing and notices.
