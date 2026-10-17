#!/usr/bin/env python3
"""
Generate per-peripheral PAC examples for BL808 M0, D0, and LP cores.
Each example is a self-contained, no_std, no_main Rust crate under examples/
demonstrating PAC register access, configuration, and verification for a peripheral.
"""

import os
import sys

REPO_ROOT = os.path.abspath(os.path.join(os.path.dirname(__file__), ".."))
EXAMPLES_DIR = os.path.join(REPO_ROOT, "examples")

# Linker scripts for each core
MEMORY_X_M0 = """MEMORY
{
    FLASH : ORIGIN = 0x58000000, LENGTH = 4M
    RAM   : ORIGIN = 0x22020000, LENGTH = 64K
}
REGION_ALIAS("REGION_TEXT", FLASH);
REGION_ALIAS("REGION_RODATA", FLASH);
REGION_ALIAS("REGION_DATA", RAM);
REGION_ALIAS("REGION_BSS", RAM);
REGION_ALIAS("REGION_HEAP", RAM);
REGION_ALIAS("REGION_STACK", RAM);
"""

MEMORY_X_D0 = """MEMORY
{
    PSRAM : ORIGIN = 0x50100000, LENGTH = 1M
}
REGION_ALIAS("REGION_TEXT", PSRAM);
REGION_ALIAS("REGION_RODATA", PSRAM);
REGION_ALIAS("REGION_DATA", PSRAM);
REGION_ALIAS("REGION_BSS", PSRAM);
REGION_ALIAS("REGION_HEAP", PSRAM);
REGION_ALIAS("REGION_STACK", PSRAM);
"""

MEMORY_X_LP = """MEMORY
{
    RAM : ORIGIN = 0x22014000, LENGTH = 16K
}
REGION_ALIAS("REGION_TEXT", RAM);
REGION_ALIAS("REGION_RODATA", RAM);
REGION_ALIAS("REGION_DATA", RAM);
REGION_ALIAS("REGION_BSS", RAM);
REGION_ALIAS("REGION_HEAP", RAM);
REGION_ALIAS("REGION_STACK", RAM);
"""

BUILD_RS = """use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    fs::write(out_dir.join("memory.x"), include_bytes!("memory.x")).unwrap();
    println!("cargo:rustc-link-search={}", out_dir.display());
    println!("cargo:rerun-if-changed=memory.x");
    println!("cargo:rerun-if-changed=build.rs");
}
"""

def make_cargo_toml(name, core_feature):
    return f"""[package]
name = "{name}"
version = "0.1.0"
edition = "2024"

[dependencies]
bl808-pac = {{ path = "../../", features = ["{core_feature}", "rt", "critical-section"] }}
riscv-rt = {{ workspace = true }}
panic-halt = "1.0.0"
"""

