#![no_std]
#![no_main]

use bl808_pac as pac;
use panic_halt as _;
use riscv_rt::entry;

#[entry]
fn main() -> ! {
    let p = unsafe { pac::d0::Peripherals::steal() };
    let spi1 = &p.spi1;

    // Configure SPI1: 8-bit frame size, master mode
    spi1.spi_config().modify(|_, w| unsafe {
        w.cr_spi_m_cont_en()
            .set_bit()
            .cr_spi_frame_size()
            .bits(0)
    });

    spi1.spi_fifo_wdata().write(|w| unsafe { w.bits(0x5A) });
    let status = spi1.spi_int_sts().read().bits();
    core::hint::black_box(status);

    loop {
        core::hint::black_box(());
    }
}
