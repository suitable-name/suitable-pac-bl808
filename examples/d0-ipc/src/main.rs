#![no_std]
#![no_main]

use bl808_pac as pac;
use panic_halt as _;
use riscv_rt::entry;

#[entry]
fn main() -> ! {
    let p = unsafe { pac::d0::Peripherals::steal() };
    let ipc = &p.ipc;

    // Read IPC raw status register
    let status = ipc.cpu0_ipc_irsrr().read().bits();
    core::hint::black_box(status);

    loop {
        core::hint::black_box(());
    }
}
