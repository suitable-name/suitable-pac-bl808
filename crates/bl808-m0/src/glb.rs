#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    soc_info0: SocInfo0,
    _reserved1: [u8; 0x4c],
    core_cfg16: CoreCfg16,
    core_cfg17: CoreCfg17,
    core_cfg18: CoreCfg18,
    core_cfg19: CoreCfg19,
    core_cfg20: CoreCfg20,
    core_cfg21: CoreCfg21,
    core_cfg22: CoreCfg22,
    core_cfg23: CoreCfg23,
    core_cfg24: CoreCfg24,
    core_cfg25: CoreCfg25,
    _reserved11: [u8; 0x18],
    sys_cfg0: SysCfg0,
    sys_cfg1: SysCfg1,
    _reserved13: [u8; 0x08],
    bus_cfg0: BusCfg0,
    _reserved14: [u8; 0x3c],
    emi_cfg0: EmiCfg0,
    _reserved15: [u8; 0x0c],
    rtc_cfg0: RtcCfg0,
    _reserved16: [u8; 0x1c],
    adc_cfg0: AdcCfg0,
    _reserved17: [u8; 0x0c],
    dac_cfg0: DacCfg0,
    dac_cfg1: DacCfg1,
    dac_cfg2: DacCfg2,
    dac_cfg3: DacCfg3,
    dma_cfg0: DmaCfg0,
    dma_cfg1: DmaCfg1,
    dma_cfg2: DmaCfg2,
    _reserved24: [u8; 0x04],
    ir_cfg0: IrCfg0,
    ir_cfg1: IrCfg1,
    _reserved26: [u8; 0x08],
    uart_cfg0: UartCfg0,
    uart_cfg1: UartCfg1,
    uart_cfg2: UartCfg2,
    _reserved29: [u8; 0x14],
    sf_cfg0: SfCfg0,
    _reserved30: [u8; 0x0c],
    i2c_cfg0: I2cCfg0,
    _reserved31: [u8; 0x0c],
    i2s_cfg0: I2sCfg0,
    _reserved32: [u8; 0x1c],
    spi_cfg0: SpiCfg0,
    _reserved33: [u8; 0x0c],
    qdec_cfg0: QdecCfg0,
    _reserved34: [u8; 0x0c],
    pwm_cfg0: PwmCfg0,
    _reserved35: [u8; 0x0c],
    pdm_cfg0: PdmCfg0,
    _reserved36: [u8; 0x6c],
    dig_clk_cfg0: DigClkCfg0,
    dig_clk_cfg1: DigClkCfg1,
    dig_clk_cfg2: DigClkCfg2,
    dig_clk_cfg3: DigClkCfg3,
    rf_cfg0: RfCfg0,
    _reserved41: [u8; 0x7c],
    dbg_cfg0: DbgCfg0,
    dbg_cfg1: DbgCfg1,
    dbg_cfg2: DbgCfg2,
    dbg_cfg3: DbgCfg3,
    dbg_cfg4: DbgCfg4,
    _reserved46: [u8; 0x0c],
    mbist_cfg0: MbistCfg0,
    _reserved47: [u8; 0x1c],
    bmx_cfg0: BmxCfg0,
    bmx_cfg1: BmxCfg1,
    bmx_cfg2: BmxCfg2,
    bmx_cfg3: BmxCfg3,
    bmx_cfg4: BmxCfg4,
    bmx_cfg5: BmxCfg5,
    bmx_cfg6: BmxCfg6,
    _reserved54: [u8; 0x04],
    audio_cfg0: AudioCfg0,
    audio_cfg1: AudioCfg1,
    _reserved56: [u8; 0x48],
    eth_cfg0: EthCfg0,
    _reserved57: [u8; 0x8c],
    cam_cfg0: CamCfg0,
    _reserved58: [u8; 0x0c],
    sdh_cfg0: SdhCfg0,
    _reserved59: [u8; 0x5c],
    tzc_cfg0: TzcCfg0,
    _reserved60: [u8; 0x7c],
    glb_parm_cfg0: GlbParmCfg0,
    _reserved61: [u8; 0x0c],
    debug_cfg0: DebugCfg0,
    debug_cfg1: DebugCfg1,
    _reserved63: [u8; 0x08],
    reset_sts0: ResetSts0,
    _reserved64: [u8; 0x0c],
    swrst_s1_ext: SwrstS1Ext,
    swrst_s1: SwrstS1,
    swrst_cfg2: SwrstCfg2,
    swrst_cfg3: SwrstCfg3,
    _reserved68: [u8; 0x30],
    cgen_m: CgenM,
    cgen_s1a: CgenS1a,
    cgen_s1_ext: CgenS1Ext,
    cgen_cfg3: CgenCfg3,
    _reserved72: [u8; 0x30],
    hw_rsv0: HwRsv0,
    hw_rsv1: HwRsv1,
    hw_rsv2: HwRsv2,
    hw_rsv3: HwRsv3,
    _reserved76: [u8; 0x30],
    sram_cfg0: SramCfg0,
    sram_cfg1: SramCfg1,
    sram_cfg2: SramCfg2,
    sram_cfg3: SramCfg3,
    sram_cfg4: SramCfg4,
    _reserved81: [u8; 0x0c],
    psram_cfg0: PsramCfg0,
    _reserved82: [u8; 0x9c],
    ldo28cis: Ldo28cis,
    ldo18io: Ldo18io,
    ldo15cis: Ldo15cis,
    ldo18flash: Ldo18flash,
    ldo12uhs: Ldo12uhs,
    _reserved87: [u8; 0x1c],
    proc_mon: ProcMon,
    _reserved88: [u8; 0x0c],
    dll_cfg0: DllCfg0,
    _reserved89: [u8; 0x8c],
    mipi_pll_cfg0: MipiPllCfg0,
    mipi_pll_cfg1: MipiPllCfg1,
    mipi_pll_cfg2: MipiPllCfg2,
    mipi_pll_cfg3: MipiPllCfg3,
    mipi_pll_cfg4: MipiPllCfg4,
    mipi_pll_cfg5: MipiPllCfg5,
    mipi_pll_cfg6: MipiPllCfg6,
    mipi_pll_cfg7: MipiPllCfg7,
    mipi_pll_cfg8: MipiPllCfg8,
    mipi_pll_cfg9: MipiPllCfg9,
    _reserved99: [u8; 0x18],
    uhs_pll_cfg0: UhsPllCfg0,
    uhs_pll_cfg1: UhsPllCfg1,
    uhs_pll_cfg2: UhsPllCfg2,
    uhs_pll_cfg3: UhsPllCfg3,
    uhs_pll_cfg4: UhsPllCfg4,
    uhs_pll_cfg5: UhsPllCfg5,
    uhs_pll_cfg6: UhsPllCfg6,
    uhs_pll_cfg7: UhsPllCfg7,
    uhs_pll_cfg8: UhsPllCfg8,
    uhs_pll_cfg9: UhsPllCfg9,
    _reserved109: [u8; 0x18],
    wifi_pll_cfg0: WifiPllCfg0,
    wifi_pll_cfg1: WifiPllCfg1,
    wifi_pll_cfg2: WifiPllCfg2,
    wifi_pll_cfg3: WifiPllCfg3,
    wifi_pll_cfg4: WifiPllCfg4,
    wifi_pll_cfg5: WifiPllCfg5,
    wifi_pll_cfg6: WifiPllCfg6,
    wifi_pll_cfg7: WifiPllCfg7,
    wifi_pll_cfg8: WifiPllCfg8,
    wifi_pll_cfg9: WifiPllCfg9,
    wifi_pll_cfg10: WifiPllCfg10,
    wifi_pll_cfg11: WifiPllCfg11,
    wifi_pll_cfg12: WifiPllCfg12,
    wifi_pll_cfg13: WifiPllCfg13,
    _reserved123: [u8; 0x5c],
    gauge: Gauge,
    _reserved124: [u8; 0x10],
    gauge_rx_fifo_ctrl: GaugeRxFifoCtrl,
    gauge_rx_fifo_status: GaugeRxFifoStatus,
    gauge_rx_fifo_data: GaugeRxFifoData,
    pin_cfg: [PinCfg; 64],
    _reserved128: [u8; 0x0100],
    gpio_cfg128: GpioCfg128,
    gpio_cfg129: GpioCfg129,
    _reserved130: [u8; 0x18],
    gpio_cfg136: GpioCfg136,
    gpio_cfg137: GpioCfg137,
    gpio_cfg138: GpioCfg138,
    gpio_cfg139: GpioCfg139,
    gpio_cfg140: GpioCfg140,
    gpio_cfg141: GpioCfg141,
    gpio_cfg142: GpioCfg142,
    gpio_cfg143: GpioCfg143,
    gpio_cfg144: GpioCfg144,
}
impl RegisterBlock {
    #[doc = "0x00 - SoC Information 0"]
    #[inline(always)]
    pub const fn soc_info0(&self) -> &SocInfo0 {
        &self.soc_info0
    }
    #[doc = "0x50 - New Peripheral Interrupt Status 0"]
    #[inline(always)]
    pub const fn core_cfg16(&self) -> &CoreCfg16 {
        &self.core_cfg16
    }
    #[doc = "0x54 - New Peripheral Interrupt Status 1"]
    #[inline(always)]
    pub const fn core_cfg17(&self) -> &CoreCfg17 {
        &self.core_cfg17
    }
    #[doc = "0x58 - New Peripheral Interrupt Mask 0"]
    #[inline(always)]
    pub const fn core_cfg18(&self) -> &CoreCfg18 {
        &self.core_cfg18
    }
    #[doc = "0x5c - New Peripheral Interrupt Mask 1"]
    #[inline(always)]
    pub const fn core_cfg19(&self) -> &CoreCfg19 {
        &self.core_cfg19
    }
    #[doc = "0x60 - New Peripheral Interrupt Clear 0"]
    #[inline(always)]
    pub const fn core_cfg20(&self) -> &CoreCfg20 {
        &self.core_cfg20
    }
    #[doc = "0x64 - New Peripheral Interrupt Clear 1"]
    #[inline(always)]
    pub const fn core_cfg21(&self) -> &CoreCfg21 {
        &self.core_cfg21
    }
    #[doc = "0x68 - E902 Interrupt Enable 0"]
    #[inline(always)]
    pub const fn core_cfg22(&self) -> &CoreCfg22 {
        &self.core_cfg22
    }
    #[doc = "0x6c - E902 Interrupt Enable 1"]
    #[inline(always)]
    pub const fn core_cfg23(&self) -> &CoreCfg23 {
        &self.core_cfg23
    }
    #[doc = "0x70 - E902 Interrupt Bus Status 0"]
    #[inline(always)]
    pub const fn core_cfg24(&self) -> &CoreCfg24 {
        &self.core_cfg24
    }
    #[doc = "0x74 - E902 Interrupt Bus Status 1"]
    #[inline(always)]
    pub const fn core_cfg25(&self) -> &CoreCfg25 {
        &self.core_cfg25
    }
    #[doc = "0x90 - System Clock Configuration 0"]
    #[inline(always)]
    pub const fn sys_cfg0(&self) -> &SysCfg0 {
        &self.sys_cfg0
    }
    #[doc = "0x94 - System Clock Configuration 1"]
    #[inline(always)]
    pub const fn sys_cfg1(&self) -> &SysCfg1 {
        &self.sys_cfg1
    }
    #[doc = "0xa0 - Bus Configuration 0"]
    #[inline(always)]
    pub const fn bus_cfg0(&self) -> &BusCfg0 {
        &self.bus_cfg0
    }
    #[doc = "0xe0 - EMI Clock Configuration"]
    #[inline(always)]
    pub const fn emi_cfg0(&self) -> &EmiCfg0 {
        &self.emi_cfg0
    }
    #[doc = "0xf0 - RTC Clock Configuration"]
    #[inline(always)]
    pub const fn rtc_cfg0(&self) -> &RtcCfg0 {
        &self.rtc_cfg0
    }
    #[doc = "0x110 - GPADC Clock Configuration"]
    #[inline(always)]
    pub const fn adc_cfg0(&self) -> &AdcCfg0 {
        &self.adc_cfg0
    }
    #[doc = "0x120 - GPDAC Configuration 0"]
    #[inline(always)]
    pub const fn dac_cfg0(&self) -> &DacCfg0 {
        &self.dac_cfg0
    }
    #[doc = "0x124 - GPDAC Configuration 1 (Channel A)"]
    #[inline(always)]
    pub const fn dac_cfg1(&self) -> &DacCfg1 {
        &self.dac_cfg1
    }
    #[doc = "0x128 - GPDAC Configuration 2 (Channel B)"]
    #[inline(always)]
    pub const fn dac_cfg2(&self) -> &DacCfg2 {
        &self.dac_cfg2
    }
    #[doc = "0x12c - GPDAC Data Register"]
    #[inline(always)]
    pub const fn dac_cfg3(&self) -> &DacCfg3 {
        &self.dac_cfg3
    }
    #[doc = "0x130 - DMA0 Channel Clock Enable"]
    #[inline(always)]
    pub const fn dma_cfg0(&self) -> &DmaCfg0 {
        &self.dma_cfg0
    }
    #[doc = "0x134 - DMA2 Channel Clock Enable"]
    #[inline(always)]
    pub const fn dma_cfg1(&self) -> &DmaCfg1 {
        &self.dma_cfg1
    }
    #[doc = "0x138 - DMA Channel Mux Selection"]
    #[inline(always)]
    pub const fn dma_cfg2(&self) -> &DmaCfg2 {
        &self.dma_cfg2
    }
    #[doc = "0x140 - IR/LED Clock Configuration"]
    #[inline(always)]
    pub const fn ir_cfg0(&self) -> &IrCfg0 {
        &self.ir_cfg0
    }
    #[doc = "0x144 - IR/LED IO Configuration"]
    #[inline(always)]
    pub const fn ir_cfg1(&self) -> &IrCfg1 {
        &self.ir_cfg1
    }
    #[doc = "0x150 - UART Clock Configuration"]
    #[inline(always)]
    pub const fn uart_cfg0(&self) -> &UartCfg0 {
        &self.uart_cfg0
    }
    #[doc = "0x154 - UART Signal Mux 0-7"]
    #[inline(always)]
    pub const fn uart_cfg1(&self) -> &UartCfg1 {
        &self.uart_cfg1
    }
    #[doc = "0x158 - UART Signal Mux 8-11"]
    #[inline(always)]
    pub const fn uart_cfg2(&self) -> &UartCfg2 {
        &self.uart_cfg2
    }
    #[doc = "0x170 - Serial Flash Clock Configuration"]
    #[inline(always)]
    pub const fn sf_cfg0(&self) -> &SfCfg0 {
        &self.sf_cfg0
    }
    #[doc = "0x180 - I2C Clock Configuration"]
    #[inline(always)]
    pub const fn i2c_cfg0(&self) -> &I2cCfg0 {
        &self.i2c_cfg0
    }
    #[doc = "0x190 - I2S Clock Configuration"]
    #[inline(always)]
    pub const fn i2s_cfg0(&self) -> &I2sCfg0 {
        &self.i2s_cfg0
    }
    #[doc = "0x1b0 - SPI Clock Configuration"]
    #[inline(always)]
    pub const fn spi_cfg0(&self) -> &SpiCfg0 {
        &self.spi_cfg0
    }
    #[doc = "0x1c0 - QDEC Configuration"]
    #[inline(always)]
    pub const fn qdec_cfg0(&self) -> &QdecCfg0 {
        &self.qdec_cfg0
    }
    #[doc = "0x1d0 - PWM IO Configuration"]
    #[inline(always)]
    pub const fn pwm_cfg0(&self) -> &PwmCfg0 {
        &self.pwm_cfg0
    }
    #[doc = "0x1e0 - PDM IO Configuration"]
    #[inline(always)]
    pub const fn pdm_cfg0(&self) -> &PdmCfg0 {
        &self.pdm_cfg0
    }
    #[doc = "0x250 - Digital Clock Config 0 (32K/512K)"]
    #[inline(always)]
    pub const fn dig_clk_cfg0(&self) -> &DigClkCfg0 {
        &self.dig_clk_cfg0
    }
    #[doc = "0x254 - Digital Clock Config 1 (MM/TOP Mux)"]
    #[inline(always)]
    pub const fn dig_clk_cfg1(&self) -> &DigClkCfg1 {
        &self.dig_clk_cfg1
    }
    #[doc = "0x258 - Digital Clock Config 2 (Chip Clock Output)"]
    #[inline(always)]
    pub const fn dig_clk_cfg2(&self) -> &DigClkCfg2 {
        &self.dig_clk_cfg2
    }
    #[doc = "0x25c - Digital Clock Config 3 (DSI/CSI Escape Clocks)"]
    #[inline(always)]
    pub const fn dig_clk_cfg3(&self) -> &DigClkCfg3 {
        &self.dig_clk_cfg3
    }
    #[doc = "0x260 - RF Clock Configuration"]
    #[inline(always)]
    pub const fn rf_cfg0(&self) -> &RfCfg0 {
        &self.rf_cfg0
    }
    #[doc = "0x2e0 - Debug Config 0 (Low-Low)"]
    #[inline(always)]
    pub const fn dbg_cfg0(&self) -> &DbgCfg0 {
        &self.dbg_cfg0
    }
    #[doc = "0x2e4 - Debug Config 1 (Low-High)"]
    #[inline(always)]
    pub const fn dbg_cfg1(&self) -> &DbgCfg1 {
        &self.dbg_cfg1
    }
    #[doc = "0x2e8 - Debug Config 2 (High-Low)"]
    #[inline(always)]
    pub const fn dbg_cfg2(&self) -> &DbgCfg2 {
        &self.dbg_cfg2
    }
    #[doc = "0x2ec - Debug Config 3 (High-High)"]
    #[inline(always)]
    pub const fn dbg_cfg3(&self) -> &DbgCfg3 {
        &self.dbg_cfg3
    }
    #[doc = "0x2f0 - Debug Config 4 (Output Enable)"]
    #[inline(always)]
    pub const fn dbg_cfg4(&self) -> &DbgCfg4 {
        &self.dbg_cfg4
    }
    #[doc = "0x300 - MBIST Configuration 0"]
    #[inline(always)]
    pub const fn mbist_cfg0(&self) -> &MbistCfg0 {
        &self.mbist_cfg0
    }
    #[doc = "0x320 - BMX Bus Matrix Config 0"]
    #[inline(always)]
    pub const fn bmx_cfg0(&self) -> &BmxCfg0 {
        &self.bmx_cfg0
    }
    #[doc = "0x324 - BMX Bus Matrix Config 1 (Bus Error Interrupt Enable / QoS)"]
    #[inline(always)]
    pub const fn bmx_cfg1(&self) -> &BmxCfg1 {
        &self.bmx_cfg1
    }
    #[doc = "0x328 - BMX Bus Matrix Config 2 (Bus Error Enable)"]
    #[inline(always)]
    pub const fn bmx_cfg2(&self) -> &BmxCfg2 {
        &self.bmx_cfg2
    }
    #[doc = "0x32c - BMX Bus Matrix Config 3 (Bus Error Clear/Status)"]
    #[inline(always)]
    pub const fn bmx_cfg3(&self) -> &BmxCfg3 {
        &self.bmx_cfg3
    }
    #[doc = "0x330 - BMX Bus Matrix Config 4 (Bus Error Source)"]
    #[inline(always)]
    pub const fn bmx_cfg4(&self) -> &BmxCfg4 {
        &self.bmx_cfg4
    }
    #[doc = "0x334 - BMX Bus Matrix Config 5 (BMX Bus Error Address)"]
    #[inline(always)]
    pub const fn bmx_cfg5(&self) -> &BmxCfg5 {
        &self.bmx_cfg5
    }
    #[doc = "0x338 - BMX Bus Matrix Config 6 (MCU Bus Error Address)"]
    #[inline(always)]
    pub const fn bmx_cfg6(&self) -> &BmxCfg6 {
        &self.bmx_cfg6
    }
    #[doc = "0x340 - Audio Clock Configuration 0 (PDM/ADC/DAC)"]
    #[inline(always)]
    pub const fn audio_cfg0(&self) -> &AudioCfg0 {
        &self.audio_cfg0
    }
    #[doc = "0x344 - Audio Clock Configuration 1 (PADC)"]
    #[inline(always)]
    pub const fn audio_cfg1(&self) -> &AudioCfg1 {
        &self.audio_cfg1
    }
    #[doc = "0x390 - Ethernet Clock Configuration"]
    #[inline(always)]
    pub const fn eth_cfg0(&self) -> &EthCfg0 {
        &self.eth_cfg0
    }
    #[doc = "0x420 - Camera Clock Configuration"]
    #[inline(always)]
    pub const fn cam_cfg0(&self) -> &CamCfg0 {
        &self.cam_cfg0
    }
    #[doc = "0x430 - SDH Clock Configuration"]
    #[inline(always)]
    pub const fn sdh_cfg0(&self) -> &SdhCfg0 {
        &self.sdh_cfg0
    }
    #[doc = "0x490 - TrustZone Controller Config 0 (Lock Bits)"]
    #[inline(always)]
    pub const fn tzc_cfg0(&self) -> &TzcCfg0 {
        &self.tzc_cfg0
    }
    #[doc = "0x510 - Global Parameter Config 0"]
    #[inline(always)]
    pub const fn glb_parm_cfg0(&self) -> &GlbParmCfg0 {
        &self.glb_parm_cfg0
    }
    #[doc = "0x520 - Debug Configuration 0"]
    #[inline(always)]
    pub const fn debug_cfg0(&self) -> &DebugCfg0 {
        &self.debug_cfg0
    }
    #[doc = "0x524 - Debug Configuration 1 (NDReset Gate)"]
    #[inline(always)]
    pub const fn debug_cfg1(&self) -> &DebugCfg1 {
        &self.debug_cfg1
    }
    #[doc = "0x530 - Reset Status Register 0"]
    #[inline(always)]
    pub const fn reset_sts0(&self) -> &ResetSts0 {
        &self.reset_sts0
    }
    #[doc = "0x540 - Software Reset Config 0 (EXT)"]
    #[inline(always)]
    pub const fn swrst_s1_ext(&self) -> &SwrstS1Ext {
        &self.swrst_s1_ext
    }
    #[doc = "0x544 - Software Reset Config 1 (Peripherals)"]
    #[inline(always)]
    pub const fn swrst_s1(&self) -> &SwrstS1 {
        &self.swrst_s1
    }
    #[doc = "0x548 - Software Reset Config 2 (CPU/Chip)"]
    #[inline(always)]
    pub const fn swrst_cfg2(&self) -> &SwrstCfg2 {
        &self.swrst_cfg2
    }
    #[doc = "0x54c - HRESET Disable Config (SWRST_CFG3)"]
    #[inline(always)]
    pub const fn swrst_cfg3(&self) -> &SwrstCfg3 {
        &self.swrst_cfg3
    }
    #[doc = "0x580 - Clock Gate Config 0 (Core)"]
    #[inline(always)]
    pub const fn cgen_m(&self) -> &CgenM {
        &self.cgen_m
    }
    #[doc = "0x584 - Clock Gate Config 1 (S1 Peripherals + S1A Peripherals)"]
    #[inline(always)]
    pub const fn cgen_s1a(&self) -> &CgenS1a {
        &self.cgen_s1a
    }
    #[doc = "0x588 - Clock Gate Config 2 (S3 + S1_EXT Peripherals)"]
    #[inline(always)]
    pub const fn cgen_s1_ext(&self) -> &CgenS1Ext {
        &self.cgen_s1_ext
    }
    #[doc = "0x58c - Clock Gate Config 3"]
    #[inline(always)]
    pub const fn cgen_cfg3(&self) -> &CgenCfg3 {
        &self.cgen_cfg3
    }
    #[doc = "0x5c0 - Hardware Reserved Register 0"]
    #[inline(always)]
    pub const fn hw_rsv0(&self) -> &HwRsv0 {
        &self.hw_rsv0
    }
    #[doc = "0x5c4 - Hardware Reserved Register 1"]
    #[inline(always)]
    pub const fn hw_rsv1(&self) -> &HwRsv1 {
        &self.hw_rsv1
    }
    #[doc = "0x5c8 - Hardware Reserved Register 2"]
    #[inline(always)]
    pub const fn hw_rsv2(&self) -> &HwRsv2 {
        &self.hw_rsv2
    }
    #[doc = "0x5cc - Hardware Reserved Register 3"]
    #[inline(always)]
    pub const fn hw_rsv3(&self) -> &HwRsv3 {
        &self.hw_rsv3
    }
    #[doc = "0x600 - SRAM Configuration 0 - Retention Control"]
    #[inline(always)]
    pub const fn sram_cfg0(&self) -> &SramCfg0 {
        &self.sram_cfg0
    }
    #[doc = "0x604 - SRAM Configuration 1 - Sleep Control"]
    #[inline(always)]
    pub const fn sram_cfg1(&self) -> &SramCfg1 {
        &self.sram_cfg1
    }
    #[doc = "0x608 - SRAM Configuration 2 - DVSE and NAP Control"]
    #[inline(always)]
    pub const fn sram_cfg2(&self) -> &SramCfg2 {
        &self.sram_cfg2
    }
    #[doc = "0x60c - SRAM Configuration 3 - EM and VRAM Selection"]
    #[inline(always)]
    pub const fn sram_cfg3(&self) -> &SramCfg3 {
        &self.sram_cfg3
    }
    #[doc = "0x610 - SRAM Configuration 4 - DVS Control"]
    #[inline(always)]
    pub const fn sram_cfg4(&self) -> &SramCfg4 {
        &self.sram_cfg4
    }
    #[doc = "0x620 - PSRAM Clock Configuration"]
    #[inline(always)]
    pub const fn psram_cfg0(&self) -> &PsramCfg0 {
        &self.psram_cfg0
    }
    #[doc = "0x6c0 - LDO 2.8V CIS Configuration"]
    #[inline(always)]
    pub const fn ldo28cis(&self) -> &Ldo28cis {
        &self.ldo28cis
    }
    #[doc = "0x6c4 - LDO 1.8V IO Configuration"]
    #[inline(always)]
    pub const fn ldo18io(&self) -> &Ldo18io {
        &self.ldo18io
    }
    #[doc = "0x6c8 - LDO 1.5V CIS Configuration"]
    #[inline(always)]
    pub const fn ldo15cis(&self) -> &Ldo15cis {
        &self.ldo15cis
    }
    #[doc = "0x6cc - LDO 1.8V Flash Configuration"]
    #[inline(always)]
    pub const fn ldo18flash(&self) -> &Ldo18flash {
        &self.ldo18flash
    }
    #[doc = "0x6d0 - LDO 1.2V UHS Configuration"]
    #[inline(always)]
    pub const fn ldo12uhs(&self) -> &Ldo12uhs {
        &self.ldo12uhs
    }
    #[doc = "0x6f0 - Process Monitor Configuration"]
    #[inline(always)]
    pub const fn proc_mon(&self) -> &ProcMon {
        &self.proc_mon
    }
    #[doc = "0x700 - DLL Configuration Register 0"]
    #[inline(always)]
    pub const fn dll_cfg0(&self) -> &DllCfg0 {
        &self.dll_cfg0
    }
    #[doc = "0x790 - MIPI PLL Configuration 0 - Power and Reset"]
    #[inline(always)]
    pub const fn mipi_pll_cfg0(&self) -> &MipiPllCfg0 {
        &self.mipi_pll_cfg0
    }
    #[doc = "0x794 - MIPI PLL Configuration 1 - Divider and Clock Select"]
    #[inline(always)]
    pub const fn mipi_pll_cfg1(&self) -> &MipiPllCfg1 {
        &self.mipi_pll_cfg1
    }
    #[doc = "0x798 - MIPI PLL Configuration 2 - Charge Pump"]
    #[inline(always)]
    pub const fn mipi_pll_cfg2(&self) -> &MipiPllCfg2 {
        &self.mipi_pll_cfg2
    }
    #[doc = "0x79c - MIPI PLL Configuration 3 - Loop Filter"]
    #[inline(always)]
    pub const fn mipi_pll_cfg3(&self) -> &MipiPllCfg3 {
        &self.mipi_pll_cfg3
    }
    #[doc = "0x7a0 - MIPI PLL Configuration 4 - Lock Detector"]
    #[inline(always)]
    pub const fn mipi_pll_cfg4(&self) -> &MipiPllCfg4 {
        &self.mipi_pll_cfg4
    }
    #[doc = "0x7a4 - MIPI PLL Configuration 5 - VCO"]
    #[inline(always)]
    pub const fn mipi_pll_cfg5(&self) -> &MipiPllCfg5 {
        &self.mipi_pll_cfg5
    }
    #[doc = "0x7a8 - MIPI PLL Configuration 6 - SDM"]
    #[inline(always)]
    pub const fn mipi_pll_cfg6(&self) -> &MipiPllCfg6 {
        &self.mipi_pll_cfg6
    }
    #[doc = "0x7ac - MIPI PLL Configuration 7 - SDM Dither"]
    #[inline(always)]
    pub const fn mipi_pll_cfg7(&self) -> &MipiPllCfg7 {
        &self.mipi_pll_cfg7
    }
    #[doc = "0x7b0 - MIPI PLL Configuration 8 - Test"]
    #[inline(always)]
    pub const fn mipi_pll_cfg8(&self) -> &MipiPllCfg8 {
        &self.mipi_pll_cfg8
    }
    #[doc = "0x7b4 - MIPI PLL Configuration 9 - SSC"]
    #[inline(always)]
    pub const fn mipi_pll_cfg9(&self) -> &MipiPllCfg9 {
        &self.mipi_pll_cfg9
    }
    #[doc = "0x7d0 - UHS PLL Configuration 0 - Power and Reset"]
    #[inline(always)]
    pub const fn uhs_pll_cfg0(&self) -> &UhsPllCfg0 {
        &self.uhs_pll_cfg0
    }
    #[doc = "0x7d4 - UHS PLL Configuration 1 - Divider and Clock Select"]
    #[inline(always)]
    pub const fn uhs_pll_cfg1(&self) -> &UhsPllCfg1 {
        &self.uhs_pll_cfg1
    }
    #[doc = "0x7d8 - UHS PLL Configuration 2 - Charge Pump"]
    #[inline(always)]
    pub const fn uhs_pll_cfg2(&self) -> &UhsPllCfg2 {
        &self.uhs_pll_cfg2
    }
    #[doc = "0x7dc - UHS PLL Configuration 3 - Loop Filter"]
    #[inline(always)]
    pub const fn uhs_pll_cfg3(&self) -> &UhsPllCfg3 {
        &self.uhs_pll_cfg3
    }
    #[doc = "0x7e0 - UHS PLL Configuration 4 - Lock Detector"]
    #[inline(always)]
    pub const fn uhs_pll_cfg4(&self) -> &UhsPllCfg4 {
        &self.uhs_pll_cfg4
    }
    #[doc = "0x7e4 - UHS PLL Configuration 5 - VCO"]
    #[inline(always)]
    pub const fn uhs_pll_cfg5(&self) -> &UhsPllCfg5 {
        &self.uhs_pll_cfg5
    }
    #[doc = "0x7e8 - UHS PLL Configuration 6 - SDM"]
    #[inline(always)]
    pub const fn uhs_pll_cfg6(&self) -> &UhsPllCfg6 {
        &self.uhs_pll_cfg6
    }
    #[doc = "0x7ec - UHS PLL Configuration 7 - SDM Dither"]
    #[inline(always)]
    pub const fn uhs_pll_cfg7(&self) -> &UhsPllCfg7 {
        &self.uhs_pll_cfg7
    }
    #[doc = "0x7f0 - UHS PLL Configuration 8 - Test"]
    #[inline(always)]
    pub const fn uhs_pll_cfg8(&self) -> &UhsPllCfg8 {
        &self.uhs_pll_cfg8
    }
    #[doc = "0x7f4 - UHS PLL Configuration 9 - SSC"]
    #[inline(always)]
    pub const fn uhs_pll_cfg9(&self) -> &UhsPllCfg9 {
        &self.uhs_pll_cfg9
    }
    #[doc = "0x810 - WIFI PLL Configuration 0 - Power and Reset"]
    #[inline(always)]
    pub const fn wifi_pll_cfg0(&self) -> &WifiPllCfg0 {
        &self.wifi_pll_cfg0
    }
    #[doc = "0x814 - WIFI PLL Configuration 1 - Divider and Clock Select"]
    #[inline(always)]
    pub const fn wifi_pll_cfg1(&self) -> &WifiPllCfg1 {
        &self.wifi_pll_cfg1
    }
    #[doc = "0x818 - WIFI PLL Configuration 2 - Charge Pump"]
    #[inline(always)]
    pub const fn wifi_pll_cfg2(&self) -> &WifiPllCfg2 {
        &self.wifi_pll_cfg2
    }
    #[doc = "0x81c - WIFI PLL Configuration 3 - Loop Filter"]
    #[inline(always)]
    pub const fn wifi_pll_cfg3(&self) -> &WifiPllCfg3 {
        &self.wifi_pll_cfg3
    }
    #[doc = "0x820 - WIFI PLL Configuration 4 - Sample Clock"]
    #[inline(always)]
    pub const fn wifi_pll_cfg4(&self) -> &WifiPllCfg4 {
        &self.wifi_pll_cfg4
    }
    #[doc = "0x824 - WIFI PLL Configuration 5 - VCO"]
    #[inline(always)]
    pub const fn wifi_pll_cfg5(&self) -> &WifiPllCfg5 {
        &self.wifi_pll_cfg5
    }
    #[doc = "0x828 - WIFI PLL Configuration 6 - SDM Input"]
    #[inline(always)]
    pub const fn wifi_pll_cfg6(&self) -> &WifiPllCfg6 {
        &self.wifi_pll_cfg6
    }
    #[doc = "0x82c - WIFI PLL Configuration 7 - SDM Dither"]
    #[inline(always)]
    pub const fn wifi_pll_cfg7(&self) -> &WifiPllCfg7 {
        &self.wifi_pll_cfg7
    }
    #[doc = "0x830 - WIFI PLL Configuration 8 - Output Divider Enable"]
    #[inline(always)]
    pub const fn wifi_pll_cfg8(&self) -> &WifiPllCfg8 {
        &self.wifi_pll_cfg8
    }
    #[doc = "0x834 - WIFI PLL Configuration 9 - Test and Debug"]
    #[inline(always)]
    pub const fn wifi_pll_cfg9(&self) -> &WifiPllCfg9 {
        &self.wifi_pll_cfg9
    }
    #[doc = "0x838 - WIFI PLL Configuration 10 - USB PLL SSC"]
    #[inline(always)]
    pub const fn wifi_pll_cfg10(&self) -> &WifiPllCfg10 {
        &self.wifi_pll_cfg10
    }
    #[doc = "0x83c - WIFI PLL Configuration 11 - USB PLL SDM"]
    #[inline(always)]
    pub const fn wifi_pll_cfg11(&self) -> &WifiPllCfg11 {
        &self.wifi_pll_cfg11
    }
    #[doc = "0x840 - WIFI PLL Configuration 12 - USB PLL SSC Count"]
    #[inline(always)]
    pub const fn wifi_pll_cfg12(&self) -> &WifiPllCfg12 {
        &self.wifi_pll_cfg12
    }
    #[doc = "0x844 - WIFI PLL Configuration 13 - Reserved and DL Control"]
    #[inline(always)]
    pub const fn wifi_pll_cfg13(&self) -> &WifiPllCfg13 {
        &self.wifi_pll_cfg13
    }
    #[doc = "0x8a4 - Gauge ADC Configuration"]
    #[inline(always)]
    pub const fn gauge(&self) -> &Gauge {
        &self.gauge
    }
    #[doc = "0x8b8 - Gauge RX FIFO Control"]
    #[inline(always)]
    pub const fn gauge_rx_fifo_ctrl(&self) -> &GaugeRxFifoCtrl {
        &self.gauge_rx_fifo_ctrl
    }
    #[doc = "0x8bc - Gauge RX FIFO Status"]
    #[inline(always)]
    pub const fn gauge_rx_fifo_status(&self) -> &GaugeRxFifoStatus {
        &self.gauge_rx_fifo_status
    }
    #[doc = "0x8c0 - Gauge RX FIFO Data"]
    #[inline(always)]
    pub const fn gauge_rx_fifo_data(&self) -> &GaugeRxFifoData {
        &self.gauge_rx_fifo_data
    }
    #[doc = "0x8c4..0x9c4 - Configuration for GPIO pin %s"]
    #[inline(always)]
    pub const fn pin_cfg(&self, n: usize) -> &PinCfg {
        &self.pin_cfg[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x8c4..0x9c4 - Configuration for GPIO pin %s"]
    #[inline(always)]
    pub fn pin_cfg_iter(&self) -> impl Iterator<Item = &PinCfg> {
        self.pin_cfg.iter()
    }
    #[doc = "0xac4 - Alternative GPIO Input Readback (Pins 0-31)"]
    #[inline(always)]
    pub const fn gpio_cfg128(&self) -> &GpioCfg128 {
        &self.gpio_cfg128
    }
    #[doc = "0xac8 - Alternative GPIO Input Readback (Pins 32-63)"]
    #[inline(always)]
    pub const fn gpio_cfg129(&self) -> &GpioCfg129 {
        &self.gpio_cfg129
    }
    #[doc = "0xae4 - Alternative GPIO Output (Pins 0-31)"]
    #[inline(always)]
    pub const fn gpio_cfg136(&self) -> &GpioCfg136 {
        &self.gpio_cfg136
    }
    #[doc = "0xae8 - Alternative GPIO Output (Pins 32-63)"]
    #[inline(always)]
    pub const fn gpio_cfg137(&self) -> &GpioCfg137 {
        &self.gpio_cfg137
    }
    #[doc = "0xaec - Alternative GPIO Atomic Set (Pins 0-31)"]
    #[inline(always)]
    pub const fn gpio_cfg138(&self) -> &GpioCfg138 {
        &self.gpio_cfg138
    }
    #[doc = "0xaf0 - Alternative GPIO Atomic Set (Pins 32-63)"]
    #[inline(always)]
    pub const fn gpio_cfg139(&self) -> &GpioCfg139 {
        &self.gpio_cfg139
    }
    #[doc = "0xaf4 - Alternative GPIO Atomic Clear (Pins 0-31)"]
    #[inline(always)]
    pub const fn gpio_cfg140(&self) -> &GpioCfg140 {
        &self.gpio_cfg140
    }
    #[doc = "0xaf8 - Alternative GPIO Atomic Clear (Pins 32-63)"]
    #[inline(always)]
    pub const fn gpio_cfg141(&self) -> &GpioCfg141 {
        &self.gpio_cfg141
    }
    #[doc = "0xafc - GPIO TX Protocol Engine Configuration"]
    #[inline(always)]
    pub const fn gpio_cfg142(&self) -> &GpioCfg142 {
        &self.gpio_cfg142
    }
    #[doc = "0xb00 - GPIO DMA and FIFO Control"]
    #[inline(always)]
    pub const fn gpio_cfg143(&self) -> &GpioCfg143 {
        &self.gpio_cfg143
    }
    #[doc = "0xb04 - GPIO DMA FIFO Write Data"]
    #[inline(always)]
    pub const fn gpio_cfg144(&self) -> &GpioCfg144 {
        &self.gpio_cfg144
    }
}
#[doc = "SOC_INFO0 (r) register accessor: SoC Information 0\n\nYou can [`read`](crate::Reg::read) this register and get [`soc_info0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@soc_info0`] module"]
#[doc(alias = "SOC_INFO0")]
pub type SocInfo0 = crate::Reg<soc_info0::SocInfo0Spec>;
#[doc = "SoC Information 0"]
pub mod soc_info0;
#[doc = "CORE_CFG16 (r) register accessor: New Peripheral Interrupt Status 0\n\nYou can [`read`](crate::Reg::read) this register and get [`core_cfg16::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_cfg16`] module"]
#[doc(alias = "CORE_CFG16")]
pub type CoreCfg16 = crate::Reg<core_cfg16::CoreCfg16Spec>;
#[doc = "New Peripheral Interrupt Status 0"]
pub mod core_cfg16;
#[doc = "CORE_CFG17 (r) register accessor: New Peripheral Interrupt Status 1\n\nYou can [`read`](crate::Reg::read) this register and get [`core_cfg17::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_cfg17`] module"]
#[doc(alias = "CORE_CFG17")]
pub type CoreCfg17 = crate::Reg<core_cfg17::CoreCfg17Spec>;
#[doc = "New Peripheral Interrupt Status 1"]
pub mod core_cfg17;
#[doc = "CORE_CFG18 (rw) register accessor: New Peripheral Interrupt Mask 0\n\nYou can [`read`](crate::Reg::read) this register and get [`core_cfg18::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`core_cfg18::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_cfg18`] module"]
#[doc(alias = "CORE_CFG18")]
pub type CoreCfg18 = crate::Reg<core_cfg18::CoreCfg18Spec>;
#[doc = "New Peripheral Interrupt Mask 0"]
pub mod core_cfg18;
#[doc = "CORE_CFG19 (rw) register accessor: New Peripheral Interrupt Mask 1\n\nYou can [`read`](crate::Reg::read) this register and get [`core_cfg19::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`core_cfg19::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_cfg19`] module"]
#[doc(alias = "CORE_CFG19")]
pub type CoreCfg19 = crate::Reg<core_cfg19::CoreCfg19Spec>;
#[doc = "New Peripheral Interrupt Mask 1"]
pub mod core_cfg19;
#[doc = "CORE_CFG20 (rw) register accessor: New Peripheral Interrupt Clear 0\n\nYou can [`read`](crate::Reg::read) this register and get [`core_cfg20::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`core_cfg20::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_cfg20`] module"]
#[doc(alias = "CORE_CFG20")]
pub type CoreCfg20 = crate::Reg<core_cfg20::CoreCfg20Spec>;
#[doc = "New Peripheral Interrupt Clear 0"]
pub mod core_cfg20;
#[doc = "CORE_CFG21 (rw) register accessor: New Peripheral Interrupt Clear 1\n\nYou can [`read`](crate::Reg::read) this register and get [`core_cfg21::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`core_cfg21::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_cfg21`] module"]
#[doc(alias = "CORE_CFG21")]
pub type CoreCfg21 = crate::Reg<core_cfg21::CoreCfg21Spec>;
#[doc = "New Peripheral Interrupt Clear 1"]
pub mod core_cfg21;
#[doc = "CORE_CFG22 (rw) register accessor: E902 Interrupt Enable 0\n\nYou can [`read`](crate::Reg::read) this register and get [`core_cfg22::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`core_cfg22::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_cfg22`] module"]
#[doc(alias = "CORE_CFG22")]
pub type CoreCfg22 = crate::Reg<core_cfg22::CoreCfg22Spec>;
#[doc = "E902 Interrupt Enable 0"]
pub mod core_cfg22;
#[doc = "CORE_CFG23 (rw) register accessor: E902 Interrupt Enable 1\n\nYou can [`read`](crate::Reg::read) this register and get [`core_cfg23::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`core_cfg23::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_cfg23`] module"]
#[doc(alias = "CORE_CFG23")]
pub type CoreCfg23 = crate::Reg<core_cfg23::CoreCfg23Spec>;
#[doc = "E902 Interrupt Enable 1"]
pub mod core_cfg23;
#[doc = "CORE_CFG24 (r) register accessor: E902 Interrupt Bus Status 0\n\nYou can [`read`](crate::Reg::read) this register and get [`core_cfg24::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_cfg24`] module"]
#[doc(alias = "CORE_CFG24")]
pub type CoreCfg24 = crate::Reg<core_cfg24::CoreCfg24Spec>;
#[doc = "E902 Interrupt Bus Status 0"]
pub mod core_cfg24;
#[doc = "CORE_CFG25 (r) register accessor: E902 Interrupt Bus Status 1\n\nYou can [`read`](crate::Reg::read) this register and get [`core_cfg25::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_cfg25`] module"]
#[doc(alias = "CORE_CFG25")]
pub type CoreCfg25 = crate::Reg<core_cfg25::CoreCfg25Spec>;
#[doc = "E902 Interrupt Bus Status 1"]
pub mod core_cfg25;
#[doc = "SYS_CFG0 (rw) register accessor: System Clock Configuration 0\n\nYou can [`read`](crate::Reg::read) this register and get [`sys_cfg0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sys_cfg0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sys_cfg0`] module"]
#[doc(alias = "SYS_CFG0")]
pub type SysCfg0 = crate::Reg<sys_cfg0::SysCfg0Spec>;
#[doc = "System Clock Configuration 0"]
pub mod sys_cfg0;
#[doc = "SYS_CFG1 (rw) register accessor: System Clock Configuration 1\n\nYou can [`read`](crate::Reg::read) this register and get [`sys_cfg1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sys_cfg1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sys_cfg1`] module"]
#[doc(alias = "SYS_CFG1")]
pub type SysCfg1 = crate::Reg<sys_cfg1::SysCfg1Spec>;
#[doc = "System Clock Configuration 1"]
pub mod sys_cfg1;
#[doc = "BUS_CFG0 (rw) register accessor: Bus Configuration 0\n\nYou can [`read`](crate::Reg::read) this register and get [`bus_cfg0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bus_cfg0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bus_cfg0`] module"]
#[doc(alias = "BUS_CFG0")]
pub type BusCfg0 = crate::Reg<bus_cfg0::BusCfg0Spec>;
#[doc = "Bus Configuration 0"]
pub mod bus_cfg0;
#[doc = "EMI_CFG0 (rw) register accessor: EMI Clock Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`emi_cfg0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`emi_cfg0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@emi_cfg0`] module"]
#[doc(alias = "EMI_CFG0")]
pub type EmiCfg0 = crate::Reg<emi_cfg0::EmiCfg0Spec>;
#[doc = "EMI Clock Configuration"]
pub mod emi_cfg0;
#[doc = "RTC_CFG0 (rw) register accessor: RTC Clock Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`rtc_cfg0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtc_cfg0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtc_cfg0`] module"]
#[doc(alias = "RTC_CFG0")]
pub type RtcCfg0 = crate::Reg<rtc_cfg0::RtcCfg0Spec>;
#[doc = "RTC Clock Configuration"]
pub mod rtc_cfg0;
#[doc = "ADC_CFG0 (rw) register accessor: GPADC Clock Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`adc_cfg0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc_cfg0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc_cfg0`] module"]
#[doc(alias = "ADC_CFG0")]
pub type AdcCfg0 = crate::Reg<adc_cfg0::AdcCfg0Spec>;
#[doc = "GPADC Clock Configuration"]
pub mod adc_cfg0;
#[doc = "DAC_CFG0 (rw) register accessor: GPDAC Configuration 0\n\nYou can [`read`](crate::Reg::read) this register and get [`dac_cfg0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dac_cfg0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dac_cfg0`] module"]
#[doc(alias = "DAC_CFG0")]
pub type DacCfg0 = crate::Reg<dac_cfg0::DacCfg0Spec>;
#[doc = "GPDAC Configuration 0"]
pub mod dac_cfg0;
#[doc = "DAC_CFG1 (rw) register accessor: GPDAC Configuration 1 (Channel A)\n\nYou can [`read`](crate::Reg::read) this register and get [`dac_cfg1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dac_cfg1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dac_cfg1`] module"]
#[doc(alias = "DAC_CFG1")]
pub type DacCfg1 = crate::Reg<dac_cfg1::DacCfg1Spec>;
#[doc = "GPDAC Configuration 1 (Channel A)"]
pub mod dac_cfg1;
#[doc = "DAC_CFG2 (rw) register accessor: GPDAC Configuration 2 (Channel B)\n\nYou can [`read`](crate::Reg::read) this register and get [`dac_cfg2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dac_cfg2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dac_cfg2`] module"]
#[doc(alias = "DAC_CFG2")]
pub type DacCfg2 = crate::Reg<dac_cfg2::DacCfg2Spec>;
#[doc = "GPDAC Configuration 2 (Channel B)"]
pub mod dac_cfg2;
#[doc = "DAC_CFG3 (rw) register accessor: GPDAC Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dac_cfg3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dac_cfg3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dac_cfg3`] module"]
#[doc(alias = "DAC_CFG3")]
pub type DacCfg3 = crate::Reg<dac_cfg3::DacCfg3Spec>;
#[doc = "GPDAC Data Register"]
pub mod dac_cfg3;
#[doc = "DMA_CFG0 (rw) register accessor: DMA0 Channel Clock Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_cfg0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_cfg0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_cfg0`] module"]
#[doc(alias = "DMA_CFG0")]
pub type DmaCfg0 = crate::Reg<dma_cfg0::DmaCfg0Spec>;
#[doc = "DMA0 Channel Clock Enable"]
pub mod dma_cfg0;
#[doc = "DMA_CFG1 (rw) register accessor: DMA2 Channel Clock Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_cfg1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_cfg1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_cfg1`] module"]
#[doc(alias = "DMA_CFG1")]
pub type DmaCfg1 = crate::Reg<dma_cfg1::DmaCfg1Spec>;
#[doc = "DMA2 Channel Clock Enable"]
pub mod dma_cfg1;
#[doc = "DMA_CFG2 (rw) register accessor: DMA Channel Mux Selection\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_cfg2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_cfg2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_cfg2`] module"]
#[doc(alias = "DMA_CFG2")]
pub type DmaCfg2 = crate::Reg<dma_cfg2::DmaCfg2Spec>;
#[doc = "DMA Channel Mux Selection"]
pub mod dma_cfg2;
#[doc = "UART_CFG0 (rw) register accessor: UART Clock Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`uart_cfg0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart_cfg0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart_cfg0`] module"]
#[doc(alias = "UART_CFG0")]
pub type UartCfg0 = crate::Reg<uart_cfg0::UartCfg0Spec>;
#[doc = "UART Clock Configuration"]
pub mod uart_cfg0;
#[doc = "UART_CFG1 (rw) register accessor: UART Signal Mux 0-7\n\nYou can [`read`](crate::Reg::read) this register and get [`uart_cfg1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart_cfg1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart_cfg1`] module"]
#[doc(alias = "UART_CFG1")]
pub type UartCfg1 = crate::Reg<uart_cfg1::UartCfg1Spec>;
#[doc = "UART Signal Mux 0-7"]
pub mod uart_cfg1;
#[doc = "UART_CFG2 (rw) register accessor: UART Signal Mux 8-11\n\nYou can [`read`](crate::Reg::read) this register and get [`uart_cfg2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart_cfg2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart_cfg2`] module"]
#[doc(alias = "UART_CFG2")]
pub type UartCfg2 = crate::Reg<uart_cfg2::UartCfg2Spec>;
#[doc = "UART Signal Mux 8-11"]
pub mod uart_cfg2;
#[doc = "IR_CFG0 (rw) register accessor: IR/LED Clock Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`ir_cfg0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ir_cfg0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ir_cfg0`] module"]
#[doc(alias = "IR_CFG0")]
pub type IrCfg0 = crate::Reg<ir_cfg0::IrCfg0Spec>;
#[doc = "IR/LED Clock Configuration"]
pub mod ir_cfg0;
#[doc = "IR_CFG1 (rw) register accessor: IR/LED IO Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`ir_cfg1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ir_cfg1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ir_cfg1`] module"]
#[doc(alias = "IR_CFG1")]
pub type IrCfg1 = crate::Reg<ir_cfg1::IrCfg1Spec>;
#[doc = "IR/LED IO Configuration"]
pub mod ir_cfg1;
#[doc = "SF_CFG0 (rw) register accessor: Serial Flash Clock Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`sf_cfg0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sf_cfg0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sf_cfg0`] module"]
#[doc(alias = "SF_CFG0")]
pub type SfCfg0 = crate::Reg<sf_cfg0::SfCfg0Spec>;
#[doc = "Serial Flash Clock Configuration"]
pub mod sf_cfg0;
#[doc = "I2C_CFG0 (rw) register accessor: I2C Clock Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c_cfg0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c_cfg0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c_cfg0`] module"]
#[doc(alias = "I2C_CFG0")]
pub type I2cCfg0 = crate::Reg<i2c_cfg0::I2cCfg0Spec>;
#[doc = "I2C Clock Configuration"]
pub mod i2c_cfg0;
#[doc = "I2S_CFG0 (rw) register accessor: I2S Clock Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`i2s_cfg0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2s_cfg0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2s_cfg0`] module"]
#[doc(alias = "I2S_CFG0")]
pub type I2sCfg0 = crate::Reg<i2s_cfg0::I2sCfg0Spec>;
#[doc = "I2S Clock Configuration"]
pub mod i2s_cfg0;
#[doc = "SPI_CFG0 (rw) register accessor: SPI Clock Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`spi_cfg0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_cfg0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_cfg0`] module"]
#[doc(alias = "SPI_CFG0")]
pub type SpiCfg0 = crate::Reg<spi_cfg0::SpiCfg0Spec>;
#[doc = "SPI Clock Configuration"]
pub mod spi_cfg0;
#[doc = "QDEC_CFG0 (rw) register accessor: QDEC Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`qdec_cfg0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qdec_cfg0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qdec_cfg0`] module"]
#[doc(alias = "QDEC_CFG0")]
pub type QdecCfg0 = crate::Reg<qdec_cfg0::QdecCfg0Spec>;
#[doc = "QDEC Configuration"]
pub mod qdec_cfg0;
#[doc = "PWM_CFG0 (rw) register accessor: PWM IO Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`pwm_cfg0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwm_cfg0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwm_cfg0`] module"]
#[doc(alias = "PWM_CFG0")]
pub type PwmCfg0 = crate::Reg<pwm_cfg0::PwmCfg0Spec>;
#[doc = "PWM IO Configuration"]
pub mod pwm_cfg0;
#[doc = "PDM_CFG0 (rw) register accessor: PDM IO Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`pdm_cfg0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdm_cfg0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pdm_cfg0`] module"]
#[doc(alias = "PDM_CFG0")]
pub type PdmCfg0 = crate::Reg<pdm_cfg0::PdmCfg0Spec>;
#[doc = "PDM IO Configuration"]
pub mod pdm_cfg0;
#[doc = "DIG_CLK_CFG0 (rw) register accessor: Digital Clock Config 0 (32K/512K)\n\nYou can [`read`](crate::Reg::read) this register and get [`dig_clk_cfg0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dig_clk_cfg0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dig_clk_cfg0`] module"]
#[doc(alias = "DIG_CLK_CFG0")]
pub type DigClkCfg0 = crate::Reg<dig_clk_cfg0::DigClkCfg0Spec>;
#[doc = "Digital Clock Config 0 (32K/512K)"]
pub mod dig_clk_cfg0;
#[doc = "DIG_CLK_CFG1 (rw) register accessor: Digital Clock Config 1 (MM/TOP Mux)\n\nYou can [`read`](crate::Reg::read) this register and get [`dig_clk_cfg1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dig_clk_cfg1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dig_clk_cfg1`] module"]
#[doc(alias = "DIG_CLK_CFG1")]
pub type DigClkCfg1 = crate::Reg<dig_clk_cfg1::DigClkCfg1Spec>;
#[doc = "Digital Clock Config 1 (MM/TOP Mux)"]
pub mod dig_clk_cfg1;
#[doc = "DIG_CLK_CFG2 (rw) register accessor: Digital Clock Config 2 (Chip Clock Output)\n\nYou can [`read`](crate::Reg::read) this register and get [`dig_clk_cfg2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dig_clk_cfg2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dig_clk_cfg2`] module"]
#[doc(alias = "DIG_CLK_CFG2")]
pub type DigClkCfg2 = crate::Reg<dig_clk_cfg2::DigClkCfg2Spec>;
#[doc = "Digital Clock Config 2 (Chip Clock Output)"]
pub mod dig_clk_cfg2;
#[doc = "DIG_CLK_CFG3 (rw) register accessor: Digital Clock Config 3 (DSI/CSI Escape Clocks)\n\nYou can [`read`](crate::Reg::read) this register and get [`dig_clk_cfg3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dig_clk_cfg3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dig_clk_cfg3`] module"]
#[doc(alias = "DIG_CLK_CFG3")]
pub type DigClkCfg3 = crate::Reg<dig_clk_cfg3::DigClkCfg3Spec>;
#[doc = "Digital Clock Config 3 (DSI/CSI Escape Clocks)"]
pub mod dig_clk_cfg3;
#[doc = "RF_CFG0 (rw) register accessor: RF Clock Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`rf_cfg0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rf_cfg0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rf_cfg0`] module"]
#[doc(alias = "RF_CFG0")]
pub type RfCfg0 = crate::Reg<rf_cfg0::RfCfg0Spec>;
#[doc = "RF Clock Configuration"]
pub mod rf_cfg0;
#[doc = "BMX_CFG0 (rw) register accessor: BMX Bus Matrix Config 0\n\nYou can [`read`](crate::Reg::read) this register and get [`bmx_cfg0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bmx_cfg0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bmx_cfg0`] module"]
#[doc(alias = "BMX_CFG0")]
pub type BmxCfg0 = crate::Reg<bmx_cfg0::BmxCfg0Spec>;
#[doc = "BMX Bus Matrix Config 0"]
pub mod bmx_cfg0;
#[doc = "BMX_CFG1 (rw) register accessor: BMX Bus Matrix Config 1 (Bus Error Interrupt Enable / QoS)\n\nYou can [`read`](crate::Reg::read) this register and get [`bmx_cfg1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bmx_cfg1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bmx_cfg1`] module"]
#[doc(alias = "BMX_CFG1")]
pub type BmxCfg1 = crate::Reg<bmx_cfg1::BmxCfg1Spec>;
#[doc = "BMX Bus Matrix Config 1 (Bus Error Interrupt Enable / QoS)"]
pub mod bmx_cfg1;
#[doc = "BMX_CFG2 (rw) register accessor: BMX Bus Matrix Config 2 (Bus Error Enable)\n\nYou can [`read`](crate::Reg::read) this register and get [`bmx_cfg2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bmx_cfg2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bmx_cfg2`] module"]
#[doc(alias = "BMX_CFG2")]
pub type BmxCfg2 = crate::Reg<bmx_cfg2::BmxCfg2Spec>;
#[doc = "BMX Bus Matrix Config 2 (Bus Error Enable)"]
pub mod bmx_cfg2;
#[doc = "BMX_CFG3 (rw) register accessor: BMX Bus Matrix Config 3 (Bus Error Clear/Status)\n\nYou can [`read`](crate::Reg::read) this register and get [`bmx_cfg3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bmx_cfg3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bmx_cfg3`] module"]
#[doc(alias = "BMX_CFG3")]
pub type BmxCfg3 = crate::Reg<bmx_cfg3::BmxCfg3Spec>;
#[doc = "BMX Bus Matrix Config 3 (Bus Error Clear/Status)"]
pub mod bmx_cfg3;
#[doc = "BMX_CFG4 (r) register accessor: BMX Bus Matrix Config 4 (Bus Error Source)\n\nYou can [`read`](crate::Reg::read) this register and get [`bmx_cfg4::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bmx_cfg4`] module"]
#[doc(alias = "BMX_CFG4")]
pub type BmxCfg4 = crate::Reg<bmx_cfg4::BmxCfg4Spec>;
#[doc = "BMX Bus Matrix Config 4 (Bus Error Source)"]
pub mod bmx_cfg4;
#[doc = "BMX_CFG5 (r) register accessor: BMX Bus Matrix Config 5 (BMX Bus Error Address)\n\nYou can [`read`](crate::Reg::read) this register and get [`bmx_cfg5::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bmx_cfg5`] module"]
#[doc(alias = "BMX_CFG5")]
pub type BmxCfg5 = crate::Reg<bmx_cfg5::BmxCfg5Spec>;
#[doc = "BMX Bus Matrix Config 5 (BMX Bus Error Address)"]
pub mod bmx_cfg5;
#[doc = "BMX_CFG6 (r) register accessor: BMX Bus Matrix Config 6 (MCU Bus Error Address)\n\nYou can [`read`](crate::Reg::read) this register and get [`bmx_cfg6::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bmx_cfg6`] module"]
#[doc(alias = "BMX_CFG6")]
pub type BmxCfg6 = crate::Reg<bmx_cfg6::BmxCfg6Spec>;
#[doc = "BMX Bus Matrix Config 6 (MCU Bus Error Address)"]
pub mod bmx_cfg6;
#[doc = "DBG_CFG0 (rw) register accessor: Debug Config 0 (Low-Low)\n\nYou can [`read`](crate::Reg::read) this register and get [`dbg_cfg0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbg_cfg0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dbg_cfg0`] module"]
#[doc(alias = "DBG_CFG0")]
pub type DbgCfg0 = crate::Reg<dbg_cfg0::DbgCfg0Spec>;
#[doc = "Debug Config 0 (Low-Low)"]
pub mod dbg_cfg0;
#[doc = "DBG_CFG1 (rw) register accessor: Debug Config 1 (Low-High)\n\nYou can [`read`](crate::Reg::read) this register and get [`dbg_cfg1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbg_cfg1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dbg_cfg1`] module"]
#[doc(alias = "DBG_CFG1")]
pub type DbgCfg1 = crate::Reg<dbg_cfg1::DbgCfg1Spec>;
#[doc = "Debug Config 1 (Low-High)"]
pub mod dbg_cfg1;
#[doc = "DBG_CFG2 (rw) register accessor: Debug Config 2 (High-Low)\n\nYou can [`read`](crate::Reg::read) this register and get [`dbg_cfg2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbg_cfg2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dbg_cfg2`] module"]
#[doc(alias = "DBG_CFG2")]
pub type DbgCfg2 = crate::Reg<dbg_cfg2::DbgCfg2Spec>;
#[doc = "Debug Config 2 (High-Low)"]
pub mod dbg_cfg2;
#[doc = "DBG_CFG3 (rw) register accessor: Debug Config 3 (High-High)\n\nYou can [`read`](crate::Reg::read) this register and get [`dbg_cfg3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbg_cfg3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dbg_cfg3`] module"]
#[doc(alias = "DBG_CFG3")]
pub type DbgCfg3 = crate::Reg<dbg_cfg3::DbgCfg3Spec>;
#[doc = "Debug Config 3 (High-High)"]
pub mod dbg_cfg3;
#[doc = "DBG_CFG4 (rw) register accessor: Debug Config 4 (Output Enable)\n\nYou can [`read`](crate::Reg::read) this register and get [`dbg_cfg4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbg_cfg4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dbg_cfg4`] module"]
#[doc(alias = "DBG_CFG4")]
pub type DbgCfg4 = crate::Reg<dbg_cfg4::DbgCfg4Spec>;
#[doc = "Debug Config 4 (Output Enable)"]
pub mod dbg_cfg4;
#[doc = "MBIST_CFG0 (rw) register accessor: MBIST Configuration 0\n\nYou can [`read`](crate::Reg::read) this register and get [`mbist_cfg0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mbist_cfg0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mbist_cfg0`] module"]
#[doc(alias = "MBIST_CFG0")]
pub type MbistCfg0 = crate::Reg<mbist_cfg0::MbistCfg0Spec>;
#[doc = "MBIST Configuration 0"]
pub mod mbist_cfg0;
#[doc = "AUDIO_CFG0 (rw) register accessor: Audio Clock Configuration 0 (PDM/ADC/DAC)\n\nYou can [`read`](crate::Reg::read) this register and get [`audio_cfg0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`audio_cfg0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@audio_cfg0`] module"]
#[doc(alias = "AUDIO_CFG0")]
pub type AudioCfg0 = crate::Reg<audio_cfg0::AudioCfg0Spec>;
#[doc = "Audio Clock Configuration 0 (PDM/ADC/DAC)"]
pub mod audio_cfg0;
#[doc = "AUDIO_CFG1 (rw) register accessor: Audio Clock Configuration 1 (PADC)\n\nYou can [`read`](crate::Reg::read) this register and get [`audio_cfg1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`audio_cfg1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@audio_cfg1`] module"]
#[doc(alias = "AUDIO_CFG1")]
pub type AudioCfg1 = crate::Reg<audio_cfg1::AudioCfg1Spec>;
#[doc = "Audio Clock Configuration 1 (PADC)"]
pub mod audio_cfg1;
#[doc = "ETH_CFG0 (rw) register accessor: Ethernet Clock Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`eth_cfg0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eth_cfg0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eth_cfg0`] module"]
#[doc(alias = "ETH_CFG0")]
pub type EthCfg0 = crate::Reg<eth_cfg0::EthCfg0Spec>;
#[doc = "Ethernet Clock Configuration"]
pub mod eth_cfg0;
#[doc = "CAM_CFG0 (rw) register accessor: Camera Clock Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`cam_cfg0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cam_cfg0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cam_cfg0`] module"]
#[doc(alias = "CAM_CFG0")]
pub type CamCfg0 = crate::Reg<cam_cfg0::CamCfg0Spec>;
#[doc = "Camera Clock Configuration"]
pub mod cam_cfg0;
#[doc = "SDH_CFG0 (rw) register accessor: SDH Clock Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`sdh_cfg0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdh_cfg0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdh_cfg0`] module"]
#[doc(alias = "SDH_CFG0")]
pub type SdhCfg0 = crate::Reg<sdh_cfg0::SdhCfg0Spec>;
#[doc = "SDH Clock Configuration"]
pub mod sdh_cfg0;
#[doc = "TZC_CFG0 (rw) register accessor: TrustZone Controller Config 0 (Lock Bits)\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_cfg0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tzc_cfg0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_cfg0`] module"]
#[doc(alias = "TZC_CFG0")]
pub type TzcCfg0 = crate::Reg<tzc_cfg0::TzcCfg0Spec>;
#[doc = "TrustZone Controller Config 0 (Lock Bits)"]
pub mod tzc_cfg0;
#[doc = "GLB_PARM_CFG0 (rw) register accessor: Global Parameter Config 0\n\nYou can [`read`](crate::Reg::read) this register and get [`glb_parm_cfg0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`glb_parm_cfg0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@glb_parm_cfg0`] module"]
#[doc(alias = "GLB_PARM_CFG0")]
pub type GlbParmCfg0 = crate::Reg<glb_parm_cfg0::GlbParmCfg0Spec>;
#[doc = "Global Parameter Config 0"]
pub mod glb_parm_cfg0;
#[doc = "DEBUG_CFG0 (rw) register accessor: Debug Configuration 0\n\nYou can [`read`](crate::Reg::read) this register and get [`debug_cfg0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`debug_cfg0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@debug_cfg0`] module"]
#[doc(alias = "DEBUG_CFG0")]
pub type DebugCfg0 = crate::Reg<debug_cfg0::DebugCfg0Spec>;
#[doc = "Debug Configuration 0"]
pub mod debug_cfg0;
#[doc = "DEBUG_CFG1 (rw) register accessor: Debug Configuration 1 (NDReset Gate)\n\nYou can [`read`](crate::Reg::read) this register and get [`debug_cfg1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`debug_cfg1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@debug_cfg1`] module"]
#[doc(alias = "DEBUG_CFG1")]
pub type DebugCfg1 = crate::Reg<debug_cfg1::DebugCfg1Spec>;
#[doc = "Debug Configuration 1 (NDReset Gate)"]
pub mod debug_cfg1;
#[doc = "RESET_STS0 (rw) register accessor: Reset Status Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`reset_sts0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reset_sts0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reset_sts0`] module"]
#[doc(alias = "RESET_STS0")]
pub type ResetSts0 = crate::Reg<reset_sts0::ResetSts0Spec>;
#[doc = "Reset Status Register 0"]
pub mod reset_sts0;
#[doc = "SWRST_S1_EXT (rw) register accessor: Software Reset Config 0 (EXT)\n\nYou can [`read`](crate::Reg::read) this register and get [`swrst_s1_ext::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swrst_s1_ext::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swrst_s1_ext`] module"]
#[doc(alias = "SWRST_S1_EXT")]
pub type SwrstS1Ext = crate::Reg<swrst_s1_ext::SwrstS1ExtSpec>;
#[doc = "Software Reset Config 0 (EXT)"]
pub mod swrst_s1_ext;
#[doc = "SWRST_S1 (rw) register accessor: Software Reset Config 1 (Peripherals)\n\nYou can [`read`](crate::Reg::read) this register and get [`swrst_s1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swrst_s1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swrst_s1`] module"]
#[doc(alias = "SWRST_S1")]
pub type SwrstS1 = crate::Reg<swrst_s1::SwrstS1Spec>;
#[doc = "Software Reset Config 1 (Peripherals)"]
pub mod swrst_s1;
#[doc = "SWRST_CFG2 (rw) register accessor: Software Reset Config 2 (CPU/Chip)\n\nYou can [`read`](crate::Reg::read) this register and get [`swrst_cfg2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swrst_cfg2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swrst_cfg2`] module"]
#[doc(alias = "SWRST_CFG2")]
pub type SwrstCfg2 = crate::Reg<swrst_cfg2::SwrstCfg2Spec>;
#[doc = "Software Reset Config 2 (CPU/Chip)"]
pub mod swrst_cfg2;
#[doc = "SWRST_CFG3 (rw) register accessor: HRESET Disable Config (SWRST_CFG3)\n\nYou can [`read`](crate::Reg::read) this register and get [`swrst_cfg3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swrst_cfg3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swrst_cfg3`] module"]
#[doc(alias = "SWRST_CFG3")]
pub type SwrstCfg3 = crate::Reg<swrst_cfg3::SwrstCfg3Spec>;
#[doc = "HRESET Disable Config (SWRST_CFG3)"]
pub mod swrst_cfg3;
#[doc = "CGEN_M (rw) register accessor: Clock Gate Config 0 (Core)\n\nYou can [`read`](crate::Reg::read) this register and get [`cgen_m::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cgen_m::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cgen_m`] module"]
#[doc(alias = "CGEN_M")]
pub type CgenM = crate::Reg<cgen_m::CgenMSpec>;
#[doc = "Clock Gate Config 0 (Core)"]
pub mod cgen_m;
#[doc = "CGEN_S1A (rw) register accessor: Clock Gate Config 1 (S1 Peripherals + S1A Peripherals)\n\nYou can [`read`](crate::Reg::read) this register and get [`cgen_s1a::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cgen_s1a::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cgen_s1a`] module"]
#[doc(alias = "CGEN_S1A")]
pub type CgenS1a = crate::Reg<cgen_s1a::CgenS1aSpec>;
#[doc = "Clock Gate Config 1 (S1 Peripherals + S1A Peripherals)"]
pub mod cgen_s1a;
#[doc = "CGEN_S1_EXT (rw) register accessor: Clock Gate Config 2 (S3 + S1_EXT Peripherals)\n\nYou can [`read`](crate::Reg::read) this register and get [`cgen_s1_ext::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cgen_s1_ext::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cgen_s1_ext`] module"]
#[doc(alias = "CGEN_S1_EXT")]
pub type CgenS1Ext = crate::Reg<cgen_s1_ext::CgenS1ExtSpec>;
#[doc = "Clock Gate Config 2 (S3 + S1_EXT Peripherals)"]
pub mod cgen_s1_ext;
#[doc = "CGEN_CFG3 (rw) register accessor: Clock Gate Config 3\n\nYou can [`read`](crate::Reg::read) this register and get [`cgen_cfg3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cgen_cfg3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cgen_cfg3`] module"]
#[doc(alias = "CGEN_CFG3")]
pub type CgenCfg3 = crate::Reg<cgen_cfg3::CgenCfg3Spec>;
#[doc = "Clock Gate Config 3"]
pub mod cgen_cfg3;
#[doc = "HW_RSV0 (rw) register accessor: Hardware Reserved Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`hw_rsv0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hw_rsv0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hw_rsv0`] module"]
#[doc(alias = "HW_RSV0")]
pub type HwRsv0 = crate::Reg<hw_rsv0::HwRsv0Spec>;
#[doc = "Hardware Reserved Register 0"]
pub mod hw_rsv0;
#[doc = "HW_RSV1 (rw) register accessor: Hardware Reserved Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`hw_rsv1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hw_rsv1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hw_rsv1`] module"]
#[doc(alias = "HW_RSV1")]
pub type HwRsv1 = crate::Reg<hw_rsv1::HwRsv1Spec>;
#[doc = "Hardware Reserved Register 1"]
pub mod hw_rsv1;
#[doc = "HW_RSV2 (rw) register accessor: Hardware Reserved Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`hw_rsv2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hw_rsv2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hw_rsv2`] module"]
#[doc(alias = "HW_RSV2")]
pub type HwRsv2 = crate::Reg<hw_rsv2::HwRsv2Spec>;
#[doc = "Hardware Reserved Register 2"]
pub mod hw_rsv2;
#[doc = "HW_RSV3 (rw) register accessor: Hardware Reserved Register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`hw_rsv3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hw_rsv3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hw_rsv3`] module"]
#[doc(alias = "HW_RSV3")]
pub type HwRsv3 = crate::Reg<hw_rsv3::HwRsv3Spec>;
#[doc = "Hardware Reserved Register 3"]
pub mod hw_rsv3;
#[doc = "SRAM_CFG0 (rw) register accessor: SRAM Configuration 0 - Retention Control\n\nYou can [`read`](crate::Reg::read) this register and get [`sram_cfg0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sram_cfg0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sram_cfg0`] module"]
#[doc(alias = "SRAM_CFG0")]
pub type SramCfg0 = crate::Reg<sram_cfg0::SramCfg0Spec>;
#[doc = "SRAM Configuration 0 - Retention Control"]
pub mod sram_cfg0;
#[doc = "SRAM_CFG1 (rw) register accessor: SRAM Configuration 1 - Sleep Control\n\nYou can [`read`](crate::Reg::read) this register and get [`sram_cfg1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sram_cfg1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sram_cfg1`] module"]
#[doc(alias = "SRAM_CFG1")]
pub type SramCfg1 = crate::Reg<sram_cfg1::SramCfg1Spec>;
#[doc = "SRAM Configuration 1 - Sleep Control"]
pub mod sram_cfg1;
#[doc = "SRAM_CFG2 (rw) register accessor: SRAM Configuration 2 - DVSE and NAP Control\n\nYou can [`read`](crate::Reg::read) this register and get [`sram_cfg2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sram_cfg2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sram_cfg2`] module"]
#[doc(alias = "SRAM_CFG2")]
pub type SramCfg2 = crate::Reg<sram_cfg2::SramCfg2Spec>;
#[doc = "SRAM Configuration 2 - DVSE and NAP Control"]
pub mod sram_cfg2;
#[doc = "SRAM_CFG3 (rw) register accessor: SRAM Configuration 3 - EM and VRAM Selection\n\nYou can [`read`](crate::Reg::read) this register and get [`sram_cfg3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sram_cfg3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sram_cfg3`] module"]
#[doc(alias = "SRAM_CFG3")]
pub type SramCfg3 = crate::Reg<sram_cfg3::SramCfg3Spec>;
#[doc = "SRAM Configuration 3 - EM and VRAM Selection"]
pub mod sram_cfg3;
#[doc = "SRAM_CFG4 (rw) register accessor: SRAM Configuration 4 - DVS Control\n\nYou can [`read`](crate::Reg::read) this register and get [`sram_cfg4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sram_cfg4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sram_cfg4`] module"]
#[doc(alias = "SRAM_CFG4")]
pub type SramCfg4 = crate::Reg<sram_cfg4::SramCfg4Spec>;
#[doc = "SRAM Configuration 4 - DVS Control"]
pub mod sram_cfg4;
#[doc = "PSRAM_CFG0 (rw) register accessor: PSRAM Clock Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`psram_cfg0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psram_cfg0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@psram_cfg0`] module"]
#[doc(alias = "PSRAM_CFG0")]
pub type PsramCfg0 = crate::Reg<psram_cfg0::PsramCfg0Spec>;
#[doc = "PSRAM Clock Configuration"]
pub mod psram_cfg0;
#[doc = "LDO28CIS (rw) register accessor: LDO 2.8V CIS Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`ldo28cis::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ldo28cis::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ldo28cis`] module"]
#[doc(alias = "LDO28CIS")]
pub type Ldo28cis = crate::Reg<ldo28cis::Ldo28cisSpec>;
#[doc = "LDO 2.8V CIS Configuration"]
pub mod ldo28cis;
#[doc = "LDO18IO (rw) register accessor: LDO 1.8V IO Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`ldo18io::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ldo18io::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ldo18io`] module"]
#[doc(alias = "LDO18IO")]
pub type Ldo18io = crate::Reg<ldo18io::Ldo18ioSpec>;
#[doc = "LDO 1.8V IO Configuration"]
pub mod ldo18io;
#[doc = "LDO15CIS (rw) register accessor: LDO 1.5V CIS Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`ldo15cis::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ldo15cis::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ldo15cis`] module"]
#[doc(alias = "LDO15CIS")]
pub type Ldo15cis = crate::Reg<ldo15cis::Ldo15cisSpec>;
#[doc = "LDO 1.5V CIS Configuration"]
pub mod ldo15cis;
#[doc = "LDO18FLASH (rw) register accessor: LDO 1.8V Flash Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`ldo18flash::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ldo18flash::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ldo18flash`] module"]
#[doc(alias = "LDO18FLASH")]
pub type Ldo18flash = crate::Reg<ldo18flash::Ldo18flashSpec>;
#[doc = "LDO 1.8V Flash Configuration"]
pub mod ldo18flash;
#[doc = "LDO12UHS (rw) register accessor: LDO 1.2V UHS Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`ldo12uhs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ldo12uhs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ldo12uhs`] module"]
#[doc(alias = "LDO12UHS")]
pub type Ldo12uhs = crate::Reg<ldo12uhs::Ldo12uhsSpec>;
#[doc = "LDO 1.2V UHS Configuration"]
pub mod ldo12uhs;
#[doc = "PROC_MON (rw) register accessor: Process Monitor Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`proc_mon::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`proc_mon::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@proc_mon`] module"]
#[doc(alias = "PROC_MON")]
pub type ProcMon = crate::Reg<proc_mon::ProcMonSpec>;
#[doc = "Process Monitor Configuration"]
pub mod proc_mon;
#[doc = "DLL_CFG0 (rw) register accessor: DLL Configuration Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`dll_cfg0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dll_cfg0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dll_cfg0`] module"]
#[doc(alias = "DLL_CFG0")]
pub type DllCfg0 = crate::Reg<dll_cfg0::DllCfg0Spec>;
#[doc = "DLL Configuration Register 0"]
pub mod dll_cfg0;
#[doc = "MIPI_PLL_CFG0 (rw) register accessor: MIPI PLL Configuration 0 - Power and Reset\n\nYou can [`read`](crate::Reg::read) this register and get [`mipi_pll_cfg0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mipi_pll_cfg0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mipi_pll_cfg0`] module"]
#[doc(alias = "MIPI_PLL_CFG0")]
pub type MipiPllCfg0 = crate::Reg<mipi_pll_cfg0::MipiPllCfg0Spec>;
#[doc = "MIPI PLL Configuration 0 - Power and Reset"]
pub mod mipi_pll_cfg0;
#[doc = "MIPI_PLL_CFG1 (rw) register accessor: MIPI PLL Configuration 1 - Divider and Clock Select\n\nYou can [`read`](crate::Reg::read) this register and get [`mipi_pll_cfg1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mipi_pll_cfg1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mipi_pll_cfg1`] module"]
#[doc(alias = "MIPI_PLL_CFG1")]
pub type MipiPllCfg1 = crate::Reg<mipi_pll_cfg1::MipiPllCfg1Spec>;
#[doc = "MIPI PLL Configuration 1 - Divider and Clock Select"]
pub mod mipi_pll_cfg1;
#[doc = "MIPI_PLL_CFG2 (rw) register accessor: MIPI PLL Configuration 2 - Charge Pump\n\nYou can [`read`](crate::Reg::read) this register and get [`mipi_pll_cfg2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mipi_pll_cfg2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mipi_pll_cfg2`] module"]
#[doc(alias = "MIPI_PLL_CFG2")]
pub type MipiPllCfg2 = crate::Reg<mipi_pll_cfg2::MipiPllCfg2Spec>;
#[doc = "MIPI PLL Configuration 2 - Charge Pump"]
pub mod mipi_pll_cfg2;
#[doc = "MIPI_PLL_CFG3 (rw) register accessor: MIPI PLL Configuration 3 - Loop Filter\n\nYou can [`read`](crate::Reg::read) this register and get [`mipi_pll_cfg3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mipi_pll_cfg3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mipi_pll_cfg3`] module"]
#[doc(alias = "MIPI_PLL_CFG3")]
pub type MipiPllCfg3 = crate::Reg<mipi_pll_cfg3::MipiPllCfg3Spec>;
#[doc = "MIPI PLL Configuration 3 - Loop Filter"]
pub mod mipi_pll_cfg3;
#[doc = "MIPI_PLL_CFG4 (rw) register accessor: MIPI PLL Configuration 4 - Lock Detector\n\nYou can [`read`](crate::Reg::read) this register and get [`mipi_pll_cfg4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mipi_pll_cfg4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mipi_pll_cfg4`] module"]
#[doc(alias = "MIPI_PLL_CFG4")]
pub type MipiPllCfg4 = crate::Reg<mipi_pll_cfg4::MipiPllCfg4Spec>;
#[doc = "MIPI PLL Configuration 4 - Lock Detector"]
pub mod mipi_pll_cfg4;
#[doc = "MIPI_PLL_CFG5 (rw) register accessor: MIPI PLL Configuration 5 - VCO\n\nYou can [`read`](crate::Reg::read) this register and get [`mipi_pll_cfg5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mipi_pll_cfg5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mipi_pll_cfg5`] module"]
#[doc(alias = "MIPI_PLL_CFG5")]
pub type MipiPllCfg5 = crate::Reg<mipi_pll_cfg5::MipiPllCfg5Spec>;
#[doc = "MIPI PLL Configuration 5 - VCO"]
pub mod mipi_pll_cfg5;
#[doc = "MIPI_PLL_CFG6 (rw) register accessor: MIPI PLL Configuration 6 - SDM\n\nYou can [`read`](crate::Reg::read) this register and get [`mipi_pll_cfg6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mipi_pll_cfg6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mipi_pll_cfg6`] module"]
#[doc(alias = "MIPI_PLL_CFG6")]
pub type MipiPllCfg6 = crate::Reg<mipi_pll_cfg6::MipiPllCfg6Spec>;
#[doc = "MIPI PLL Configuration 6 - SDM"]
pub mod mipi_pll_cfg6;
#[doc = "MIPI_PLL_CFG7 (rw) register accessor: MIPI PLL Configuration 7 - SDM Dither\n\nYou can [`read`](crate::Reg::read) this register and get [`mipi_pll_cfg7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mipi_pll_cfg7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mipi_pll_cfg7`] module"]
#[doc(alias = "MIPI_PLL_CFG7")]
pub type MipiPllCfg7 = crate::Reg<mipi_pll_cfg7::MipiPllCfg7Spec>;
#[doc = "MIPI PLL Configuration 7 - SDM Dither"]
pub mod mipi_pll_cfg7;
#[doc = "MIPI_PLL_CFG8 (rw) register accessor: MIPI PLL Configuration 8 - Test\n\nYou can [`read`](crate::Reg::read) this register and get [`mipi_pll_cfg8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mipi_pll_cfg8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mipi_pll_cfg8`] module"]
#[doc(alias = "MIPI_PLL_CFG8")]
pub type MipiPllCfg8 = crate::Reg<mipi_pll_cfg8::MipiPllCfg8Spec>;
#[doc = "MIPI PLL Configuration 8 - Test"]
pub mod mipi_pll_cfg8;
#[doc = "MIPI_PLL_CFG9 (rw) register accessor: MIPI PLL Configuration 9 - SSC\n\nYou can [`read`](crate::Reg::read) this register and get [`mipi_pll_cfg9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mipi_pll_cfg9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mipi_pll_cfg9`] module"]
#[doc(alias = "MIPI_PLL_CFG9")]
pub type MipiPllCfg9 = crate::Reg<mipi_pll_cfg9::MipiPllCfg9Spec>;
#[doc = "MIPI PLL Configuration 9 - SSC"]
pub mod mipi_pll_cfg9;
#[doc = "UHS_PLL_CFG0 (rw) register accessor: UHS PLL Configuration 0 - Power and Reset\n\nYou can [`read`](crate::Reg::read) this register and get [`uhs_pll_cfg0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uhs_pll_cfg0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uhs_pll_cfg0`] module"]
#[doc(alias = "UHS_PLL_CFG0")]
pub type UhsPllCfg0 = crate::Reg<uhs_pll_cfg0::UhsPllCfg0Spec>;
#[doc = "UHS PLL Configuration 0 - Power and Reset"]
pub mod uhs_pll_cfg0;
#[doc = "UHS_PLL_CFG1 (rw) register accessor: UHS PLL Configuration 1 - Divider and Clock Select\n\nYou can [`read`](crate::Reg::read) this register and get [`uhs_pll_cfg1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uhs_pll_cfg1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uhs_pll_cfg1`] module"]
#[doc(alias = "UHS_PLL_CFG1")]
pub type UhsPllCfg1 = crate::Reg<uhs_pll_cfg1::UhsPllCfg1Spec>;
#[doc = "UHS PLL Configuration 1 - Divider and Clock Select"]
pub mod uhs_pll_cfg1;
#[doc = "UHS_PLL_CFG2 (rw) register accessor: UHS PLL Configuration 2 - Charge Pump\n\nYou can [`read`](crate::Reg::read) this register and get [`uhs_pll_cfg2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uhs_pll_cfg2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uhs_pll_cfg2`] module"]
#[doc(alias = "UHS_PLL_CFG2")]
pub type UhsPllCfg2 = crate::Reg<uhs_pll_cfg2::UhsPllCfg2Spec>;
#[doc = "UHS PLL Configuration 2 - Charge Pump"]
pub mod uhs_pll_cfg2;
#[doc = "UHS_PLL_CFG3 (rw) register accessor: UHS PLL Configuration 3 - Loop Filter\n\nYou can [`read`](crate::Reg::read) this register and get [`uhs_pll_cfg3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uhs_pll_cfg3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uhs_pll_cfg3`] module"]
#[doc(alias = "UHS_PLL_CFG3")]
pub type UhsPllCfg3 = crate::Reg<uhs_pll_cfg3::UhsPllCfg3Spec>;
#[doc = "UHS PLL Configuration 3 - Loop Filter"]
pub mod uhs_pll_cfg3;
#[doc = "UHS_PLL_CFG4 (rw) register accessor: UHS PLL Configuration 4 - Lock Detector\n\nYou can [`read`](crate::Reg::read) this register and get [`uhs_pll_cfg4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uhs_pll_cfg4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uhs_pll_cfg4`] module"]
#[doc(alias = "UHS_PLL_CFG4")]
pub type UhsPllCfg4 = crate::Reg<uhs_pll_cfg4::UhsPllCfg4Spec>;
#[doc = "UHS PLL Configuration 4 - Lock Detector"]
pub mod uhs_pll_cfg4;
#[doc = "UHS_PLL_CFG5 (rw) register accessor: UHS PLL Configuration 5 - VCO\n\nYou can [`read`](crate::Reg::read) this register and get [`uhs_pll_cfg5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uhs_pll_cfg5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uhs_pll_cfg5`] module"]
#[doc(alias = "UHS_PLL_CFG5")]
pub type UhsPllCfg5 = crate::Reg<uhs_pll_cfg5::UhsPllCfg5Spec>;
#[doc = "UHS PLL Configuration 5 - VCO"]
pub mod uhs_pll_cfg5;
#[doc = "UHS_PLL_CFG6 (rw) register accessor: UHS PLL Configuration 6 - SDM\n\nYou can [`read`](crate::Reg::read) this register and get [`uhs_pll_cfg6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uhs_pll_cfg6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uhs_pll_cfg6`] module"]
#[doc(alias = "UHS_PLL_CFG6")]
pub type UhsPllCfg6 = crate::Reg<uhs_pll_cfg6::UhsPllCfg6Spec>;
#[doc = "UHS PLL Configuration 6 - SDM"]
pub mod uhs_pll_cfg6;
#[doc = "UHS_PLL_CFG7 (rw) register accessor: UHS PLL Configuration 7 - SDM Dither\n\nYou can [`read`](crate::Reg::read) this register and get [`uhs_pll_cfg7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uhs_pll_cfg7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uhs_pll_cfg7`] module"]
#[doc(alias = "UHS_PLL_CFG7")]
pub type UhsPllCfg7 = crate::Reg<uhs_pll_cfg7::UhsPllCfg7Spec>;
#[doc = "UHS PLL Configuration 7 - SDM Dither"]
pub mod uhs_pll_cfg7;
#[doc = "UHS_PLL_CFG8 (rw) register accessor: UHS PLL Configuration 8 - Test\n\nYou can [`read`](crate::Reg::read) this register and get [`uhs_pll_cfg8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uhs_pll_cfg8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uhs_pll_cfg8`] module"]
#[doc(alias = "UHS_PLL_CFG8")]
pub type UhsPllCfg8 = crate::Reg<uhs_pll_cfg8::UhsPllCfg8Spec>;
#[doc = "UHS PLL Configuration 8 - Test"]
pub mod uhs_pll_cfg8;
#[doc = "UHS_PLL_CFG9 (rw) register accessor: UHS PLL Configuration 9 - SSC\n\nYou can [`read`](crate::Reg::read) this register and get [`uhs_pll_cfg9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uhs_pll_cfg9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uhs_pll_cfg9`] module"]
#[doc(alias = "UHS_PLL_CFG9")]
pub type UhsPllCfg9 = crate::Reg<uhs_pll_cfg9::UhsPllCfg9Spec>;
#[doc = "UHS PLL Configuration 9 - SSC"]
pub mod uhs_pll_cfg9;
#[doc = "WIFI_PLL_CFG0 (rw) register accessor: WIFI PLL Configuration 0 - Power and Reset\n\nYou can [`read`](crate::Reg::read) this register and get [`wifi_pll_cfg0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wifi_pll_cfg0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wifi_pll_cfg0`] module"]
#[doc(alias = "WIFI_PLL_CFG0")]
pub type WifiPllCfg0 = crate::Reg<wifi_pll_cfg0::WifiPllCfg0Spec>;
#[doc = "WIFI PLL Configuration 0 - Power and Reset"]
pub mod wifi_pll_cfg0;
#[doc = "WIFI_PLL_CFG1 (rw) register accessor: WIFI PLL Configuration 1 - Divider and Clock Select\n\nYou can [`read`](crate::Reg::read) this register and get [`wifi_pll_cfg1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wifi_pll_cfg1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wifi_pll_cfg1`] module"]
#[doc(alias = "WIFI_PLL_CFG1")]
pub type WifiPllCfg1 = crate::Reg<wifi_pll_cfg1::WifiPllCfg1Spec>;
#[doc = "WIFI PLL Configuration 1 - Divider and Clock Select"]
pub mod wifi_pll_cfg1;
#[doc = "WIFI_PLL_CFG2 (rw) register accessor: WIFI PLL Configuration 2 - Charge Pump\n\nYou can [`read`](crate::Reg::read) this register and get [`wifi_pll_cfg2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wifi_pll_cfg2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wifi_pll_cfg2`] module"]
#[doc(alias = "WIFI_PLL_CFG2")]
pub type WifiPllCfg2 = crate::Reg<wifi_pll_cfg2::WifiPllCfg2Spec>;
#[doc = "WIFI PLL Configuration 2 - Charge Pump"]
pub mod wifi_pll_cfg2;
#[doc = "WIFI_PLL_CFG3 (rw) register accessor: WIFI PLL Configuration 3 - Loop Filter\n\nYou can [`read`](crate::Reg::read) this register and get [`wifi_pll_cfg3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wifi_pll_cfg3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wifi_pll_cfg3`] module"]
#[doc(alias = "WIFI_PLL_CFG3")]
pub type WifiPllCfg3 = crate::Reg<wifi_pll_cfg3::WifiPllCfg3Spec>;
#[doc = "WIFI PLL Configuration 3 - Loop Filter"]
pub mod wifi_pll_cfg3;
#[doc = "WIFI_PLL_CFG4 (rw) register accessor: WIFI PLL Configuration 4 - Sample Clock\n\nYou can [`read`](crate::Reg::read) this register and get [`wifi_pll_cfg4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wifi_pll_cfg4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wifi_pll_cfg4`] module"]
#[doc(alias = "WIFI_PLL_CFG4")]
pub type WifiPllCfg4 = crate::Reg<wifi_pll_cfg4::WifiPllCfg4Spec>;
#[doc = "WIFI PLL Configuration 4 - Sample Clock"]
pub mod wifi_pll_cfg4;
#[doc = "WIFI_PLL_CFG5 (rw) register accessor: WIFI PLL Configuration 5 - VCO\n\nYou can [`read`](crate::Reg::read) this register and get [`wifi_pll_cfg5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wifi_pll_cfg5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wifi_pll_cfg5`] module"]
#[doc(alias = "WIFI_PLL_CFG5")]
pub type WifiPllCfg5 = crate::Reg<wifi_pll_cfg5::WifiPllCfg5Spec>;
#[doc = "WIFI PLL Configuration 5 - VCO"]
pub mod wifi_pll_cfg5;
#[doc = "WIFI_PLL_CFG6 (rw) register accessor: WIFI PLL Configuration 6 - SDM Input\n\nYou can [`read`](crate::Reg::read) this register and get [`wifi_pll_cfg6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wifi_pll_cfg6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wifi_pll_cfg6`] module"]
#[doc(alias = "WIFI_PLL_CFG6")]
pub type WifiPllCfg6 = crate::Reg<wifi_pll_cfg6::WifiPllCfg6Spec>;
#[doc = "WIFI PLL Configuration 6 - SDM Input"]
pub mod wifi_pll_cfg6;
#[doc = "WIFI_PLL_CFG7 (rw) register accessor: WIFI PLL Configuration 7 - SDM Dither\n\nYou can [`read`](crate::Reg::read) this register and get [`wifi_pll_cfg7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wifi_pll_cfg7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wifi_pll_cfg7`] module"]
#[doc(alias = "WIFI_PLL_CFG7")]
pub type WifiPllCfg7 = crate::Reg<wifi_pll_cfg7::WifiPllCfg7Spec>;
#[doc = "WIFI PLL Configuration 7 - SDM Dither"]
pub mod wifi_pll_cfg7;
#[doc = "WIFI_PLL_CFG8 (rw) register accessor: WIFI PLL Configuration 8 - Output Divider Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`wifi_pll_cfg8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wifi_pll_cfg8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wifi_pll_cfg8`] module"]
#[doc(alias = "WIFI_PLL_CFG8")]
pub type WifiPllCfg8 = crate::Reg<wifi_pll_cfg8::WifiPllCfg8Spec>;
#[doc = "WIFI PLL Configuration 8 - Output Divider Enable"]
pub mod wifi_pll_cfg8;
#[doc = "WIFI_PLL_CFG9 (rw) register accessor: WIFI PLL Configuration 9 - Test and Debug\n\nYou can [`read`](crate::Reg::read) this register and get [`wifi_pll_cfg9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wifi_pll_cfg9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wifi_pll_cfg9`] module"]
#[doc(alias = "WIFI_PLL_CFG9")]
pub type WifiPllCfg9 = crate::Reg<wifi_pll_cfg9::WifiPllCfg9Spec>;
#[doc = "WIFI PLL Configuration 9 - Test and Debug"]
pub mod wifi_pll_cfg9;
#[doc = "WIFI_PLL_CFG10 (rw) register accessor: WIFI PLL Configuration 10 - USB PLL SSC\n\nYou can [`read`](crate::Reg::read) this register and get [`wifi_pll_cfg10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wifi_pll_cfg10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wifi_pll_cfg10`] module"]
#[doc(alias = "WIFI_PLL_CFG10")]
pub type WifiPllCfg10 = crate::Reg<wifi_pll_cfg10::WifiPllCfg10Spec>;
#[doc = "WIFI PLL Configuration 10 - USB PLL SSC"]
pub mod wifi_pll_cfg10;
#[doc = "WIFI_PLL_CFG11 (rw) register accessor: WIFI PLL Configuration 11 - USB PLL SDM\n\nYou can [`read`](crate::Reg::read) this register and get [`wifi_pll_cfg11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wifi_pll_cfg11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wifi_pll_cfg11`] module"]
#[doc(alias = "WIFI_PLL_CFG11")]
pub type WifiPllCfg11 = crate::Reg<wifi_pll_cfg11::WifiPllCfg11Spec>;
#[doc = "WIFI PLL Configuration 11 - USB PLL SDM"]
pub mod wifi_pll_cfg11;
#[doc = "WIFI_PLL_CFG12 (rw) register accessor: WIFI PLL Configuration 12 - USB PLL SSC Count\n\nYou can [`read`](crate::Reg::read) this register and get [`wifi_pll_cfg12::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wifi_pll_cfg12::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wifi_pll_cfg12`] module"]
#[doc(alias = "WIFI_PLL_CFG12")]
pub type WifiPllCfg12 = crate::Reg<wifi_pll_cfg12::WifiPllCfg12Spec>;
#[doc = "WIFI PLL Configuration 12 - USB PLL SSC Count"]
pub mod wifi_pll_cfg12;
#[doc = "WIFI_PLL_CFG13 (rw) register accessor: WIFI PLL Configuration 13 - Reserved and DL Control\n\nYou can [`read`](crate::Reg::read) this register and get [`wifi_pll_cfg13::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wifi_pll_cfg13::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wifi_pll_cfg13`] module"]
#[doc(alias = "WIFI_PLL_CFG13")]
pub type WifiPllCfg13 = crate::Reg<wifi_pll_cfg13::WifiPllCfg13Spec>;
#[doc = "WIFI PLL Configuration 13 - Reserved and DL Control"]
pub mod wifi_pll_cfg13;
#[doc = "GAUGE (rw) register accessor: Gauge ADC Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`gauge::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gauge::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gauge`] module"]
#[doc(alias = "GAUGE")]
pub type Gauge = crate::Reg<gauge::GaugeSpec>;
#[doc = "Gauge ADC Configuration"]
pub mod gauge;
#[doc = "GAUGE_RX_FIFO_CTRL (rw) register accessor: Gauge RX FIFO Control\n\nYou can [`read`](crate::Reg::read) this register and get [`gauge_rx_fifo_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gauge_rx_fifo_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gauge_rx_fifo_ctrl`] module"]
#[doc(alias = "GAUGE_RX_FIFO_CTRL")]
pub type GaugeRxFifoCtrl = crate::Reg<gauge_rx_fifo_ctrl::GaugeRxFifoCtrlSpec>;
#[doc = "Gauge RX FIFO Control"]
pub mod gauge_rx_fifo_ctrl;
#[doc = "GAUGE_RX_FIFO_STATUS (rw) register accessor: Gauge RX FIFO Status\n\nYou can [`read`](crate::Reg::read) this register and get [`gauge_rx_fifo_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gauge_rx_fifo_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gauge_rx_fifo_status`] module"]
#[doc(alias = "GAUGE_RX_FIFO_STATUS")]
pub type GaugeRxFifoStatus = crate::Reg<gauge_rx_fifo_status::GaugeRxFifoStatusSpec>;
#[doc = "Gauge RX FIFO Status"]
pub mod gauge_rx_fifo_status;
#[doc = "GAUGE_RX_FIFO_DATA (rw) register accessor: Gauge RX FIFO Data\n\nYou can [`read`](crate::Reg::read) this register and get [`gauge_rx_fifo_data::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gauge_rx_fifo_data::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gauge_rx_fifo_data`] module"]
#[doc(alias = "GAUGE_RX_FIFO_DATA")]
pub type GaugeRxFifoData = crate::Reg<gauge_rx_fifo_data::GaugeRxFifoDataSpec>;
#[doc = "Gauge RX FIFO Data"]
pub mod gauge_rx_fifo_data;
#[doc = "Configuration for GPIO pin %s"]
pub use self::pin_cfg::PinCfg;
#[doc = r"Cluster"]
#[doc = "Configuration for GPIO pin %s"]
pub mod pin_cfg;
#[doc = "GPIO_CFG128 (r) register accessor: Alternative GPIO Input Readback (Pins 0-31)\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio_cfg128::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio_cfg128`] module"]
#[doc(alias = "GPIO_CFG128")]
pub type GpioCfg128 = crate::Reg<gpio_cfg128::GpioCfg128Spec>;
#[doc = "Alternative GPIO Input Readback (Pins 0-31)"]
pub mod gpio_cfg128;
#[doc = "GPIO_CFG129 (r) register accessor: Alternative GPIO Input Readback (Pins 32-63)\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio_cfg129::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio_cfg129`] module"]
#[doc(alias = "GPIO_CFG129")]
pub type GpioCfg129 = crate::Reg<gpio_cfg129::GpioCfg129Spec>;
#[doc = "Alternative GPIO Input Readback (Pins 32-63)"]
pub mod gpio_cfg129;
#[doc = "GPIO_CFG136 (rw) register accessor: Alternative GPIO Output (Pins 0-31)\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio_cfg136::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio_cfg136::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio_cfg136`] module"]
#[doc(alias = "GPIO_CFG136")]
pub type GpioCfg136 = crate::Reg<gpio_cfg136::GpioCfg136Spec>;
#[doc = "Alternative GPIO Output (Pins 0-31)"]
pub mod gpio_cfg136;
#[doc = "GPIO_CFG137 (rw) register accessor: Alternative GPIO Output (Pins 32-63)\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio_cfg137::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio_cfg137::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio_cfg137`] module"]
#[doc(alias = "GPIO_CFG137")]
pub type GpioCfg137 = crate::Reg<gpio_cfg137::GpioCfg137Spec>;
#[doc = "Alternative GPIO Output (Pins 32-63)"]
pub mod gpio_cfg137;
#[doc = "GPIO_CFG138 (w) register accessor: Alternative GPIO Atomic Set (Pins 0-31)\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio_cfg138::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio_cfg138`] module"]
#[doc(alias = "GPIO_CFG138")]
pub type GpioCfg138 = crate::Reg<gpio_cfg138::GpioCfg138Spec>;
#[doc = "Alternative GPIO Atomic Set (Pins 0-31)"]
pub mod gpio_cfg138;
#[doc = "GPIO_CFG139 (w) register accessor: Alternative GPIO Atomic Set (Pins 32-63)\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio_cfg139::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio_cfg139`] module"]
#[doc(alias = "GPIO_CFG139")]
pub type GpioCfg139 = crate::Reg<gpio_cfg139::GpioCfg139Spec>;
#[doc = "Alternative GPIO Atomic Set (Pins 32-63)"]
pub mod gpio_cfg139;
#[doc = "GPIO_CFG140 (w) register accessor: Alternative GPIO Atomic Clear (Pins 0-31)\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio_cfg140::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio_cfg140`] module"]
#[doc(alias = "GPIO_CFG140")]
pub type GpioCfg140 = crate::Reg<gpio_cfg140::GpioCfg140Spec>;
#[doc = "Alternative GPIO Atomic Clear (Pins 0-31)"]
pub mod gpio_cfg140;
#[doc = "GPIO_CFG141 (w) register accessor: Alternative GPIO Atomic Clear (Pins 32-63)\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio_cfg141::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio_cfg141`] module"]
#[doc(alias = "GPIO_CFG141")]
pub type GpioCfg141 = crate::Reg<gpio_cfg141::GpioCfg141Spec>;
#[doc = "Alternative GPIO Atomic Clear (Pins 32-63)"]
pub mod gpio_cfg141;
#[doc = "GPIO_CFG142 (rw) register accessor: GPIO TX Protocol Engine Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio_cfg142::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio_cfg142::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio_cfg142`] module"]
#[doc(alias = "GPIO_CFG142")]
pub type GpioCfg142 = crate::Reg<gpio_cfg142::GpioCfg142Spec>;
#[doc = "GPIO TX Protocol Engine Configuration"]
pub mod gpio_cfg142;
#[doc = "GPIO_CFG143 (rw) register accessor: GPIO DMA and FIFO Control\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio_cfg143::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio_cfg143::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio_cfg143`] module"]
#[doc(alias = "GPIO_CFG143")]
pub type GpioCfg143 = crate::Reg<gpio_cfg143::GpioCfg143Spec>;
#[doc = "GPIO DMA and FIFO Control"]
pub mod gpio_cfg143;
#[doc = "GPIO_CFG144 (w) register accessor: GPIO DMA FIFO Write Data\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio_cfg144::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio_cfg144`] module"]
#[doc(alias = "GPIO_CFG144")]
pub type GpioCfg144 = crate::Reg<gpio_cfg144::GpioCfg144Spec>;
#[doc = "GPIO DMA FIFO Write Data"]
pub mod gpio_cfg144;
