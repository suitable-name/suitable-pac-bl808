#![no_std]
#![no_main]

use bl808_pac as pac;
use panic_halt as _;
use riscv_rt::entry;

#[entry]
fn main() -> ! {
    let p = unsafe { pac::m0::Peripherals::steal() };
    let gpip = &p.gpip;

    // Read and verify GPADC configuration
    let gpadc_cfg = gpip.gpadc_config().read().bits();
    core::hint::black_box(gpadc_cfg);

    loop {
        core::hint::black_box(());
    }
}
