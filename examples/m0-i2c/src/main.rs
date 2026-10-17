#![no_std]
#![no_main]

use bl808_pac as pac;
use panic_halt as _;
use riscv_rt::entry;

#[entry]
fn main() -> ! {
    let p = unsafe { pac::m0::Peripherals::steal() };
    let i2c0 = &p.i2c0;

    // Configure I2C0 Master: sub-address 0, 7-bit slave addr
    i2c0.i2c_config().modify(|_, w| unsafe {
        w.cr_i2c_m_en()
            .set_bit()
            .cr_i2c_sub_addr_en()
            .clear_bit()
            .cr_i2c_slv_addr()
            .bits(0x50)
    });

    // Read back configuration to verify
    let cfg = i2c0.i2c_config().read().cr_i2c_slv_addr().bits();
    core::hint::black_box(cfg);

    loop {
        core::hint::black_box(());
    }
}
