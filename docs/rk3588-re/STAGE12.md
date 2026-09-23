# Stage 12 — executable boot-flow slices

Stage 12 connects the Stage-11 MMIO primitives into larger, bounded pieces of the reference flows.

## rkbin
- shared `rk3588-boot-support`: execute USBPlug/RAMBOOT bootstrap through relocation or secondary-core mailbox handoff, stopping before the final branch; GRF field writer.
- BL31: PMU-MCU idle wait and start register/poll sequence; recovered embedded-MCU image descriptor.
- BL32: byte-granular OTP reads reconstructed on top of the 16-word base transaction and redundant secure-boot marker decoding.
- SPL v1.14/v5: actual raw pull/drive/schmitt GRF writes using the binary-derived tables.
- USBPlug/RAMBOOT: component entry wrappers over the common bootstrap executor.
- PCIe: mode-2 prelude register sequence.
- DDR core: exact debug UART initialization sequence.

## Previous four tracks
- DDR: exact debug UART register sequence.
- Mali: early reset register/MPU/cache program (excluding the unresolved auxiliary-control helper and relocation branch).
- BCM43752: correction of Stage-11 ROM register ABI classification and explicit A/B modify-family boundary.
- BCM4362A2: no-alloc HCD region/layout analyzer.

No hardware-facing Stage-12 routine is marked hardware-tested.
