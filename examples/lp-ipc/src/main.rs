#![no_std]
#![no_main]

use bl808_pac as pac;
use panic_halt as _;
use riscv_rt::entry;

#[entry]
fn main() -> ! {
    let p = unsafe { pac::lp::Peripherals::steal() };
    let ipc = &p.ipc;

    let stat = ipc.cpu0_ipc_irsrr().read().bits();
    core::hint::black_box(stat);

    loop {
        core::hint::black_box(());
    }
}
