# rk3588_ddr_lp4_1848MHz_lp5_2112MHz_v1.24

Rust source project replacing `bin/rk35/rk3588_ddr_lp4_1848MHz_lp5_2112MHz_v1.24.bin`.

Reference artifact: `79443` bytes, SHA-256 `7d114862d4958841886afc97cf82affb2bd88bc3b968f636e7e9ed898410c353`. Profile: LPDDR4 1848 MHz / LPDDR5 2112 MHz; eyescan=false; single_vdd2=false; mode=`normal`.

This wrapper reuses `bin/rk35/rk3588_ddr_core_v1.24`, which contains the behavior recovered so far from the latest standard DDR v1.24 firmware. For the diagnostic v1.21 artifacts, the wrapper records their distinct mode while additional diagnostic-only behavior remains to be reconstructed.

Status: **source reconstruction in progress; not hardware-tested and not yet a drop-in initializer.**
