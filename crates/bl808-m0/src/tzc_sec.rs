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
    #[doc = "0x40 - ROM Security Region Generator Control"]
    #[inline(always)]
    pub const fn tzc_rom_tzsrg_ctrl(&self) -> &TzcRomTzsrgCtrl {
        &self.tzc_rom_tzsrg_ctrl
    }
    #[doc = "0x44 - ROM Security Region Address Mask"]
    #[inline(always)]
    pub const fn tzc_rom_tzsrg_adr_mask(&self) -> &TzcRomTzsrgAdrMask {
        &self.tzc_rom_tzsrg_adr_mask
    }
    #[doc = "0x48 - ROM Security Region 0 Start/End Address"]
    #[inline(always)]
    pub const fn tzc_rom_tzsrg_r0(&self) -> &TzcRomTzsrgR0 {
        &self.tzc_rom_tzsrg_r0
    }
    #[doc = "0x4c - ROM Security Region 1 Start/End Address"]
    #[inline(always)]
    pub const fn tzc_rom_tzsrg_r1(&self) -> &TzcRomTzsrgR1 {
        &self.tzc_rom_tzsrg_r1
    }
    #[doc = "0x50 - ROM Security Region 2 Start/End Address"]
    #[inline(always)]
    pub const fn tzc_rom_tzsrg_r2(&self) -> &TzcRomTzsrgR2 {
        &self.tzc_rom_tzsrg_r2
    }
    #[doc = "0x54 - ROM Security Region 3 (Reserved/Dummy Register)"]
    #[inline(always)]
    pub const fn tzc_rom_tzsrg_r3(&self) -> &TzcRomTzsrgR3 {
        &self.tzc_rom_tzsrg_r3
    }
    #[doc = "0x100 - Bus Matrix Master TZID Assignment"]
    #[inline(always)]
    pub const fn tzc_bmx_tzmid(&self) -> &TzcBmxTzmid {
        &self.tzc_bmx_tzmid
    }
    #[doc = "0x104 - Bus Matrix Master TZID Lock (Write-Once)"]
    #[inline(always)]
    pub const fn tzc_bmx_tzmid_lock(&self) -> &TzcBmxTzmidLock {
        &self.tzc_bmx_tzmid_lock
    }
    #[doc = "0x108 - Bus Matrix Slave 0 Access Control Enable (MM, DMA, PWR, SDH, EMAC)"]
    #[inline(always)]
    pub const fn tzc_bmx_s0(&self) -> &TzcBmxS0 {
        &self.tzc_bmx_s0
    }
    #[doc = "0x10c - Bus Matrix Slave 1 Access Control Enable (Peripherals: UART, SPI, I2C, etc.)"]
    #[inline(always)]
    pub const fn tzc_bmx_s1(&self) -> &TzcBmxS1 {
        &self.tzc_bmx_s1
    }
    #[doc = "0x110 - Bus Matrix Slave 2 Access Control Enable (Ext/Wireless)"]
    #[inline(always)]
    pub const fn tzc_bmx_s2(&self) -> &TzcBmxS2 {
        &self.tzc_bmx_s2
    }
    #[doc = "0x114 - Bus Matrix Slave Access Control Lock (Write-Once)"]
    #[inline(always)]
    pub const fn tzc_bmx_s_lock(&self) -> &TzcBmxSLock {
        &self.tzc_bmx_s_lock
    }
    #[doc = "0x140 - OCRAM Security Region Generator Control"]
    #[inline(always)]
    pub const fn tzc_ocram_tzsrg_ctrl(&self) -> &TzcOcramTzsrgCtrl {
        &self.tzc_ocram_tzsrg_ctrl
    }
    #[doc = "0x144 - OCRAM Security Region Address Mask"]
    #[inline(always)]
    pub const fn tzc_ocram_tzsrg_adr_mask(&self) -> &TzcOcramTzsrgAdrMask {
        &self.tzc_ocram_tzsrg_adr_mask
    }
    #[doc = "0x148 - OCRAM Security Region 0 Start/End Address"]
    #[inline(always)]
    pub const fn tzc_ocram_tzsrg_r0(&self) -> &TzcOcramTzsrgR0 {
        &self.tzc_ocram_tzsrg_r0
    }
    #[doc = "0x14c - OCRAM Security Region 1 Start/End Address"]
    #[inline(always)]
    pub const fn tzc_ocram_tzsrg_r1(&self) -> &TzcOcramTzsrgR1 {
        &self.tzc_ocram_tzsrg_r1
    }
    #[doc = "0x150 - OCRAM Security Region 2 Start/End Address"]
    #[inline(always)]
    pub const fn tzc_ocram_tzsrg_r2(&self) -> &TzcOcramTzsrgR2 {
        &self.tzc_ocram_tzsrg_r2
    }
    #[doc = "0x154 - OCRAM Security Region 3 (Reserved/Dummy Register)"]
    #[inline(always)]
    pub const fn tzc_ocram_tzsrg_r3(&self) -> &TzcOcramTzsrgR3 {
        &self.tzc_ocram_tzsrg_r3
    }
    #[doc = "0x180 - WRAM Security Region Generator Control"]
    #[inline(always)]
    pub const fn tzc_wram_tzsrg_ctrl(&self) -> &TzcWramTzsrgCtrl {
        &self.tzc_wram_tzsrg_ctrl
    }
    #[doc = "0x184 - WRAM Security Region Address Mask"]
    #[inline(always)]
    pub const fn tzc_wram_tzsrg_adr_mask(&self) -> &TzcWramTzsrgAdrMask {
        &self.tzc_wram_tzsrg_adr_mask
    }
    #[doc = "0x188 - WRAM Security Region 0 Start/End Address"]
    #[inline(always)]
    pub const fn tzc_wram_tzsrg_r0(&self) -> &TzcWramTzsrgR0 {
        &self.tzc_wram_tzsrg_r0
    }
    #[doc = "0x18c - WRAM Security Region 1 Start/End Address"]
    #[inline(always)]
    pub const fn tzc_wram_tzsrg_r1(&self) -> &TzcWramTzsrgR1 {
        &self.tzc_wram_tzsrg_r1
    }
    #[doc = "0x190 - WRAM Security Region 2 Start/End Address"]
    #[inline(always)]
    pub const fn tzc_wram_tzsrg_r2(&self) -> &TzcWramTzsrgR2 {
        &self.tzc_wram_tzsrg_r2
    }
    #[doc = "0x194 - WRAM Security Region 3 (Reserved/Dummy Register)"]
    #[inline(always)]
    pub const fn tzc_wram_tzsrg_r3(&self) -> &TzcWramTzsrgR3 {
        &self.tzc_wram_tzsrg_r3
    }
    #[doc = "0x198 - WiFi Debug Disable Control"]
    #[inline(always)]
    pub const fn tzc_wifi_dbg(&self) -> &TzcWifiDbg {
        &self.tzc_wifi_dbg
    }
    #[doc = "0x240 - PDM TrustZone Control (Reserved)"]
    #[inline(always)]
    pub const fn tzc_pdm_ctrl(&self) -> &TzcPdmCtrl {
        &self.tzc_pdm_ctrl
    }
    #[doc = "0x244 - UART TrustZone Control (Reserved)"]
    #[inline(always)]
    pub const fn tzc_uart_ctrl(&self) -> &TzcUartCtrl {
        &self.tzc_uart_ctrl
    }
    #[doc = "0x248 - I2C TrustZone Control (Reserved)"]
    #[inline(always)]
    pub const fn tzc_i2c_ctrl(&self) -> &TzcI2cCtrl {
        &self.tzc_i2c_ctrl
    }
    #[doc = "0x24c - TIMER TrustZone Control (Reserved)"]
    #[inline(always)]
    pub const fn tzc_timer_ctrl(&self) -> &TzcTimerCtrl {
        &self.tzc_timer_ctrl
    }
    #[doc = "0x250 - I2S TrustZone Control (Reserved)"]
    #[inline(always)]
    pub const fn tzc_i2s_ctrl(&self) -> &TzcI2sCtrl {
        &self.tzc_i2s_ctrl
    }
    #[doc = "0x280 - Serial Flash (XIP) Security Region Generator Control"]
    #[inline(always)]
    pub const fn tzc_sf_tzsrg_ctrl(&self) -> &TzcSfTzsrgCtrl {
        &self.tzc_sf_tzsrg_ctrl
    }
    #[doc = "0x284 - Serial Flash Security Region Address Mask"]
    #[inline(always)]
    pub const fn tzc_sf_tzsrg_adr_mask(&self) -> &TzcSfTzsrgAdrMask {
        &self.tzc_sf_tzsrg_adr_mask
    }
    #[doc = "0x288 - Serial Flash Security Region 0 Start/End Address"]
    #[inline(always)]
    pub const fn tzc_sf_tzsrg_r0(&self) -> &TzcSfTzsrgR0 {
        &self.tzc_sf_tzsrg_r0
    }
    #[doc = "0x28c - Serial Flash Security Region 1 Start/End Address"]
    #[inline(always)]
    pub const fn tzc_sf_tzsrg_r1(&self) -> &TzcSfTzsrgR1 {
        &self.tzc_sf_tzsrg_r1
    }
    #[doc = "0x290 - Serial Flash Security Region 2 Start/End Address"]
    #[inline(always)]
    pub const fn tzc_sf_tzsrg_r2(&self) -> &TzcSfTzsrgR2 {
        &self.tzc_sf_tzsrg_r2
    }
    #[doc = "0x294 - Serial Flash Security Region 3 Start/End Address"]
    #[inline(always)]
    pub const fn tzc_sf_tzsrg_r3(&self) -> &TzcSfTzsrgR3 {
        &self.tzc_sf_tzsrg_r3
    }
    #[doc = "0x298 - Serial Flash Security Region Address MSBs"]
    #[inline(always)]
    pub const fn tzc_sf_tzsrg_msb(&self) -> &TzcSfTzsrgMsb {
        &self.tzc_sf_tzsrg_msb
    }
    #[doc = "0x300 - Multimedia Bus Matrix Master TZID Assignment"]
    #[inline(always)]
    pub const fn tzc_mm_bmx_tzmid(&self) -> &TzcMmBmxTzmid {
        &self.tzc_mm_bmx_tzmid
    }
    #[doc = "0x304 - Multimedia Bus Matrix Master TZID Lock (Write-Once)"]
    #[inline(always)]
    pub const fn tzc_mm_bmx_tzmid_lock(&self) -> &TzcMmBmxTzmidLock {
        &self.tzc_mm_bmx_tzmid_lock
    }
    #[doc = "0x308 - Multimedia Bus Matrix Slave 0 Access Control Enable (MM Peripherals)"]
    #[inline(always)]
    pub const fn tzc_mm_bmx_s0(&self) -> &TzcMmBmxS0 {
        &self.tzc_mm_bmx_s0
    }
    #[doc = "0x30c - Multimedia Bus Matrix Slave 1 Access Control Enable (Codec Peripherals)"]
    #[inline(always)]
    pub const fn tzc_mm_bmx_s1(&self) -> &TzcMmBmxS1 {
        &self.tzc_mm_bmx_s1
    }
    #[doc = "0x310 - Multimedia Bus Matrix Slave 2 Access Control Enable (Codec Slaves)"]
    #[inline(always)]
    pub const fn tzc_mm_bmx_s2(&self) -> &TzcMmBmxS2 {
        &self.tzc_mm_bmx_s2
    }
    #[doc = "0x314 - Multimedia Bus Matrix Slave Lock 0 (Write-Once)"]
    #[inline(always)]
    pub const fn tzc_mm_bmx_s_lock0(&self) -> &TzcMmBmxSLock0 {
        &self.tzc_mm_bmx_s_lock0
    }
    #[doc = "0x318 - Multimedia Bus Matrix Slave Lock 1 (Write-Once)"]
    #[inline(always)]
    pub const fn tzc_mm_bmx_s_lock1(&self) -> &TzcMmBmxSLock1 {
        &self.tzc_mm_bmx_s_lock1
    }
    #[doc = "0x340 - L2SRAM Security Region Generator Control"]
    #[inline(always)]
    pub const fn tzc_l2sram_tzsrg_ctrl(&self) -> &TzcL2sramTzsrgCtrl {
        &self.tzc_l2sram_tzsrg_ctrl
    }
    #[doc = "0x344 - L2SRAM Security Region Address Mask"]
    #[inline(always)]
    pub const fn tzc_l2sram_tzsrg_adr_mask(&self) -> &TzcL2sramTzsrgAdrMask {
        &self.tzc_l2sram_tzsrg_adr_mask
    }
    #[doc = "0x348 - L2SRAM Security Region 0 Start/End Address"]
    #[inline(always)]
    pub const fn tzc_l2sram_tzsrg_r0(&self) -> &TzcL2sramTzsrgR0 {
        &self.tzc_l2sram_tzsrg_r0
    }
    #[doc = "0x34c - L2SRAM Security Region 1 Start/End Address"]
    #[inline(always)]
    pub const fn tzc_l2sram_tzsrg_r1(&self) -> &TzcL2sramTzsrgR1 {
        &self.tzc_l2sram_tzsrg_r1
    }
    #[doc = "0x350 - L2SRAM Security Region 2 Start/End Address"]
    #[inline(always)]
    pub const fn tzc_l2sram_tzsrg_r2(&self) -> &TzcL2sramTzsrgR2 {
        &self.tzc_l2sram_tzsrg_r2
    }
    #[doc = "0x354 - L2SRAM Security Region 3 (Reserved/Dummy Register)"]
    #[inline(always)]
    pub const fn tzc_l2sram_tzsrg_r3(&self) -> &TzcL2sramTzsrgR3 {
        &self.tzc_l2sram_tzsrg_r3
    }
    #[doc = "0x360 - VRAM Security Region Generator Control"]
    #[inline(always)]
    pub const fn tzc_vram_tzsrg_ctrl(&self) -> &TzcVramTzsrgCtrl {
        &self.tzc_vram_tzsrg_ctrl
    }
    #[doc = "0x364 - VRAM Security Region Address Mask"]
    #[inline(always)]
    pub const fn tzc_vram_tzsrg_adr_mask(&self) -> &TzcVramTzsrgAdrMask {
        &self.tzc_vram_tzsrg_adr_mask
    }
    #[doc = "0x368 - VRAM Security Region 0 Start/End Address"]
    #[inline(always)]
    pub const fn tzc_vram_tzsrg_r0(&self) -> &TzcVramTzsrgR0 {
        &self.tzc_vram_tzsrg_r0
    }
    #[doc = "0x36c - VRAM Security Region 1 Start/End Address"]
    #[inline(always)]
    pub const fn tzc_vram_tzsrg_r1(&self) -> &TzcVramTzsrgR1 {
        &self.tzc_vram_tzsrg_r1
    }
    #[doc = "0x370 - VRAM Security Region 2 Start/End Address"]
    #[inline(always)]
    pub const fn tzc_vram_tzsrg_r2(&self) -> &TzcVramTzsrgR2 {
        &self.tzc_vram_tzsrg_r2
    }
    #[doc = "0x374 - VRAM Security Region 3 (Reserved/Dummy Register)"]
    #[inline(always)]
    pub const fn tzc_vram_tzsrg_r3(&self) -> &TzcVramTzsrgR3 {
        &self.tzc_vram_tzsrg_r3
    }
    #[doc = "0x380 - PSRAM-A Security Region Generator Control"]
    #[inline(always)]
    pub const fn tzc_psrama_tzsrg_ctrl(&self) -> &TzcPsramaTzsrgCtrl {
        &self.tzc_psrama_tzsrg_ctrl
    }
    #[doc = "0x384 - PSRAM-A Security Region Address Mask"]
    #[inline(always)]
    pub const fn tzc_psrama_tzsrg_adr_mask(&self) -> &TzcPsramaTzsrgAdrMask {
        &self.tzc_psrama_tzsrg_adr_mask
    }
    #[doc = "0x388 - PSRAM-A Security Region 0 Start/End Address"]
    #[inline(always)]
    pub const fn tzc_psrama_tzsrg_r0(&self) -> &TzcPsramaTzsrgR0 {
        &self.tzc_psrama_tzsrg_r0
    }
    #[doc = "0x38c - PSRAM-A Security Region 1 Start/End Address"]
    #[inline(always)]
    pub const fn tzc_psrama_tzsrg_r1(&self) -> &TzcPsramaTzsrgR1 {
        &self.tzc_psrama_tzsrg_r1
    }
    #[doc = "0x390 - PSRAM-A Security Region 2 Start/End Address"]
    #[inline(always)]
    pub const fn tzc_psrama_tzsrg_r2(&self) -> &TzcPsramaTzsrgR2 {
        &self.tzc_psrama_tzsrg_r2
    }
    #[doc = "0x394 - PSRAM-A Security Region 3 (Reserved/Dummy Register)"]
    #[inline(always)]
    pub const fn tzc_psrama_tzsrg_r3(&self) -> &TzcPsramaTzsrgR3 {
        &self.tzc_psrama_tzsrg_r3
    }
    #[doc = "0x3a0 - PSRAM-B Security Region Generator Control"]
    #[inline(always)]
    pub const fn tzc_psramb_tzsrg_ctrl(&self) -> &TzcPsrambTzsrgCtrl {
        &self.tzc_psramb_tzsrg_ctrl
    }
    #[doc = "0x3a4 - PSRAM-B Security Region Address Mask"]
    #[inline(always)]
    pub const fn tzc_psramb_tzsrg_adr_mask(&self) -> &TzcPsrambTzsrgAdrMask {
        &self.tzc_psramb_tzsrg_adr_mask
    }
    #[doc = "0x3a8 - PSRAM-B Security Region 0 Start/End Address"]
    #[inline(always)]
    pub const fn tzc_psramb_tzsrg_r0(&self) -> &TzcPsrambTzsrgR0 {
        &self.tzc_psramb_tzsrg_r0
    }
    #[doc = "0x3ac - PSRAM-B Security Region 1 Start/End Address"]
    #[inline(always)]
    pub const fn tzc_psramb_tzsrg_r1(&self) -> &TzcPsrambTzsrgR1 {
        &self.tzc_psramb_tzsrg_r1
    }
    #[doc = "0x3b0 - PSRAM-B Security Region 2 Start/End Address"]
    #[inline(always)]
    pub const fn tzc_psramb_tzsrg_r2(&self) -> &TzcPsrambTzsrgR2 {
        &self.tzc_psramb_tzsrg_r2
    }
    #[doc = "0x3b4 - PSRAM-B Security Region 3 (Reserved/Dummy Register)"]
    #[inline(always)]
    pub const fn tzc_psramb_tzsrg_r3(&self) -> &TzcPsrambTzsrgR3 {
        &self.tzc_psramb_tzsrg_r3
    }
    #[doc = "0x3c0 - XRAM Security Region Generator Control"]
    #[inline(always)]
    pub const fn tzc_xram_tzsrg_ctrl(&self) -> &TzcXramTzsrgCtrl {
        &self.tzc_xram_tzsrg_ctrl
    }
    #[doc = "0x3c4 - XRAM Security Region Address Mask"]
    #[inline(always)]
    pub const fn tzc_xram_tzsrg_adr_mask(&self) -> &TzcXramTzsrgAdrMask {
        &self.tzc_xram_tzsrg_adr_mask
    }
    #[doc = "0x3c8 - XRAM Security Region 0 Start/End Address"]
    #[inline(always)]
    pub const fn tzc_xram_tzsrg_r0(&self) -> &TzcXramTzsrgR0 {
        &self.tzc_xram_tzsrg_r0
    }
    #[doc = "0x3cc - XRAM Security Region 1 Start/End Address"]
    #[inline(always)]
    pub const fn tzc_xram_tzsrg_r1(&self) -> &TzcXramTzsrgR1 {
        &self.tzc_xram_tzsrg_r1
    }
    #[doc = "0x3d0 - XRAM Security Region 2 Start/End Address"]
    #[inline(always)]
    pub const fn tzc_xram_tzsrg_r2(&self) -> &TzcXramTzsrgR2 {
        &self.tzc_xram_tzsrg_r2
    }
    #[doc = "0x3d4 - XRAM Security Region 3 (Reserved/Dummy Register)"]
    #[inline(always)]
    pub const fn tzc_xram_tzsrg_r3(&self) -> &TzcXramTzsrgR3 {
        &self.tzc_xram_tzsrg_r3
    }
    #[doc = "0xf00 - Global TrustZone Control Register 0"]
    #[inline(always)]
    pub const fn tzc_glb_ctrl_0(&self) -> &TzcGlbCtrl0 {
        &self.tzc_glb_ctrl_0
    }
    #[doc = "0xf04 - Global TrustZone Control Register 1 (Reserved)"]
    #[inline(always)]
    pub const fn tzc_glb_ctrl_1(&self) -> &TzcGlbCtrl1 {
        &self.tzc_glb_ctrl_1
    }
    #[doc = "0xf08 - Global TrustZone Control Register 2 (Lock)"]
    #[inline(always)]
    pub const fn tzc_glb_ctrl_2(&self) -> &TzcGlbCtrl2 {
        &self.tzc_glb_ctrl_2
    }
    #[doc = "0xf20 - Multimedia TrustZone Control Register 0"]
    #[inline(always)]
    pub const fn tzc_mm_ctrl_0(&self) -> &TzcMmCtrl0 {
        &self.tzc_mm_ctrl_0
    }
    #[doc = "0xf24 - Multimedia TrustZone Control Register 1 (Reserved)"]
    #[inline(always)]
    pub const fn tzc_mm_ctrl_1(&self) -> &TzcMmCtrl1 {
        &self.tzc_mm_ctrl_1
    }
    #[doc = "0xf28 - Multimedia TrustZone Control Register 2 (Lock)"]
    #[inline(always)]
    pub const fn tzc_mm_ctrl_2(&self) -> &TzcMmCtrl2 {
        &self.tzc_mm_ctrl_2
    }
    #[doc = "0xf40 - Security Engine TrustZone Control Register 0"]
    #[inline(always)]
    pub const fn tzc_se_ctrl_0(&self) -> &TzcSeCtrl0 {
        &self.tzc_se_ctrl_0
    }
    #[doc = "0xf44 - Security Engine TrustZone Control Register 1"]
    #[inline(always)]
    pub const fn tzc_se_ctrl_1(&self) -> &TzcSeCtrl1 {
        &self.tzc_se_ctrl_1
    }
    #[doc = "0xf48 - Security Engine TrustZone Control Register 2 (Lock)"]
    #[inline(always)]
    pub const fn tzc_se_ctrl_2(&self) -> &TzcSeCtrl2 {
        &self.tzc_se_ctrl_2
    }
}
#[doc = "TZC_ROM_TZSRG_CTRL (rw) register accessor: ROM Security Region Generator Control\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_rom_tzsrg_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tzc_rom_tzsrg_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_rom_tzsrg_ctrl`] module"]
#[doc(alias = "TZC_ROM_TZSRG_CTRL")]
pub type TzcRomTzsrgCtrl = crate::Reg<tzc_rom_tzsrg_ctrl::TzcRomTzsrgCtrlSpec>;
#[doc = "ROM Security Region Generator Control"]
pub mod tzc_rom_tzsrg_ctrl;
#[doc = "TZC_ROM_TZSRG_ADR_MASK (rw) register accessor: ROM Security Region Address Mask\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_rom_tzsrg_adr_mask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tzc_rom_tzsrg_adr_mask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_rom_tzsrg_adr_mask`] module"]
#[doc(alias = "TZC_ROM_TZSRG_ADR_MASK")]
pub type TzcRomTzsrgAdrMask = crate::Reg<tzc_rom_tzsrg_adr_mask::TzcRomTzsrgAdrMaskSpec>;
#[doc = "ROM Security Region Address Mask"]
pub mod tzc_rom_tzsrg_adr_mask;
#[doc = "TZC_ROM_TZSRG_R0 (rw) register accessor: ROM Security Region 0 Start/End Address\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_rom_tzsrg_r0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tzc_rom_tzsrg_r0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_rom_tzsrg_r0`] module"]
#[doc(alias = "TZC_ROM_TZSRG_R0")]
pub type TzcRomTzsrgR0 = crate::Reg<tzc_rom_tzsrg_r0::TzcRomTzsrgR0Spec>;
#[doc = "ROM Security Region 0 Start/End Address"]
pub mod tzc_rom_tzsrg_r0;
#[doc = "TZC_ROM_TZSRG_R1 (rw) register accessor: ROM Security Region 1 Start/End Address\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_rom_tzsrg_r1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tzc_rom_tzsrg_r1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_rom_tzsrg_r1`] module"]
#[doc(alias = "TZC_ROM_TZSRG_R1")]
pub type TzcRomTzsrgR1 = crate::Reg<tzc_rom_tzsrg_r1::TzcRomTzsrgR1Spec>;
#[doc = "ROM Security Region 1 Start/End Address"]
pub mod tzc_rom_tzsrg_r1;
#[doc = "TZC_ROM_TZSRG_R2 (rw) register accessor: ROM Security Region 2 Start/End Address\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_rom_tzsrg_r2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tzc_rom_tzsrg_r2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_rom_tzsrg_r2`] module"]
#[doc(alias = "TZC_ROM_TZSRG_R2")]
pub type TzcRomTzsrgR2 = crate::Reg<tzc_rom_tzsrg_r2::TzcRomTzsrgR2Spec>;
#[doc = "ROM Security Region 2 Start/End Address"]
pub mod tzc_rom_tzsrg_r2;
#[doc = "TZC_ROM_TZSRG_R3 (r) register accessor: ROM Security Region 3 (Reserved/Dummy Register)\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_rom_tzsrg_r3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_rom_tzsrg_r3`] module"]
#[doc(alias = "TZC_ROM_TZSRG_R3")]
pub type TzcRomTzsrgR3 = crate::Reg<tzc_rom_tzsrg_r3::TzcRomTzsrgR3Spec>;
#[doc = "ROM Security Region 3 (Reserved/Dummy Register)"]
pub mod tzc_rom_tzsrg_r3;
#[doc = "TZC_BMX_TZMID (rw) register accessor: Bus Matrix Master TZID Assignment\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_bmx_tzmid::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tzc_bmx_tzmid::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_bmx_tzmid`] module"]
#[doc(alias = "TZC_BMX_TZMID")]
pub type TzcBmxTzmid = crate::Reg<tzc_bmx_tzmid::TzcBmxTzmidSpec>;
#[doc = "Bus Matrix Master TZID Assignment"]
pub mod tzc_bmx_tzmid;
#[doc = "TZC_BMX_TZMID_LOCK (rw) register accessor: Bus Matrix Master TZID Lock (Write-Once)\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_bmx_tzmid_lock::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tzc_bmx_tzmid_lock::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_bmx_tzmid_lock`] module"]
#[doc(alias = "TZC_BMX_TZMID_LOCK")]
pub type TzcBmxTzmidLock = crate::Reg<tzc_bmx_tzmid_lock::TzcBmxTzmidLockSpec>;
#[doc = "Bus Matrix Master TZID Lock (Write-Once)"]
pub mod tzc_bmx_tzmid_lock;
#[doc = "TZC_BMX_S0 (rw) register accessor: Bus Matrix Slave 0 Access Control Enable (MM, DMA, PWR, SDH, EMAC)\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_bmx_s0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tzc_bmx_s0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_bmx_s0`] module"]
#[doc(alias = "TZC_BMX_S0")]
pub type TzcBmxS0 = crate::Reg<tzc_bmx_s0::TzcBmxS0Spec>;
#[doc = "Bus Matrix Slave 0 Access Control Enable (MM, DMA, PWR, SDH, EMAC)"]
pub mod tzc_bmx_s0;
#[doc = "TZC_BMX_S1 (rw) register accessor: Bus Matrix Slave 1 Access Control Enable (Peripherals: UART, SPI, I2C, etc.)\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_bmx_s1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tzc_bmx_s1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_bmx_s1`] module"]
#[doc(alias = "TZC_BMX_S1")]
pub type TzcBmxS1 = crate::Reg<tzc_bmx_s1::TzcBmxS1Spec>;
#[doc = "Bus Matrix Slave 1 Access Control Enable (Peripherals: UART, SPI, I2C, etc.)"]
pub mod tzc_bmx_s1;
#[doc = "TZC_BMX_S2 (rw) register accessor: Bus Matrix Slave 2 Access Control Enable (Ext/Wireless)\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_bmx_s2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tzc_bmx_s2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_bmx_s2`] module"]
#[doc(alias = "TZC_BMX_S2")]
pub type TzcBmxS2 = crate::Reg<tzc_bmx_s2::TzcBmxS2Spec>;
#[doc = "Bus Matrix Slave 2 Access Control Enable (Ext/Wireless)"]
pub mod tzc_bmx_s2;
#[doc = "TZC_BMX_S_LOCK (rw) register accessor: Bus Matrix Slave Access Control Lock (Write-Once)\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_bmx_s_lock::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tzc_bmx_s_lock::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_bmx_s_lock`] module"]
#[doc(alias = "TZC_BMX_S_LOCK")]
pub type TzcBmxSLock = crate::Reg<tzc_bmx_s_lock::TzcBmxSLockSpec>;
#[doc = "Bus Matrix Slave Access Control Lock (Write-Once)"]
pub mod tzc_bmx_s_lock;
#[doc = "TZC_OCRAM_TZSRG_CTRL (rw) register accessor: OCRAM Security Region Generator Control\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_ocram_tzsrg_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tzc_ocram_tzsrg_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_ocram_tzsrg_ctrl`] module"]
#[doc(alias = "TZC_OCRAM_TZSRG_CTRL")]
pub type TzcOcramTzsrgCtrl = crate::Reg<tzc_ocram_tzsrg_ctrl::TzcOcramTzsrgCtrlSpec>;
#[doc = "OCRAM Security Region Generator Control"]
pub mod tzc_ocram_tzsrg_ctrl;
#[doc = "TZC_OCRAM_TZSRG_ADR_MASK (rw) register accessor: OCRAM Security Region Address Mask\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_ocram_tzsrg_adr_mask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tzc_ocram_tzsrg_adr_mask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_ocram_tzsrg_adr_mask`] module"]
#[doc(alias = "TZC_OCRAM_TZSRG_ADR_MASK")]
pub type TzcOcramTzsrgAdrMask = crate::Reg<tzc_ocram_tzsrg_adr_mask::TzcOcramTzsrgAdrMaskSpec>;
#[doc = "OCRAM Security Region Address Mask"]
pub mod tzc_ocram_tzsrg_adr_mask;
#[doc = "TZC_OCRAM_TZSRG_R0 (rw) register accessor: OCRAM Security Region 0 Start/End Address\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_ocram_tzsrg_r0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tzc_ocram_tzsrg_r0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_ocram_tzsrg_r0`] module"]
#[doc(alias = "TZC_OCRAM_TZSRG_R0")]
pub type TzcOcramTzsrgR0 = crate::Reg<tzc_ocram_tzsrg_r0::TzcOcramTzsrgR0Spec>;
#[doc = "OCRAM Security Region 0 Start/End Address"]
pub mod tzc_ocram_tzsrg_r0;
#[doc = "TZC_OCRAM_TZSRG_R1 (rw) register accessor: OCRAM Security Region 1 Start/End Address\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_ocram_tzsrg_r1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tzc_ocram_tzsrg_r1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_ocram_tzsrg_r1`] module"]
#[doc(alias = "TZC_OCRAM_TZSRG_R1")]
pub type TzcOcramTzsrgR1 = crate::Reg<tzc_ocram_tzsrg_r1::TzcOcramTzsrgR1Spec>;
#[doc = "OCRAM Security Region 1 Start/End Address"]
pub mod tzc_ocram_tzsrg_r1;
#[doc = "TZC_OCRAM_TZSRG_R2 (rw) register accessor: OCRAM Security Region 2 Start/End Address\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_ocram_tzsrg_r2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tzc_ocram_tzsrg_r2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_ocram_tzsrg_r2`] module"]
#[doc(alias = "TZC_OCRAM_TZSRG_R2")]
pub type TzcOcramTzsrgR2 = crate::Reg<tzc_ocram_tzsrg_r2::TzcOcramTzsrgR2Spec>;
#[doc = "OCRAM Security Region 2 Start/End Address"]
pub mod tzc_ocram_tzsrg_r2;
#[doc = "TZC_OCRAM_TZSRG_R3 (r) register accessor: OCRAM Security Region 3 (Reserved/Dummy Register)\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_ocram_tzsrg_r3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_ocram_tzsrg_r3`] module"]
#[doc(alias = "TZC_OCRAM_TZSRG_R3")]
pub type TzcOcramTzsrgR3 = crate::Reg<tzc_ocram_tzsrg_r3::TzcOcramTzsrgR3Spec>;
#[doc = "OCRAM Security Region 3 (Reserved/Dummy Register)"]
pub mod tzc_ocram_tzsrg_r3;
#[doc = "TZC_WRAM_TZSRG_CTRL (rw) register accessor: WRAM Security Region Generator Control\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_wram_tzsrg_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tzc_wram_tzsrg_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_wram_tzsrg_ctrl`] module"]
#[doc(alias = "TZC_WRAM_TZSRG_CTRL")]
pub type TzcWramTzsrgCtrl = crate::Reg<tzc_wram_tzsrg_ctrl::TzcWramTzsrgCtrlSpec>;
#[doc = "WRAM Security Region Generator Control"]
pub mod tzc_wram_tzsrg_ctrl;
#[doc = "TZC_WRAM_TZSRG_ADR_MASK (rw) register accessor: WRAM Security Region Address Mask\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_wram_tzsrg_adr_mask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tzc_wram_tzsrg_adr_mask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_wram_tzsrg_adr_mask`] module"]
#[doc(alias = "TZC_WRAM_TZSRG_ADR_MASK")]
pub type TzcWramTzsrgAdrMask = crate::Reg<tzc_wram_tzsrg_adr_mask::TzcWramTzsrgAdrMaskSpec>;
#[doc = "WRAM Security Region Address Mask"]
pub mod tzc_wram_tzsrg_adr_mask;
#[doc = "TZC_WRAM_TZSRG_R0 (rw) register accessor: WRAM Security Region 0 Start/End Address\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_wram_tzsrg_r0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tzc_wram_tzsrg_r0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_wram_tzsrg_r0`] module"]
#[doc(alias = "TZC_WRAM_TZSRG_R0")]
pub type TzcWramTzsrgR0 = crate::Reg<tzc_wram_tzsrg_r0::TzcWramTzsrgR0Spec>;
#[doc = "WRAM Security Region 0 Start/End Address"]
pub mod tzc_wram_tzsrg_r0;
#[doc = "TZC_WRAM_TZSRG_R1 (rw) register accessor: WRAM Security Region 1 Start/End Address\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_wram_tzsrg_r1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tzc_wram_tzsrg_r1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_wram_tzsrg_r1`] module"]
#[doc(alias = "TZC_WRAM_TZSRG_R1")]
pub type TzcWramTzsrgR1 = crate::Reg<tzc_wram_tzsrg_r1::TzcWramTzsrgR1Spec>;
#[doc = "WRAM Security Region 1 Start/End Address"]
pub mod tzc_wram_tzsrg_r1;
#[doc = "TZC_WRAM_TZSRG_R2 (rw) register accessor: WRAM Security Region 2 Start/End Address\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_wram_tzsrg_r2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tzc_wram_tzsrg_r2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_wram_tzsrg_r2`] module"]
#[doc(alias = "TZC_WRAM_TZSRG_R2")]
pub type TzcWramTzsrgR2 = crate::Reg<tzc_wram_tzsrg_r2::TzcWramTzsrgR2Spec>;
#[doc = "WRAM Security Region 2 Start/End Address"]
pub mod tzc_wram_tzsrg_r2;
#[doc = "TZC_WRAM_TZSRG_R3 (r) register accessor: WRAM Security Region 3 (Reserved/Dummy Register)\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_wram_tzsrg_r3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_wram_tzsrg_r3`] module"]
#[doc(alias = "TZC_WRAM_TZSRG_R3")]
pub type TzcWramTzsrgR3 = crate::Reg<tzc_wram_tzsrg_r3::TzcWramTzsrgR3Spec>;
#[doc = "WRAM Security Region 3 (Reserved/Dummy Register)"]
pub mod tzc_wram_tzsrg_r3;
#[doc = "TZC_WIFI_DBG (rw) register accessor: WiFi Debug Disable Control\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_wifi_dbg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tzc_wifi_dbg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_wifi_dbg`] module"]
#[doc(alias = "TZC_WIFI_DBG")]
pub type TzcWifiDbg = crate::Reg<tzc_wifi_dbg::TzcWifiDbgSpec>;
#[doc = "WiFi Debug Disable Control"]
pub mod tzc_wifi_dbg;
#[doc = "TZC_PDM_CTRL (r) register accessor: PDM TrustZone Control (Reserved)\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_pdm_ctrl::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_pdm_ctrl`] module"]
#[doc(alias = "TZC_PDM_CTRL")]
pub type TzcPdmCtrl = crate::Reg<tzc_pdm_ctrl::TzcPdmCtrlSpec>;
#[doc = "PDM TrustZone Control (Reserved)"]
pub mod tzc_pdm_ctrl;
#[doc = "TZC_UART_CTRL (r) register accessor: UART TrustZone Control (Reserved)\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_uart_ctrl::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_uart_ctrl`] module"]
#[doc(alias = "TZC_UART_CTRL")]
pub type TzcUartCtrl = crate::Reg<tzc_uart_ctrl::TzcUartCtrlSpec>;
#[doc = "UART TrustZone Control (Reserved)"]
pub mod tzc_uart_ctrl;
#[doc = "TZC_I2C_CTRL (r) register accessor: I2C TrustZone Control (Reserved)\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_i2c_ctrl::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_i2c_ctrl`] module"]
#[doc(alias = "TZC_I2C_CTRL")]
pub type TzcI2cCtrl = crate::Reg<tzc_i2c_ctrl::TzcI2cCtrlSpec>;
#[doc = "I2C TrustZone Control (Reserved)"]
pub mod tzc_i2c_ctrl;
#[doc = "TZC_TIMER_CTRL (r) register accessor: TIMER TrustZone Control (Reserved)\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_timer_ctrl::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_timer_ctrl`] module"]
#[doc(alias = "TZC_TIMER_CTRL")]
pub type TzcTimerCtrl = crate::Reg<tzc_timer_ctrl::TzcTimerCtrlSpec>;
#[doc = "TIMER TrustZone Control (Reserved)"]
pub mod tzc_timer_ctrl;
#[doc = "TZC_I2S_CTRL (r) register accessor: I2S TrustZone Control (Reserved)\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_i2s_ctrl::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_i2s_ctrl`] module"]
#[doc(alias = "TZC_I2S_CTRL")]
pub type TzcI2sCtrl = crate::Reg<tzc_i2s_ctrl::TzcI2sCtrlSpec>;
#[doc = "I2S TrustZone Control (Reserved)"]
pub mod tzc_i2s_ctrl;
#[doc = "TZC_SF_TZSRG_CTRL (rw) register accessor: Serial Flash (XIP) Security Region Generator Control\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_sf_tzsrg_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tzc_sf_tzsrg_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_sf_tzsrg_ctrl`] module"]
#[doc(alias = "TZC_SF_TZSRG_CTRL")]
pub type TzcSfTzsrgCtrl = crate::Reg<tzc_sf_tzsrg_ctrl::TzcSfTzsrgCtrlSpec>;
#[doc = "Serial Flash (XIP) Security Region Generator Control"]
pub mod tzc_sf_tzsrg_ctrl;
#[doc = "TZC_SF_TZSRG_ADR_MASK (rw) register accessor: Serial Flash Security Region Address Mask\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_sf_tzsrg_adr_mask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tzc_sf_tzsrg_adr_mask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_sf_tzsrg_adr_mask`] module"]
#[doc(alias = "TZC_SF_TZSRG_ADR_MASK")]
pub type TzcSfTzsrgAdrMask = crate::Reg<tzc_sf_tzsrg_adr_mask::TzcSfTzsrgAdrMaskSpec>;
#[doc = "Serial Flash Security Region Address Mask"]
pub mod tzc_sf_tzsrg_adr_mask;
#[doc = "TZC_SF_TZSRG_R0 (rw) register accessor: Serial Flash Security Region 0 Start/End Address\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_sf_tzsrg_r0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tzc_sf_tzsrg_r0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_sf_tzsrg_r0`] module"]
#[doc(alias = "TZC_SF_TZSRG_R0")]
pub type TzcSfTzsrgR0 = crate::Reg<tzc_sf_tzsrg_r0::TzcSfTzsrgR0Spec>;
#[doc = "Serial Flash Security Region 0 Start/End Address"]
pub mod tzc_sf_tzsrg_r0;
#[doc = "TZC_SF_TZSRG_R1 (rw) register accessor: Serial Flash Security Region 1 Start/End Address\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_sf_tzsrg_r1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tzc_sf_tzsrg_r1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_sf_tzsrg_r1`] module"]
#[doc(alias = "TZC_SF_TZSRG_R1")]
pub type TzcSfTzsrgR1 = crate::Reg<tzc_sf_tzsrg_r1::TzcSfTzsrgR1Spec>;
#[doc = "Serial Flash Security Region 1 Start/End Address"]
pub mod tzc_sf_tzsrg_r1;
#[doc = "TZC_SF_TZSRG_R2 (rw) register accessor: Serial Flash Security Region 2 Start/End Address\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_sf_tzsrg_r2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tzc_sf_tzsrg_r2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_sf_tzsrg_r2`] module"]
#[doc(alias = "TZC_SF_TZSRG_R2")]
pub type TzcSfTzsrgR2 = crate::Reg<tzc_sf_tzsrg_r2::TzcSfTzsrgR2Spec>;
#[doc = "Serial Flash Security Region 2 Start/End Address"]
pub mod tzc_sf_tzsrg_r2;
#[doc = "TZC_SF_TZSRG_R3 (rw) register accessor: Serial Flash Security Region 3 Start/End Address\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_sf_tzsrg_r3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tzc_sf_tzsrg_r3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_sf_tzsrg_r3`] module"]
#[doc(alias = "TZC_SF_TZSRG_R3")]
pub type TzcSfTzsrgR3 = crate::Reg<tzc_sf_tzsrg_r3::TzcSfTzsrgR3Spec>;
#[doc = "Serial Flash Security Region 3 Start/End Address"]
pub mod tzc_sf_tzsrg_r3;
#[doc = "TZC_SF_TZSRG_MSB (rw) register accessor: Serial Flash Security Region Address MSBs\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_sf_tzsrg_msb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tzc_sf_tzsrg_msb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_sf_tzsrg_msb`] module"]
#[doc(alias = "TZC_SF_TZSRG_MSB")]
pub type TzcSfTzsrgMsb = crate::Reg<tzc_sf_tzsrg_msb::TzcSfTzsrgMsbSpec>;
#[doc = "Serial Flash Security Region Address MSBs"]
pub mod tzc_sf_tzsrg_msb;
#[doc = "TZC_MM_BMX_TZMID (rw) register accessor: Multimedia Bus Matrix Master TZID Assignment\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_mm_bmx_tzmid::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tzc_mm_bmx_tzmid::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_mm_bmx_tzmid`] module"]
#[doc(alias = "TZC_MM_BMX_TZMID")]
pub type TzcMmBmxTzmid = crate::Reg<tzc_mm_bmx_tzmid::TzcMmBmxTzmidSpec>;
#[doc = "Multimedia Bus Matrix Master TZID Assignment"]
pub mod tzc_mm_bmx_tzmid;
#[doc = "TZC_MM_BMX_TZMID_LOCK (rw) register accessor: Multimedia Bus Matrix Master TZID Lock (Write-Once)\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_mm_bmx_tzmid_lock::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tzc_mm_bmx_tzmid_lock::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_mm_bmx_tzmid_lock`] module"]
#[doc(alias = "TZC_MM_BMX_TZMID_LOCK")]
pub type TzcMmBmxTzmidLock = crate::Reg<tzc_mm_bmx_tzmid_lock::TzcMmBmxTzmidLockSpec>;
#[doc = "Multimedia Bus Matrix Master TZID Lock (Write-Once)"]
pub mod tzc_mm_bmx_tzmid_lock;
#[doc = "TZC_MM_BMX_S0 (rw) register accessor: Multimedia Bus Matrix Slave 0 Access Control Enable (MM Peripherals)\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_mm_bmx_s0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tzc_mm_bmx_s0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_mm_bmx_s0`] module"]
#[doc(alias = "TZC_MM_BMX_S0")]
pub type TzcMmBmxS0 = crate::Reg<tzc_mm_bmx_s0::TzcMmBmxS0Spec>;
#[doc = "Multimedia Bus Matrix Slave 0 Access Control Enable (MM Peripherals)"]
pub mod tzc_mm_bmx_s0;
#[doc = "TZC_MM_BMX_S1 (rw) register accessor: Multimedia Bus Matrix Slave 1 Access Control Enable (Codec Peripherals)\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_mm_bmx_s1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tzc_mm_bmx_s1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_mm_bmx_s1`] module"]
#[doc(alias = "TZC_MM_BMX_S1")]
pub type TzcMmBmxS1 = crate::Reg<tzc_mm_bmx_s1::TzcMmBmxS1Spec>;
#[doc = "Multimedia Bus Matrix Slave 1 Access Control Enable (Codec Peripherals)"]
pub mod tzc_mm_bmx_s1;
#[doc = "TZC_MM_BMX_S2 (rw) register accessor: Multimedia Bus Matrix Slave 2 Access Control Enable (Codec Slaves)\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_mm_bmx_s2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tzc_mm_bmx_s2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_mm_bmx_s2`] module"]
#[doc(alias = "TZC_MM_BMX_S2")]
pub type TzcMmBmxS2 = crate::Reg<tzc_mm_bmx_s2::TzcMmBmxS2Spec>;
#[doc = "Multimedia Bus Matrix Slave 2 Access Control Enable (Codec Slaves)"]
pub mod tzc_mm_bmx_s2;
#[doc = "TZC_MM_BMX_S_LOCK0 (rw) register accessor: Multimedia Bus Matrix Slave Lock 0 (Write-Once)\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_mm_bmx_s_lock0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tzc_mm_bmx_s_lock0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_mm_bmx_s_lock0`] module"]
#[doc(alias = "TZC_MM_BMX_S_LOCK0")]
pub type TzcMmBmxSLock0 = crate::Reg<tzc_mm_bmx_s_lock0::TzcMmBmxSLock0Spec>;
#[doc = "Multimedia Bus Matrix Slave Lock 0 (Write-Once)"]
pub mod tzc_mm_bmx_s_lock0;
#[doc = "TZC_MM_BMX_S_LOCK1 (rw) register accessor: Multimedia Bus Matrix Slave Lock 1 (Write-Once)\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_mm_bmx_s_lock1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tzc_mm_bmx_s_lock1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_mm_bmx_s_lock1`] module"]
#[doc(alias = "TZC_MM_BMX_S_LOCK1")]
pub type TzcMmBmxSLock1 = crate::Reg<tzc_mm_bmx_s_lock1::TzcMmBmxSLock1Spec>;
#[doc = "Multimedia Bus Matrix Slave Lock 1 (Write-Once)"]
pub mod tzc_mm_bmx_s_lock1;
#[doc = "TZC_PSRAMA_TZSRG_CTRL (rw) register accessor: PSRAM-A Security Region Generator Control\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_psrama_tzsrg_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tzc_psrama_tzsrg_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_psrama_tzsrg_ctrl`] module"]
#[doc(alias = "TZC_PSRAMA_TZSRG_CTRL")]
pub type TzcPsramaTzsrgCtrl = crate::Reg<tzc_psrama_tzsrg_ctrl::TzcPsramaTzsrgCtrlSpec>;
#[doc = "PSRAM-A Security Region Generator Control"]
pub mod tzc_psrama_tzsrg_ctrl;
#[doc = "TZC_PSRAMA_TZSRG_ADR_MASK (rw) register accessor: PSRAM-A Security Region Address Mask\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_psrama_tzsrg_adr_mask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tzc_psrama_tzsrg_adr_mask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_psrama_tzsrg_adr_mask`] module"]
#[doc(alias = "TZC_PSRAMA_TZSRG_ADR_MASK")]
pub type TzcPsramaTzsrgAdrMask = crate::Reg<tzc_psrama_tzsrg_adr_mask::TzcPsramaTzsrgAdrMaskSpec>;
#[doc = "PSRAM-A Security Region Address Mask"]
pub mod tzc_psrama_tzsrg_adr_mask;
#[doc = "TZC_PSRAMA_TZSRG_R0 (rw) register accessor: PSRAM-A Security Region 0 Start/End Address\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_psrama_tzsrg_r0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tzc_psrama_tzsrg_r0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_psrama_tzsrg_r0`] module"]
#[doc(alias = "TZC_PSRAMA_TZSRG_R0")]
pub type TzcPsramaTzsrgR0 = crate::Reg<tzc_psrama_tzsrg_r0::TzcPsramaTzsrgR0Spec>;
#[doc = "PSRAM-A Security Region 0 Start/End Address"]
pub mod tzc_psrama_tzsrg_r0;
#[doc = "TZC_PSRAMA_TZSRG_R1 (rw) register accessor: PSRAM-A Security Region 1 Start/End Address\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_psrama_tzsrg_r1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tzc_psrama_tzsrg_r1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_psrama_tzsrg_r1`] module"]
#[doc(alias = "TZC_PSRAMA_TZSRG_R1")]
pub type TzcPsramaTzsrgR1 = crate::Reg<tzc_psrama_tzsrg_r1::TzcPsramaTzsrgR1Spec>;
#[doc = "PSRAM-A Security Region 1 Start/End Address"]
pub mod tzc_psrama_tzsrg_r1;
#[doc = "TZC_PSRAMA_TZSRG_R2 (rw) register accessor: PSRAM-A Security Region 2 Start/End Address\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_psrama_tzsrg_r2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tzc_psrama_tzsrg_r2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_psrama_tzsrg_r2`] module"]
#[doc(alias = "TZC_PSRAMA_TZSRG_R2")]
pub type TzcPsramaTzsrgR2 = crate::Reg<tzc_psrama_tzsrg_r2::TzcPsramaTzsrgR2Spec>;
#[doc = "PSRAM-A Security Region 2 Start/End Address"]
pub mod tzc_psrama_tzsrg_r2;
#[doc = "TZC_PSRAMA_TZSRG_R3 (r) register accessor: PSRAM-A Security Region 3 (Reserved/Dummy Register)\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_psrama_tzsrg_r3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_psrama_tzsrg_r3`] module"]
#[doc(alias = "TZC_PSRAMA_TZSRG_R3")]
pub type TzcPsramaTzsrgR3 = crate::Reg<tzc_psrama_tzsrg_r3::TzcPsramaTzsrgR3Spec>;
#[doc = "PSRAM-A Security Region 3 (Reserved/Dummy Register)"]
pub mod tzc_psrama_tzsrg_r3;
#[doc = "TZC_PSRAMB_TZSRG_CTRL (rw) register accessor: PSRAM-B Security Region Generator Control\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_psramb_tzsrg_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tzc_psramb_tzsrg_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_psramb_tzsrg_ctrl`] module"]
#[doc(alias = "TZC_PSRAMB_TZSRG_CTRL")]
pub type TzcPsrambTzsrgCtrl = crate::Reg<tzc_psramb_tzsrg_ctrl::TzcPsrambTzsrgCtrlSpec>;
#[doc = "PSRAM-B Security Region Generator Control"]
pub mod tzc_psramb_tzsrg_ctrl;
#[doc = "TZC_PSRAMB_TZSRG_ADR_MASK (rw) register accessor: PSRAM-B Security Region Address Mask\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_psramb_tzsrg_adr_mask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tzc_psramb_tzsrg_adr_mask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_psramb_tzsrg_adr_mask`] module"]
#[doc(alias = "TZC_PSRAMB_TZSRG_ADR_MASK")]
pub type TzcPsrambTzsrgAdrMask = crate::Reg<tzc_psramb_tzsrg_adr_mask::TzcPsrambTzsrgAdrMaskSpec>;
#[doc = "PSRAM-B Security Region Address Mask"]
pub mod tzc_psramb_tzsrg_adr_mask;
#[doc = "TZC_PSRAMB_TZSRG_R0 (rw) register accessor: PSRAM-B Security Region 0 Start/End Address\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_psramb_tzsrg_r0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tzc_psramb_tzsrg_r0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_psramb_tzsrg_r0`] module"]
#[doc(alias = "TZC_PSRAMB_TZSRG_R0")]
pub type TzcPsrambTzsrgR0 = crate::Reg<tzc_psramb_tzsrg_r0::TzcPsrambTzsrgR0Spec>;
#[doc = "PSRAM-B Security Region 0 Start/End Address"]
pub mod tzc_psramb_tzsrg_r0;
#[doc = "TZC_PSRAMB_TZSRG_R1 (rw) register accessor: PSRAM-B Security Region 1 Start/End Address\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_psramb_tzsrg_r1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tzc_psramb_tzsrg_r1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_psramb_tzsrg_r1`] module"]
#[doc(alias = "TZC_PSRAMB_TZSRG_R1")]
pub type TzcPsrambTzsrgR1 = crate::Reg<tzc_psramb_tzsrg_r1::TzcPsrambTzsrgR1Spec>;
#[doc = "PSRAM-B Security Region 1 Start/End Address"]
pub mod tzc_psramb_tzsrg_r1;
#[doc = "TZC_PSRAMB_TZSRG_R2 (rw) register accessor: PSRAM-B Security Region 2 Start/End Address\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_psramb_tzsrg_r2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tzc_psramb_tzsrg_r2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_psramb_tzsrg_r2`] module"]
#[doc(alias = "TZC_PSRAMB_TZSRG_R2")]
pub type TzcPsrambTzsrgR2 = crate::Reg<tzc_psramb_tzsrg_r2::TzcPsrambTzsrgR2Spec>;
#[doc = "PSRAM-B Security Region 2 Start/End Address"]
pub mod tzc_psramb_tzsrg_r2;
#[doc = "TZC_PSRAMB_TZSRG_R3 (r) register accessor: PSRAM-B Security Region 3 (Reserved/Dummy Register)\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_psramb_tzsrg_r3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_psramb_tzsrg_r3`] module"]
#[doc(alias = "TZC_PSRAMB_TZSRG_R3")]
pub type TzcPsrambTzsrgR3 = crate::Reg<tzc_psramb_tzsrg_r3::TzcPsrambTzsrgR3Spec>;
#[doc = "PSRAM-B Security Region 3 (Reserved/Dummy Register)"]
pub mod tzc_psramb_tzsrg_r3;
#[doc = "TZC_XRAM_TZSRG_CTRL (rw) register accessor: XRAM Security Region Generator Control\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_xram_tzsrg_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tzc_xram_tzsrg_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_xram_tzsrg_ctrl`] module"]
#[doc(alias = "TZC_XRAM_TZSRG_CTRL")]
pub type TzcXramTzsrgCtrl = crate::Reg<tzc_xram_tzsrg_ctrl::TzcXramTzsrgCtrlSpec>;
#[doc = "XRAM Security Region Generator Control"]
pub mod tzc_xram_tzsrg_ctrl;
#[doc = "TZC_XRAM_TZSRG_ADR_MASK (rw) register accessor: XRAM Security Region Address Mask\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_xram_tzsrg_adr_mask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tzc_xram_tzsrg_adr_mask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_xram_tzsrg_adr_mask`] module"]
#[doc(alias = "TZC_XRAM_TZSRG_ADR_MASK")]
pub type TzcXramTzsrgAdrMask = crate::Reg<tzc_xram_tzsrg_adr_mask::TzcXramTzsrgAdrMaskSpec>;
#[doc = "XRAM Security Region Address Mask"]
pub mod tzc_xram_tzsrg_adr_mask;
#[doc = "TZC_XRAM_TZSRG_R0 (rw) register accessor: XRAM Security Region 0 Start/End Address\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_xram_tzsrg_r0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tzc_xram_tzsrg_r0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_xram_tzsrg_r0`] module"]
#[doc(alias = "TZC_XRAM_TZSRG_R0")]
pub type TzcXramTzsrgR0 = crate::Reg<tzc_xram_tzsrg_r0::TzcXramTzsrgR0Spec>;
#[doc = "XRAM Security Region 0 Start/End Address"]
pub mod tzc_xram_tzsrg_r0;
#[doc = "TZC_XRAM_TZSRG_R1 (rw) register accessor: XRAM Security Region 1 Start/End Address\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_xram_tzsrg_r1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tzc_xram_tzsrg_r1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_xram_tzsrg_r1`] module"]
#[doc(alias = "TZC_XRAM_TZSRG_R1")]
pub type TzcXramTzsrgR1 = crate::Reg<tzc_xram_tzsrg_r1::TzcXramTzsrgR1Spec>;
#[doc = "XRAM Security Region 1 Start/End Address"]
pub mod tzc_xram_tzsrg_r1;
#[doc = "TZC_XRAM_TZSRG_R2 (rw) register accessor: XRAM Security Region 2 Start/End Address\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_xram_tzsrg_r2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tzc_xram_tzsrg_r2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_xram_tzsrg_r2`] module"]
#[doc(alias = "TZC_XRAM_TZSRG_R2")]
pub type TzcXramTzsrgR2 = crate::Reg<tzc_xram_tzsrg_r2::TzcXramTzsrgR2Spec>;
#[doc = "XRAM Security Region 2 Start/End Address"]
pub mod tzc_xram_tzsrg_r2;
#[doc = "TZC_XRAM_TZSRG_R3 (r) register accessor: XRAM Security Region 3 (Reserved/Dummy Register)\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_xram_tzsrg_r3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_xram_tzsrg_r3`] module"]
#[doc(alias = "TZC_XRAM_TZSRG_R3")]
pub type TzcXramTzsrgR3 = crate::Reg<tzc_xram_tzsrg_r3::TzcXramTzsrgR3Spec>;
#[doc = "XRAM Security Region 3 (Reserved/Dummy Register)"]
pub mod tzc_xram_tzsrg_r3;
#[doc = "TZC_L2SRAM_TZSRG_CTRL (rw) register accessor: L2SRAM Security Region Generator Control\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_l2sram_tzsrg_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tzc_l2sram_tzsrg_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_l2sram_tzsrg_ctrl`] module"]
#[doc(alias = "TZC_L2SRAM_TZSRG_CTRL")]
pub type TzcL2sramTzsrgCtrl = crate::Reg<tzc_l2sram_tzsrg_ctrl::TzcL2sramTzsrgCtrlSpec>;
#[doc = "L2SRAM Security Region Generator Control"]
pub mod tzc_l2sram_tzsrg_ctrl;
#[doc = "TZC_L2SRAM_TZSRG_ADR_MASK (rw) register accessor: L2SRAM Security Region Address Mask\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_l2sram_tzsrg_adr_mask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tzc_l2sram_tzsrg_adr_mask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_l2sram_tzsrg_adr_mask`] module"]
#[doc(alias = "TZC_L2SRAM_TZSRG_ADR_MASK")]
pub type TzcL2sramTzsrgAdrMask = crate::Reg<tzc_l2sram_tzsrg_adr_mask::TzcL2sramTzsrgAdrMaskSpec>;
#[doc = "L2SRAM Security Region Address Mask"]
pub mod tzc_l2sram_tzsrg_adr_mask;
#[doc = "TZC_L2SRAM_TZSRG_R0 (rw) register accessor: L2SRAM Security Region 0 Start/End Address\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_l2sram_tzsrg_r0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tzc_l2sram_tzsrg_r0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_l2sram_tzsrg_r0`] module"]
#[doc(alias = "TZC_L2SRAM_TZSRG_R0")]
pub type TzcL2sramTzsrgR0 = crate::Reg<tzc_l2sram_tzsrg_r0::TzcL2sramTzsrgR0Spec>;
#[doc = "L2SRAM Security Region 0 Start/End Address"]
pub mod tzc_l2sram_tzsrg_r0;
#[doc = "TZC_L2SRAM_TZSRG_R1 (rw) register accessor: L2SRAM Security Region 1 Start/End Address\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_l2sram_tzsrg_r1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tzc_l2sram_tzsrg_r1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_l2sram_tzsrg_r1`] module"]
#[doc(alias = "TZC_L2SRAM_TZSRG_R1")]
pub type TzcL2sramTzsrgR1 = crate::Reg<tzc_l2sram_tzsrg_r1::TzcL2sramTzsrgR1Spec>;
#[doc = "L2SRAM Security Region 1 Start/End Address"]
pub mod tzc_l2sram_tzsrg_r1;
#[doc = "TZC_L2SRAM_TZSRG_R2 (rw) register accessor: L2SRAM Security Region 2 Start/End Address\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_l2sram_tzsrg_r2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tzc_l2sram_tzsrg_r2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_l2sram_tzsrg_r2`] module"]
#[doc(alias = "TZC_L2SRAM_TZSRG_R2")]
pub type TzcL2sramTzsrgR2 = crate::Reg<tzc_l2sram_tzsrg_r2::TzcL2sramTzsrgR2Spec>;
#[doc = "L2SRAM Security Region 2 Start/End Address"]
pub mod tzc_l2sram_tzsrg_r2;
#[doc = "TZC_L2SRAM_TZSRG_R3 (r) register accessor: L2SRAM Security Region 3 (Reserved/Dummy Register)\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_l2sram_tzsrg_r3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_l2sram_tzsrg_r3`] module"]
#[doc(alias = "TZC_L2SRAM_TZSRG_R3")]
pub type TzcL2sramTzsrgR3 = crate::Reg<tzc_l2sram_tzsrg_r3::TzcL2sramTzsrgR3Spec>;
#[doc = "L2SRAM Security Region 3 (Reserved/Dummy Register)"]
pub mod tzc_l2sram_tzsrg_r3;
#[doc = "TZC_VRAM_TZSRG_CTRL (rw) register accessor: VRAM Security Region Generator Control\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_vram_tzsrg_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tzc_vram_tzsrg_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_vram_tzsrg_ctrl`] module"]
#[doc(alias = "TZC_VRAM_TZSRG_CTRL")]
pub type TzcVramTzsrgCtrl = crate::Reg<tzc_vram_tzsrg_ctrl::TzcVramTzsrgCtrlSpec>;
#[doc = "VRAM Security Region Generator Control"]
pub mod tzc_vram_tzsrg_ctrl;
#[doc = "TZC_VRAM_TZSRG_ADR_MASK (rw) register accessor: VRAM Security Region Address Mask\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_vram_tzsrg_adr_mask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tzc_vram_tzsrg_adr_mask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_vram_tzsrg_adr_mask`] module"]
#[doc(alias = "TZC_VRAM_TZSRG_ADR_MASK")]
pub type TzcVramTzsrgAdrMask = crate::Reg<tzc_vram_tzsrg_adr_mask::TzcVramTzsrgAdrMaskSpec>;
#[doc = "VRAM Security Region Address Mask"]
pub mod tzc_vram_tzsrg_adr_mask;
#[doc = "TZC_VRAM_TZSRG_R0 (rw) register accessor: VRAM Security Region 0 Start/End Address\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_vram_tzsrg_r0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tzc_vram_tzsrg_r0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_vram_tzsrg_r0`] module"]
#[doc(alias = "TZC_VRAM_TZSRG_R0")]
pub type TzcVramTzsrgR0 = crate::Reg<tzc_vram_tzsrg_r0::TzcVramTzsrgR0Spec>;
#[doc = "VRAM Security Region 0 Start/End Address"]
pub mod tzc_vram_tzsrg_r0;
#[doc = "TZC_VRAM_TZSRG_R1 (rw) register accessor: VRAM Security Region 1 Start/End Address\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_vram_tzsrg_r1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tzc_vram_tzsrg_r1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_vram_tzsrg_r1`] module"]
#[doc(alias = "TZC_VRAM_TZSRG_R1")]
pub type TzcVramTzsrgR1 = crate::Reg<tzc_vram_tzsrg_r1::TzcVramTzsrgR1Spec>;
#[doc = "VRAM Security Region 1 Start/End Address"]
pub mod tzc_vram_tzsrg_r1;
#[doc = "TZC_VRAM_TZSRG_R2 (rw) register accessor: VRAM Security Region 2 Start/End Address\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_vram_tzsrg_r2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tzc_vram_tzsrg_r2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_vram_tzsrg_r2`] module"]
#[doc(alias = "TZC_VRAM_TZSRG_R2")]
pub type TzcVramTzsrgR2 = crate::Reg<tzc_vram_tzsrg_r2::TzcVramTzsrgR2Spec>;
#[doc = "VRAM Security Region 2 Start/End Address"]
pub mod tzc_vram_tzsrg_r2;
#[doc = "TZC_VRAM_TZSRG_R3 (r) register accessor: VRAM Security Region 3 (Reserved/Dummy Register)\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_vram_tzsrg_r3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_vram_tzsrg_r3`] module"]
#[doc(alias = "TZC_VRAM_TZSRG_R3")]
pub type TzcVramTzsrgR3 = crate::Reg<tzc_vram_tzsrg_r3::TzcVramTzsrgR3Spec>;
#[doc = "VRAM Security Region 3 (Reserved/Dummy Register)"]
pub mod tzc_vram_tzsrg_r3;
#[doc = "TZC_GLB_CTRL_0 (rw) register accessor: Global TrustZone Control Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_glb_ctrl_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tzc_glb_ctrl_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_glb_ctrl_0`] module"]
#[doc(alias = "TZC_GLB_CTRL_0")]
pub type TzcGlbCtrl0 = crate::Reg<tzc_glb_ctrl_0::TzcGlbCtrl0Spec>;
#[doc = "Global TrustZone Control Register 0"]
pub mod tzc_glb_ctrl_0;
#[doc = "TZC_GLB_CTRL_1 (r) register accessor: Global TrustZone Control Register 1 (Reserved)\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_glb_ctrl_1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_glb_ctrl_1`] module"]
#[doc(alias = "TZC_GLB_CTRL_1")]
pub type TzcGlbCtrl1 = crate::Reg<tzc_glb_ctrl_1::TzcGlbCtrl1Spec>;
#[doc = "Global TrustZone Control Register 1 (Reserved)"]
pub mod tzc_glb_ctrl_1;
#[doc = "TZC_GLB_CTRL_2 (rw) register accessor: Global TrustZone Control Register 2 (Lock)\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_glb_ctrl_2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tzc_glb_ctrl_2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_glb_ctrl_2`] module"]
#[doc(alias = "TZC_GLB_CTRL_2")]
pub type TzcGlbCtrl2 = crate::Reg<tzc_glb_ctrl_2::TzcGlbCtrl2Spec>;
#[doc = "Global TrustZone Control Register 2 (Lock)"]
pub mod tzc_glb_ctrl_2;
#[doc = "TZC_MM_CTRL_0 (rw) register accessor: Multimedia TrustZone Control Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_mm_ctrl_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tzc_mm_ctrl_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_mm_ctrl_0`] module"]
#[doc(alias = "TZC_MM_CTRL_0")]
pub type TzcMmCtrl0 = crate::Reg<tzc_mm_ctrl_0::TzcMmCtrl0Spec>;
#[doc = "Multimedia TrustZone Control Register 0"]
pub mod tzc_mm_ctrl_0;
#[doc = "TZC_MM_CTRL_1 (r) register accessor: Multimedia TrustZone Control Register 1 (Reserved)\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_mm_ctrl_1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_mm_ctrl_1`] module"]
#[doc(alias = "TZC_MM_CTRL_1")]
pub type TzcMmCtrl1 = crate::Reg<tzc_mm_ctrl_1::TzcMmCtrl1Spec>;
#[doc = "Multimedia TrustZone Control Register 1 (Reserved)"]
pub mod tzc_mm_ctrl_1;
#[doc = "TZC_MM_CTRL_2 (rw) register accessor: Multimedia TrustZone Control Register 2 (Lock)\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_mm_ctrl_2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tzc_mm_ctrl_2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_mm_ctrl_2`] module"]
#[doc(alias = "TZC_MM_CTRL_2")]
pub type TzcMmCtrl2 = crate::Reg<tzc_mm_ctrl_2::TzcMmCtrl2Spec>;
#[doc = "Multimedia TrustZone Control Register 2 (Lock)"]
pub mod tzc_mm_ctrl_2;
#[doc = "TZC_SE_CTRL_0 (rw) register accessor: Security Engine TrustZone Control Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_se_ctrl_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tzc_se_ctrl_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_se_ctrl_0`] module"]
#[doc(alias = "TZC_SE_CTRL_0")]
pub type TzcSeCtrl0 = crate::Reg<tzc_se_ctrl_0::TzcSeCtrl0Spec>;
#[doc = "Security Engine TrustZone Control Register 0"]
pub mod tzc_se_ctrl_0;
#[doc = "TZC_SE_CTRL_1 (rw) register accessor: Security Engine TrustZone Control Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_se_ctrl_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tzc_se_ctrl_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_se_ctrl_1`] module"]
#[doc(alias = "TZC_SE_CTRL_1")]
pub type TzcSeCtrl1 = crate::Reg<tzc_se_ctrl_1::TzcSeCtrl1Spec>;
#[doc = "Security Engine TrustZone Control Register 1"]
pub mod tzc_se_ctrl_1;
#[doc = "TZC_SE_CTRL_2 (rw) register accessor: Security Engine TrustZone Control Register 2 (Lock)\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_se_ctrl_2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tzc_se_ctrl_2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_se_ctrl_2`] module"]
#[doc(alias = "TZC_SE_CTRL_2")]
pub type TzcSeCtrl2 = crate::Reg<tzc_se_ctrl_2::TzcSeCtrl2Spec>;
#[doc = "Security Engine TrustZone Control Register 2 (Lock)"]
pub mod tzc_se_ctrl_2;
