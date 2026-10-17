#![no_std]
#![no_main]

use bl808_pac as pac;
use panic_halt as _;
use riscv_rt::entry;

#[entry]
fn main() -> ! {
    let p = unsafe { pac::lp::Peripherals::steal() };
    let audio = &p.audio;

    let fifo = audio.rx_fifo_ctrl().read().bits();
    core::hint::black_box(fifo);

    loop {
        core::hint::black_box(());
    }
}
