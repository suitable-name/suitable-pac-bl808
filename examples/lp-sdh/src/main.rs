#![no_std]
#![no_main]

use bl808_pac as pac;
use panic_halt as _;
use riscv_rt::entry;

#[entry]
fn main() -> ! {
    let p = unsafe { pac::lp::Peripherals::steal() };
    let sdh = &p.sdh;

    let ctrl = sdh.sd_host_ctrl().read().bits();
    core::hint::black_box(ctrl);

    loop {
        core::hint::black_box(());
    }
}
