#![no_std]
#![no_main]

use bl808_pac as pac;
use panic_halt as _;
use riscv_rt::entry;

#[entry]
fn main() -> ! {
    let p = unsafe { pac::d0::Peripherals::steal() };
    let csi = &p.csi;

    // Read CSI configuration register
    let cfg = csi.mipi_config().read().bits();
    core::hint::black_box(cfg);

    loop {
        core::hint::black_box(());
    }
}
