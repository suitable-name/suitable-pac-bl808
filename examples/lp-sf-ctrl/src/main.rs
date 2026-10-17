#![no_std]
#![no_main]

use bl808_pac as pac;
use panic_halt as _;
use riscv_rt::entry;

#[entry]
fn main() -> ! {
    let p = unsafe { pac::lp::Peripherals::steal() };
    let sf_ctrl = &p.sf_ctrl;

    let cfg = sf_ctrl.sf_ctrl_0().read().bits();
    core::hint::black_box(cfg);

    loop {
        core::hint::black_box(());
    }
}
