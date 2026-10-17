#![no_std]
#![no_main]

use bl808_pac as pac;
use panic_halt as _;
use riscv_rt::entry;

#[entry]
fn main() -> ! {
    let p = unsafe { pac::d0::Peripherals::steal() };
    let dvp_tsrc0 = &p.dvp_tsrc0;

    // Read DVP Timing Source 0 config register
    let cfg = dvp_tsrc0.config().read().bits();
    core::hint::black_box(cfg);

    loop {
        core::hint::black_box(());
    }
}
