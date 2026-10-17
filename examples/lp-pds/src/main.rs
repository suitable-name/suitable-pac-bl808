#![no_std]
#![no_main]

use bl808_pac as pac;
use panic_halt as _;
use riscv_rt::entry;

#[entry]
fn main() -> ! {
    let p = unsafe { pac::lp::Peripherals::steal() };
    let pds = &p.pds;

    let ctl = pds.pds_ctl().read().bits();
    core::hint::black_box(ctl);

    loop {
        core::hint::black_box(());
    }
}
