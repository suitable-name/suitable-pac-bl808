#![no_std]
#![no_main]

use bl808_pac as pac;
use panic_halt as _;
use riscv_rt::entry;

#[entry]
fn main() -> ! {
    let p = unsafe { pac::m0::Peripherals::steal() };
    let ir = &p.ir;

    // Read Infrared TX configuration register
    let tx_cfg = ir.irtx_config().read().bits();
    core::hint::black_box(tx_cfg);

    loop {
        core::hint::black_box(());
    }
}
