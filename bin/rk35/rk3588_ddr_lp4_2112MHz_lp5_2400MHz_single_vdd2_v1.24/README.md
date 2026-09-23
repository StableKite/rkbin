# rk3588_ddr_lp4_2112MHz_lp5_2400MHz_single_vdd2_v1.24

Rust source project replacing `bin/rk35/rk3588_ddr_lp4_2112MHz_lp5_2400MHz_single_vdd2_v1.24.bin`.

Reference artifact: `79443` bytes, SHA-256 `966ce9b7167c7432a225506df92d024cba4b7b15b45635557f45f09c9a69119b`. Profile: LPDDR4 2112 MHz / LPDDR5 2400 MHz; eyescan=false; single_vdd2=true; mode=`normal`.

This wrapper reuses `bin/rk35/rk3588_ddr_core_v1.24`, which contains the behavior recovered so far from the latest standard DDR v1.24 firmware. For the diagnostic v1.21 artifacts, the wrapper records their distinct mode while additional diagnostic-only behavior remains to be reconstructed.

Status: **source reconstruction in progress; not hardware-tested and not yet a drop-in initializer.**
