#![no_std]
#![no_main]

use bl808_pac as pac;
use panic_halt as _;
use riscv_rt::entry;

#[entry]
fn main() -> ! {
    let p = unsafe { pac::lp::Peripherals::steal() };
    let spi0 = &p.spi0;

    spi0.spi_config().modify(|_, w| unsafe {
        w.cr_spi_m_cont_en()
            .set_bit()
            .cr_spi_frame_size()
            .bits(0)
    });

    let cfg = spi0.spi_config().read().bits();
    core::hint::black_box(cfg);

    loop {
        core::hint::black_box(());
    }
}
