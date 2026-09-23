# rk3588-xtask

Rust build orchestrator for RK3588/RK3588S source replacements. It is the repository's source-build analogue to a CMake top-level driver.

`cargo run -p rk3588-xtask -- status` prints reconstruction state.

`cargo run -p rk3588-xtask -- emit-ready` creates only artifacts whose reconstruction is exact enough to emit. At present that is the two 512-byte RAMBOOT null images. Other targets remain gated rather than generating misleading firmware.
