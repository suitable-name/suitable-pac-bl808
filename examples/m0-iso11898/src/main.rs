#![no_std]
#![no_main]

use bl808_pac as pac;
use panic_halt as _;
use riscv_rt::entry;

#[entry]
fn main() -> ! {
    let p = unsafe { pac::m0::Peripherals::steal() };
    let iso11898 = &p.iso11898;

    // Read CAN Controller mode and bus status
    let mode = iso11898.mode().read().bits();
    let status = iso11898.status().read().bits();
    core::hint::black_box((mode, status));

    loop {
        core::hint::black_box(());
    }
}
