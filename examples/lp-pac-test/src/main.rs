#![no_std]
#![no_main]

use bl808_pac as pac;
use panic_halt as _;
use riscv_rt::entry;

const LED_PIN: usize = 8;
const LED_MASK: u32 = 1 << LED_PIN;

#[entry]
fn main() -> ! {
    let p = unsafe { pac::lp::Peripherals::steal() };

    let glb = &p.glb;
    let timer0 = &p.timer0;
    let uart0 = &p.uart0;
    let spi0 = &p.spi0;
    let i2c0 = &p.i2c0;
    let pwm = &p.pwm;
    let dma0 = &p.dma0;
    let hbn = &p.hbn;
    let pds = &p.pds;
    let aon = &p.aon;
    let sf_ctrl = &p.sf_ctrl;
    let efuse = &p.efuse;
    let lz4d = &p.lz4d;
    let gpip = &p.gpip;
    let audio = &p.audio;
    let ir = &p.ir;
    let ipc = &p.ipc;
    let sec_eng = &p.sec_eng;
    let sdh = &p.sdh;
    let usb = &p.usb;
    let emac = &p.emac;

    let mut passed = 0u32;
    let mut bitmap = 0u32;

    // 1. GLB
    let soc = glb.soc_info0().read();
    if soc.chip_rdy().bit_is_set() && soc.glb_id().bits() == 6 {
        passed += 1;
        bitmap |= 1 << 0;
    }

    // 2. TIMER0
    let t0 = timer0.tcr2().read().bits();
    if t0 != 0xFFFF_FFFF {
        passed += 1;
        bitmap |= 1 << 1;
    }

    // 3. UART0
    let uart = uart0.uart_bit_prd().read().bits();
    if uart != 0xFFFF_FFFF {
        passed += 1;
        bitmap |= 1 << 2;
    }

    // 4. SPI0
    let spi = spi0.spi_config().read().bits();
    if spi != 0xFFFF_FFFF {
        passed += 1;
        bitmap |= 1 << 3;
    }

    // 5. I2C0
    let i2c = i2c0.i2c_config().read().bits();
    if i2c != 0xFFFF_FFFF {
        passed += 1;
        bitmap |= 1 << 4;
    }

    // 6. PWM
    let pwm_val = pwm.grp(0).config0().read().bits();
    if pwm_val != 0xFFFF_FFFF {
        passed += 1;
        bitmap |= 1 << 5;
    }

    // 7. DMA0
    let dma = dma0.int_status().read().bits();
    if dma != 0xFFFF_FFFF {
        passed += 1;
        bitmap |= 1 << 6;
    }

    // 8. HBN
    let hbn_val = hbn.hbn_ctl().read().bits();
    if hbn_val != 0xFFFF_FFFF {
        passed += 1;
        bitmap |= 1 << 7;
    }

    // 9. PDS
    let pds_val = pds.pds_ctl().read().bits();
    if pds_val != 0xFFFF_FFFF {
        passed += 1;
        bitmap |= 1 << 8;
    }

    // 10. AON
    let aon_val = aon.aon().read().bits();
    if aon_val != 0xFFFF_FFFF {
        passed += 1;
        bitmap |= 1 << 9;
    }

    // 11. SF_CTRL
    let sf = sf_ctrl.sf_ctrl_0().read().bits();
    if sf != 0xFFFF_FFFF {
        passed += 1;
        bitmap |= 1 << 10;
    }

    // 12. EFUSE
    let ef = efuse.ef_wifi_mac_low().read().bits();
    if ef != 0xFFFF_FFFF {
        passed += 1;
        bitmap |= 1 << 11;
    }

    // 13. LZ4D
    let lz4 = lz4d.lz4_config().read().bits();
    if lz4 != 0xFFFF_FFFF {
        passed += 1;
        bitmap |= 1 << 12;
    }

    // 14. GPIP
    let gp = gpip.gpadc_config().read().bits();
    if gp != 0xFFFF_FFFF {
        passed += 1;
        bitmap |= 1 << 13;
    }

    // 15. AUDIO
    let aud = audio.rx_fifo_ctrl().read().bits();
    if aud != 0xFFFF_FFFF {
        passed += 1;
        bitmap |= 1 << 14;
    }

    // 16. IR
    let ir_val = ir.irtx_config().read().bits();
    if ir_val != 0xFFFF_FFFF {
        passed += 1;
        bitmap |= 1 << 15;
    }

    // 17. IPC
    let ipc_val = ipc.cpu0_ipc_irsrr().read().bits();
    if ipc_val != 0xFFFF_FFFF {
        passed += 1;
        bitmap |= 1 << 16;
    }

    // 18. SEC_ENG
    let sec = sec_eng.se_trng_0_ctrl_0().read().bits();
    if sec != 0xFFFF_FFFF {
        passed += 1;
        bitmap |= 1 << 17;
    }

    // 19. SDH
    let sdh_ctrl = sdh.sd_host_ctrl().read().bits();
    if sdh_ctrl != 0xFFFF {
        passed += 1;
        bitmap |= 1 << 18;
    }

    // 20. USB
    let usb_param = usb.hcsparams().read().bits();
    if usb_param != 0xFFFF_FFFF {
        passed += 1;
        bitmap |= 1 << 19;
    }

    // 21. EMAC
    let emac_mode = emac.mode().read().bits();
    if emac_mode != 0xFFFF_FFFF {
        passed += 1;
        bitmap |= 1 << 20;
    }

    core::hint::black_box((passed, bitmap));

    // Configure GPIO8 and blink
    glb.pin_cfg(LED_PIN).reg().modify(|_, w| {
        w.func_sel().gpio().ie().clear_bit().oe().set_bit()
    });

    loop {
        glb.gpio_cfg140().write(|w| unsafe { w.bits(LED_MASK) });
        delay(2_000_000);
        glb.gpio_cfg138().write(|w| unsafe { w.bits(LED_MASK) });
        delay(2_000_000);
    }
}

#[inline(never)]
fn delay(cycles: u32) {
    for _ in 0..cycles {
        core::hint::black_box(());
    }
}
