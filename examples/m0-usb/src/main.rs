#![no_std]
#![no_main]

use bl808_pac as pac;
use panic_halt as _;
use riscv_rt::entry;

#[entry]
fn main() -> ! {
    let p = unsafe { pac::m0::Peripherals::steal() };
    let usb = &p.usb;

    // Read USB host structural parameters
    let params = usb.hcsparams().read().bits();
    core::hint::black_box(params);

    loop {
        core::hint::black_box(());
    }
}
