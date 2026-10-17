#![no_std]
#![no_main]

use bl808_pac as pac;
use panic_halt as _;
use riscv_rt::entry;

static SRC: [u32; 4] = [0x11223344, 0x55667788, 0x99AABBCC, 0xDDEEFF00];
static mut DST: [u32; 4] = [0; 4];

#[entry]
fn main() -> ! {
    let p = unsafe { pac::m0::Peripherals::steal() };
    let dma0 = &p.dma0;

    // Enable DMA0 Top Controller
    dma0.top_config().modify(|_, w| w.e().set_bit());

    // Configure Channel 0: Mem-to-Mem (flow 0), 32-bit transfer
    let ch0 = dma0.ch(0);
    ch0.src_addr().write(|w| unsafe { w.bits(SRC.as_ptr() as u32) });
    let dst_ptr = core::ptr::addr_of_mut!(DST) as *mut u32 as u32;
    ch0.dst_addr().write(|w| unsafe { w.bits(dst_ptr) });
    ch0.control().modify(|_, w| unsafe {
        w.transfer_size()
            .bits(4)
            .si()
            .set_bit()
            .di()
            .set_bit()
            .swidth()
            .bits(2) // 32-bit width
            .dwidth()
            .bits(2)
    });

    let en = dma0.top_config().read().e().bit_is_set();
    core::hint::black_box(en);

    loop {
        core::hint::black_box(());
    }
}
