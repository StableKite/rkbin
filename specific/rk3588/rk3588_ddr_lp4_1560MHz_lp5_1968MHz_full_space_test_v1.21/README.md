# rk3588_ddr_lp4_1560MHz_lp5_1968MHz_full_space_test_v1.21

Rust source project replacing `specific/rk3588/rk3588_ddr_lp4_1560MHz_lp5_1968MHz_full_space_test_v1.21.bin`.

Reference artifact: `58928` bytes, SHA-256 `780e9d32aa4c5e11fee4e00e3b53d6079dd9b404cc7f276faba3e8b0db5713cf`. Profile: LPDDR4 1560 MHz / LPDDR5 1968 MHz; eyescan=false; single_vdd2=false; mode=`full-space-test`.

This wrapper reuses `bin/rk35/rk3588_ddr_core_v1.24`, which contains the behavior recovered so far from the latest standard DDR v1.24 firmware. For the diagnostic v1.21 artifacts, the wrapper records their distinct mode while additional diagnostic-only behavior remains to be reconstructed.

Status: **source reconstruction in progress; not hardware-tested and not yet a drop-in initializer.**