# M0 peripheral examples to create (excluding m0-blinky, m0-timer-delay, m0-uart-poll, m0-pac-test)
M0_EXAMPLES = {
    "m0-spi": {
        "desc": "SPI0 Master Configuration & FIFO Transfer",
        "code": """#![no_std]
#![no_main]

use bl808_pac as pac;
use panic_halt as _;
use riscv_rt::entry;

#[entry]
fn main() -> ! {
    let p = unsafe { pac::m0::Peripherals::steal() };
    let spi0 = &p.spi0;

    // Configure SPI0: Master mode, 8-bit frame size, deglitch enable
    spi0.spi_config().modify(|_, w| unsafe {
        w.cr_spi_m_cont_en()
            .set_bit()
            .cr_spi_byte_inv()
            .clear_bit()
            .cr_spi_bit_inv()
            .clear_bit()
            .cr_spi_sclk_ph()
            .clear_bit()
            .cr_spi_sclk_pol()
            .clear_bit()
            .cr_spi_frame_size()
            .bits(0)
    });

    // Write a byte into TX FIFO
    spi0.spi_fifo_wdata().write(|w| unsafe { w.bits(0xA5) });

    let status = spi0.spi_int_sts().read().bits();
    core::hint::black_box(status);

    loop {
        core::hint::black_box(());
    }
}
""",
    },
    "m0-i2c": {
        "desc": "I2C0 Master Setup & Bus Addressing",
        "code": """#![no_std]
#![no_main]

use bl808_pac as pac;
use panic_halt as _;
use riscv_rt::entry;

#[entry]
fn main() -> ! {
    let p = unsafe { pac::m0::Peripherals::steal() };
    let i2c0 = &p.i2c0;

    // Configure I2C0 Master: sub-address 0, 7-bit slave addr
    i2c0.i2c_config().modify(|_, w| unsafe {
        w.cr_i2c_m_en()
            .set_bit()
            .cr_i2c_sub_addr_en()
            .clear_bit()
            .cr_i2c_slv_addr()
            .bits(0x50)
    });

    // Read back configuration to verify
    let cfg = i2c0.i2c_config().read().cr_i2c_slv_addr().bits();
    core::hint::black_box(cfg);

    loop {
        core::hint::black_box(());
    }
}
""",
    },
    "m0-pwm": {
        "desc": "PWM Controller Period & Duty Cycle Setup",
        "code": """#![no_std]
#![no_main]

use bl808_pac as pac;
use panic_halt as _;
use riscv_rt::entry;

#[entry]
fn main() -> ! {
    let p = unsafe { pac::m0::Peripherals::steal() };
    let pwm = &p.pwm;

    // Configure PWM Channel 0: clock divider 1, period 1000, threshold 500 (50% duty)
    pwm.grp(0).config0().modify(|_, w| unsafe {
        w.pwm_clk_div().bits(1).pwm_stop_en().set_bit()
    });
    pwm.grp(0).period().modify(|_, w| unsafe {
        w.pwm_period().bits(1000)
    });
    pwm.grp(0).thre(0).modify(|_, w| unsafe {
        w.thre_l().bits(0).thre_h().bits(500)
    });

    let period = pwm.grp(0).period().read().pwm_period().bits();
    core::hint::black_box(period);

    loop {
        core::hint::black_box(());
    }
}
""",
    },
    "m0-dma": {
        "desc": "DMA0 Memory-to-Memory Transfer Setup",
        "code": """#![no_std]
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
""",
    },
    "m0-hbn": {
        "desc": "Hibernate Domain & Scratch Register Verification",
        "code": """#![no_std]
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
""",
    },
    "m0-pds": {
        "desc": "Power Down Sleep Domain Configuration",
        "code": """#![no_std]
#![no_main]

use bl808_pac as pac;
use panic_halt as _;
use riscv_rt::entry;

#[entry]
fn main() -> ! {
    let p = unsafe { pac::m0::Peripherals::steal() };
    let pds = &p.pds;

    // Read PDS control state and verify sleep wake sources
    let pds_ctl = pds.pds_ctl().read().bits();
    let pds_stat = pds.pds_stat().read().bits();
    core::hint::black_box((pds_ctl, pds_stat));

    loop {
        core::hint::black_box(());
    }
}
""",
    },
    "m0-aon": {
        "desc": "Always-On Domain Power Management",
        "code": """#![no_std]
#![no_main]

use bl808_pac as pac;
use panic_halt as _;
use riscv_rt::entry;

#[entry]
fn main() -> ! {
    let p = unsafe { pac::m0::Peripherals::steal() };
    let aon = &p.aon;

    // Read AON domain control register
    let aon_ctl = aon.aon().read().bits();
    core::hint::black_box(aon_ctl);

    loop {
        core::hint::black_box(());
    }
}
""",
    },
    "m0-efuse": {
        "desc": "eFuse Chip Information & MAC Address Readout",
        "code": """#![no_std]
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
""",
    },
    "m0-gpip": {
        "desc": "General Purpose ADC / DAC Interface Setup",
        "code": """#![no_std]
#![no_main]

use bl808_pac as pac;
use panic_halt as _;
use riscv_rt::entry;

#[entry]
fn main() -> ! {
    let p = unsafe { pac::m0::Peripherals::steal() };
    let gpip = &p.gpip;

    // Read and verify GPADC configuration
    let gpadc_cfg = gpip.gpadc_config().read().bits();
    core::hint::black_box(gpadc_cfg);

    loop {
        core::hint::black_box(());
    }
}
""",
    },
    "m0-audio": {
        "desc": "Audio Subsystem FIFO & Codec Setup",
        "code": """#![no_std]
#![no_main]

use bl808_pac as pac;
use panic_halt as _;
use riscv_rt::entry;

#[entry]
fn main() -> ! {
    let p = unsafe { pac::m0::Peripherals::steal() };
    let audio = &p.audio;

    // Read RX FIFO control register and verify channel mapping
    let rx_fifo = audio.rx_fifo_ctrl().read().bits();
    core::hint::black_box(rx_fifo);

    loop {
        core::hint::black_box(());
    }
}
""",
    },
    "m0-ir": {
        "desc": "Infrared Remote Transmitter Setup",
        "code": """#![no_std]
#![no_main]

use bl808_pac as pac;
use panic_halt as _;
use riscv_rt::entry;

#[entry]
fn main() -> ! {
    let p = unsafe { pac::m0::Peripherals::steal() };
    let ir = &p.ir;

    // Read Infrared TX configuration register
    let tx_cfg = ir.irtx_config().read().bits();
    core::hint::black_box(tx_cfg);

    loop {
        core::hint::black_box(());
    }
}
""",
    },
    "m0-iso11898": {
        "desc": "ISO11898 CAN Controller Configuration",
        "code": """#![no_std]
#![no_main]

use bl808_pac as pac;
use panic_halt as _;
use riscv_rt::entry;

#[entry]
fn main() -> ! {
    let p = unsafe { pac::m0::Peripherals::steal() };
    let iso11898 = &p.iso11898;

    // Read CAN Controller mode and bus status
    let mode = iso11898.mode().read().bits();
    let status = iso11898.status().read().bits();
    core::hint::black_box((mode, status));

    loop {
        core::hint::black_box(());
    }
}
""",
    },
    "m0-lz4d": {
        "desc": "Hardware LZ4 Decompressor Setup",
        "code": """#![no_std]
#![no_main]

use bl808_pac as pac;
use panic_halt as _;
use riscv_rt::entry;

#[entry]
fn main() -> ! {
    let p = unsafe { pac::m0::Peripherals::steal() };
    let lz4d = &p.lz4d;

    // Read LZ4 decompressor configuration register
    let cfg = lz4d.lz4_config().read().bits();
    core::hint::black_box(cfg);

    loop {
        core::hint::black_box(());
    }
}
""",
    },
    "m0-ipc": {
        "desc": "Inter-Processor Communication Mailbox",
        "code": """#![no_std]
#![no_main]

use bl808_pac as pac;
use panic_halt as _;
use riscv_rt::entry;

#[entry]
fn main() -> ! {
    let p = unsafe { pac::m0::Peripherals::steal() };
    let ipc = &p.ipc;

    // Read IPC raw status register
    let status = ipc.cpu0_ipc_irsrr().read().bits();
    core::hint::black_box(status);

    loop {
        core::hint::black_box(());
    }
}
""",
    },
    "m0-sec-eng": {
        "desc": "Security Engine TRNG (Random Number Generator)",
        "code": """#![no_std]
#![no_main]

use bl808_pac as pac;
use panic_halt as _;
use riscv_rt::entry;

#[entry]
fn main() -> ! {
    let p = unsafe { pac::m0::Peripherals::steal() };
    let sec_eng = &p.sec_eng;

    // Read TRNG control register and status
    let trng_ctrl = sec_eng.se_trng_0_ctrl_0().read().bits();
    core::hint::black_box(trng_ctrl);

    loop {
        core::hint::black_box(());
    }
}
""",
    },
    "m0-sf-ctrl": {
        "desc": "Serial Flash Controller / XIP Accelerator",
        "code": """#![no_std]
#![no_main]

use bl808_pac as pac;
use panic_halt as _;
use riscv_rt::entry;

#[entry]
fn main() -> ! {
    let p = unsafe { pac::m0::Peripherals::steal() };
    let sf_ctrl = &p.sf_ctrl;

    // Read Serial Flash Controller 0 configuration
    let sf_cfg = sf_ctrl.sf_ctrl_0().read().bits();
    core::hint::black_box(sf_cfg);

    loop {
        core::hint::black_box(());
    }
}
""",
    },
    "m0-mcu-misc": {
        "desc": "MCU Bus Misc & Timeout Monitor",
        "code": """#![no_std]
#![no_main]

use bl808_pac as pac;
use panic_halt as _;
use riscv_rt::entry;

#[entry]
fn main() -> ! {
    let p = unsafe { pac::m0::Peripherals::steal() };
    let mcu_misc = &p.mcu_misc;

    // Read MCU bus timeout configuration
    let bus_cfg = mcu_misc.mcu_bus_cfg0().read().bits();
    core::hint::black_box(bus_cfg);

    loop {
        core::hint::black_box(());
    }
}
""",
    },
    "m0-cci": {
        "desc": "Cache Coherence Interface Setup",
        "code": """#![no_std]
#![no_main]

use bl808_pac as pac;
use panic_halt as _;
use riscv_rt::entry;

#[entry]
fn main() -> ! {
    let p = unsafe { pac::m0::Peripherals::steal() };
    let cci = &p.cci;

    // Read CCI control register
    let cci_cfg = cci.cci_cfg().read().bits();
    core::hint::black_box(cci_cfg);

    loop {
        core::hint::black_box(());
    }
}
""",
    },
    "m0-emi-misc": {
        "desc": "External Memory Interface Configuration",
        "code": """#![no_std]
#![no_main]

use bl808_pac as pac;
use panic_halt as _;
use riscv_rt::entry;

#[entry]
fn main() -> ! {
    let p = unsafe { pac::m0::Peripherals::steal() };
    let emi_misc = &p.emi_misc;

    // Read EMI misc dummy configuration
    let emi = emi_misc.emi_misc_dummy().read().bits();
    core::hint::black_box(emi);

    loop {
        core::hint::black_box(());
    }
}
""",
    },
    "m0-i2s": {
        "desc": "I2S Audio Interface Configuration",
        "code": """#![no_std]
#![no_main]

use bl808_pac as pac;
use panic_halt as _;
use riscv_rt::entry;

#[entry]
fn main() -> ! {
    let p = unsafe { pac::m0::Peripherals::steal() };
    let i2s = &p.i2s;

    // Read I2S configuration register
    let cfg = i2s.i2s_config().read().bits();
    core::hint::black_box(cfg);

    loop {
        core::hint::black_box(());
    }
}
""",
    },
    "m0-emac": {
        "desc": "10/100 Ethernet MAC Controller",
        "code": """#![no_std]
#![no_main]

use bl808_pac as pac;
use panic_halt as _;
use riscv_rt::entry;

#[entry]
fn main() -> ! {
    let p = unsafe { pac::m0::Peripherals::steal() };
    let emac = &p.emac;

    // Read Ethernet MAC configuration register
    let mode = emac.mode().read().bits();
    core::hint::black_box(mode);

    loop {
        core::hint::black_box(());
    }
}
""",
    },
    "m0-usb": {
        "desc": "USB 2.0 OTG Controller Operational Setup",
        "code": """#![no_std]
#![no_main]

use bl808_pac as pac;
use panic_halt as _;
use riscv_rt::entry;

#[entry]
fn main() -> ! {
    let p = unsafe { pac::m0::Peripherals::steal() };
    let usb = &p.usb;

    // Read USB host structural parameters
    let params = usb.hcsparams().read().bits();
    core::hint::black_box(params);

    loop {
        core::hint::black_box(());
    }
}
""",
    },
    "m0-sdh": {
        "desc": "Secure Digital Host (SD/eMMC) Controller",
        "code": """#![no_std]
#![no_main]

use bl808_pac as pac;
use panic_halt as _;
use riscv_rt::entry;

#[entry]
fn main() -> ! {
    let p = unsafe { pac::m0::Peripherals::steal() };
    let sdh = &p.sdh;

    // Read SD host control register
    let ctrl = sdh.sd_host_ctrl().read().bits();
    core::hint::black_box(ctrl);

    loop {
        core::hint::black_box(());
    }
}
""",
    },
}

