#![no_std]
#![no_main]

use bl808_pac as pac;
use panic_halt as _;
use riscv_rt::entry;

#[entry]
fn main() -> ! {
    let p = unsafe { pac::m0::Peripherals::steal() };
    let sf_ctrl = &p.sf_ctrl;

    // Read Serial Flash Controller 0 configuration
    let sf_cfg = sf_ctrl.sf_ctrl_0().read().bits();
    core::hint::black_box(sf_cfg);

    loop {
        core::hint::black_box(());
    }
}
