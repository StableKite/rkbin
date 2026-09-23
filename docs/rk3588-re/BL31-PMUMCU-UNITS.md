# BL31 PMU-MCU `0x17C00` unit discrepancy

The reference uses the same literal in two helpers with different units.

- `sub_6F580(dst, src, 0x17C00)` loops over DWORD indices, therefore copies `0x17C00 * 4 = 0x5F000` bytes.
- `sub_796E8(addr, 0x17C00)` treats the second argument as a byte length for cache clean+invalidate.

Stage 13 intentionally exposes `PMUMCU_COPY_WORDS` and `PMUMCU_CACHE_BYTES` separately. This mismatch is preserved as reference behavior instead of being normalized away.