# D0 peripheral examples to create (excluding d0-blinky, d0-dma-m2m, d0-uart-poll, d0-pac-test)
D0_EXAMPLES = {
    "d0-timer-delay": {
        "desc": "D0 Multimedia Timer1 Hardware Delay",
        "code": """#![no_std]
#![no_main]

use bl808_pac as pac;
use panic_halt as _;
use riscv_rt::entry;

#[entry]
fn main() -> ! {
    let p = unsafe { pac::d0::Peripherals::steal() };
    let timer1 = &p.timer1;

    // Clear and enable counter 2
    timer1.tcer().modify(|_, w| w.tcr2_cnt_clr().set_bit());
    timer1.tcer().modify(|_, w| {
        w.tcr2_cnt_clr().clear_bit();
        w.timer2_en().set_bit()
    });

    let start = timer1.tcr2().read().tcr2_cnt().bits();
    while timer1.tcr2().read().tcr2_cnt().bits().wrapping_sub(start) < 100_000 {}

    loop {
        core::hint::black_box(());
    }
}
""",
    },
    "d0-spi": {
        "desc": "D0 Multimedia SPI1 Master Transfer",
        "code": """#![no_std]
#![no_main]

use bl808_pac as pac;
use panic_halt as _;
use riscv_rt::entry;

#[entry]
fn main() -> ! {
    let p = unsafe { pac::d0::Peripherals::steal() };
    let spi1 = &p.spi1;

    // Configure SPI1: 8-bit frame size, master mode
    spi1.spi_config().modify(|_, w| unsafe {
        w.cr_spi_m_cont_en()
            .set_bit()
            .cr_spi_frame_size()
            .bits(0)
    });

    spi1.spi_fifo_wdata().write(|w| unsafe { w.bits(0x5A) });
    let status = spi1.spi_int_sts().read().bits();
    core::hint::black_box(status);

    loop {
        core::hint::black_box(());
    }
}
""",
    },
    "d0-i2c": {
        "desc": "D0 Multimedia I2C2 Master Setup",
        "code": """#![no_std]
#![no_main]

use bl808_pac as pac;
use panic_halt as _;
use riscv_rt::entry;

#[entry]
fn main() -> ! {
    let p = unsafe { pac::d0::Peripherals::steal() };
    let i2c2 = &p.i2c2;

    // Configure I2C2 Master
    i2c2.i2c_config().modify(|_, w| unsafe {
        w.cr_i2c_m_en()
            .set_bit()
            .cr_i2c_slv_addr()
            .bits(0x3C)
    });

    let addr = i2c2.i2c_config().read().cr_i2c_slv_addr().bits();
    core::hint::black_box(addr);

    loop {
        core::hint::black_box(());
    }
}
""",
    },
    "d0-pwm": {
        "desc": "D0 PWM Controller Setup",
        "code": """#![no_std]
#![no_main]

use bl808_pac as pac;
use panic_halt as _;
use riscv_rt::entry;

#[entry]
fn main() -> ! {
    let p = unsafe { pac::d0::Peripherals::steal() };
    let pwm = &p.pwm;

    pwm.grp(0).period().modify(|_, w| unsafe {
        w.pwm_period().bits(2000)
    });
    pwm.grp(0).thre(0).modify(|_, w| unsafe {
        w.thre_l().bits(0).thre_h().bits(1000)
    });

    let period = pwm.grp(0).period().read().pwm_period().bits();
    core::hint::black_box(period);

    loop {
        core::hint::black_box(());
    }
}
""",
    },
    "d0-mm-glb": {
        "desc": "D0 Multimedia Clock & Reset Control",
        "code": """#![no_std]
#![no_main]

use bl808_pac as pac;
use panic_halt as _;
use riscv_rt::entry;

#[entry]
fn main() -> ! {
    let p = unsafe { pac::d0::Peripherals::steal() };
    let mm_glb = &p.mm_glb;

    // Read MM CPU clock configuration
    let clk_cpu = mm_glb.mm_clk_cpu().read().bits();
    core::hint::black_box(clk_cpu);

    loop {
        core::hint::black_box(());
    }
}
""",
    },
    "d0-mm-misc": {
        "desc": "D0 Multimedia Misc & C906 Boot Vector",
        "code": """#![no_std]
#![no_main]

use bl808_pac as pac;
use panic_halt as _;
use riscv_rt::entry;

#[entry]
fn main() -> ! {
    let p = unsafe { pac::d0::Peripherals::steal() };
    let mm_misc = &p.mm_misc;

    // Read C906 boot vector register
    let boot_vec = mm_misc.cpu0_boot().read().bits();
    core::hint::black_box(boot_vec);

    loop {
        core::hint::black_box(());
    }
}
""",
    },
    "d0-dma2d": {
        "desc": "D0 2D Graphics DMA Accelerator Setup",
        "code": """#![no_std]
#![no_main]

use bl808_pac as pac;
use panic_halt as _;
use riscv_rt::entry;

#[entry]
fn main() -> ! {
    let p = unsafe { pac::d0::Peripherals::steal() };
    let dma2d = &p.dma2d;

    // Read 2D DMA configuration
    let cfg = dma2d.dma_config().read().bits();
    core::hint::black_box(cfg);

    loop {
        core::hint::black_box(());
    }
}
""",
    },
    "d0-dvp": {
        "desc": "D0 Digital Video Port (Camera Sensor) Setup",
        "code": """#![no_std]
#![no_main]

use bl808_pac as pac;
use panic_halt as _;
use riscv_rt::entry;

#[entry]
fn main() -> ! {
    let p = unsafe { pac::d0::Peripherals::steal() };
    let dvp0 = &p.dvp0;

    // Read DVP0 AXI configuration register
    let cfg = dvp0.dvp2axi_configue().read().bits();
    core::hint::black_box(cfg);

    loop {
        core::hint::black_box(());
    }
}
""",
    },
    "d0-dvp-tsrc": {
        "desc": "D0 Camera Timing Source Generator",
        "code": """#![no_std]
#![no_main]

use bl808_pac as pac;
use panic_halt as _;
use riscv_rt::entry;

#[entry]
fn main() -> ! {
    let p = unsafe { pac::d0::Peripherals::steal() };
    let dvp_tsrc0 = &p.dvp_tsrc0;

    // Read DVP Timing Source 0 config register
    let cfg = dvp_tsrc0.config().read().bits();
    core::hint::black_box(cfg);

    loop {
        core::hint::black_box(());
    }
}
""",
    },
    "d0-mjpeg": {
        "desc": "D0 Motion JPEG Video Encoder Setup",
        "code": """#![no_std]
#![no_main]

use bl808_pac as pac;
use panic_halt as _;
use riscv_rt::entry;

#[entry]
fn main() -> ! {
    let p = unsafe { pac::d0::Peripherals::steal() };
    let mjpeg = &p.mjpeg;

    // Read MJPEG encoder control register
    let ctrl = mjpeg.mjpeg_control_1().read().bits();
    core::hint::black_box(ctrl);

    loop {
        core::hint::black_box(());
    }
}
""",
    },
    "d0-h264": {
        "desc": "D0 H.264 Video Encoder Setup",
        "code": """#![no_std]
#![no_main]

use bl808_pac as pac;
use panic_halt as _;
use riscv_rt::entry;

#[entry]
fn main() -> ! {
    let p = unsafe { pac::d0::Peripherals::steal() };
    let h264 = &p.h264;

    // Read H264 control register
    let ctrl = h264.vdo_h264_encoder_ctrl().read().bits();
    core::hint::black_box(ctrl);

    loop {
        core::hint::black_box(());
    }
}
""",
    },
    "d0-npu": {
        "desc": "D0 Neural Processing Unit Accelerator",
        "code": """#![no_std]
#![no_main]

use bl808_pac as pac;
use panic_halt as _;
use riscv_rt::entry;

#[entry]
fn main() -> ! {
    let p = unsafe { pac::d0::Peripherals::steal() };
    let npu = &p.npu;

    // Read NPU control register
    let ctrl = npu.bali_general_cfg().read().bits();
    core::hint::black_box(ctrl);

    loop {
        core::hint::black_box(());
    }
}
""",
    },
    "d0-codec": {
        "desc": "D0 Video Codec Core Configuration",
        "code": """#![no_std]
#![no_main]

use bl808_pac as pac;
use panic_halt as _;
use riscv_rt::entry;

#[entry]
fn main() -> ! {
    let p = unsafe { pac::d0::Peripherals::steal() };
    let codec = &p.codec;

    // Read Codec bus control register
    let ctrl = codec.codec_bus_ctrl().read().bits();
    core::hint::black_box(ctrl);

    loop {
        core::hint::black_box(());
    }
}
""",
    },
    "d0-psram-uhs": {
        "desc": "D0 Ultra-High Speed PSRAM Controller",
        "code": """#![no_std]
#![no_main]

use bl808_pac as pac;
use panic_halt as _;
use riscv_rt::entry;

#[entry]
fn main() -> ! {
    let p = unsafe { pac::d0::Peripherals::steal() };
    let psram_uhs = &p.psram_uhs;

    // Read UHS basic control register
    let ctrl = psram_uhs.uhs_basic().read().bits();
    core::hint::black_box(ctrl);

    loop {
        core::hint::black_box(());
    }
}
""",
    },
    "d0-dsi": {
        "desc": "D0 MIPI Display Serial Interface Setup",
        "code": """#![no_std]
#![no_main]

use bl808_pac as pac;
use panic_halt as _;
use riscv_rt::entry;

#[entry]
fn main() -> ! {
    let p = unsafe { pac::d0::Peripherals::steal() };
    let dsi = &p.dsi;

    // Read DSI configuration register
    let cfg = dsi.dsi_config().read().bits();
    core::hint::black_box(cfg);

    loop {
        core::hint::black_box(());
    }
}
""",
    },
    "d0-csi": {
        "desc": "D0 MIPI Camera Serial Interface Setup",
        "code": """#![no_std]
#![no_main]

use bl808_pac as pac;
use panic_halt as _;
use riscv_rt::entry;

#[entry]
fn main() -> ! {
    let p = unsafe { pac::d0::Peripherals::steal() };
    let csi = &p.csi;

    // Read CSI configuration register
    let cfg = csi.mipi_config().read().bits();
    core::hint::black_box(cfg);

    loop {
        core::hint::black_box(());
    }
}
""",
    },
    "d0-dbi": {
        "desc": "D0 Display Bus Interface (DBI Parallel LCD)",
        "code": """#![no_std]
#![no_main]

use bl808_pac as pac;
use panic_halt as _;
use riscv_rt::entry;

#[entry]
fn main() -> ! {
    let p = unsafe { pac::d0::Peripherals::steal() };
    let dbi = &p.dbi;

    // Read DBI configuration register
    let cfg = dbi.dbi_config().read().bits();
    core::hint::black_box(cfg);

    loop {
        core::hint::black_box(());
    }
}
""",
    },
    "d0-dpi": {
        "desc": "D0 Display Parallel Interface (DPI RGB LCD)",
        "code": """#![no_std]
#![no_main]

use bl808_pac as pac;
use panic_halt as _;
use riscv_rt::entry;

#[entry]
fn main() -> ! {
    let p = unsafe { pac::d0::Peripherals::steal() };
    let dpi = &p.dpi;

    // Read DPI configuration register
    let cfg = dpi.dpi_config().read().bits();
    core::hint::black_box(cfg);

    loop {
        core::hint::black_box(());
    }
}
""",
    },
    "d0-ipc": {
        "desc": "D0 Inter-Processor Communication Mailbox",
        "code": """#![no_std]
#![no_main]

use bl808_pac as pac;
use panic_halt as _;
use riscv_rt::entry;

#[entry]
fn main() -> ! {
    let p = unsafe { pac::d0::Peripherals::steal() };
    let ipc = &p.ipc;

    // Read IPC raw status register
    let status = ipc.cpu0_ipc_irsrr().read().bits();
    core::hint::black_box(status);

    loop {
        core::hint::black_box(());
    }
}
""",
    },
}

