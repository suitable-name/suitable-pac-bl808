#![no_std]
#![no_main]

use bl808_pac as pac;
use panic_halt as _;
use riscv_rt::entry;

#[entry]
fn main() -> ! {
    let p = unsafe { pac::m0::Peripherals::steal() };
    let emi_misc = &p.emi_misc;

    // Read EMI misc dummy configuration
    let emi = emi_misc.emi_misc_dummy().read().bits();
    core::hint::black_box(emi);

    loop {
        core::hint::black_box(());
    }
}
