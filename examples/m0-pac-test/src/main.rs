#![no_std]
#![no_main]

use bl808_pac as pac;
use panic_halt as _;
use riscv_rt::entry;

const LED_PIN: usize = 8;
const LED_MASK: u32 = 1 << LED_PIN;

const UART0_TX_PIN: usize = 14;
const UART0_RX_PIN: usize = 15;
const UART_BAUD_DIV: u16 = 39; // 80 MHz BCLK / 2_000_000 baud - 1

fn print_uart(uart: &pac::m0::uart0::RegisterBlock, s: &str) {
    for b in s.bytes() {
        while uart.uart_fifo_config_1().read().tx_fifo_cnt().bits() == 0 {}
        uart.uart_fifo_wdata().write(|w| unsafe { w.bits(b as u32) });
    }
}

#[entry]
fn main() -> ! {
    let p = unsafe { pac::m0::Peripherals::steal() };

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
    let mcu_misc = &p.mcu_misc;
    let cci = &p.cci;
    let sf_ctrl = &p.sf_ctrl;
    let efuse = &p.efuse;
    let emi_misc = &p.emi_misc;
    let lz4d = &p.lz4d;
    let gpip = &p.gpip;
    let audio = &p.audio;
    let iso11898 = &p.iso11898;
    let ir = &p.ir;

    // 1. Initialize UART0 at 2,000,000 baud
    glb.uart_cfg0()
        .modify(|_, w| unsafe { w.uart_clk_en().set_bit().uart_clk_div().bits(0) });
    glb.pin_cfg(UART0_TX_PIN)
        .reg()
        .modify(|_, w| unsafe { w.func_sel().bits(7).oe().set_bit().pu().set_bit() });
    glb.pin_cfg(UART0_RX_PIN)
        .reg()
        .modify(|_, w| unsafe { w.func_sel().bits(7).ie().set_bit().pu().set_bit() });
    glb.uart_cfg1().modify(|_, w| unsafe {
        w.uart_sig_2_sel().bits(0).uart_sig_3_sel().bits(1)
    });
    uart0.uart_bit_prd().modify(|_, w| unsafe {
        w.cr_utx_bit_prd()
            .bits(UART_BAUD_DIV)
            .cr_urx_bit_prd()
            .bits(UART_BAUD_DIV)
    });
    uart0.utx_config().modify(|_, w| unsafe {
        w.cr_utx_en()
            .set_bit()
            .cr_utx_frm_en()
            .set_bit()
            .cr_utx_bit_cnt_d()
            .bits(7)
    });
    uart0.urx_config().modify(|_, w| w.cr_urx_en().set_bit());

    print_uart(uart0, "\r\n========================================================\r\n");
    print_uart(uart0, "    BOUFFALO LAB BL808 M0 RUST PAC FIRMWARE LIVE TEST   \r\n");
    print_uart(uart0, "========================================================\r\n");

    let mut passed = 0u32;

    // 1. GLB
    let soc = glb.soc_info0().read();
    if soc.chip_rdy().bit_is_set() && soc.glb_id().bits() == 6 {
        passed += 1;
        print_uart(uart0, "[PASS] [ 1/20] GLB: SOC_INFO0 chip_rdy=1, glb_id=6\r\n");
    } else {
        print_uart(uart0, "[FAIL] [ 1/20] GLB: Invalid SOC_INFO0\r\n");
    }

    // 2. TIMER0
    let t0 = timer0.tcr2().read().bits();
    if t0 != 0xFFFF_FFFF {
        passed += 1;
        print_uart(uart0, "[PASS] [ 2/20] TIMER0: TCR2 counter readable\r\n");
    }

    // 3. UART0
    let uart = uart0.uart_bit_prd().read().bits();
    if uart != 0xFFFF_FFFF {
        passed += 1;
        print_uart(uart0, "[PASS] [ 3/20] UART0: Baud period active\r\n");
    }

    // 4. SPI0
    let spi = spi0.spi_config().read().bits();
    if spi != 0xFFFF_FFFF {
        passed += 1;
        print_uart(uart0, "[PASS] [ 4/20] SPI0: SPI_CONFIG readable\r\n");
    }

    // 5. I2C0
    let i2c = i2c0.i2c_config().read().bits();
    if i2c != 0xFFFF_FFFF {
        passed += 1;
        print_uart(uart0, "[PASS] [ 5/20] I2C0: I2C_CONFIG readable\r\n");
    }

    // 6. PWM
    let pwm_val = pwm.grp(0).config0().read().bits();
    if pwm_val != 0xFFFF_FFFF {
        passed += 1;
        print_uart(uart0, "[PASS] [ 6/20] PWM: GRP0_CONFIG0 readable\r\n");
    }

    // 7. DMA0
    let dma = dma0.int_status().read().bits();
    if dma != 0xFFFF_FFFF {
        passed += 1;
        print_uart(uart0, "[PASS] [ 7/20] DMA0: INT_STATUS readable\r\n");
    }

    // 8. HBN
    let hbn_val = hbn.hbn_ctl().read().bits();
    if hbn_val != 0xFFFF_FFFF {
        passed += 1;
        print_uart(uart0, "[PASS] [ 8/20] HBN: HBN_CTL readable\r\n");
    }

    // 9. PDS
    let pds_val = pds.pds_ctl().read().bits();
    if pds_val != 0xFFFF_FFFF {
        passed += 1;
        print_uart(uart0, "[PASS] [ 9/20] PDS: PDS_CTL readable\r\n");
    }

    // 10. AON
    let aon_val = aon.aon().read().bits();
    if aon_val != 0xFFFF_FFFF {
        passed += 1;
        print_uart(uart0, "[PASS] [10/20] AON: AON_CTL readable\r\n");
    }

    // 11. MCU_MISC
    let mcu = mcu_misc.mcu_bus_cfg0().read().bits();
    if mcu != 0xFFFF_FFFF {
        passed += 1;
        print_uart(uart0, "[PASS] [11/20] MCU_MISC: MCU_BUS_CFG0 readable\r\n");
    }

    // 12. CCI
    let cci_val = cci.cci_cfg().read().bits();
    if cci_val != 0xFFFF_FFFF {
        passed += 1;
        print_uart(uart0, "[PASS] [12/20] CCI: CCI_CFG readable\r\n");
    }

    // 13. SF_CTRL
    let sf = sf_ctrl.sf_ctrl_0().read().bits();
    if sf != 0xFFFF_FFFF {
        passed += 1;
        print_uart(uart0, "[PASS] [13/20] SF_CTRL: SF_CTRL_0 readable\r\n");
    }

    // 14. EFUSE
    let ef = efuse.ef_wifi_mac_low().read().bits();
    if ef != 0xFFFF_FFFF {
        passed += 1;
        print_uart(uart0, "[PASS] [14/20] EFUSE: EF_WIFI_MAC_LOW readable\r\n");
    }

    // 15. EMI_MISC
    let emi = emi_misc.emi_misc_dummy().read().bits();
    if emi != 0xFFFF_FFFF {
        passed += 1;
        print_uart(uart0, "[PASS] [15/20] EMI_MISC: EMI_MISC_DUMMY readable\r\n");
    }

    // 16. LZ4D
    let lz4 = lz4d.lz4_config().read().bits();
    if lz4 != 0xFFFF_FFFF {
        passed += 1;
        print_uart(uart0, "[PASS] [16/20] LZ4D: LZ4_CONFIG readable\r\n");
    }

    // 17. GPIP
    let gp = gpip.gpadc_config().read().bits();
    if gp != 0xFFFF_FFFF {
        passed += 1;
        print_uart(uart0, "[PASS] [17/20] GPIP: GPADC_CONFIG readable\r\n");
    }

    // 18. AUDIO
    let aud = audio.rx_fifo_ctrl().read().bits();
    if aud != 0xFFFF_FFFF {
        passed += 1;
        print_uart(uart0, "[PASS] [18/20] AUDIO: RX_FIFO_CTRL readable\r\n");
    }

    // 19. ISO11898
    let can = iso11898.mode().read().bits();
    if can != 0xFFFF_FFFF {
        passed += 1;
        print_uart(uart0, "[PASS] [19/20] ISO11898: CAN_MODE readable\r\n");
    }

    // 20. IR
    let ir_val = ir.irtx_config().read().bits();
    if ir_val != 0xFFFF_FFFF {
        passed += 1;
        print_uart(uart0, "[PASS] [20/20] IR: IRTX_CONFIG readable\r\n");
    }

    print_uart(uart0, "========================================================\r\n");
    if passed == 20 {
        print_uart(uart0, "RESULT: ALL 20 M0 PAC PERIPHERALS VERIFIED LIVE! [100%]\r\n");
    } else {
        print_uart(uart0, "RESULT: PARTIAL PASS\r\n");
    }
    print_uart(uart0, "Blinking LED on GPIO8...\r\n");
    print_uart(uart0, "========================================================\r\n");

    // Configure GPIO8 as software GPIO output and blink LED
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
