#![no_std]
#![no_main]

use bl808_pac as pac;
use panic_halt as _;
use riscv_rt::entry;

#[entry]
fn main() -> ! {
    let p = unsafe { pac::m0::Peripherals::steal() };
    let mcu_misc = &p.mcu_misc;

    // Read MCU bus timeout configuration
    let bus_cfg = mcu_misc.mcu_bus_cfg0().read().bits();
    core::hint::black_box(bus_cfg);

    loop {
        core::hint::black_box(());
    }
}
