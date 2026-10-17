#![no_std]
#![no_main]

use bl808_pac as pac;
use panic_halt as _;
use riscv_rt::entry;

#[entry]
fn main() -> ! {
    let p = unsafe { pac::m0::Peripherals::steal() };
    let spi0 = &p.spi0;

    // Configure SPI0: Master mode, 8-bit frame size, deglitch enable
    spi0.spi_config().modify(|_, w| unsafe {
        w.cr_spi_m_cont_en()
            .set_bit()
            .cr_spi_byte_inv()
            .clear_bit()
            .cr_spi_bit_inv()
            .clear_bit()
            .cr_spi_sclk_ph()
            .clear_bit()
            .cr_spi_sclk_pol()
            .clear_bit()
            .cr_spi_frame_size()
            .bits(0)
    });

    // Write a byte into TX FIFO
    spi0.spi_fifo_wdata().write(|w| unsafe { w.bits(0xA5) });

    let status = spi0.spi_int_sts().read().bits();
    core::hint::black_box(status);

    loop {
        core::hint::black_box(());
    }
}
