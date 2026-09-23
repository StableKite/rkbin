# rk3588_ramboot_null1

Exact source replacement for `bin/rk35/rk3588_ramboot_null1.bin`. The upstream file is 512 zero bytes (SHA-256 `076a27c79e5ace2a3d47f9dd2e83e4ff6ea8872b3c2218f66c92b89b55f36560`), so this component is fully reconstructed as the constant `[0u8; 512]`.

The root `rk3588-xtask` can emit this file byte-for-byte without storing a prebuilt binary in Git.
