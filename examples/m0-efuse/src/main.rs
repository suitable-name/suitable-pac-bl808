#![no_std]
#![no_main]

use bl808_pac as pac;
use panic_halt as _;
use riscv_rt::entry;

#[entry]
fn main() -> ! {
    let p = unsafe { pac::m0::Peripherals::steal() };
    let efuse = &p.efuse;

    // Read factory-programmed Wi-Fi MAC address words
    let mac_low = efuse.ef_wifi_mac_low().read().bits();
    let mac_high = efuse.ef_wifi_mac_high().read().bits();
    core::hint::black_box((mac_low, mac_high));

    loop {
        core::hint::black_box(());
    }
}
