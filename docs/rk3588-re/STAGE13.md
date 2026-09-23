# Stage 13 — connected state and handoff behavior

Stage 13 advances all eleven tracks without a new IDA pass. It only promotes behavior tied to existing Stage-4/7 decompiler evidence.

## rkbin
- BL31: PMU-MCU save/restore copy paths with the binary-exact `0x17C00` unit discrepancy preserved (`DWORD count` for copy, `bytes` for cache maintenance).
- BL32: secure-boot OTP marker enable path and redundant marker read.
- SPL v1.14/v5: execute the already recovered reset plans and return the observed `-EINPROGRESS`/unsupported values.
- USBPlug/RAMBOOT: final transfer is abstracted behind `ControlTransfer`, so the complete recovered bootstrap decision can be exercised without branching on the host.
- PCIe: endpoint vendor/device, generation, lane-width register program and LTSSM summary.
- DDR core: architectural timer initialization (`sub_11424`).
- shared `rk3588-boot-support`: memory-transfer/cache-range/final-transfer interfaces.

## Previous four tracks
- DDR: architectural timer MMIO sequence.
- Mali: conditional initialized-data relocation + BSS clear and SysTick EXC_RETURN dispatch test.
- BCM43752: independent NVRAM `name=value\0` lookup and integer parser; ROM addresses remain behavioral labels, not exact vendor symbols.
- BCM4362A2: exact known AP6275P PatchRAM structural profile (3 regions, 462 writes, 69,895 payload bytes, one sentinel launch).

No hardware-facing Stage-13 routine is marked hardware-tested.
