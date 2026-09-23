# USBPlug / RAMBOOT bootstrap header

Both images use the same bootstrap routine at file offset `0x208` and a qword descriptor beginning at `0x320`. The descriptor captures VBAR_EL3, magic `0x4B415351`, the three secondary-core mailbox addresses, secondary entry/state constants, relocation source/destination/end, and the trailing marker file offset. USBPlug and RAMBOOT differ in VBAR, copy end, opaque field, marker offset, and post-relocation target.

The Rust layer models the decision only. It does not execute EL3 system-register writes or secondary-core jumps on the host.
