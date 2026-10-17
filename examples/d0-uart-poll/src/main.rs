#![no_std]
#![no_main]

use bl808_pac as pac;
use core::fmt::Write;
use panic_halt as _;
use riscv_rt::entry;

// M1s Dock: D0 (C906) debug UART is UART0, TX=GPIO16, RX=GPIO17.
// Source: m1s_start/src/start_main.c (UART_TX_C906=16, UART_RX_C906=17).
// Connected via the on-board USB-UART bridge (lower USB-C connector, same
// physical port as M0 UART — only one core should be active at a time).
const UART0_TX_PIN: usize = 16;
const UART0_RX_PIN: usize = 17;

// Baud rate: 2 000 000 bps (matches M1s SDK default).
// D0 UART clock source: XCLK (40 MHz XTAL).
// Baud divisor = 40_000_000 / 2_000_000 - 1 = 19.
const UART_BAUD_DIV: u16 = 19;

struct Uart0<'a> {
    regs: &'a pac::d0::uart0::RegisterBlock,
}

impl<'a> core::fmt::Write for Uart0<'a> {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for c in s.bytes() {
            // Wait until TX FIFO has space (FIFO depth = 32).
            while self.regs.uart_fifo_config_1().read().tx_fifo_cnt().bits() == 0 {}
            self.regs
                .uart_fifo_wdata()
                .write(|w| unsafe { w.bits(c as u32) });
        }
        Ok(())
    }
}

#[entry]
fn main() -> ! {
    let p = unsafe { pac::d0::Peripherals::steal() };
    let glb = &p.glb;
    let uart0 = &p.uart0;

    // 1. Enable UART0 clock via GLB (source = BCLK, div = 0).
    glb.uart_cfg0()
        .modify(|_, w| unsafe { w.uart_clk_en().set_bit().uart_clk_div().bits(0) });

    // 2. Mux GPIO16 → UART function (func_sel = 7).
    //    GPIO16 % 12 = 4 → uart sig-slot 4; swap group GPIO12-GPIO23 active,
    //    so effective slot = (16 + 6) % 12 = 10. Map slot 10 to UART0_TXD (fun=0).
    glb.pin_cfg(UART0_TX_PIN)
        .reg()
        .modify(|_, w| unsafe { w.func_sel().bits(7).oe().set_bit().pu().set_bit() });
    glb.uart_cfg2().modify(|_, w| unsafe {
        w.uart_sig_10_sel().bits(0) // UART0_TXD
    });

    // 3. Mux GPIO17 → UART function (func_sel = 7).
    //    Effective slot = (17 + 6) % 12 = 11. Map slot 11 to UART0_RXD (fun=1).
    glb.pin_cfg(UART0_RX_PIN)
        .reg()
        .modify(|_, w| unsafe { w.func_sel().bits(7).ie().set_bit().pu().set_bit() });
    glb.uart_cfg2().modify(|_, w| unsafe {
        w.uart_sig_11_sel().bits(1) // UART0_RXD
    });

    // 4. Enable UART sig swap for group GPIO12-GPIO23.
    glb.glb_parm_cfg0()
        .modify(|_, w| unsafe { w.uart_swap_set().bits(0b100) });

    // 5. Set baud rate.
    uart0.uart_bit_prd().modify(|_, w| unsafe {
        w.cr_utx_bit_prd()
            .bits(UART_BAUD_DIV)
            .cr_urx_bit_prd()
            .bits(UART_BAUD_DIV)
    });

    // 6. Enable TX (free-run) and RX.
    uart0.utx_config().modify(|_, w| unsafe {
        w.cr_utx_en()
            .set_bit()
            .cr_utx_frm_en()
            .set_bit()
            .cr_utx_bit_cnt_d()
            .bits(7) // 8 data bits
    });
    uart0.urx_config().modify(|_, w| w.cr_urx_en().set_bit());

    let mut serial = Uart0 { regs: uart0 };

    let mut count = 0u32;
    loop {
        let _ = writeln!(serial, "Hello from BL808 D0 (C906)! Count: {}", count);
        count = count.wrapping_add(1);
        delay(20_000_000);
    }
}

#[inline(never)]
fn delay(cycles: u32) {
    for _ in 0..cycles {
        core::hint::black_box(());
    }
}
