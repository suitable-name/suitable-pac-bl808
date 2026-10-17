#![no_std]
#![no_main]

use bl808_pac as pac;
use panic_halt as _;
use riscv_rt::entry;

#[entry]
fn main() -> ! {
    let p = unsafe { pac::lp::Peripherals::steal() };
    let i2c0 = &p.i2c0;

    i2c0.i2c_config().modify(|_, w| unsafe {
        w.cr_i2c_m_en()
            .set_bit()
            .cr_i2c_slv_addr()
            .bits(0x48)
    });

    let addr = i2c0.i2c_config().read().cr_i2c_slv_addr().bits();
    core::hint::black_box(addr);

    loop {
        core::hint::black_box(());
    }
}
