#![no_std]
#![no_main]

use bl808_pac as pac;
use panic_halt as _;
use riscv_rt::entry;

static SOURCE: [u32; 8] = [1, 2, 3, 4, 5, 6, 7, 8];
static mut DEST: [u32; 8] = [0; 8];

#[entry]
fn main() -> ! {
    let p = unsafe { pac::d0::Peripherals::steal() };
    let dma0 = &p.dma0;

    // 1. Globally enable DMA0.
    dma0.top_config().modify(|_, w| w.e().set_bit());

    // 2. Configure Channel 0 for Memory-to-Memory.
    let ch0 = dma0.ch(0);

    // Set source and destination addresses.
    ch0.src_addr()
        .write(|w| unsafe { w.bits(SOURCE.as_ptr() as u32) });
    ch0.dst_addr()
        .write(|w| unsafe { w.bits(core::ptr::addr_of!(DEST) as u32) });

    // Control: TransferSize=8, SBSize=1 (4-beat burst), SWidth=2 (32-bit),
    //          DWidth=2 (32-bit), SI=1 (src increment), DI=1 (dst increment).
    ch0.control().write(|w| unsafe {
        w.transfer_size()
            .bits(8)
            .sbsize()
            .bits(1)
            .dbsize()
            .bits(1)
            .swidth()
            .bits(2)
            .dwidth()
            .bits(2)
            .si()
            .set_bit()
            .di()
            .set_bit()
    });

    // Config: FlowCtrl=0 (Mem2Mem), Enable=1.
    ch0.config()
        .write(|w| unsafe { w.e().set_bit().flow_cntrl().bits(0) });

    // 3. Wait for Transfer Complete.
    while dma0.int_tcstatus().read().int_tcstatus().bits() & 0x01 == 0 {}

    // Clear TC status.
    dma0.int_tcclear()
        .write(|w| unsafe { w.int_tcclear().bits(0x01) });

    // 4. Blink LED to signal pass/fail.
    //    Fast blink = transfer verified (DEST[7] == 8).
    //    Slow blink  = verification failed.
    let glb = &p.glb;
    glb.pin_cfg(8)
        .reg()
        .modify(|_, w| w.func_sel().gpio().oe().set_bit());

    loop {
        // SAFETY: DEST is only mutated by DMA, which has already completed.
        let ok = unsafe { DEST[7] == 8 };
        let (on_cyc, off_cyc) = if ok {
            (1_000_000u32, 1_000_000u32)
        } else {
            (5_000_000u32, 5_000_000u32)
        };

        glb.gpio_cfg140().write(|w| unsafe { w.bits(1 << 8) });
        delay(on_cyc);
        glb.gpio_cfg138().write(|w| unsafe { w.bits(1 << 8) });
        delay(off_cyc);
    }
}

#[inline(never)]
fn delay(cycles: u32) {
    for _ in 0..cycles {
        core::hint::black_box(());
    }
}
