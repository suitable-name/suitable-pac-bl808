#![no_std]
#![no_main]

use bl808_pac as pac;
use panic_halt as _;
use riscv_rt::entry;

#[entry]
fn main() -> ! {
    let p = unsafe { pac::m0::Peripherals::steal() };
    let aon = &p.aon;

    // Read AON domain control register
    let aon_ctl = aon.aon().read().bits();
    core::hint::black_box(aon_ctl);

    loop {
        core::hint::black_box(());
    }
}
