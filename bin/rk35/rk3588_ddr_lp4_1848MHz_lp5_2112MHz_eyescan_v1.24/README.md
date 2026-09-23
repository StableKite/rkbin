# rk3588_ddr_lp4_1848MHz_lp5_2112MHz_eyescan_v1.24

Rust source project replacing `bin/rk35/rk3588_ddr_lp4_1848MHz_lp5_2112MHz_eyescan_v1.24.bin`.

Reference artifact: `79443` bytes, SHA-256 `2d521982466aa0de32f30897646ef8912ec8671d4b08a36ece144607cda154de`. Profile: LPDDR4 1848 MHz / LPDDR5 2112 MHz; eyescan=true; single_vdd2=false; mode=`normal`.

This wrapper reuses `bin/rk35/rk3588_ddr_core_v1.24`, which contains the behavior recovered so far from the latest standard DDR v1.24 firmware. For the diagnostic v1.21 artifacts, the wrapper records their distinct mode while additional diagnostic-only behavior remains to be reconstructed.

Status: **source reconstruction in progress; not hardware-tested and not yet a drop-in initializer.**
