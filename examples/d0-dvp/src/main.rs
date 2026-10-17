#![no_std]
#![no_main]

use bl808_pac as pac;
use panic_halt as _;
use riscv_rt::entry;

#[entry]
fn main() -> ! {
    let p = unsafe { pac::d0::Peripherals::steal() };
    let dvp0 = &p.dvp0;

    // Read DVP0 AXI configuration register
    let cfg = dvp0.dvp2axi_configue().read().bits();
    core::hint::black_box(cfg);

    loop {
        core::hint::black_box(());
    }
}