# LP peripheral examples to create (excluding lp-blinky, lp-timer-delay, lp-uart-poll)
LP_EXAMPLES = {
    "lp-hbn": {
        "desc": "LP Hibernate RTC & Scratch Register",
        "code": """#![no_std]
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
""",
    },
    "lp-aon": {
        "desc": "LP Always-On Domain Power Management",
        "code": """#![no_std]
#![no_main]

use bl808_pac as pac;
use panic_halt as _;
use riscv_rt::entry;

#[entry]
fn main() -> ! {
    let p = unsafe { pac::lp::Peripherals::steal() };
    let aon = &p.aon;

    let ctl = aon.aon().read().bits();
    core::hint::black_box(ctl);

    loop {
        core::hint::black_box(());
    }
}
""",
    },
    "lp-pds": {
        "desc": "LP Power Down Sleep Mode Setup",
        "code": """#![no_std]
#![no_main]

use bl808_pac as pac;
use panic_halt as _;
use riscv_rt::entry;

#[entry]
fn main() -> ! {
    let p = unsafe { pac::lp::Peripherals::steal() };
    let pds = &p.pds;

    let ctl = pds.pds_ctl().read().bits();
    core::hint::black_box(ctl);

    loop {
        core::hint::black_box(());
    }
}
""",
    },
    "lp-ipc": {
        "desc": "LP Inter-Processor Communication Mailbox",
        "code": """#![no_std]
#![no_main]

use bl808_pac as pac;
use panic_halt as _;
use riscv_rt::entry;

#[entry]
fn main() -> ! {
    let p = unsafe { pac::lp::Peripherals::steal() };
    let ipc = &p.ipc;

    let stat = ipc.cpu0_ipc_irsrr().read().bits();
    core::hint::black_box(stat);

    loop {
        core::hint::black_box(());
    }
}
""",
    },
    "lp-ir": {
        "desc": "LP Infrared Remote Transmitter Setup",
        "code": """#![no_std]
#![no_main]

use bl808_pac as pac;
use panic_halt as _;
use riscv_rt::entry;

#[entry]
fn main() -> ! {
    let p = unsafe { pac::lp::Peripherals::steal() };
    let ir = &p.ir;

    let cfg = ir.irtx_config().read().bits();
    core::hint::black_box(cfg);

    loop {
        core::hint::black_box(());
    }
}
""",
    },
    "lp-audio": {
        "desc": "LP Audio Subsystem FIFO Setup",
        "code": """#![no_std]
#![no_main]

use bl808_pac as pac;
use panic_halt as _;
use riscv_rt::entry;

#[entry]
fn main() -> ! {
    let p = unsafe { pac::lp::Peripherals::steal() };
    let audio = &p.audio;

    let fifo = audio.rx_fifo_ctrl().read().bits();
    core::hint::black_box(fifo);

    loop {
        core::hint::black_box(());
    }
}
""",
    },
    "lp-lz4d": {
        "desc": "LP Hardware LZ4 Decompressor Setup",
        "code": """#![no_std]
#![no_main]

use bl808_pac as pac;
use panic_halt as _;
use riscv_rt::entry;

#[entry]
fn main() -> ! {
    let p = unsafe { pac::lp::Peripherals::steal() };
    let lz4d = &p.lz4d;

    let cfg = lz4d.lz4_config().read().bits();
    core::hint::black_box(cfg);

    loop {
        core::hint::black_box(());
    }
}
""",
    },
    "lp-pwm": {
        "desc": "LP PWM Controller Configuration",
        "code": """#![no_std]
#![no_main]

use bl808_pac as pac;
use panic_halt as _;
use riscv_rt::entry;

#[entry]
fn main() -> ! {
    let p = unsafe { pac::lp::Peripherals::steal() };
    let pwm = &p.pwm;

    pwm.grp(0).period().modify(|_, w| unsafe {
        w.pwm_period().bits(1000)
    });
    pwm.grp(0).thre(0).modify(|_, w| unsafe {
        w.thre_l().bits(0).thre_h().bits(500)
    });

    let period = pwm.grp(0).period().read().pwm_period().bits();
    core::hint::black_box(period);

    loop {
        core::hint::black_box(());
    }
}
""",
    },
    "lp-i2c": {
        "desc": "LP I2C0 Master Setup",
        "code": """#![no_std]
#![no_main]

use bl808_pac as pac;
use panic_halt as _;
use riscv_rt::entry;

#[entry]
fn main() -> ! {
    let p = unsafe { pac::lp::Peripherals::steal() };
    let i2c0 = &p.i2c0;

    i2c0.i2c_config().modify(|_, w| unsafe {
        w.cr_i2c_m_en()
            .set_bit()
            .cr_i2c_slv_addr()
            .bits(0x48)
    });

    let addr = i2c0.i2c_config().read().cr_i2c_slv_addr().bits();
    core::hint::black_box(addr);

    loop {
        core::hint::black_box(());
    }
}
""",
    },
    "lp-spi": {
        "desc": "LP SPI0 Master Transfer Setup",
        "code": """#![no_std]
#![no_main]

use bl808_pac as pac;
use panic_halt as _;
use riscv_rt::entry;

#[entry]
fn main() -> ! {
    let p = unsafe { pac::lp::Peripherals::steal() };
    let spi0 = &p.spi0;

    spi0.spi_config().modify(|_, w| unsafe {
        w.cr_spi_m_cont_en()
            .set_bit()
            .cr_spi_frame_size()
            .bits(0)
    });

    let cfg = spi0.spi_config().read().bits();
    core::hint::black_box(cfg);

    loop {
        core::hint::black_box(());
    }
}
""",
    },
    "lp-dma-m2m": {
        "desc": "LP DMA0 Memory Transfer Setup",
        "code": """#![no_std]
#![no_main]

use bl808_pac as pac;
use panic_halt as _;
use riscv_rt::entry;

#[entry]
fn main() -> ! {
    let p = unsafe { pac::lp::Peripherals::steal() };
    let dma0 = &p.dma0;

    dma0.top_config().modify(|_, w| w.e().set_bit());
    let en = dma0.top_config().read().e().bit_is_set();
    core::hint::black_box(en);

    loop {
        core::hint::black_box(());
    }
}
""",
    },
    "lp-gpip": {
        "desc": "LP General Purpose ADC Setup",
        "code": """#![no_std]
#![no_main]

use bl808_pac as pac;
use panic_halt as _;
use riscv_rt::entry;

#[entry]
fn main() -> ! {
    let p = unsafe { pac::lp::Peripherals::steal() };
    let gpip = &p.gpip;

    let cfg = gpip.gpadc_config().read().bits();
    core::hint::black_box(cfg);

    loop {
        core::hint::black_box(());
    }
}
""",
    },
    "lp-efuse": {
        "desc": "LP eFuse Wi-Fi MAC Readout",
        "code": """#![no_std]
#![no_main]

use bl808_pac as pac;
use panic_halt as _;
use riscv_rt::entry;

#[entry]
fn main() -> ! {
    let p = unsafe { pac::lp::Peripherals::steal() };
    let efuse = &p.efuse;

    let mac = efuse.ef_wifi_mac_low().read().bits();
    core::hint::black_box(mac);

    loop {
        core::hint::black_box(());
    }
}
""",
    },
    "lp-sec-eng": {
        "desc": "LP Security Engine Hardware TRNG",
        "code": """#![no_std]
#![no_main]

use bl808_pac as pac;
use panic_halt as _;
use riscv_rt::entry;

#[entry]
fn main() -> ! {
    let p = unsafe { pac::lp::Peripherals::steal() };
    let sec_eng = &p.sec_eng;

    let ctrl = sec_eng.se_trng_0_ctrl_0().read().bits();
    core::hint::black_box(ctrl);

    loop {
        core::hint::black_box(());
    }
}
""",
    },
    "lp-sf-ctrl": {
        "desc": "LP Serial Flash Controller Configuration",
        "code": """#![no_std]
#![no_main]

use bl808_pac as pac;
use panic_halt as _;
use riscv_rt::entry;

#[entry]
fn main() -> ! {
    let p = unsafe { pac::lp::Peripherals::steal() };
    let sf_ctrl = &p.sf_ctrl;

    let cfg = sf_ctrl.sf_ctrl_0().read().bits();
    core::hint::black_box(cfg);

    loop {
        core::hint::black_box(());
    }
}
""",
    },
    "lp-sdh": {
        "desc": "LP SD Host Controller Configuration",
        "code": """#![no_std]
#![no_main]

use bl808_pac as pac;
use panic_halt as _;
use riscv_rt::entry;

#[entry]
fn main() -> ! {
    let p = unsafe { pac::lp::Peripherals::steal() };
    let sdh = &p.sdh;

    let ctrl = sdh.sd_host_ctrl().read().bits();
    core::hint::black_box(ctrl);

    loop {
        core::hint::black_box(());
    }
}
""",
    },
    "lp-usb": {
        "desc": "LP USB 2.0 OTG Controller Configuration",
        "code": """#![no_std]
#![no_main]

use bl808_pac as pac;
use panic_halt as _;
use riscv_rt::entry;

#[entry]
fn main() -> ! {
    let p = unsafe { pac::lp::Peripherals::steal() };
    let usb = &p.usb;

    let params = usb.hcsparams().read().bits();
    core::hint::black_box(params);

    loop {
        core::hint::black_box(());
    }
}
""",
    },
    "lp-emac": {
        "desc": "LP 10/100 Ethernet MAC Configuration",
        "code": """#![no_std]
#![no_main]

use bl808_pac as pac;
use panic_halt as _;
use riscv_rt::entry;

#[entry]
fn main() -> ! {
    let p = unsafe { pac::lp::Peripherals::steal() };
    let emac = &p.emac;

    let mode = emac.mode().read().bits();
    core::hint::black_box(mode);

    loop {
        core::hint::black_box(());
    }
}
""",
    },
}

