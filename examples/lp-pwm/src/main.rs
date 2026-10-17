#![no_std]
#![no_main]

use bl808_pac as pac;
use panic_halt as _;
use riscv_rt::entry;

#[entry]
fn main() -> ! {
    let p = unsafe { pac::lp::Peripherals::steal() };
    let pwm = &p.pwm;

    pwm.grp(0).period().modify(|_, w| unsafe {
        w.pwm_period().bits(1000)
    });
    pwm.grp(0).thre(0).modify(|_, w| unsafe {
        w.thre_l().bits(0).thre_h().bits(500)
    });

    let period = pwm.grp(0).period().read().pwm_period().bits();
    core::hint::black_box(period);

    loop {
        core::hint::black_box(());
    }
}
