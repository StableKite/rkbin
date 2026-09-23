# Stage 14 — larger connected runtime paths

Stage 14 uses existing Stage-4/7 decompiler evidence and does not run IDA.

## rkbin
- BL31: PMU-MCU sleep-tag transfer to `0xFF000C00`, 0x400-byte external-tag bound, 104-byte fallback, restore→start composition.
- BL32: secure-boot OTP enable plus redundant verification policy.
- SPL v1.14: exception-level/vector/CPTR/CPACR/SCR/SCTLR/HCR entry register sequence.
- SPL-v5: its distinct EL entry sequence and conditional `CNTFRQ_EL0` programming.
- USBPlug/RAMBOOT: cross-check the RSAK/header relocation geometry against the recovered runtime mapping.
- PCIe: LTSSM transition tracking followed by the 300 ms / 3000x100 us hot-reset-or-L2 wait.
- DDR core: controller/OTP probe prefix from `ddr_init_sequence@0x770` connected to a training boundary.

## Previous four tracks
- DDR: same `ddr_init_sequence@0x770` probe/training boundary in the standalone reconstruction.
- Mali: register reset + conditional data relocation/BSS clear composed into one reset prefix ending at `0x00803718`.
- BCM43752: independent comma-separated NVRAM integer-array accessor for the unresolved `0x6E80C..0x6E878` behavior family.
- BCM4362A2: HCD application can now be gated on the exact AP6275P structural profile before any PatchRAM writes are emitted.

No Stage-14 hardware routine is marked hardware-tested.
