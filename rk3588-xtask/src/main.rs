use std::{env, fs, path::Path};
fn main() {
    let cmd = env::args().nth(1).unwrap_or_else(|| "status".into());
    match cmd.as_str() {
        "status" => {
            println!(
                "RK3588 reconstruction: DDR=partial; BL31/BL32/SPL/USBPlug/RamBoot/PCIe=analysis pending; RAMBOOT null0/null1=exact"
            );
        }
        "emit-ready" => {
            emit(
                "bin/rk35/rk3588_ramboot_null0.bin",
                &rk3588_ramboot_null0::IMAGE,
            );
            emit(
                "bin/rk35/rk3588_ramboot_null1.bin",
                &rk3588_ramboot_null1::IMAGE,
            );
        }
        _ => {
            eprintln!("usage: cargo run -p rk3588-xtask -- [status|emit-ready]");
            std::process::exit(2)
        }
    }
}
fn emit(path: &str, data: &[u8]) {
    let p = Path::new(path);
    if let Some(parent) = p.parent() {
        fs::create_dir_all(parent).unwrap();
    }
    fs::write(p, data).unwrap();
    println!("wrote {path} ({} bytes)", data.len());
}
