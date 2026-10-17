#![no_std]
#![no_main]

use bl808_pac as pac;
use panic_halt as _;
use riscv_rt::entry;

const LED_PIN: usize = 8;
const LED_MASK: u32 = 1 << LED_PIN;

#[entry]
fn main() -> ! {
    let p = unsafe { pac::m0::Peripherals::steal() };
    let glb = &p.glb;
    let timer0 = &p.timer0;

    // 1. Configure LED pin
    glb.pin_cfg(LED_PIN).reg().modify(|_, w| unsafe {
        w.func_sel()
            .gpio()
            .ie()
            .clear_bit()
            .oe()
            .set_bit()
            .pu()
            .clear_bit()
            .pd()
            .clear_bit()
            .drv()
            .bits(0)
    });

    // 2. Configure Timer0 Channel 2
    timer0.tcdr().modify(|_, w| unsafe { w.tcdr2().bits(0) });

    // Clear and enable counter
    timer0.tcer().modify(|_, w| w.tcr2_cnt_clr().set_bit());
    timer0.tcer().modify(|_, w| {
        w.tcr2_cnt_clr().clear_bit();
        w.timer2_en().set_bit()
    });

    loop {
        // Toggle LED ON
        glb.gpio_cfg140().write(|w| unsafe { w.bits(LED_MASK) });

        let start = timer0.tcr2().read().tcr2_cnt().bits();
        while timer0.tcr2().read().tcr2_cnt().bits().wrapping_sub(start) < 1_000_000 {}

        // Toggle LED OFF
        glb.gpio_cfg138().write(|w| unsafe { w.bits(LED_MASK) });

        let start = timer0.tcr2().read().tcr2_cnt().bits();
        while timer0.tcr2().read().tcr2_cnt().bits().wrapping_sub(start) < 1_000_000 {}
    }
}
