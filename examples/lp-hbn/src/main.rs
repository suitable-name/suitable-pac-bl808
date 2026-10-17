#![no_std]
#![no_main]

use bl808_pac as pac;
use panic_halt as _;
use riscv_rt::entry;

#[entry]
fn main() -> ! {
    let p = unsafe { pac::lp::Peripherals::steal() };
    let hbn = &p.hbn;

    let pattern = 0xAA55_33CCu32;
    hbn.hbn_rsv0().write(|w| unsafe { w.bits(pattern) });
    let val = hbn.hbn_rsv0().read().bits();
    let ok = val == pattern;
    core::hint::black_box(ok);

    loop {
        core::hint::black_box(());
    }
}
