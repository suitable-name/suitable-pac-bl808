#![no_std]
#![no_main]

use bl808_pac as pac;
use panic_halt as _;
use riscv_rt::entry;

#[entry]
fn main() -> ! {
    let p = unsafe { pac::m0::Peripherals::steal() };
    let cci = &p.cci;

    // Read CCI control register
    let cci_cfg = cci.cci_cfg().read().bits();
    core::hint::black_box(cci_cfg);

    loop {
        core::hint::black_box(());
    }
}
