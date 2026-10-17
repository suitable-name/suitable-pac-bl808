#![no_std]
#![no_main]

use bl808_pac as pac;
use panic_halt as _;
use riscv_rt::entry;

#[entry]
fn main() -> ! {
    let p = unsafe { pac::m0::Peripherals::steal() };
    let sec_eng = &p.sec_eng;

    // Read TRNG control register and status
    let trng_ctrl = sec_eng.se_trng_0_ctrl_0().read().bits();
    core::hint::black_box(trng_ctrl);

    loop {
        core::hint::black_box(());
    }
}
