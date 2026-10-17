#![no_std]
#![no_main]

use bl808_pac as pac;
use panic_halt as _;
use riscv_rt::entry;

// Sipeed M1s Dock: onboard LED is on GPIO8, active-low.
const LED_PIN: usize = 8;
const LED_MASK: u32 = 1 << LED_PIN;

#[entry]
fn main() -> ! {
    let p = unsafe { pac::d0::Peripherals::steal() };
    let glb = &p.glb;

    // Configure GPIO8 as software-controlled GPIO output.
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

    loop {
        // LED ON (drive LOW)
        glb.gpio_cfg140().write(|w| unsafe { w.bits(LED_MASK) });
        delay(10_000_000);

        // LED OFF (drive HIGH)
        glb.gpio_cfg138().write(|w| unsafe { w.bits(LED_MASK) });
        delay(10_000_000);
    }
}

#[inline(never)]
fn delay(cycles: u32) {
    for _ in 0..cycles {
        core::hint::black_box(());
    }
}
