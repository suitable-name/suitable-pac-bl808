#![no_std]
#![no_main]

use bl808_pac as pac;
use panic_halt as _;
use riscv_rt::entry;

#[entry]
fn main() -> ! {
    let p = unsafe { pac::m0::Peripherals::steal() };
    let hbn = &p.hbn;

    // Read and test HBN scratch register (HBN_RSV0)
    let pattern = 0x55AA_1234u32;
    hbn.hbn_rsv0().write(|w| unsafe { w.bits(pattern) });
    let readback = hbn.hbn_rsv0().read().bits();
    let matches = readback == pattern;
    core::hint::black_box(matches);

    loop {
        core::hint::black_box(());
    }
}
