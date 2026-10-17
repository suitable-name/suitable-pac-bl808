#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x40],
    tzc_rom_tzsrg_ctrl: TzcRomTzsrgCtrl,
    tzc_rom_tzsrg_adr_mask: TzcRomTzsrgAdrMask,
    tzc_rom_tzsrg_r0: TzcRomTzsrgR0,
    tzc_rom_tzsrg_r1: TzcRomTzsrgR1,
    tzc_rom_tzsrg_r2: TzcRomTzsrgR2,
    tzc_rom_tzsrg_r3: TzcRomTzsrgR3,
    _reserved6: [u8; 0xa8],
    tzc_bmx_tzmid: TzcBmxTzmid,
    tzc_bmx_tzmid_lock: TzcBmxTzmidLock,
    tzc_bmx_s0: TzcBmxS0,
    tzc_bmx_s1: TzcBmxS1,
    tzc_bmx_s2: TzcBmxS2,
    tzc_bmx_s_lock: TzcBmxSLock,
    _reserved12: [u8; 0x28],
    tzc_ocram_tzsrg_ctrl: TzcOcramTzsrgCtrl,
    tzc_ocram_tzsrg_adr_mask: TzcOcramTzsrgAdrMask,
    tzc_ocram_tzsrg_r0: TzcOcramTzsrgR0,
    tzc_ocram_tzsrg_r1: TzcOcramTzsrgR1,
    tzc_ocram_tzsrg_r2: TzcOcramTzsrgR2,
    tzc_ocram_tzsrg_r3: TzcOcramTzsrgR3,
    _reserved18: [u8; 0x28],
    tzc_wram_tzsrg_ctrl: TzcWramTzsrgCtrl,
    tzc_wram_tzsrg_adr_mask: TzcWramTzsrgAdrMask,
    tzc_wram_tzsrg_r0: TzcWramTzsrgR0,
    tzc_wram_tzsrg_r1: TzcWramTzsrgR1,
    tzc_wram_tzsrg_r2: TzcWramTzsrgR2,
    tzc_wram_tzsrg_r3: TzcWramTzsrgR3,
    tzc_wifi_dbg: TzcWifiDbg,
    _reserved25: [u8; 0xa4],
    tzc_pdm_ctrl: TzcPdmCtrl,
    tzc_uart_ctrl: TzcUartCtrl,
    tzc_i2c_ctrl: TzcI2cCtrl,
    tzc_timer_ctrl: TzcTimerCtrl,
    tzc_i2s_ctrl: TzcI2sCtrl,
    _reserved30: [u8; 0x2c],
    tzc_sf_tzsrg_ctrl: TzcSfTzsrgCtrl,
    tzc_sf_tzsrg_adr_mask: TzcSfTzsrgAdrMask,
    tzc_sf_tzsrg_r0: TzcSfTzsrgR0,
    tzc_sf_tzsrg_r1: TzcSfTzsrgR1,
    tzc_sf_tzsrg_r2: TzcSfTzsrgR2,
    tzc_sf_tzsrg_r3: TzcSfTzsrgR3,
    tzc_sf_tzsrg_msb: TzcSfTzsrgMsb,
    _reserved37: [u8; 0x64],
    tzc_mm_bmx_tzmid: TzcMmBmxTzmid,
    tzc_mm_bmx_tzmid_lock: TzcMmBmxTzmidLock,
    tzc_mm_bmx_s0: TzcMmBmxS0,
    tzc_mm_bmx_s1: TzcMmBmxS1,
    tzc_mm_bmx_s2: TzcMmBmxS2,
    tzc_mm_bmx_s_lock0: TzcMmBmxSLock0,
    tzc_mm_bmx_s_lock1: TzcMmBmxSLock1,
    _reserved44: [u8; 0x24],
    tzc_l2sram_tzsrg_ctrl: TzcL2sramTzsrgCtrl,
    tzc_l2sram_tzsrg_adr_mask: TzcL2sramTzsrgAdrMask,
    tzc_l2sram_tzsrg_r0: TzcL2sramTzsrgR0,
    tzc_l2sram_tzsrg_r1: TzcL2sramTzsrgR1,
    tzc_l2sram_tzsrg_r2: TzcL2sramTzsrgR2,
    tzc_l2sram_tzsrg_r3: TzcL2sramTzsrgR3,
    _reserved50: [u8; 0x08],
    tzc_vram_tzsrg_ctrl: TzcVramTzsrgCtrl,
    tzc_vram_tzsrg_adr_mask: TzcVramTzsrgAdrMask,
    tzc_vram_tzsrg_r0: TzcVramTzsrgR0,
    tzc_vram_tzsrg_r1: TzcVramTzsrgR1,
    tzc_vram_tzsrg_r2: TzcVramTzsrgR2,
    tzc_vram_tzsrg_r3: TzcVramTzsrgR3,
    _reserved56: [u8; 0x08],
    tzc_psrama_tzsrg_ctrl: TzcPsramaTzsrgCtrl,
    tzc_psrama_tzsrg_adr_mask: TzcPsramaTzsrgAdrMask,
    tzc_psrama_tzsrg_r0: TzcPsramaTzsrgR0,
    tzc_psrama_tzsrg_r1: TzcPsramaTzsrgR1,
    tzc_psrama_tzsrg_r2: TzcPsramaTzsrgR2,
    tzc_psrama_tzsrg_r3: TzcPsramaTzsrgR3,
    _reserved62: [u8; 0x08],
    tzc_psramb_tzsrg_ctrl: TzcPsrambTzsrgCtrl,
    tzc_psramb_tzsrg_adr_mask: TzcPsrambTzsrgAdrMask,
    tzc_psramb_tzsrg_r0: TzcPsrambTzsrgR0,
    tzc_psramb_tzsrg_r1: TzcPsrambTzsrgR1,
    tzc_psramb_tzsrg_r2: TzcPsrambTzsrgR2,
    tzc_psramb_tzsrg_r3: TzcPsrambTzsrgR3,
    _reserved68: [u8; 0x08],
    tzc_xram_tzsrg_ctrl: TzcXramTzsrgCtrl,
    tzc_xram_tzsrg_adr_mask: TzcXramTzsrgAdrMask,
    tzc_xram_tzsrg_r0: TzcXramTzsrgR0,
    tzc_xram_tzsrg_r1: TzcXramTzsrgR1,
    tzc_xram_tzsrg_r2: TzcXramTzsrgR2,
    tzc_xram_tzsrg_r3: TzcXramTzsrgR3,
    _reserved74: [u8; 0x0b28],
    tzc_glb_ctrl_0: TzcGlbCtrl0,
    tzc_glb_ctrl_1: TzcGlbCtrl1,
    tzc_glb_ctrl_2: TzcGlbCtrl2,
    _reserved77: [u8; 0x14],
    tzc_mm_ctrl_0: TzcMmCtrl0,
    tzc_mm_ctrl_1: TzcMmCtrl1,
    tzc_mm_ctrl_2: TzcMmCtrl2,
    _reserved80: [u8; 0x14],
    tzc_se_ctrl_0: TzcSeCtrl0,
    tzc_se_ctrl_1: TzcSeCtrl1,
    tzc_se_ctrl_2: TzcSeCtrl2,
}
impl RegisterBlock {
    #[doc = "0x40 - ROM TZSRG Control Register (Region Enable/ID Enable/Lock)"]
    #[inline(always)]
    pub const fn tzc_rom_tzsrg_ctrl(&self) -> &TzcRomTzsrgCtrl {
        &self.tzc_rom_tzsrg_ctrl
    }
    #[doc = "0x44 - ROM TZSRG Address Mask Register"]
    #[inline(always)]
    pub const fn tzc_rom_tzsrg_adr_mask(&self) -> &TzcRomTzsrgAdrMask {
        &self.tzc_rom_tzsrg_adr_mask
    }
    #[doc = "0x48 - ROM TZSRG Region 0 Start/End"]
    #[inline(always)]
    pub const fn tzc_rom_tzsrg_r0(&self) -> &TzcRomTzsrgR0 {
        &self.tzc_rom_tzsrg_r0
    }
    #[doc = "0x4c - ROM TZSRG Region 1 Start/End"]
    #[inline(always)]
    pub const fn tzc_rom_tzsrg_r1(&self) -> &TzcRomTzsrgR1 {
        &self.tzc_rom_tzsrg_r1
    }
    #[doc = "0x50 - ROM TZSRG Region 2 Start/End"]
    #[inline(always)]
    pub const fn tzc_rom_tzsrg_r2(&self) -> &TzcRomTzsrgR2 {
        &self.tzc_rom_tzsrg_r2
    }
    #[doc = "0x54 - ROM TZSRG Region 3 (Reserved)"]
    #[inline(always)]
    pub const fn tzc_rom_tzsrg_r3(&self) -> &TzcRomTzsrgR3 {
        &self.tzc_rom_tzsrg_r3
    }
    #[doc = "0x100 - Bus Master ID Security State (TZMID) Enable"]
    #[inline(always)]
    pub const fn tzc_bmx_tzmid(&self) -> &TzcBmxTzmid {
        &self.tzc_bmx_tzmid
    }
    #[doc = "0x104 - Bus Master ID Security State Lock (TZMID_LOCK)"]
    #[inline(always)]
    pub const fn tzc_bmx_tzmid_lock(&self) -> &TzcBmxTzmidLock {
        &self.tzc_bmx_tzmid_lock
    }
    #[doc = "0x108 - BMX Slave 0 Security ID Enable"]
    #[inline(always)]
    pub const fn tzc_bmx_s0(&self) -> &TzcBmxS0 {
        &self.tzc_bmx_s0
    }
    #[doc = "0x10c - BMX Slave 1 Security ID Enable (TZSID for AHB Slaves 0x10 - 0x1F)"]
    #[inline(always)]
    pub const fn tzc_bmx_s1(&self) -> &TzcBmxS1 {
        &self.tzc_bmx_s1
    }
    #[doc = "0x110 - BMX Slave 2 Security ID Enable (TZSID for AHB Slaves 0x20 - 0x2F)"]
    #[inline(always)]
    pub const fn tzc_bmx_s2(&self) -> &TzcBmxS2 {
        &self.tzc_bmx_s2
    }
    #[doc = "0x114 - BMX Slave TZSID Lock"]
    #[inline(always)]
    pub const fn tzc_bmx_s_lock(&self) -> &TzcBmxSLock {
        &self.tzc_bmx_s_lock
    }
    #[doc = "0x140 - OCRAM TZSRG Control Register (Region Enable/ID Enable/Lock)"]
    #[inline(always)]
    pub const fn tzc_ocram_tzsrg_ctrl(&self) -> &TzcOcramTzsrgCtrl {
        &self.tzc_ocram_tzsrg_ctrl
    }
    #[doc = "0x144 - OCRAM TZSRG Address Mask Register"]
    #[inline(always)]
    pub const fn tzc_ocram_tzsrg_adr_mask(&self) -> &TzcOcramTzsrgAdrMask {
        &self.tzc_ocram_tzsrg_adr_mask
    }
    #[doc = "0x148 - OCRAM TZSRG Region 0 Start/End"]
    #[inline(always)]
    pub const fn tzc_ocram_tzsrg_r0(&self) -> &TzcOcramTzsrgR0 {
        &self.tzc_ocram_tzsrg_r0
    }
    #[doc = "0x14c - OCRAM TZSRG Region 1 Start/End"]
    #[inline(always)]
    pub const fn tzc_ocram_tzsrg_r1(&self) -> &TzcOcramTzsrgR1 {
        &self.tzc_ocram_tzsrg_r1
    }
    #[doc = "0x150 - OCRAM TZSRG Region 2 Start/End"]
    #[inline(always)]
    pub const fn tzc_ocram_tzsrg_r2(&self) -> &TzcOcramTzsrgR2 {
        &self.tzc_ocram_tzsrg_r2
    }
    #[doc = "0x154 - OCRAM TZSRG Region 3 (Reserved)"]
    #[inline(always)]
    pub const fn tzc_ocram_tzsrg_r3(&self) -> &TzcOcramTzsrgR3 {
        &self.tzc_ocram_tzsrg_r3
    }
    #[doc = "0x180 - WRAM (MCU/Wireless RAM) TZSRG Control Register"]
    #[inline(always)]
    pub const fn tzc_wram_tzsrg_ctrl(&self) -> &TzcWramTzsrgCtrl {
        &self.tzc_wram_tzsrg_ctrl
    }
    #[doc = "0x184 - WRAM TZSRG Address Mask Register"]
    #[inline(always)]
    pub const fn tzc_wram_tzsrg_adr_mask(&self) -> &TzcWramTzsrgAdrMask {
        &self.tzc_wram_tzsrg_adr_mask
    }
    #[doc = "0x188 - WRAM TZSRG Region 0 Start/End"]
    #[inline(always)]
    pub const fn tzc_wram_tzsrg_r0(&self) -> &TzcWramTzsrgR0 {
        &self.tzc_wram_tzsrg_r0
    }
    #[doc = "0x18c - WRAM TZSRG Region 1 Start/End"]
    #[inline(always)]
    pub const fn tzc_wram_tzsrg_r1(&self) -> &TzcWramTzsrgR1 {
        &self.tzc_wram_tzsrg_r1
    }
    #[doc = "0x190 - WRAM TZSRG Region 2 Start/End"]
    #[inline(always)]
    pub const fn tzc_wram_tzsrg_r2(&self) -> &TzcWramTzsrgR2 {
        &self.tzc_wram_tzsrg_r2
    }
    #[doc = "0x194 - WRAM TZSRG Region 3 (Reserved)"]
    #[inline(always)]
    pub const fn tzc_wram_tzsrg_r3(&self) -> &TzcWramTzsrgR3 {
        &self.tzc_wram_tzsrg_r3
    }
    #[doc = "0x198 - WIFI Debug Control"]
    #[inline(always)]
    pub const fn tzc_wifi_dbg(&self) -> &TzcWifiDbg {
        &self.tzc_wifi_dbg
    }
    #[doc = "0x240 - PDM Control (Reserved/Placeholder for TZSID)"]
    #[inline(always)]
    pub const fn tzc_pdm_ctrl(&self) -> &TzcPdmCtrl {
        &self.tzc_pdm_ctrl
    }
    #[doc = "0x244 - UART Control (Reserved/Placeholder for TZSID)"]
    #[inline(always)]
    pub const fn tzc_uart_ctrl(&self) -> &TzcUartCtrl {
        &self.tzc_uart_ctrl
    }
    #[doc = "0x248 - I2C Control (Reserved/Placeholder for TZSID)"]
    #[inline(always)]
    pub const fn tzc_i2c_ctrl(&self) -> &TzcI2cCtrl {
        &self.tzc_i2c_ctrl
    }
    #[doc = "0x24c - TIMER Control (Reserved/Placeholder for TZSID)"]
    #[inline(always)]
    pub const fn tzc_timer_ctrl(&self) -> &TzcTimerCtrl {
        &self.tzc_timer_ctrl
    }
    #[doc = "0x250 - I2S Control (Reserved/Placeholder for TZSID)"]
    #[inline(always)]
    pub const fn tzc_i2s_ctrl(&self) -> &TzcI2sCtrl {
        &self.tzc_i2s_ctrl
    }
    #[doc = "0x280 - SF TZSRG Control Register"]
    #[inline(always)]
    pub const fn tzc_sf_tzsrg_ctrl(&self) -> &TzcSfTzsrgCtrl {
        &self.tzc_sf_tzsrg_ctrl
    }
    #[doc = "0x284 - SF TZSRG Address Mask Register"]
    #[inline(always)]
    pub const fn tzc_sf_tzsrg_adr_mask(&self) -> &TzcSfTzsrgAdrMask {
        &self.tzc_sf_tzsrg_adr_mask
    }
    #[doc = "0x288 - SF TZSRG Region 0 Start/End (Bits \\[18:0\\])"]
    #[inline(always)]
    pub const fn tzc_sf_tzsrg_r0(&self) -> &TzcSfTzsrgR0 {
        &self.tzc_sf_tzsrg_r0
    }
    #[doc = "0x28c - SF TZSRG Region 1 Start/End (Bits \\[18:0\\])"]
    #[inline(always)]
    pub const fn tzc_sf_tzsrg_r1(&self) -> &TzcSfTzsrgR1 {
        &self.tzc_sf_tzsrg_r1
    }
    #[doc = "0x290 - SF TZSRG Region 2 Start/End (Bits \\[18:0\\])"]
    #[inline(always)]
    pub const fn tzc_sf_tzsrg_r2(&self) -> &TzcSfTzsrgR2 {
        &self.tzc_sf_tzsrg_r2
    }
    #[doc = "0x294 - SF TZSRG Region 3 Start/End (Bits \\[18:0\\])"]
    #[inline(always)]
    pub const fn tzc_sf_tzsrg_r3(&self) -> &TzcSfTzsrgR3 {
        &self.tzc_sf_tzsrg_r3
    }
    #[doc = "0x298 - SF TZSRG Address MSB Configuration (Bits \\[21:19\\] for 64M Flash)"]
    #[inline(always)]
    pub const fn tzc_sf_tzsrg_msb(&self) -> &TzcSfTzsrgMsb {
        &self.tzc_sf_tzsrg_msb
    }
    #[doc = "0x300 - MM BMX Bus Master ID Security State"]
    #[inline(always)]
    pub const fn tzc_mm_bmx_tzmid(&self) -> &TzcMmBmxTzmid {
        &self.tzc_mm_bmx_tzmid
    }
    #[doc = "0x304 - MM BMX Bus Master ID Security State Lock"]
    #[inline(always)]
    pub const fn tzc_mm_bmx_tzmid_lock(&self) -> &TzcMmBmxTzmidLock {
        &self.tzc_mm_bmx_tzmid_lock
    }
    #[doc = "0x308 - MM BMX Slave 0 Security ID Enable (MM Peripherals)"]
    #[inline(always)]
    pub const fn tzc_mm_bmx_s0(&self) -> &TzcMmBmxS0 {
        &self.tzc_mm_bmx_s0
    }
    #[doc = "0x30c - MM BMX Slave 1 Security ID Enable (CODEC Subsystem)"]
    #[inline(always)]
    pub const fn tzc_mm_bmx_s1(&self) -> &TzcMmBmxS1 {
        &self.tzc_mm_bmx_s1
    }
    #[doc = "0x310 - MM BMX Slave 2 Security ID Enable (CODEC Subsystem Slaves)"]
    #[inline(always)]
    pub const fn tzc_mm_bmx_s2(&self) -> &TzcMmBmxS2 {
        &self.tzc_mm_bmx_s2
    }
    #[doc = "0x314 - MM BMX Slave Lock 0"]
    #[inline(always)]
    pub const fn tzc_mm_bmx_s_lock0(&self) -> &TzcMmBmxSLock0 {
        &self.tzc_mm_bmx_s_lock0
    }
    #[doc = "0x318 - MM BMX Slave Lock 1"]
    #[inline(always)]
    pub const fn tzc_mm_bmx_s_lock1(&self) -> &TzcMmBmxSLock1 {
        &self.tzc_mm_bmx_s_lock1
    }
    #[doc = "0x340 - L2SRAM TZSRG Control Register"]
    #[inline(always)]
    pub const fn tzc_l2sram_tzsrg_ctrl(&self) -> &TzcL2sramTzsrgCtrl {
        &self.tzc_l2sram_tzsrg_ctrl
    }
    #[doc = "0x344 - L2SRAM TZSRG Address Mask Register"]
    #[inline(always)]
    pub const fn tzc_l2sram_tzsrg_adr_mask(&self) -> &TzcL2sramTzsrgAdrMask {
        &self.tzc_l2sram_tzsrg_adr_mask
    }
    #[doc = "0x348 - L2SRAM TZSRG Region 0 Start/End"]
    #[inline(always)]
    pub const fn tzc_l2sram_tzsrg_r0(&self) -> &TzcL2sramTzsrgR0 {
        &self.tzc_l2sram_tzsrg_r0
    }
    #[doc = "0x34c - L2SRAM TZSRG Region 1 Start/End"]
    #[inline(always)]
    pub const fn tzc_l2sram_tzsrg_r1(&self) -> &TzcL2sramTzsrgR1 {
        &self.tzc_l2sram_tzsrg_r1
    }
    #[doc = "0x350 - L2SRAM TZSRG Region 2 Start/End"]
    #[inline(always)]
    pub const fn tzc_l2sram_tzsrg_r2(&self) -> &TzcL2sramTzsrgR2 {
        &self.tzc_l2sram_tzsrg_r2
    }
    #[doc = "0x354 - L2SRAM TZSRG Region 3 (Reserved)"]
    #[inline(always)]
    pub const fn tzc_l2sram_tzsrg_r3(&self) -> &TzcL2sramTzsrgR3 {
        &self.tzc_l2sram_tzsrg_r3
    }
    #[doc = "0x360 - VRAM TZSRG Control Register"]
    #[inline(always)]
    pub const fn tzc_vram_tzsrg_ctrl(&self) -> &TzcVramTzsrgCtrl {
        &self.tzc_vram_tzsrg_ctrl
    }
    #[doc = "0x364 - VRAM TZSRG Address Mask Register"]
    #[inline(always)]
    pub const fn tzc_vram_tzsrg_adr_mask(&self) -> &TzcVramTzsrgAdrMask {
        &self.tzc_vram_tzsrg_adr_mask
    }
    #[doc = "0x368 - VRAM TZSRG Region 0 Start/End"]
    #[inline(always)]
    pub const fn tzc_vram_tzsrg_r0(&self) -> &TzcVramTzsrgR0 {
        &self.tzc_vram_tzsrg_r0
    }
    #[doc = "0x36c - VRAM TZSRG Region 1 Start/End"]
    #[inline(always)]
    pub const fn tzc_vram_tzsrg_r1(&self) -> &TzcVramTzsrgR1 {
        &self.tzc_vram_tzsrg_r1
    }
    #[doc = "0x370 - VRAM TZSRG Region 2 Start/End"]
    #[inline(always)]
    pub const fn tzc_vram_tzsrg_r2(&self) -> &TzcVramTzsrgR2 {
        &self.tzc_vram_tzsrg_r2
    }
    #[doc = "0x374 - VRAM TZSRG Region 3 (Reserved)"]
    #[inline(always)]
    pub const fn tzc_vram_tzsrg_r3(&self) -> &TzcVramTzsrgR3 {
        &self.tzc_vram_tzsrg_r3
    }
    #[doc = "0x380 - PSRAM-A TZSRG Control Register"]
    #[inline(always)]
    pub const fn tzc_psrama_tzsrg_ctrl(&self) -> &TzcPsramaTzsrgCtrl {
        &self.tzc_psrama_tzsrg_ctrl
    }
    #[doc = "0x384 - PSRAM-A TZSRG Address Mask Register"]
    #[inline(always)]
    pub const fn tzc_psrama_tzsrg_adr_mask(&self) -> &TzcPsramaTzsrgAdrMask {
        &self.tzc_psrama_tzsrg_adr_mask
    }
    #[doc = "0x388 - PSRAM-A TZSRG Region 0 Start/End"]
    #[inline(always)]
    pub const fn tzc_psrama_tzsrg_r0(&self) -> &TzcPsramaTzsrgR0 {
        &self.tzc_psrama_tzsrg_r0
    }
    #[doc = "0x38c - PSRAM-A TZSRG Region 1 Start/End"]
    #[inline(always)]
    pub const fn tzc_psrama_tzsrg_r1(&self) -> &TzcPsramaTzsrgR1 {
        &self.tzc_psrama_tzsrg_r1
    }
    #[doc = "0x390 - PSRAM-A TZSRG Region 2 Start/End"]
    #[inline(always)]
    pub const fn tzc_psrama_tzsrg_r2(&self) -> &TzcPsramaTzsrgR2 {
        &self.tzc_psrama_tzsrg_r2
    }
    #[doc = "0x394 - PSRAM-A TZSRG Region 3 (Reserved)"]
    #[inline(always)]
    pub const fn tzc_psrama_tzsrg_r3(&self) -> &TzcPsramaTzsrgR3 {
        &self.tzc_psrama_tzsrg_r3
    }
    #[doc = "0x3a0 - PSRAM-B TZSRG Control Register"]
    #[inline(always)]
    pub const fn tzc_psramb_tzsrg_ctrl(&self) -> &TzcPsrambTzsrgCtrl {
        &self.tzc_psramb_tzsrg_ctrl
    }
    #[doc = "0x3a4 - PSRAM-B TZSRG Address Mask Register"]
    #[inline(always)]
    pub const fn tzc_psramb_tzsrg_adr_mask(&self) -> &TzcPsrambTzsrgAdrMask {
        &self.tzc_psramb_tzsrg_adr_mask
    }
    #[doc = "0x3a8 - PSRAM-B TZSRG Region 0 Start/End"]
    #[inline(always)]
    pub const fn tzc_psramb_tzsrg_r0(&self) -> &TzcPsrambTzsrgR0 {
        &self.tzc_psramb_tzsrg_r0
    }
    #[doc = "0x3ac - PSRAM-B TZSRG Region 1 Start/End"]
    #[inline(always)]
    pub const fn tzc_psramb_tzsrg_r1(&self) -> &TzcPsrambTzsrgR1 {
        &self.tzc_psramb_tzsrg_r1
    }
    #[doc = "0x3b0 - PSRAM-B TZSRG Region 2 Start/End"]
    #[inline(always)]
    pub const fn tzc_psramb_tzsrg_r2(&self) -> &TzcPsrambTzsrgR2 {
        &self.tzc_psramb_tzsrg_r2
    }
    #[doc = "0x3b4 - PSRAM-B TZSRG Region 3 (Reserved)"]
    #[inline(always)]
    pub const fn tzc_psramb_tzsrg_r3(&self) -> &TzcPsrambTzsrgR3 {
        &self.tzc_psramb_tzsrg_r3
    }
    #[doc = "0x3c0 - XRAM TZSRG Control Register"]
    #[inline(always)]
    pub const fn tzc_xram_tzsrg_ctrl(&self) -> &TzcXramTzsrgCtrl {
        &self.tzc_xram_tzsrg_ctrl
    }
    #[doc = "0x3c4 - XRAM TZSRG Address Mask Register"]
    #[inline(always)]
    pub const fn tzc_xram_tzsrg_adr_mask(&self) -> &TzcXramTzsrgAdrMask {
        &self.tzc_xram_tzsrg_adr_mask
    }
    #[doc = "0x3c8 - XRAM TZSRG Region 0 Start/End"]
    #[inline(always)]
    pub const fn tzc_xram_tzsrg_r0(&self) -> &TzcXramTzsrgR0 {
        &self.tzc_xram_tzsrg_r0
    }
    #[doc = "0x3cc - XRAM TZSRG Region 1 Start/End"]
    #[inline(always)]
    pub const fn tzc_xram_tzsrg_r1(&self) -> &TzcXramTzsrgR1 {
        &self.tzc_xram_tzsrg_r1
    }
    #[doc = "0x3d0 - XRAM TZSRG Region 2 Start/End"]
    #[inline(always)]
    pub const fn tzc_xram_tzsrg_r2(&self) -> &TzcXramTzsrgR2 {
        &self.tzc_xram_tzsrg_r2
    }
    #[doc = "0x3d4 - XRAM TZSRG Region 3 (Reserved)"]
    #[inline(always)]
    pub const fn tzc_xram_tzsrg_r3(&self) -> &TzcXramTzsrgR3 {
        &self.tzc_xram_tzsrg_r3
    }
    #[doc = "0xf00 - Global Control 0 (System Reset TZID Enable)"]
    #[inline(always)]
    pub const fn tzc_glb_ctrl_0(&self) -> &TzcGlbCtrl0 {
        &self.tzc_glb_ctrl_0
    }
    #[doc = "0xf04 - Global Control 1 (Reserved)"]
    #[inline(always)]
    pub const fn tzc_glb_ctrl_1(&self) -> &TzcGlbCtrl1 {
        &self.tzc_glb_ctrl_1
    }
    #[doc = "0xf08 - Global Control 2 (System Reset TZID Lock)"]
    #[inline(always)]
    pub const fn tzc_glb_ctrl_2(&self) -> &TzcGlbCtrl2 {
        &self.tzc_glb_ctrl_2
    }
    #[doc = "0xf20 - MM Control 0 (MM System Reset TZID Enable)"]
    #[inline(always)]
    pub const fn tzc_mm_ctrl_0(&self) -> &TzcMmCtrl0 {
        &self.tzc_mm_ctrl_0
    }
    #[doc = "0xf24 - MM Control 1 (Reserved)"]
    #[inline(always)]
    pub const fn tzc_mm_ctrl_1(&self) -> &TzcMmCtrl1 {
        &self.tzc_mm_ctrl_1
    }
    #[doc = "0xf28 - MM Control 2 (MM System Reset TZID Lock)"]
    #[inline(always)]
    pub const fn tzc_mm_ctrl_2(&self) -> &TzcMmCtrl2 {
        &self.tzc_mm_ctrl_2
    }
    #[doc = "0xf40 - Security Engine Control 0 (TZID Enable)"]
    #[inline(always)]
    pub const fn tzc_se_ctrl_0(&self) -> &TzcSeCtrl0 {
        &self.tzc_se_ctrl_0
    }
    #[doc = "0xf44 - Security Engine Control 1 (Flash/SF Controls)"]
    #[inline(always)]
    pub const fn tzc_se_ctrl_1(&self) -> &TzcSeCtrl1 {
        &self.tzc_se_ctrl_1
    }
    #[doc = "0xf48 - Security Engine Control 2 (TZID Lock)"]
    #[inline(always)]
    pub const fn tzc_se_ctrl_2(&self) -> &TzcSeCtrl2 {
        &self.tzc_se_ctrl_2
    }
}
#[doc = "TZC_ROM_TZSRG_CTRL (r) register accessor: ROM TZSRG Control Register (Region Enable/ID Enable/Lock)\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_rom_tzsrg_ctrl::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_rom_tzsrg_ctrl`] module"]
#[doc(alias = "TZC_ROM_TZSRG_CTRL")]
pub type TzcRomTzsrgCtrl = crate::Reg<tzc_rom_tzsrg_ctrl::TzcRomTzsrgCtrlSpec>;
#[doc = "ROM TZSRG Control Register (Region Enable/ID Enable/Lock)"]
pub mod tzc_rom_tzsrg_ctrl;
#[doc = "TZC_ROM_TZSRG_ADR_MASK (r) register accessor: ROM TZSRG Address Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_rom_tzsrg_adr_mask::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_rom_tzsrg_adr_mask`] module"]
#[doc(alias = "TZC_ROM_TZSRG_ADR_MASK")]
pub type TzcRomTzsrgAdrMask = crate::Reg<tzc_rom_tzsrg_adr_mask::TzcRomTzsrgAdrMaskSpec>;
#[doc = "ROM TZSRG Address Mask Register"]
pub mod tzc_rom_tzsrg_adr_mask;
#[doc = "TZC_ROM_TZSRG_R0 (r) register accessor: ROM TZSRG Region 0 Start/End\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_rom_tzsrg_r0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_rom_tzsrg_r0`] module"]
#[doc(alias = "TZC_ROM_TZSRG_R0")]
pub type TzcRomTzsrgR0 = crate::Reg<tzc_rom_tzsrg_r0::TzcRomTzsrgR0Spec>;
#[doc = "ROM TZSRG Region 0 Start/End"]
pub mod tzc_rom_tzsrg_r0;
#[doc = "TZC_ROM_TZSRG_R1 (r) register accessor: ROM TZSRG Region 1 Start/End\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_rom_tzsrg_r1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_rom_tzsrg_r1`] module"]
#[doc(alias = "TZC_ROM_TZSRG_R1")]
pub type TzcRomTzsrgR1 = crate::Reg<tzc_rom_tzsrg_r1::TzcRomTzsrgR1Spec>;
#[doc = "ROM TZSRG Region 1 Start/End"]
pub mod tzc_rom_tzsrg_r1;
#[doc = "TZC_ROM_TZSRG_R2 (r) register accessor: ROM TZSRG Region 2 Start/End\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_rom_tzsrg_r2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_rom_tzsrg_r2`] module"]
#[doc(alias = "TZC_ROM_TZSRG_R2")]
pub type TzcRomTzsrgR2 = crate::Reg<tzc_rom_tzsrg_r2::TzcRomTzsrgR2Spec>;
#[doc = "ROM TZSRG Region 2 Start/End"]
pub mod tzc_rom_tzsrg_r2;
#[doc = "TZC_ROM_TZSRG_R3 (rw) register accessor: ROM TZSRG Region 3 (Reserved)\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_rom_tzsrg_r3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tzc_rom_tzsrg_r3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_rom_tzsrg_r3`] module"]
#[doc(alias = "TZC_ROM_TZSRG_R3")]
pub type TzcRomTzsrgR3 = crate::Reg<tzc_rom_tzsrg_r3::TzcRomTzsrgR3Spec>;
#[doc = "ROM TZSRG Region 3 (Reserved)"]
pub mod tzc_rom_tzsrg_r3;
#[doc = "TZC_BMX_TZMID (r) register accessor: Bus Master ID Security State (TZMID) Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_bmx_tzmid::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_bmx_tzmid`] module"]
#[doc(alias = "TZC_BMX_TZMID")]
pub type TzcBmxTzmid = crate::Reg<tzc_bmx_tzmid::TzcBmxTzmidSpec>;
#[doc = "Bus Master ID Security State (TZMID) Enable"]
pub mod tzc_bmx_tzmid;
#[doc = "TZC_BMX_TZMID_LOCK (r) register accessor: Bus Master ID Security State Lock (TZMID_LOCK)\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_bmx_tzmid_lock::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_bmx_tzmid_lock`] module"]
#[doc(alias = "TZC_BMX_TZMID_LOCK")]
pub type TzcBmxTzmidLock = crate::Reg<tzc_bmx_tzmid_lock::TzcBmxTzmidLockSpec>;
#[doc = "Bus Master ID Security State Lock (TZMID_LOCK)"]
pub mod tzc_bmx_tzmid_lock;
#[doc = "TZC_BMX_S0 (r) register accessor: BMX Slave 0 Security ID Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_bmx_s0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_bmx_s0`] module"]
#[doc(alias = "TZC_BMX_S0")]
pub type TzcBmxS0 = crate::Reg<tzc_bmx_s0::TzcBmxS0Spec>;
#[doc = "BMX Slave 0 Security ID Enable"]
pub mod tzc_bmx_s0;
#[doc = "TZC_BMX_S1 (r) register accessor: BMX Slave 1 Security ID Enable (TZSID for AHB Slaves 0x10 - 0x1F)\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_bmx_s1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_bmx_s1`] module"]
#[doc(alias = "TZC_BMX_S1")]
pub type TzcBmxS1 = crate::Reg<tzc_bmx_s1::TzcBmxS1Spec>;
#[doc = "BMX Slave 1 Security ID Enable (TZSID for AHB Slaves 0x10 - 0x1F)"]
pub mod tzc_bmx_s1;
#[doc = "TZC_BMX_S2 (r) register accessor: BMX Slave 2 Security ID Enable (TZSID for AHB Slaves 0x20 - 0x2F)\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_bmx_s2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_bmx_s2`] module"]
#[doc(alias = "TZC_BMX_S2")]
pub type TzcBmxS2 = crate::Reg<tzc_bmx_s2::TzcBmxS2Spec>;
#[doc = "BMX Slave 2 Security ID Enable (TZSID for AHB Slaves 0x20 - 0x2F)"]
pub mod tzc_bmx_s2;
#[doc = "TZC_BMX_S_LOCK (r) register accessor: BMX Slave TZSID Lock\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_bmx_s_lock::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_bmx_s_lock`] module"]
#[doc(alias = "TZC_BMX_S_LOCK")]
pub type TzcBmxSLock = crate::Reg<tzc_bmx_s_lock::TzcBmxSLockSpec>;
#[doc = "BMX Slave TZSID Lock"]
pub mod tzc_bmx_s_lock;
#[doc = "TZC_OCRAM_TZSRG_CTRL (r) register accessor: OCRAM TZSRG Control Register (Region Enable/ID Enable/Lock)\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_ocram_tzsrg_ctrl::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_ocram_tzsrg_ctrl`] module"]
#[doc(alias = "TZC_OCRAM_TZSRG_CTRL")]
pub type TzcOcramTzsrgCtrl = crate::Reg<tzc_ocram_tzsrg_ctrl::TzcOcramTzsrgCtrlSpec>;
#[doc = "OCRAM TZSRG Control Register (Region Enable/ID Enable/Lock)"]
pub mod tzc_ocram_tzsrg_ctrl;
#[doc = "TZC_OCRAM_TZSRG_ADR_MASK (r) register accessor: OCRAM TZSRG Address Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_ocram_tzsrg_adr_mask::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_ocram_tzsrg_adr_mask`] module"]
#[doc(alias = "TZC_OCRAM_TZSRG_ADR_MASK")]
pub type TzcOcramTzsrgAdrMask = crate::Reg<tzc_ocram_tzsrg_adr_mask::TzcOcramTzsrgAdrMaskSpec>;
#[doc = "OCRAM TZSRG Address Mask Register"]
pub mod tzc_ocram_tzsrg_adr_mask;
#[doc = "TZC_OCRAM_TZSRG_R0 (r) register accessor: OCRAM TZSRG Region 0 Start/End\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_ocram_tzsrg_r0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_ocram_tzsrg_r0`] module"]
#[doc(alias = "TZC_OCRAM_TZSRG_R0")]
pub type TzcOcramTzsrgR0 = crate::Reg<tzc_ocram_tzsrg_r0::TzcOcramTzsrgR0Spec>;
#[doc = "OCRAM TZSRG Region 0 Start/End"]
pub mod tzc_ocram_tzsrg_r0;
#[doc = "TZC_OCRAM_TZSRG_R1 (r) register accessor: OCRAM TZSRG Region 1 Start/End\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_ocram_tzsrg_r1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_ocram_tzsrg_r1`] module"]
#[doc(alias = "TZC_OCRAM_TZSRG_R1")]
pub type TzcOcramTzsrgR1 = crate::Reg<tzc_ocram_tzsrg_r1::TzcOcramTzsrgR1Spec>;
#[doc = "OCRAM TZSRG Region 1 Start/End"]
pub mod tzc_ocram_tzsrg_r1;
#[doc = "TZC_OCRAM_TZSRG_R2 (r) register accessor: OCRAM TZSRG Region 2 Start/End\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_ocram_tzsrg_r2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_ocram_tzsrg_r2`] module"]
#[doc(alias = "TZC_OCRAM_TZSRG_R2")]
pub type TzcOcramTzsrgR2 = crate::Reg<tzc_ocram_tzsrg_r2::TzcOcramTzsrgR2Spec>;
#[doc = "OCRAM TZSRG Region 2 Start/End"]
pub mod tzc_ocram_tzsrg_r2;
#[doc = "TZC_OCRAM_TZSRG_R3 (rw) register accessor: OCRAM TZSRG Region 3 (Reserved)\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_ocram_tzsrg_r3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tzc_ocram_tzsrg_r3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_ocram_tzsrg_r3`] module"]
#[doc(alias = "TZC_OCRAM_TZSRG_R3")]
pub type TzcOcramTzsrgR3 = crate::Reg<tzc_ocram_tzsrg_r3::TzcOcramTzsrgR3Spec>;
#[doc = "OCRAM TZSRG Region 3 (Reserved)"]
pub mod tzc_ocram_tzsrg_r3;
#[doc = "TZC_WRAM_TZSRG_CTRL (r) register accessor: WRAM (MCU/Wireless RAM) TZSRG Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_wram_tzsrg_ctrl::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_wram_tzsrg_ctrl`] module"]
#[doc(alias = "TZC_WRAM_TZSRG_CTRL")]
pub type TzcWramTzsrgCtrl = crate::Reg<tzc_wram_tzsrg_ctrl::TzcWramTzsrgCtrlSpec>;
#[doc = "WRAM (MCU/Wireless RAM) TZSRG Control Register"]
pub mod tzc_wram_tzsrg_ctrl;
#[doc = "TZC_WRAM_TZSRG_ADR_MASK (r) register accessor: WRAM TZSRG Address Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_wram_tzsrg_adr_mask::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_wram_tzsrg_adr_mask`] module"]
#[doc(alias = "TZC_WRAM_TZSRG_ADR_MASK")]
pub type TzcWramTzsrgAdrMask = crate::Reg<tzc_wram_tzsrg_adr_mask::TzcWramTzsrgAdrMaskSpec>;
#[doc = "WRAM TZSRG Address Mask Register"]
pub mod tzc_wram_tzsrg_adr_mask;
#[doc = "TZC_WRAM_TZSRG_R0 (r) register accessor: WRAM TZSRG Region 0 Start/End\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_wram_tzsrg_r0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_wram_tzsrg_r0`] module"]
#[doc(alias = "TZC_WRAM_TZSRG_R0")]
pub type TzcWramTzsrgR0 = crate::Reg<tzc_wram_tzsrg_r0::TzcWramTzsrgR0Spec>;
#[doc = "WRAM TZSRG Region 0 Start/End"]
pub mod tzc_wram_tzsrg_r0;
#[doc = "TZC_WRAM_TZSRG_R1 (r) register accessor: WRAM TZSRG Region 1 Start/End\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_wram_tzsrg_r1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_wram_tzsrg_r1`] module"]
#[doc(alias = "TZC_WRAM_TZSRG_R1")]
pub type TzcWramTzsrgR1 = crate::Reg<tzc_wram_tzsrg_r1::TzcWramTzsrgR1Spec>;
#[doc = "WRAM TZSRG Region 1 Start/End"]
pub mod tzc_wram_tzsrg_r1;
#[doc = "TZC_WRAM_TZSRG_R2 (r) register accessor: WRAM TZSRG Region 2 Start/End\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_wram_tzsrg_r2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_wram_tzsrg_r2`] module"]
#[doc(alias = "TZC_WRAM_TZSRG_R2")]
pub type TzcWramTzsrgR2 = crate::Reg<tzc_wram_tzsrg_r2::TzcWramTzsrgR2Spec>;
#[doc = "WRAM TZSRG Region 2 Start/End"]
pub mod tzc_wram_tzsrg_r2;
#[doc = "TZC_WRAM_TZSRG_R3 (rw) register accessor: WRAM TZSRG Region 3 (Reserved)\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_wram_tzsrg_r3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tzc_wram_tzsrg_r3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_wram_tzsrg_r3`] module"]
#[doc(alias = "TZC_WRAM_TZSRG_R3")]
pub type TzcWramTzsrgR3 = crate::Reg<tzc_wram_tzsrg_r3::TzcWramTzsrgR3Spec>;
#[doc = "WRAM TZSRG Region 3 (Reserved)"]
pub mod tzc_wram_tzsrg_r3;
#[doc = "TZC_WIFI_DBG (r) register accessor: WIFI Debug Control\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_wifi_dbg::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_wifi_dbg`] module"]
#[doc(alias = "TZC_WIFI_DBG")]
pub type TzcWifiDbg = crate::Reg<tzc_wifi_dbg::TzcWifiDbgSpec>;
#[doc = "WIFI Debug Control"]
pub mod tzc_wifi_dbg;
#[doc = "TZC_SF_TZSRG_CTRL (r) register accessor: SF TZSRG Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_sf_tzsrg_ctrl::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_sf_tzsrg_ctrl`] module"]
#[doc(alias = "TZC_SF_TZSRG_CTRL")]
pub type TzcSfTzsrgCtrl = crate::Reg<tzc_sf_tzsrg_ctrl::TzcSfTzsrgCtrlSpec>;
#[doc = "SF TZSRG Control Register"]
pub mod tzc_sf_tzsrg_ctrl;
#[doc = "TZC_SF_TZSRG_ADR_MASK (r) register accessor: SF TZSRG Address Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_sf_tzsrg_adr_mask::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_sf_tzsrg_adr_mask`] module"]
#[doc(alias = "TZC_SF_TZSRG_ADR_MASK")]
pub type TzcSfTzsrgAdrMask = crate::Reg<tzc_sf_tzsrg_adr_mask::TzcSfTzsrgAdrMaskSpec>;
#[doc = "SF TZSRG Address Mask Register"]
pub mod tzc_sf_tzsrg_adr_mask;
#[doc = "TZC_SF_TZSRG_R0 (r) register accessor: SF TZSRG Region 0 Start/End (Bits \\[18:0\\])\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_sf_tzsrg_r0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_sf_tzsrg_r0`] module"]
#[doc(alias = "TZC_SF_TZSRG_R0")]
pub type TzcSfTzsrgR0 = crate::Reg<tzc_sf_tzsrg_r0::TzcSfTzsrgR0Spec>;
#[doc = "SF TZSRG Region 0 Start/End (Bits \\[18:0\\])"]
pub mod tzc_sf_tzsrg_r0;
#[doc = "TZC_SF_TZSRG_R1 (r) register accessor: SF TZSRG Region 1 Start/End (Bits \\[18:0\\])\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_sf_tzsrg_r1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_sf_tzsrg_r1`] module"]
#[doc(alias = "TZC_SF_TZSRG_R1")]
pub type TzcSfTzsrgR1 = crate::Reg<tzc_sf_tzsrg_r1::TzcSfTzsrgR1Spec>;
#[doc = "SF TZSRG Region 1 Start/End (Bits \\[18:0\\])"]
pub mod tzc_sf_tzsrg_r1;
#[doc = "TZC_SF_TZSRG_R2 (r) register accessor: SF TZSRG Region 2 Start/End (Bits \\[18:0\\])\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_sf_tzsrg_r2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_sf_tzsrg_r2`] module"]
#[doc(alias = "TZC_SF_TZSRG_R2")]
pub type TzcSfTzsrgR2 = crate::Reg<tzc_sf_tzsrg_r2::TzcSfTzsrgR2Spec>;
#[doc = "SF TZSRG Region 2 Start/End (Bits \\[18:0\\])"]
pub mod tzc_sf_tzsrg_r2;
#[doc = "TZC_SF_TZSRG_R3 (r) register accessor: SF TZSRG Region 3 Start/End (Bits \\[18:0\\])\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_sf_tzsrg_r3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_sf_tzsrg_r3`] module"]
#[doc(alias = "TZC_SF_TZSRG_R3")]
pub type TzcSfTzsrgR3 = crate::Reg<tzc_sf_tzsrg_r3::TzcSfTzsrgR3Spec>;
#[doc = "SF TZSRG Region 3 Start/End (Bits \\[18:0\\])"]
pub mod tzc_sf_tzsrg_r3;
#[doc = "TZC_SF_TZSRG_MSB (r) register accessor: SF TZSRG Address MSB Configuration (Bits \\[21:19\\] for 64M Flash)\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_sf_tzsrg_msb::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_sf_tzsrg_msb`] module"]
#[doc(alias = "TZC_SF_TZSRG_MSB")]
pub type TzcSfTzsrgMsb = crate::Reg<tzc_sf_tzsrg_msb::TzcSfTzsrgMsbSpec>;
#[doc = "SF TZSRG Address MSB Configuration (Bits \\[21:19\\] for 64M Flash)"]
pub mod tzc_sf_tzsrg_msb;
#[doc = "TZC_MM_BMX_TZMID (r) register accessor: MM BMX Bus Master ID Security State\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_mm_bmx_tzmid::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_mm_bmx_tzmid`] module"]
#[doc(alias = "TZC_MM_BMX_TZMID")]
pub type TzcMmBmxTzmid = crate::Reg<tzc_mm_bmx_tzmid::TzcMmBmxTzmidSpec>;
#[doc = "MM BMX Bus Master ID Security State"]
pub mod tzc_mm_bmx_tzmid;
#[doc = "TZC_MM_BMX_TZMID_LOCK (r) register accessor: MM BMX Bus Master ID Security State Lock\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_mm_bmx_tzmid_lock::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_mm_bmx_tzmid_lock`] module"]
#[doc(alias = "TZC_MM_BMX_TZMID_LOCK")]
pub type TzcMmBmxTzmidLock = crate::Reg<tzc_mm_bmx_tzmid_lock::TzcMmBmxTzmidLockSpec>;
#[doc = "MM BMX Bus Master ID Security State Lock"]
pub mod tzc_mm_bmx_tzmid_lock;
#[doc = "TZC_MM_BMX_S0 (r) register accessor: MM BMX Slave 0 Security ID Enable (MM Peripherals)\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_mm_bmx_s0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_mm_bmx_s0`] module"]
#[doc(alias = "TZC_MM_BMX_S0")]
pub type TzcMmBmxS0 = crate::Reg<tzc_mm_bmx_s0::TzcMmBmxS0Spec>;
#[doc = "MM BMX Slave 0 Security ID Enable (MM Peripherals)"]
pub mod tzc_mm_bmx_s0;
#[doc = "TZC_MM_BMX_S1 (r) register accessor: MM BMX Slave 1 Security ID Enable (CODEC Subsystem)\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_mm_bmx_s1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_mm_bmx_s1`] module"]
#[doc(alias = "TZC_MM_BMX_S1")]
pub type TzcMmBmxS1 = crate::Reg<tzc_mm_bmx_s1::TzcMmBmxS1Spec>;
#[doc = "MM BMX Slave 1 Security ID Enable (CODEC Subsystem)"]
pub mod tzc_mm_bmx_s1;
#[doc = "TZC_MM_BMX_S2 (r) register accessor: MM BMX Slave 2 Security ID Enable (CODEC Subsystem Slaves)\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_mm_bmx_s2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_mm_bmx_s2`] module"]
#[doc(alias = "TZC_MM_BMX_S2")]
pub type TzcMmBmxS2 = crate::Reg<tzc_mm_bmx_s2::TzcMmBmxS2Spec>;
#[doc = "MM BMX Slave 2 Security ID Enable (CODEC Subsystem Slaves)"]
pub mod tzc_mm_bmx_s2;
#[doc = "TZC_MM_BMX_S_LOCK0 (r) register accessor: MM BMX Slave Lock 0\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_mm_bmx_s_lock0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_mm_bmx_s_lock0`] module"]
#[doc(alias = "TZC_MM_BMX_S_LOCK0")]
pub type TzcMmBmxSLock0 = crate::Reg<tzc_mm_bmx_s_lock0::TzcMmBmxSLock0Spec>;
#[doc = "MM BMX Slave Lock 0"]
pub mod tzc_mm_bmx_s_lock0;
#[doc = "TZC_MM_BMX_S_LOCK1 (r) register accessor: MM BMX Slave Lock 1\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_mm_bmx_s_lock1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_mm_bmx_s_lock1`] module"]
#[doc(alias = "TZC_MM_BMX_S_LOCK1")]
pub type TzcMmBmxSLock1 = crate::Reg<tzc_mm_bmx_s_lock1::TzcMmBmxSLock1Spec>;
#[doc = "MM BMX Slave Lock 1"]
pub mod tzc_mm_bmx_s_lock1;
#[doc = "TZC_L2SRAM_TZSRG_CTRL (r) register accessor: L2SRAM TZSRG Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_l2sram_tzsrg_ctrl::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_l2sram_tzsrg_ctrl`] module"]
#[doc(alias = "TZC_L2SRAM_TZSRG_CTRL")]
pub type TzcL2sramTzsrgCtrl = crate::Reg<tzc_l2sram_tzsrg_ctrl::TzcL2sramTzsrgCtrlSpec>;
#[doc = "L2SRAM TZSRG Control Register"]
pub mod tzc_l2sram_tzsrg_ctrl;
#[doc = "TZC_L2SRAM_TZSRG_ADR_MASK (r) register accessor: L2SRAM TZSRG Address Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_l2sram_tzsrg_adr_mask::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_l2sram_tzsrg_adr_mask`] module"]
#[doc(alias = "TZC_L2SRAM_TZSRG_ADR_MASK")]
pub type TzcL2sramTzsrgAdrMask = crate::Reg<tzc_l2sram_tzsrg_adr_mask::TzcL2sramTzsrgAdrMaskSpec>;
#[doc = "L2SRAM TZSRG Address Mask Register"]
pub mod tzc_l2sram_tzsrg_adr_mask;
#[doc = "TZC_L2SRAM_TZSRG_R0 (r) register accessor: L2SRAM TZSRG Region 0 Start/End\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_l2sram_tzsrg_r0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_l2sram_tzsrg_r0`] module"]
#[doc(alias = "TZC_L2SRAM_TZSRG_R0")]
pub type TzcL2sramTzsrgR0 = crate::Reg<tzc_l2sram_tzsrg_r0::TzcL2sramTzsrgR0Spec>;
#[doc = "L2SRAM TZSRG Region 0 Start/End"]
pub mod tzc_l2sram_tzsrg_r0;
#[doc = "TZC_L2SRAM_TZSRG_R1 (r) register accessor: L2SRAM TZSRG Region 1 Start/End\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_l2sram_tzsrg_r1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_l2sram_tzsrg_r1`] module"]
#[doc(alias = "TZC_L2SRAM_TZSRG_R1")]
pub type TzcL2sramTzsrgR1 = crate::Reg<tzc_l2sram_tzsrg_r1::TzcL2sramTzsrgR1Spec>;
#[doc = "L2SRAM TZSRG Region 1 Start/End"]
pub mod tzc_l2sram_tzsrg_r1;
#[doc = "TZC_L2SRAM_TZSRG_R2 (r) register accessor: L2SRAM TZSRG Region 2 Start/End\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_l2sram_tzsrg_r2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_l2sram_tzsrg_r2`] module"]
#[doc(alias = "TZC_L2SRAM_TZSRG_R2")]
pub type TzcL2sramTzsrgR2 = crate::Reg<tzc_l2sram_tzsrg_r2::TzcL2sramTzsrgR2Spec>;
#[doc = "L2SRAM TZSRG Region 2 Start/End"]
pub mod tzc_l2sram_tzsrg_r2;
#[doc = "TZC_L2SRAM_TZSRG_R3 (rw) register accessor: L2SRAM TZSRG Region 3 (Reserved)\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_l2sram_tzsrg_r3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tzc_l2sram_tzsrg_r3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_l2sram_tzsrg_r3`] module"]
#[doc(alias = "TZC_L2SRAM_TZSRG_R3")]
pub type TzcL2sramTzsrgR3 = crate::Reg<tzc_l2sram_tzsrg_r3::TzcL2sramTzsrgR3Spec>;
#[doc = "L2SRAM TZSRG Region 3 (Reserved)"]
pub mod tzc_l2sram_tzsrg_r3;
#[doc = "TZC_VRAM_TZSRG_CTRL (r) register accessor: VRAM TZSRG Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_vram_tzsrg_ctrl::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_vram_tzsrg_ctrl`] module"]
#[doc(alias = "TZC_VRAM_TZSRG_CTRL")]
pub type TzcVramTzsrgCtrl = crate::Reg<tzc_vram_tzsrg_ctrl::TzcVramTzsrgCtrlSpec>;
#[doc = "VRAM TZSRG Control Register"]
pub mod tzc_vram_tzsrg_ctrl;
#[doc = "TZC_VRAM_TZSRG_ADR_MASK (r) register accessor: VRAM TZSRG Address Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_vram_tzsrg_adr_mask::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_vram_tzsrg_adr_mask`] module"]
#[doc(alias = "TZC_VRAM_TZSRG_ADR_MASK")]
pub type TzcVramTzsrgAdrMask = crate::Reg<tzc_vram_tzsrg_adr_mask::TzcVramTzsrgAdrMaskSpec>;
#[doc = "VRAM TZSRG Address Mask Register"]
pub mod tzc_vram_tzsrg_adr_mask;
#[doc = "TZC_VRAM_TZSRG_R0 (r) register accessor: VRAM TZSRG Region 0 Start/End\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_vram_tzsrg_r0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_vram_tzsrg_r0`] module"]
#[doc(alias = "TZC_VRAM_TZSRG_R0")]
pub type TzcVramTzsrgR0 = crate::Reg<tzc_vram_tzsrg_r0::TzcVramTzsrgR0Spec>;
#[doc = "VRAM TZSRG Region 0 Start/End"]
pub mod tzc_vram_tzsrg_r0;
#[doc = "TZC_VRAM_TZSRG_R1 (r) register accessor: VRAM TZSRG Region 1 Start/End\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_vram_tzsrg_r1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_vram_tzsrg_r1`] module"]
#[doc(alias = "TZC_VRAM_TZSRG_R1")]
pub type TzcVramTzsrgR1 = crate::Reg<tzc_vram_tzsrg_r1::TzcVramTzsrgR1Spec>;
#[doc = "VRAM TZSRG Region 1 Start/End"]
pub mod tzc_vram_tzsrg_r1;
#[doc = "TZC_VRAM_TZSRG_R2 (r) register accessor: VRAM TZSRG Region 2 Start/End\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_vram_tzsrg_r2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_vram_tzsrg_r2`] module"]
#[doc(alias = "TZC_VRAM_TZSRG_R2")]
pub type TzcVramTzsrgR2 = crate::Reg<tzc_vram_tzsrg_r2::TzcVramTzsrgR2Spec>;
#[doc = "VRAM TZSRG Region 2 Start/End"]
pub mod tzc_vram_tzsrg_r2;
#[doc = "TZC_VRAM_TZSRG_R3 (rw) register accessor: VRAM TZSRG Region 3 (Reserved)\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_vram_tzsrg_r3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tzc_vram_tzsrg_r3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_vram_tzsrg_r3`] module"]
#[doc(alias = "TZC_VRAM_TZSRG_R3")]
pub type TzcVramTzsrgR3 = crate::Reg<tzc_vram_tzsrg_r3::TzcVramTzsrgR3Spec>;
#[doc = "VRAM TZSRG Region 3 (Reserved)"]
pub mod tzc_vram_tzsrg_r3;
#[doc = "TZC_PSRAMA_TZSRG_CTRL (r) register accessor: PSRAM-A TZSRG Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_psrama_tzsrg_ctrl::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_psrama_tzsrg_ctrl`] module"]
#[doc(alias = "TZC_PSRAMA_TZSRG_CTRL")]
pub type TzcPsramaTzsrgCtrl = crate::Reg<tzc_psrama_tzsrg_ctrl::TzcPsramaTzsrgCtrlSpec>;
#[doc = "PSRAM-A TZSRG Control Register"]
pub mod tzc_psrama_tzsrg_ctrl;
#[doc = "TZC_PSRAMA_TZSRG_ADR_MASK (r) register accessor: PSRAM-A TZSRG Address Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_psrama_tzsrg_adr_mask::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_psrama_tzsrg_adr_mask`] module"]
#[doc(alias = "TZC_PSRAMA_TZSRG_ADR_MASK")]
pub type TzcPsramaTzsrgAdrMask = crate::Reg<tzc_psrama_tzsrg_adr_mask::TzcPsramaTzsrgAdrMaskSpec>;
#[doc = "PSRAM-A TZSRG Address Mask Register"]
pub mod tzc_psrama_tzsrg_adr_mask;
#[doc = "TZC_PSRAMA_TZSRG_R0 (r) register accessor: PSRAM-A TZSRG Region 0 Start/End\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_psrama_tzsrg_r0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_psrama_tzsrg_r0`] module"]
#[doc(alias = "TZC_PSRAMA_TZSRG_R0")]
pub type TzcPsramaTzsrgR0 = crate::Reg<tzc_psrama_tzsrg_r0::TzcPsramaTzsrgR0Spec>;
#[doc = "PSRAM-A TZSRG Region 0 Start/End"]
pub mod tzc_psrama_tzsrg_r0;
#[doc = "TZC_PSRAMA_TZSRG_R1 (r) register accessor: PSRAM-A TZSRG Region 1 Start/End\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_psrama_tzsrg_r1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_psrama_tzsrg_r1`] module"]
#[doc(alias = "TZC_PSRAMA_TZSRG_R1")]
pub type TzcPsramaTzsrgR1 = crate::Reg<tzc_psrama_tzsrg_r1::TzcPsramaTzsrgR1Spec>;
#[doc = "PSRAM-A TZSRG Region 1 Start/End"]
pub mod tzc_psrama_tzsrg_r1;
#[doc = "TZC_PSRAMA_TZSRG_R2 (r) register accessor: PSRAM-A TZSRG Region 2 Start/End\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_psrama_tzsrg_r2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_psrama_tzsrg_r2`] module"]
#[doc(alias = "TZC_PSRAMA_TZSRG_R2")]
pub type TzcPsramaTzsrgR2 = crate::Reg<tzc_psrama_tzsrg_r2::TzcPsramaTzsrgR2Spec>;
#[doc = "PSRAM-A TZSRG Region 2 Start/End"]
pub mod tzc_psrama_tzsrg_r2;
#[doc = "TZC_PSRAMA_TZSRG_R3 (rw) register accessor: PSRAM-A TZSRG Region 3 (Reserved)\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_psrama_tzsrg_r3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tzc_psrama_tzsrg_r3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_psrama_tzsrg_r3`] module"]
#[doc(alias = "TZC_PSRAMA_TZSRG_R3")]
pub type TzcPsramaTzsrgR3 = crate::Reg<tzc_psrama_tzsrg_r3::TzcPsramaTzsrgR3Spec>;
#[doc = "PSRAM-A TZSRG Region 3 (Reserved)"]
pub mod tzc_psrama_tzsrg_r3;
#[doc = "TZC_PSRAMB_TZSRG_CTRL (r) register accessor: PSRAM-B TZSRG Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_psramb_tzsrg_ctrl::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_psramb_tzsrg_ctrl`] module"]
#[doc(alias = "TZC_PSRAMB_TZSRG_CTRL")]
pub type TzcPsrambTzsrgCtrl = crate::Reg<tzc_psramb_tzsrg_ctrl::TzcPsrambTzsrgCtrlSpec>;
#[doc = "PSRAM-B TZSRG Control Register"]
pub mod tzc_psramb_tzsrg_ctrl;
#[doc = "TZC_PSRAMB_TZSRG_ADR_MASK (r) register accessor: PSRAM-B TZSRG Address Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_psramb_tzsrg_adr_mask::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_psramb_tzsrg_adr_mask`] module"]
#[doc(alias = "TZC_PSRAMB_TZSRG_ADR_MASK")]
pub type TzcPsrambTzsrgAdrMask = crate::Reg<tzc_psramb_tzsrg_adr_mask::TzcPsrambTzsrgAdrMaskSpec>;
#[doc = "PSRAM-B TZSRG Address Mask Register"]
pub mod tzc_psramb_tzsrg_adr_mask;
#[doc = "TZC_PSRAMB_TZSRG_R0 (r) register accessor: PSRAM-B TZSRG Region 0 Start/End\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_psramb_tzsrg_r0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_psramb_tzsrg_r0`] module"]
#[doc(alias = "TZC_PSRAMB_TZSRG_R0")]
pub type TzcPsrambTzsrgR0 = crate::Reg<tzc_psramb_tzsrg_r0::TzcPsrambTzsrgR0Spec>;
#[doc = "PSRAM-B TZSRG Region 0 Start/End"]
pub mod tzc_psramb_tzsrg_r0;
#[doc = "TZC_PSRAMB_TZSRG_R1 (r) register accessor: PSRAM-B TZSRG Region 1 Start/End\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_psramb_tzsrg_r1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_psramb_tzsrg_r1`] module"]
#[doc(alias = "TZC_PSRAMB_TZSRG_R1")]
pub type TzcPsrambTzsrgR1 = crate::Reg<tzc_psramb_tzsrg_r1::TzcPsrambTzsrgR1Spec>;
#[doc = "PSRAM-B TZSRG Region 1 Start/End"]
pub mod tzc_psramb_tzsrg_r1;
#[doc = "TZC_PSRAMB_TZSRG_R2 (r) register accessor: PSRAM-B TZSRG Region 2 Start/End\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_psramb_tzsrg_r2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_psramb_tzsrg_r2`] module"]
#[doc(alias = "TZC_PSRAMB_TZSRG_R2")]
pub type TzcPsrambTzsrgR2 = crate::Reg<tzc_psramb_tzsrg_r2::TzcPsrambTzsrgR2Spec>;
#[doc = "PSRAM-B TZSRG Region 2 Start/End"]
pub mod tzc_psramb_tzsrg_r2;
#[doc = "TZC_PSRAMB_TZSRG_R3 (rw) register accessor: PSRAM-B TZSRG Region 3 (Reserved)\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_psramb_tzsrg_r3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tzc_psramb_tzsrg_r3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_psramb_tzsrg_r3`] module"]
#[doc(alias = "TZC_PSRAMB_TZSRG_R3")]
pub type TzcPsrambTzsrgR3 = crate::Reg<tzc_psramb_tzsrg_r3::TzcPsrambTzsrgR3Spec>;
#[doc = "PSRAM-B TZSRG Region 3 (Reserved)"]
pub mod tzc_psramb_tzsrg_r3;
#[doc = "TZC_XRAM_TZSRG_CTRL (r) register accessor: XRAM TZSRG Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_xram_tzsrg_ctrl::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_xram_tzsrg_ctrl`] module"]
#[doc(alias = "TZC_XRAM_TZSRG_CTRL")]
pub type TzcXramTzsrgCtrl = crate::Reg<tzc_xram_tzsrg_ctrl::TzcXramTzsrgCtrlSpec>;
#[doc = "XRAM TZSRG Control Register"]
pub mod tzc_xram_tzsrg_ctrl;
#[doc = "TZC_XRAM_TZSRG_ADR_MASK (r) register accessor: XRAM TZSRG Address Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_xram_tzsrg_adr_mask::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_xram_tzsrg_adr_mask`] module"]
#[doc(alias = "TZC_XRAM_TZSRG_ADR_MASK")]
pub type TzcXramTzsrgAdrMask = crate::Reg<tzc_xram_tzsrg_adr_mask::TzcXramTzsrgAdrMaskSpec>;
#[doc = "XRAM TZSRG Address Mask Register"]
pub mod tzc_xram_tzsrg_adr_mask;
#[doc = "TZC_XRAM_TZSRG_R0 (r) register accessor: XRAM TZSRG Region 0 Start/End\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_xram_tzsrg_r0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_xram_tzsrg_r0`] module"]
#[doc(alias = "TZC_XRAM_TZSRG_R0")]
pub type TzcXramTzsrgR0 = crate::Reg<tzc_xram_tzsrg_r0::TzcXramTzsrgR0Spec>;
#[doc = "XRAM TZSRG Region 0 Start/End"]
pub mod tzc_xram_tzsrg_r0;
#[doc = "TZC_XRAM_TZSRG_R1 (r) register accessor: XRAM TZSRG Region 1 Start/End\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_xram_tzsrg_r1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_xram_tzsrg_r1`] module"]
#[doc(alias = "TZC_XRAM_TZSRG_R1")]
pub type TzcXramTzsrgR1 = crate::Reg<tzc_xram_tzsrg_r1::TzcXramTzsrgR1Spec>;
#[doc = "XRAM TZSRG Region 1 Start/End"]
pub mod tzc_xram_tzsrg_r1;
#[doc = "TZC_XRAM_TZSRG_R2 (r) register accessor: XRAM TZSRG Region 2 Start/End\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_xram_tzsrg_r2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_xram_tzsrg_r2`] module"]
#[doc(alias = "TZC_XRAM_TZSRG_R2")]
pub type TzcXramTzsrgR2 = crate::Reg<tzc_xram_tzsrg_r2::TzcXramTzsrgR2Spec>;
#[doc = "XRAM TZSRG Region 2 Start/End"]
pub mod tzc_xram_tzsrg_r2;
#[doc = "TZC_XRAM_TZSRG_R3 (rw) register accessor: XRAM TZSRG Region 3 (Reserved)\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_xram_tzsrg_r3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tzc_xram_tzsrg_r3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_xram_tzsrg_r3`] module"]
#[doc(alias = "TZC_XRAM_TZSRG_R3")]
pub type TzcXramTzsrgR3 = crate::Reg<tzc_xram_tzsrg_r3::TzcXramTzsrgR3Spec>;
#[doc = "XRAM TZSRG Region 3 (Reserved)"]
pub mod tzc_xram_tzsrg_r3;
#[doc = "TZC_GLB_CTRL_0 (r) register accessor: Global Control 0 (System Reset TZID Enable)\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_glb_ctrl_0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_glb_ctrl_0`] module"]
#[doc(alias = "TZC_GLB_CTRL_0")]
pub type TzcGlbCtrl0 = crate::Reg<tzc_glb_ctrl_0::TzcGlbCtrl0Spec>;
#[doc = "Global Control 0 (System Reset TZID Enable)"]
pub mod tzc_glb_ctrl_0;
#[doc = "TZC_GLB_CTRL_1 (rw) register accessor: Global Control 1 (Reserved)\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_glb_ctrl_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tzc_glb_ctrl_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_glb_ctrl_1`] module"]
#[doc(alias = "TZC_GLB_CTRL_1")]
pub type TzcGlbCtrl1 = crate::Reg<tzc_glb_ctrl_1::TzcGlbCtrl1Spec>;
#[doc = "Global Control 1 (Reserved)"]
pub mod tzc_glb_ctrl_1;
#[doc = "TZC_GLB_CTRL_2 (r) register accessor: Global Control 2 (System Reset TZID Lock)\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_glb_ctrl_2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_glb_ctrl_2`] module"]
#[doc(alias = "TZC_GLB_CTRL_2")]
pub type TzcGlbCtrl2 = crate::Reg<tzc_glb_ctrl_2::TzcGlbCtrl2Spec>;
#[doc = "Global Control 2 (System Reset TZID Lock)"]
pub mod tzc_glb_ctrl_2;
#[doc = "TZC_MM_CTRL_0 (r) register accessor: MM Control 0 (MM System Reset TZID Enable)\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_mm_ctrl_0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_mm_ctrl_0`] module"]
#[doc(alias = "TZC_MM_CTRL_0")]
pub type TzcMmCtrl0 = crate::Reg<tzc_mm_ctrl_0::TzcMmCtrl0Spec>;
#[doc = "MM Control 0 (MM System Reset TZID Enable)"]
pub mod tzc_mm_ctrl_0;
#[doc = "TZC_MM_CTRL_1 (rw) register accessor: MM Control 1 (Reserved)\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_mm_ctrl_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tzc_mm_ctrl_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_mm_ctrl_1`] module"]
#[doc(alias = "TZC_MM_CTRL_1")]
pub type TzcMmCtrl1 = crate::Reg<tzc_mm_ctrl_1::TzcMmCtrl1Spec>;
#[doc = "MM Control 1 (Reserved)"]
pub mod tzc_mm_ctrl_1;
#[doc = "TZC_MM_CTRL_2 (r) register accessor: MM Control 2 (MM System Reset TZID Lock)\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_mm_ctrl_2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_mm_ctrl_2`] module"]
#[doc(alias = "TZC_MM_CTRL_2")]
pub type TzcMmCtrl2 = crate::Reg<tzc_mm_ctrl_2::TzcMmCtrl2Spec>;
#[doc = "MM Control 2 (MM System Reset TZID Lock)"]
pub mod tzc_mm_ctrl_2;
#[doc = "TZC_SE_CTRL_0 (r) register accessor: Security Engine Control 0 (TZID Enable)\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_se_ctrl_0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_se_ctrl_0`] module"]
#[doc(alias = "TZC_SE_CTRL_0")]
pub type TzcSeCtrl0 = crate::Reg<tzc_se_ctrl_0::TzcSeCtrl0Spec>;
#[doc = "Security Engine Control 0 (TZID Enable)"]
pub mod tzc_se_ctrl_0;
#[doc = "TZC_SE_CTRL_1 (r) register accessor: Security Engine Control 1 (Flash/SF Controls)\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_se_ctrl_1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_se_ctrl_1`] module"]
#[doc(alias = "TZC_SE_CTRL_1")]
pub type TzcSeCtrl1 = crate::Reg<tzc_se_ctrl_1::TzcSeCtrl1Spec>;
#[doc = "Security Engine Control 1 (Flash/SF Controls)"]
pub mod tzc_se_ctrl_1;
#[doc = "TZC_SE_CTRL_2 (r) register accessor: Security Engine Control 2 (TZID Lock)\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_se_ctrl_2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_se_ctrl_2`] module"]
#[doc(alias = "TZC_SE_CTRL_2")]
pub type TzcSeCtrl2 = crate::Reg<tzc_se_ctrl_2::TzcSeCtrl2Spec>;
#[doc = "Security Engine Control 2 (TZID Lock)"]
pub mod tzc_se_ctrl_2;
#[doc = "TZC_PDM_CTRL (rw) register accessor: PDM Control (Reserved/Placeholder for TZSID)\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_pdm_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tzc_pdm_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_pdm_ctrl`] module"]
#[doc(alias = "TZC_PDM_CTRL")]
pub type TzcPdmCtrl = crate::Reg<tzc_pdm_ctrl::TzcPdmCtrlSpec>;
#[doc = "PDM Control (Reserved/Placeholder for TZSID)"]
pub mod tzc_pdm_ctrl;
#[doc = "TZC_UART_CTRL (rw) register accessor: UART Control (Reserved/Placeholder for TZSID)\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_uart_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tzc_uart_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_uart_ctrl`] module"]
#[doc(alias = "TZC_UART_CTRL")]
pub type TzcUartCtrl = crate::Reg<tzc_uart_ctrl::TzcUartCtrlSpec>;
#[doc = "UART Control (Reserved/Placeholder for TZSID)"]
pub mod tzc_uart_ctrl;
#[doc = "TZC_I2C_CTRL (rw) register accessor: I2C Control (Reserved/Placeholder for TZSID)\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_i2c_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tzc_i2c_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_i2c_ctrl`] module"]
#[doc(alias = "TZC_I2C_CTRL")]
pub type TzcI2cCtrl = crate::Reg<tzc_i2c_ctrl::TzcI2cCtrlSpec>;
#[doc = "I2C Control (Reserved/Placeholder for TZSID)"]
pub mod tzc_i2c_ctrl;
#[doc = "TZC_TIMER_CTRL (rw) register accessor: TIMER Control (Reserved/Placeholder for TZSID)\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_timer_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tzc_timer_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_timer_ctrl`] module"]
#[doc(alias = "TZC_TIMER_CTRL")]
pub type TzcTimerCtrl = crate::Reg<tzc_timer_ctrl::TzcTimerCtrlSpec>;
#[doc = "TIMER Control (Reserved/Placeholder for TZSID)"]
pub mod tzc_timer_ctrl;
#[doc = "TZC_I2S_CTRL (rw) register accessor: I2S Control (Reserved/Placeholder for TZSID)\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_i2s_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tzc_i2s_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_i2s_ctrl`] module"]
#[doc(alias = "TZC_I2S_CTRL")]
pub type TzcI2sCtrl = crate::Reg<tzc_i2s_ctrl::TzcI2sCtrlSpec>;
#[doc = "I2S Control (Reserved/Placeholder for TZSID)"]
pub mod tzc_i2s_ctrl;
