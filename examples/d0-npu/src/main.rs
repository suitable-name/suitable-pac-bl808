#![no_std]
#![no_main]

use bl808_pac as pac;
use panic_halt as _;
use riscv_rt::entry;

#[entry]
fn main() -> ! {
    let p = unsafe { pac::d0::Peripherals::steal() };
    let npu = &p.npu;

    // Read NPU control register
    let ctrl = npu.bali_general_cfg().read().bits();
    core::hint::black_box(ctrl);

    loop {
        core::hint::black_box(());
    }
}
