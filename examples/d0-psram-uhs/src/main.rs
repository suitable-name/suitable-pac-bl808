#![no_std]
#![no_main]

use bl808_pac as pac;
use panic_halt as _;
use riscv_rt::entry;

#[entry]
fn main() -> ! {
    let p = unsafe { pac::d0::Peripherals::steal() };
    let psram_uhs = &p.psram_uhs;

    // Read UHS basic control register
    let ctrl = psram_uhs.uhs_basic().read().bits();
    core::hint::black_box(ctrl);

    loop {
        core::hint::black_box(());
    }
}
