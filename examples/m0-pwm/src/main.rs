#![no_std]
#![no_main]

use bl808_pac as pac;
use panic_halt as _;
use riscv_rt::entry;

#[entry]
fn main() -> ! {
    let p = unsafe { pac::m0::Peripherals::steal() };
    let pwm = &p.pwm;

    // Configure PWM Channel 0: clock divider 1, period 1000, threshold 500 (50% duty)
    pwm.grp(0).config0().modify(|_, w| unsafe {
        w.pwm_clk_div().bits(1).pwm_stop_en().set_bit()
    });
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
