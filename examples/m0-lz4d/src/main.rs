#![no_std]
#![no_main]

use bl808_pac as pac;
use panic_halt as _;
use riscv_rt::entry;

#[entry]
fn main() -> ! {
    let p = unsafe { pac::m0::Peripherals::steal() };
    let lz4d = &p.lz4d;

    // Read LZ4 decompressor configuration register
    let cfg = lz4d.lz4_config().read().bits();
    core::hint::black_box(cfg);

    loop {
        core::hint::black_box(());
    }
}
