#![no_std]
#![no_main]

use bl808_pac as pac;
use panic_halt as _;
use riscv_rt::entry;

// Sipeed M1s Dock: onboard LED is on GPIO8, active-low.
// Schematic net: IO8_PWM_LED. Circuit: 3.3V -> R18(510R) -> LED -> GPIO8.
// LED ON  = GPIO8 driven LOW  (write to gpio_cfg140, the output-clear register)
// LED OFF = GPIO8 driven HIGH (write to gpio_cfg138, the output-set register)
const LED_PIN: usize = 8;
const LED_MASK: u32 = 1 << LED_PIN;

#[entry]
fn main() -> ! {
    let p = unsafe { pac::m0::Peripherals::steal() };
    let glb = &p.glb;

    // Configure GPIO8 as software-controlled GPIO output (function 11 = SWGPIO).
    glb.pin_cfg(LED_PIN).reg().modify(|_, w| unsafe {
        w.func_sel()
            .gpio() // function 11: software GPIO
            .ie()
            .clear_bit() // input disable
            .oe()
            .set_bit() // output enable
            .pu()
            .clear_bit() // no pull-up
            .pd()
            .clear_bit() // no pull-down
            .drv()
            .bits(0) // standard drive strength
    });

    loop {
        // Drive GPIO8 LOW -> LED ON (active-low).
        // gpio_cfg140 is the BL808 GLB GPIO output-clear register:
        // writing a 1 in bit N drives GPIO N output to 0 V.
        glb.gpio_cfg140().write(|w| unsafe { w.bits(LED_MASK) });
        delay(2_000_000);

        // Drive GPIO8 HIGH -> LED OFF.
        // gpio_cfg138 is the BL808 GLB GPIO output-set register:
        // writing a 1 in bit N drives GPIO N output to 3.3 V.
        glb.gpio_cfg138().write(|w| unsafe { w.bits(LED_MASK) });
        delay(2_000_000);
    }
}

#[inline(never)]
fn delay(cycles: u32) {
    for _ in 0..cycles {
        core::hint::black_box(());
    }
}
