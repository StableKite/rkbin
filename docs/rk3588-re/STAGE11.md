# Stage 11 — state machines and MMIO boundaries

Stage 11 moves from isolated pure helpers to bounded, testable state machines behind small MMIO/delay traits.

## rkbin
- BL31: exact PMU2 bus-idle request/poll and GPIO level write plan.
- BL32: bounded OTP controller read transaction.
- SPL v1.14/v5: binary-exact sysreset write selection; the two builds deliberately differ.
- USBPlug/RAMBOOT: typed 0x320 bootstrap headers and boot/secondary-core entry plans.
- PCIe: controller/PHY bring-up register sequence, 500-iteration lock poll, and persisted retry decision; actual reset remains outside the Rust function.
- DDR core: ZQ warning classification.

## Previous four tracks
- DDR: ZQ warning classifier.
- Mali: MPU region-0 program and SysTick scheduler transition.
- BCM43752: PHY register read/write/modify behavioral ABI boundary.
- BCM4362A2: transport-independent HCD PatchRAM application state machine.

All hardware-facing code remains marked reconstructed rather than hardware-tested.
