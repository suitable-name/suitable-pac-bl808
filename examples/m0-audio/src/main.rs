#![no_std]
#![no_main]

use bl808_pac as pac;
use panic_halt as _;
use riscv_rt::entry;

#[entry]
fn main() -> ! {
    let p = unsafe { pac::m0::Peripherals::steal() };
    let audio = &p.audio;

    // Read RX FIFO control register and verify channel mapping
    let rx_fifo = audio.rx_fifo_ctrl().read().bits();
    core::hint::black_box(rx_fifo);

    loop {
        core::hint::black_box(());
    }
}
