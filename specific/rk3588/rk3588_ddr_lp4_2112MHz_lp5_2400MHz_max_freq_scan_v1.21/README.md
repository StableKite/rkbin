# rk3588_ddr_lp4_2112MHz_lp5_2400MHz_max_freq_scan_v1.21

Rust source project replacing `specific/rk3588/rk3588_ddr_lp4_2112MHz_lp5_2400MHz_max_freq_scan_v1.21.bin`.

Reference artifact: `63376` bytes, SHA-256 `eebfabf3aee2b355c680b046217b48691810631d431efa5d4a0dd41eaf805950`. Profile: LPDDR4 2112 MHz / LPDDR5 2400 MHz; eyescan=false; single_vdd2=false; mode=`max-freq-scan`.

This wrapper reuses `bin/rk35/rk3588_ddr_core_v1.24`, which contains the behavior recovered so far from the latest standard DDR v1.24 firmware. For the diagnostic v1.21 artifacts, the wrapper records their distinct mode while additional diagnostic-only behavior remains to be reconstructed.

Status: **source reconstruction in progress; not hardware-tested and not yet a drop-in initializer.**
