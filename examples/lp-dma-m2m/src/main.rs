#![no_std]
#![no_main]

use bl808_pac as pac;
use panic_halt as _;
use riscv_rt::entry;

#[entry]
fn main() -> ! {
    let p = unsafe { pac::lp::Peripherals::steal() };
    let dma0 = &p.dma0;

    dma0.top_config().modify(|_, w| w.e().set_bit());
    let en = dma0.top_config().read().e().bit_is_set();
    core::hint::black_box(en);

    loop {
        core::hint::black_box(());
    }
}
