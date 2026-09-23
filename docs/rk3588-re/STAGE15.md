# Stage 15 — composed boot/runtime subsystems

Stage 15 does not run IDA. It composes previously recovered binary-derived primitives into larger, testable boundaries while keeping composition-level confidence explicit.

## rkbin
- BL31: restore-first PMU-MCU resume policy with wake-tag fallback (reconstructed composition).
- BL32: secure-boot marker policy combined with the recovered OTP security-level decoder; exact security-byte offsets remain caller-supplied.
- SPL v1.14 / SPL-v5: entry-state programming can be composed with arrays of recovered RK3588 pinctrl operations.
- PCIe: mode prelude -> PHY lock/retry -> endpoint identity/width -> LTSSM link wait -> hot-reset/L2 wait.
- USBPlug/RAMBOOT: layout-gated bootstrap execution before final control transfer.
- DDR core: architectural timer -> debug UART -> controller probe -> training boundary.
- shared boot support: bounded MMIO polling primitive.

## Previous four tracks
- DDR: same timer -> UART -> probe/training composition.
- Mali: reset prefix composed with first recovered scheduler-tick transition.
- BCM43752: typed no-allocation NVRAM view and explicit behaviorally resolved ROM-boundary table.
- BCM4362A2: validated AP6275P patch-program object; profile validation is separated from transport application.

No Stage-15 hardware path is marked hardware-tested.
