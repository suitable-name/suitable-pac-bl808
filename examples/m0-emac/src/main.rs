#![no_std]
#![no_main]

use bl808_pac as pac;
use panic_halt as _;
use riscv_rt::entry;

#[entry]
fn main() -> ! {
    let p = unsafe { pac::m0::Peripherals::steal() };
    let emac = &p.emac;

    // Read Ethernet MAC configuration register
    let mode = emac.mode().read().bits();
    core::hint::black_box(mode);

    loop {
        core::hint::black_box(());
    }
}
