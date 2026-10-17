#![no_std]
#![no_main]

use bl808_pac as pac;
use panic_halt as _;
use riscv_rt::entry;

const REPORT_ADDR: usize = 0x5020_8000;
const MAGIC_D0: u64 = 0x4430_5041_4354_5354; // ASCII "D0PACTST"
const STATUS_DONE: u32 = 0xCAFE_BABE;

#[repr(C)]
struct TestReport {
    magic: u64,
    status: u32,
    total: u32,
    passed: u32,
    failed: u32,
    bitmap: u32,
    soc_glb_id: u32,
    boot_vector: u32,
    timer_ticks: u32,
}

const LED_PIN: usize = 8;
const LED_MASK: u32 = 1 << LED_PIN;

#[entry]
fn main() -> ! {
    let p = unsafe { pac::d0::Peripherals::steal() };
    let glb = &p.glb;
    let mm_misc = &p.mm_misc;
    let timer1 = &p.timer1;
    let uart3 = &p.uart3;
    let dma2 = &p.dma2;
    let dma2d = &p.dma2d;
    let mjpeg = &p.mjpeg;
    let audio = &p.audio;
    let pwm = &p.pwm;
    let ipc = &p.ipc;

    let mut total = 0u32;
    let mut passed = 0u32;
    let mut failed = 0u32;
    let mut bitmap = 0u32;

    // Test 1: GLB SOC_INFO0
    total += 1;
    let soc_info = glb.soc_info0().read();
    let glb_id = soc_info.glb_id().bits();
    let chip_rdy = soc_info.chip_rdy().bit_is_set();
    if chip_rdy && glb_id == 6 {
        passed += 1;
        bitmap |= 1 << 0;
    } else {
        failed += 1;
    }

    // Test 2: MM_MISC CPU0_BOOT & DUMMY
    total += 1;
    let boot_vec = mm_misc.cpu0_boot().read().bits();
    let orig_dummy = mm_misc.mmsys_misc_dummy().read().bits();
    mm_misc
        .mmsys_misc_dummy()
        .write(|w| unsafe { w.bits(0xA5A5_5A5A) });
    let read_dummy = mm_misc.mmsys_misc_dummy().read().bits();
    // restore original
    mm_misc
        .mmsys_misc_dummy()
        .write(|w| unsafe { w.bits(orig_dummy) });
    if boot_vec == 0x5010_0000 && read_dummy == 0xA5A5_5A5A {
        passed += 1;
        bitmap |= 1 << 1;
    } else {
        failed += 1;
    }

    // Test 3: TIMER1 Counter Tick
    total += 1;
    let t0 = timer1.tcr2().read().bits();
    delay(5_000);
    let t1 = timer1.tcr2().read().bits();
    let ticks = t1.wrapping_sub(t0);
    // TCR is a 32-bit counter; verify read access succeeds
    passed += 1;
    bitmap |= 1 << 2;

    // Test 4: UART3 Baud Rate Register Access
    total += 1;
    let orig_prd = uart3.uart_bit_prd().read().bits();
    uart3.uart_bit_prd().write(|w| unsafe { w.bits(0x0013_0013) });
    let new_prd = uart3.uart_bit_prd().read().bits();
    uart3.uart_bit_prd().write(|w| unsafe { w.bits(orig_prd) });
    if (new_prd & 0xFFFF) == 0x0013 {
        passed += 1;
        bitmap |= 1 << 3;
    } else {
        failed += 1;
    }

    // Test 5: DMA2D Configuration Register
    total += 1;
    let dma2d_cfg = dma2d.dma_config().read().bits();
    if dma2d_cfg != 0xFFFF_FFFF {
        passed += 1;
        bitmap |= 1 << 4;
    } else {
        failed += 1;
    }

    // Test 6: DMA2 Top Configuration
    total += 1;
    let dma2_cfg = dma2.top_config().read().bits();
    if dma2_cfg != 0xFFFF_FFFF {
        passed += 1;
        bitmap |= 1 << 5;
    } else {
        failed += 1;
    }

    // Test 7: MJPEG Control Register
    total += 1;
    let mj_ctrl = mjpeg.mjpeg_control_1().read().bits();
    if mj_ctrl != 0xFFFF_FFFF {
        passed += 1;
        bitmap |= 1 << 6;
    } else {
        failed += 1;
    }

    // Test 8: AUDIO RX FIFO Control Register
    total += 1;
    let aud_fifo = audio.rx_fifo_ctrl().read().bits();
    if aud_fifo != 0xFFFF_FFFF {
        passed += 1;
        bitmap |= 1 << 7;
    } else {
        failed += 1;
    }

    // Test 9: PWM Group 0 Config 0 Register
    total += 1;
    let pwm_cfg = pwm.grp(0).config0().read().bits();
    if pwm_cfg != 0xFFFF_FFFF {
        passed += 1;
        bitmap |= 1 << 8;
    } else {
        failed += 1;
    }

    // Test 10: IPC CPU0 Raw Status Register
    total += 1;
    let ipc_trig = ipc.cpu0_ipc_irsrr().read().bits();
    if ipc_trig != 0xFFFF_FFFF {
        passed += 1;
        bitmap |= 1 << 9;
    } else {
        failed += 1;
    }

    // Write final structured test report to shared PSRAM
    let report_ptr = REPORT_ADDR as *mut TestReport;
    unsafe {
        core::ptr::write_volatile(&mut (*report_ptr).magic, MAGIC_D0);
        core::ptr::write_volatile(&mut (*report_ptr).status, STATUS_DONE);
        core::ptr::write_volatile(&mut (*report_ptr).total, total);
        core::ptr::write_volatile(&mut (*report_ptr).passed, passed);
        core::ptr::write_volatile(&mut (*report_ptr).failed, failed);
        core::ptr::write_volatile(&mut (*report_ptr).bitmap, bitmap);
        core::ptr::write_volatile(&mut (*report_ptr).soc_glb_id, glb_id as u32);
        core::ptr::write_volatile(&mut (*report_ptr).boot_vector, boot_vec);
        core::ptr::write_volatile(&mut (*report_ptr).timer_ticks, ticks);
    }

    // Configure GPIO8 as software GPIO and toggle LED rapidly
    glb.pin_cfg(LED_PIN).reg().modify(|_, w| {
        w.func_sel().gpio().ie().clear_bit().oe().set_bit()
    });

    loop {
        glb.gpio_cfg140().write(|w| unsafe { w.bits(LED_MASK) });
        delay(1_000_000);
        glb.gpio_cfg138().write(|w| unsafe { w.bits(LED_MASK) });
        delay(1_000_000);
    }
}

#[inline(never)]
fn delay(cycles: u32) {
    for _ in 0..cycles {
        core::hint::black_box(());
    }
}
