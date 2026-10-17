#![no_std]
#![no_main]

use bl808_pac as pac;
use panic_halt as _;
use riscv_rt::entry;

#[entry]
fn main() -> ! {
    let p = unsafe { pac::m0::Peripherals::steal() };
    let pds = &p.pds;

    // Read PDS control state and verify sleep wake sources
    let pds_ctl = pds.pds_ctl().read().bits();
    let pds_stat = pds.pds_stat().read().bits();
    core::hint::black_box((pds_ctl, pds_stat));

    loop {
        core::hint::black_box(());
    }
}
