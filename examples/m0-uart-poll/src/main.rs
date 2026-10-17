#![no_std]
#![no_main]

use bl808_pac as pac;
use core::fmt::Write;
use panic_halt as _;
use riscv_rt::entry;

// M1s Dock: M0 (E907) debug UART is UART0, TX=GPIO14, RX=GPIO15.
// Connected via the on-board USB-UART bridge (lower USB-C connector).
const UART0_TX_PIN: usize = 14;
const UART0_RX_PIN: usize = 15;

// UART0 baud rate: 2 000 000 bps (matches Boot2 and M1s SDK default).
// M0 UART clock source: BCLK (typically 80 MHz from RC32M or XTAL path).
// Baud divisor = clock / baud - 1 = 80_000_000 / 2_000_000 - 1 = 39.
const UART_BAUD_DIV: u16 = 39;

struct Uart0<'a> {
    regs: &'a pac::m0::uart0::RegisterBlock,
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
    let p = unsafe { pac::m0::Peripherals::steal() };
    let glb = &p.glb;
    let uart0 = &p.uart0;

    // 1. Enable UART0 clock via GLB (source = BCLK, div = 0 → no extra division).
    glb.uart_cfg0()
        .modify(|_, w| unsafe { w.uart_clk_en().set_bit().uart_clk_div().bits(0) });

    // 2. Mux GPIO14 → UART function (func_sel = 7).
    glb.pin_cfg(UART0_TX_PIN)
        .reg()
        .modify(|_, w| unsafe { w.func_sel().bits(7).oe().set_bit().pu().set_bit() });

    // 3. Mux GPIO15 → UART function (func_sel = 7), input enabled.
    glb.pin_cfg(UART0_RX_PIN)
        .reg()
        .modify(|_, w| unsafe { w.func_sel().bits(7).ie().set_bit().pu().set_bit() });

    // 4. Configure UART signal routing in GLB: map sig-slot 2 (GPIO14 % 12 = 2) to
    //    UART0_TXD (fun=0) and sig-slot 3 (GPIO15 % 12 = 3) to UART0_RXD (fun=1).
    glb.uart_cfg1().modify(|_, w| unsafe {
        w.uart_sig_2_sel()
            .bits(0) // UART0_TXD
            .uart_sig_3_sel()
            .bits(1) // UART0_RXD
    });

    // 5. Set baud rate in UTX and URX config registers.
    uart0
        .utx_config()
        .modify(|_, w| unsafe { w.cr_utx_bit_cnt_d().bits(7) }); // 8 data bits
    uart0.uart_bit_prd().modify(|_, w| unsafe {
        w.cr_utx_bit_prd()
            .bits(UART_BAUD_DIV)
            .cr_urx_bit_prd()
            .bits(UART_BAUD_DIV)
    });

    // 6. Enable TX (free-run mode) and RX.
    uart0
        .utx_config()
        .modify(|_, w| w.cr_utx_en().set_bit().cr_utx_frm_en().set_bit());
    uart0.urx_config().modify(|_, w| w.cr_urx_en().set_bit());

    let mut serial = Uart0 { regs: uart0 };

    let mut count = 0u32;
    loop {
        let _ = writeln!(serial, "Hello from BL808 M0 (E907)! Count: {}", count);
        count = count.wrapping_add(1);
        delay(4_000_000);
    }
}

#[inline(never)]
fn delay(cycles: u32) {
    for _ in 0..cycles {
        core::hint::black_box(());
    }
}
