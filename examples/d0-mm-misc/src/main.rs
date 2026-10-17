#![no_std]
#![no_main]

use bl808_pac as pac;
use panic_halt as _;
use riscv_rt::entry;

#[entry]
fn main() -> ! {
    let p = unsafe { pac::d0::Peripherals::steal() };
    let mm_misc = &p.mm_misc;

    // Read C906 boot vector register
    let boot_vec = mm_misc.cpu0_boot().read().bits();
    core::hint::black_box(boot_vec);

    loop {
        core::hint::black_box(());
    }
}