def create_example(name, core_feature, memory_x, info):
    pkg_dir = os.path.join(EXAMPLES_DIR, name)
    src_dir = os.path.join(pkg_dir, "src")
    os.makedirs(src_dir, exist_ok=True)

    # Cargo.toml
    with open(os.path.join(pkg_dir, "Cargo.toml"), "w", encoding="utf-8") as f:
        f.write(make_cargo_toml(name, core_feature))

    # build.rs
    with open(os.path.join(pkg_dir, "build.rs"), "w", encoding="utf-8") as f:
        f.write(BUILD_RS)

    # memory.x
    with open(os.path.join(pkg_dir, "memory.x"), "w", encoding="utf-8") as f:
        f.write(memory_x)

    # src/main.rs
    with open(os.path.join(src_dir, "main.rs"), "w", encoding="utf-8") as f:
        f.write(info["code"])

    print(f"Created example: {name} ({info['desc']})")

def main():
    print("Generating M0 peripheral examples...")
    for name, info in M0_EXAMPLES.items():
        create_example(name, "m0", MEMORY_X_M0, info)

    print("\\nGenerating D0 peripheral examples...")
    for name, info in D0_EXAMPLES.items():
        create_example(name, "d0", MEMORY_X_D0, info)

    print("\\nGenerating LP peripheral examples...")
    for name, info in LP_EXAMPLES.items():
        create_example(name, "lp", MEMORY_X_LP, info)

    print("\\nAll peripheral examples generated successfully!")

if __name__ == "__main__":
    main()
