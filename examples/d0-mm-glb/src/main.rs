#![no_std]
#![no_main]

use bl808_pac as pac;
use panic_halt as _;
use riscv_rt::entry;

#[entry]
fn main() -> ! {
    let p = unsafe { pac::d0::Peripherals::steal() };
    let mm_glb = &p.mm_glb;

    // Read MM CPU clock configuration
    let clk_cpu = mm_glb.mm_clk_cpu().read().bits();
    core::hint::black_box(clk_cpu);

    loop {
        core::hint::black_box(());
    }
}
