#![no_std]
#![no_main]

use bl808_pac as pac;
use panic_halt as _;
use riscv_rt::entry;

#[entry]
fn main() -> ! {
    let p = unsafe { pac::d0::Peripherals::steal() };
    let timer1 = &p.timer1;

    // Clear and enable counter 2
    timer1.tcer().modify(|_, w| w.tcr2_cnt_clr().set_bit());
    timer1.tcer().modify(|_, w| {
        w.tcr2_cnt_clr().clear_bit();
        w.timer2_en().set_bit()
    });

    let start = timer1.tcr2().read().tcr2_cnt().bits();
    while timer1.tcr2().read().tcr2_cnt().bits().wrapping_sub(start) < 100_000 {}

    loop {
        core::hint::black_box(());
    }
}
