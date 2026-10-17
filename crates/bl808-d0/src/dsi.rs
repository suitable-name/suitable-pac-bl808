#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    dsi_config: DsiConfig,
    dsi_esc_config: DsiEscConfig,
    dsi_lpdt_tx_config: DsiLpdtTxConfig,
    dsi_hstx_config: DsiHstxConfig,
    dsi_int_status: DsiIntStatus,
    dsi_int_mask: DsiIntMask,
    dsi_int_clear: DsiIntClear,
    dsi_int_enable: DsiIntEnable,
    dsi_frame_num: DsiFrameNum,
    _reserved9: [u8; 0x3c],
    dsi_fifo_config_0: DsiFifoConfig0,
    dsi_fifo_config_1: DsiFifoConfig1,
    dsi_fifo_wdata: DsiFifoWdata,
    dsi_fifo_rdata: DsiFifoRdata,
    _reserved13: [u8; 0x10],
    dphy_config_0: DphyConfig0,
    dphy_config_1: DphyConfig1,
    dphy_config_2: DphyConfig2,
    dphy_config_3: DphyConfig3,
    dphy_config_4: DphyConfig4,
    dphy_config_5: DphyConfig5,
    dphy_config_6: DphyConfig6,
    dphy_config_7: DphyConfig7,
    dphy_config_8: DphyConfig8,
    dphy_config_9: DphyConfig9,
    dphy_config_10: DphyConfig10,
    dphy_config_11: DphyConfig11,
    dphy_config_12: DphyConfig12,
    dphy_config_13: DphyConfig13,
    dphy_config_14: DphyConfig14,
    dphy_config_15: DphyConfig15,
    dphy_config_16: DphyConfig16,
    _reserved30: [u8; 0x38],
    dummy_reg: DummyReg,
}
impl RegisterBlock {
    #[doc = "0x00 - DSI Main Configuration Register"]
    #[inline(always)]
    pub const fn dsi_config(&self) -> &DsiConfig {
        &self.dsi_config
    }
    #[doc = "0x04 - Escape Mode Configuration"]
    #[inline(always)]
    pub const fn dsi_esc_config(&self) -> &DsiEscConfig {
        &self.dsi_esc_config
    }
    #[doc = "0x08 - Low Power Data Transmission Configuration"]
    #[inline(always)]
    pub const fn dsi_lpdt_tx_config(&self) -> &DsiLpdtTxConfig {
        &self.dsi_lpdt_tx_config
    }
    #[doc = "0x0c - High Speed Transmission Configuration"]
    #[inline(always)]
    pub const fn dsi_hstx_config(&self) -> &DsiHstxConfig {
        &self.dsi_hstx_config
    }
    #[doc = "0x10 - DSI Interrupt Status"]
    #[inline(always)]
    pub const fn dsi_int_status(&self) -> &DsiIntStatus {
        &self.dsi_int_status
    }
    #[doc = "0x14 - DSI Interrupt Mask"]
    #[inline(always)]
    pub const fn dsi_int_mask(&self) -> &DsiIntMask {
        &self.dsi_int_mask
    }
    #[doc = "0x18 - DSI Interrupt Clear"]
    #[inline(always)]
    pub const fn dsi_int_clear(&self) -> &DsiIntClear {
        &self.dsi_int_clear
    }
    #[doc = "0x1c - DSI Interrupt Enable"]
    #[inline(always)]
    pub const fn dsi_int_enable(&self) -> &DsiIntEnable {
        &self.dsi_int_enable
    }
    #[doc = "0x20 - DSI Frame Number Configuration"]
    #[inline(always)]
    pub const fn dsi_frame_num(&self) -> &DsiFrameNum {
        &self.dsi_frame_num
    }
    #[doc = "0x60 - FIFO Configuration 0"]
    #[inline(always)]
    pub const fn dsi_fifo_config_0(&self) -> &DsiFifoConfig0 {
        &self.dsi_fifo_config_0
    }
    #[doc = "0x64 - FIFO Configuration 1 (Counters and Thresholds)"]
    #[inline(always)]
    pub const fn dsi_fifo_config_1(&self) -> &DsiFifoConfig1 {
        &self.dsi_fifo_config_1
    }
    #[doc = "0x68 - FIFO Write Data"]
    #[inline(always)]
    pub const fn dsi_fifo_wdata(&self) -> &DsiFifoWdata {
        &self.dsi_fifo_wdata
    }
    #[doc = "0x6c - FIFO Read Data"]
    #[inline(always)]
    pub const fn dsi_fifo_rdata(&self) -> &DsiFifoRdata {
        &self.dsi_fifo_rdata
    }
    #[doc = "0x80 - DPHY Configuration 0"]
    #[inline(always)]
    pub const fn dphy_config_0(&self) -> &DphyConfig0 {
        &self.dphy_config_0
    }
    #[doc = "0x84 - DPHY Clock Lane Timing"]
    #[inline(always)]
    pub const fn dphy_config_1(&self) -> &DphyConfig1 {
        &self.dphy_config_1
    }
    #[doc = "0x88 - DPHY Data Lane Timing 0"]
    #[inline(always)]
    pub const fn dphy_config_2(&self) -> &DphyConfig2 {
        &self.dphy_config_2
    }
    #[doc = "0x8c - DPHY Data Lane Timing 1"]
    #[inline(always)]
    pub const fn dphy_config_3(&self) -> &DphyConfig3 {
        &self.dphy_config_3
    }
    #[doc = "0x90 - DPHY Wakeup Timing"]
    #[inline(always)]
    pub const fn dphy_config_4(&self) -> &DphyConfig4 {
        &self.dphy_config_4
    }
    #[doc = "0x94 - DPHY Trigger Code Configuration"]
    #[inline(always)]
    pub const fn dphy_config_5(&self) -> &DphyConfig5 {
        &self.dphy_config_5
    }
    #[doc = "0x98 - DPHY ULPS and LPDT Code Configuration"]
    #[inline(always)]
    pub const fn dphy_config_6(&self) -> &DphyConfig6 {
        &self.dphy_config_6
    }
    #[doc = "0x9c - DPHY Analog and Force Control"]
    #[inline(always)]
    pub const fn dphy_config_7(&self) -> &DphyConfig7 {
        &self.dphy_config_7
    }
    #[doc = "0xa0 - DPHY Analog TX/RX Control"]
    #[inline(always)]
    pub const fn dphy_config_8(&self) -> &DphyConfig8 {
        &self.dphy_config_8
    }
    #[doc = "0xa4 - DPHY HS Data Output"]
    #[inline(always)]
    pub const fn dphy_config_9(&self) -> &DphyConfig9 {
        &self.dphy_config_9
    }
    #[doc = "0xa8 - DPHY Impedance and Voltage Trim"]
    #[inline(always)]
    pub const fn dphy_config_10(&self) -> &DphyConfig10 {
        &self.dphy_config_10
    }
    #[doc = "0xac - DPHY Analog Control 0"]
    #[inline(always)]
    pub const fn dphy_config_11(&self) -> &DphyConfig11 {
        &self.dphy_config_11
    }
    #[doc = "0xb0 - DPHY Analog Control 1"]
    #[inline(always)]
    pub const fn dphy_config_12(&self) -> &DphyConfig12 {
        &self.dphy_config_12
    }
    #[doc = "0xb4 - DPHY Digital Test Bus Readout"]
    #[inline(always)]
    pub const fn dphy_config_13(&self) -> &DphyConfig13 {
        &self.dphy_config_13
    }
    #[doc = "0xb8 - DPHY Pattern Configuration"]
    #[inline(always)]
    pub const fn dphy_config_14(&self) -> &DphyConfig14 {
        &self.dphy_config_14
    }
    #[doc = "0xbc - DPHY Power and Reset Control"]
    #[inline(always)]
    pub const fn dphy_config_15(&self) -> &DphyConfig15 {
        &self.dphy_config_15
    }
    #[doc = "0xc0 - DPHY Clock Inversion Control"]
    #[inline(always)]
    pub const fn dphy_config_16(&self) -> &DphyConfig16 {
        &self.dphy_config_16
    }
    #[doc = "0xfc - DSI Dummy Register"]
    #[inline(always)]
    pub const fn dummy_reg(&self) -> &DummyReg {
        &self.dummy_reg
    }
}
#[doc = "DSI_CONFIG (rw) register accessor: DSI Main Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dsi_config::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsi_config::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsi_config`] module"]
#[doc(alias = "DSI_CONFIG")]
pub type DsiConfig = crate::Reg<dsi_config::DsiConfigSpec>;
#[doc = "DSI Main Configuration Register"]
pub mod dsi_config;
#[doc = "DSI_ESC_CONFIG (rw) register accessor: Escape Mode Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`dsi_esc_config::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsi_esc_config::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsi_esc_config`] module"]
#[doc(alias = "DSI_ESC_CONFIG")]
pub type DsiEscConfig = crate::Reg<dsi_esc_config::DsiEscConfigSpec>;
#[doc = "Escape Mode Configuration"]
pub mod dsi_esc_config;
#[doc = "DSI_LPDT_TX_CONFIG (rw) register accessor: Low Power Data Transmission Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`dsi_lpdt_tx_config::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsi_lpdt_tx_config::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsi_lpdt_tx_config`] module"]
#[doc(alias = "DSI_LPDT_TX_CONFIG")]
pub type DsiLpdtTxConfig = crate::Reg<dsi_lpdt_tx_config::DsiLpdtTxConfigSpec>;
#[doc = "Low Power Data Transmission Configuration"]
pub mod dsi_lpdt_tx_config;
#[doc = "DSI_HSTX_CONFIG (rw) register accessor: High Speed Transmission Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`dsi_hstx_config::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsi_hstx_config::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsi_hstx_config`] module"]
#[doc(alias = "DSI_HSTX_CONFIG")]
pub type DsiHstxConfig = crate::Reg<dsi_hstx_config::DsiHstxConfigSpec>;
#[doc = "High Speed Transmission Configuration"]
pub mod dsi_hstx_config;
#[doc = "DSI_INT_STATUS (r) register accessor: DSI Interrupt Status\n\nYou can [`read`](crate::Reg::read) this register and get [`dsi_int_status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsi_int_status`] module"]
#[doc(alias = "DSI_INT_STATUS")]
pub type DsiIntStatus = crate::Reg<dsi_int_status::DsiIntStatusSpec>;
#[doc = "DSI Interrupt Status"]
pub mod dsi_int_status;
#[doc = "DSI_INT_MASK (rw) register accessor: DSI Interrupt Mask\n\nYou can [`read`](crate::Reg::read) this register and get [`dsi_int_mask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsi_int_mask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsi_int_mask`] module"]
#[doc(alias = "DSI_INT_MASK")]
pub type DsiIntMask = crate::Reg<dsi_int_mask::DsiIntMaskSpec>;
#[doc = "DSI Interrupt Mask"]
pub mod dsi_int_mask;
#[doc = "DSI_INT_CLEAR (w) register accessor: DSI Interrupt Clear\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsi_int_clear::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsi_int_clear`] module"]
#[doc(alias = "DSI_INT_CLEAR")]
pub type DsiIntClear = crate::Reg<dsi_int_clear::DsiIntClearSpec>;
#[doc = "DSI Interrupt Clear"]
pub mod dsi_int_clear;
#[doc = "DSI_INT_ENABLE (rw) register accessor: DSI Interrupt Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`dsi_int_enable::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsi_int_enable::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsi_int_enable`] module"]
#[doc(alias = "DSI_INT_ENABLE")]
pub type DsiIntEnable = crate::Reg<dsi_int_enable::DsiIntEnableSpec>;
#[doc = "DSI Interrupt Enable"]
pub mod dsi_int_enable;
#[doc = "DSI_FRAME_NUM (rw) register accessor: DSI Frame Number Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`dsi_frame_num::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsi_frame_num::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsi_frame_num`] module"]
#[doc(alias = "DSI_FRAME_NUM")]
pub type DsiFrameNum = crate::Reg<dsi_frame_num::DsiFrameNumSpec>;
#[doc = "DSI Frame Number Configuration"]
pub mod dsi_frame_num;
#[doc = "DSI_FIFO_CONFIG_0 (rw) register accessor: FIFO Configuration 0\n\nYou can [`read`](crate::Reg::read) this register and get [`dsi_fifo_config_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsi_fifo_config_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsi_fifo_config_0`] module"]
#[doc(alias = "DSI_FIFO_CONFIG_0")]
pub type DsiFifoConfig0 = crate::Reg<dsi_fifo_config_0::DsiFifoConfig0Spec>;
#[doc = "FIFO Configuration 0"]
pub mod dsi_fifo_config_0;
#[doc = "DSI_FIFO_CONFIG_1 (rw) register accessor: FIFO Configuration 1 (Counters and Thresholds)\n\nYou can [`read`](crate::Reg::read) this register and get [`dsi_fifo_config_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsi_fifo_config_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsi_fifo_config_1`] module"]
#[doc(alias = "DSI_FIFO_CONFIG_1")]
pub type DsiFifoConfig1 = crate::Reg<dsi_fifo_config_1::DsiFifoConfig1Spec>;
#[doc = "FIFO Configuration 1 (Counters and Thresholds)"]
pub mod dsi_fifo_config_1;
#[doc = "DSI_FIFO_WDATA (w) register accessor: FIFO Write Data\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsi_fifo_wdata::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsi_fifo_wdata`] module"]
#[doc(alias = "DSI_FIFO_WDATA")]
pub type DsiFifoWdata = crate::Reg<dsi_fifo_wdata::DsiFifoWdataSpec>;
#[doc = "FIFO Write Data"]
pub mod dsi_fifo_wdata;
#[doc = "DSI_FIFO_RDATA (r) register accessor: FIFO Read Data\n\nYou can [`read`](crate::Reg::read) this register and get [`dsi_fifo_rdata::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsi_fifo_rdata`] module"]
#[doc(alias = "DSI_FIFO_RDATA")]
pub type DsiFifoRdata = crate::Reg<dsi_fifo_rdata::DsiFifoRdataSpec>;
#[doc = "FIFO Read Data"]
pub mod dsi_fifo_rdata;
#[doc = "DPHY_CONFIG_0 (rw) register accessor: DPHY Configuration 0\n\nYou can [`read`](crate::Reg::read) this register and get [`dphy_config_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dphy_config_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dphy_config_0`] module"]
#[doc(alias = "DPHY_CONFIG_0")]
pub type DphyConfig0 = crate::Reg<dphy_config_0::DphyConfig0Spec>;
#[doc = "DPHY Configuration 0"]
pub mod dphy_config_0;
#[doc = "DPHY_CONFIG_1 (rw) register accessor: DPHY Clock Lane Timing\n\nYou can [`read`](crate::Reg::read) this register and get [`dphy_config_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dphy_config_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dphy_config_1`] module"]
#[doc(alias = "DPHY_CONFIG_1")]
pub type DphyConfig1 = crate::Reg<dphy_config_1::DphyConfig1Spec>;
#[doc = "DPHY Clock Lane Timing"]
pub mod dphy_config_1;
#[doc = "DPHY_CONFIG_2 (rw) register accessor: DPHY Data Lane Timing 0\n\nYou can [`read`](crate::Reg::read) this register and get [`dphy_config_2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dphy_config_2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dphy_config_2`] module"]
#[doc(alias = "DPHY_CONFIG_2")]
pub type DphyConfig2 = crate::Reg<dphy_config_2::DphyConfig2Spec>;
#[doc = "DPHY Data Lane Timing 0"]
pub mod dphy_config_2;
#[doc = "DPHY_CONFIG_3 (rw) register accessor: DPHY Data Lane Timing 1\n\nYou can [`read`](crate::Reg::read) this register and get [`dphy_config_3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dphy_config_3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dphy_config_3`] module"]
#[doc(alias = "DPHY_CONFIG_3")]
pub type DphyConfig3 = crate::Reg<dphy_config_3::DphyConfig3Spec>;
#[doc = "DPHY Data Lane Timing 1"]
pub mod dphy_config_3;
#[doc = "DPHY_CONFIG_4 (rw) register accessor: DPHY Wakeup Timing\n\nYou can [`read`](crate::Reg::read) this register and get [`dphy_config_4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dphy_config_4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dphy_config_4`] module"]
#[doc(alias = "DPHY_CONFIG_4")]
pub type DphyConfig4 = crate::Reg<dphy_config_4::DphyConfig4Spec>;
#[doc = "DPHY Wakeup Timing"]
pub mod dphy_config_4;
#[doc = "DPHY_CONFIG_5 (rw) register accessor: DPHY Trigger Code Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`dphy_config_5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dphy_config_5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dphy_config_5`] module"]
#[doc(alias = "DPHY_CONFIG_5")]
pub type DphyConfig5 = crate::Reg<dphy_config_5::DphyConfig5Spec>;
#[doc = "DPHY Trigger Code Configuration"]
pub mod dphy_config_5;
#[doc = "DPHY_CONFIG_6 (rw) register accessor: DPHY ULPS and LPDT Code Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`dphy_config_6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dphy_config_6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dphy_config_6`] module"]
#[doc(alias = "DPHY_CONFIG_6")]
pub type DphyConfig6 = crate::Reg<dphy_config_6::DphyConfig6Spec>;
#[doc = "DPHY ULPS and LPDT Code Configuration"]
pub mod dphy_config_6;
#[doc = "DPHY_CONFIG_7 (rw) register accessor: DPHY Analog and Force Control\n\nYou can [`read`](crate::Reg::read) this register and get [`dphy_config_7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dphy_config_7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dphy_config_7`] module"]
#[doc(alias = "DPHY_CONFIG_7")]
pub type DphyConfig7 = crate::Reg<dphy_config_7::DphyConfig7Spec>;
#[doc = "DPHY Analog and Force Control"]
pub mod dphy_config_7;
#[doc = "DPHY_CONFIG_8 (rw) register accessor: DPHY Analog TX/RX Control\n\nYou can [`read`](crate::Reg::read) this register and get [`dphy_config_8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dphy_config_8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dphy_config_8`] module"]
#[doc(alias = "DPHY_CONFIG_8")]
pub type DphyConfig8 = crate::Reg<dphy_config_8::DphyConfig8Spec>;
#[doc = "DPHY Analog TX/RX Control"]
pub mod dphy_config_8;
#[doc = "DPHY_CONFIG_9 (rw) register accessor: DPHY HS Data Output\n\nYou can [`read`](crate::Reg::read) this register and get [`dphy_config_9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dphy_config_9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dphy_config_9`] module"]
#[doc(alias = "DPHY_CONFIG_9")]
pub type DphyConfig9 = crate::Reg<dphy_config_9::DphyConfig9Spec>;
#[doc = "DPHY HS Data Output"]
pub mod dphy_config_9;
#[doc = "DPHY_CONFIG_10 (rw) register accessor: DPHY Impedance and Voltage Trim\n\nYou can [`read`](crate::Reg::read) this register and get [`dphy_config_10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dphy_config_10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dphy_config_10`] module"]
#[doc(alias = "DPHY_CONFIG_10")]
pub type DphyConfig10 = crate::Reg<dphy_config_10::DphyConfig10Spec>;
#[doc = "DPHY Impedance and Voltage Trim"]
pub mod dphy_config_10;
#[doc = "DPHY_CONFIG_11 (rw) register accessor: DPHY Analog Control 0\n\nYou can [`read`](crate::Reg::read) this register and get [`dphy_config_11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dphy_config_11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dphy_config_11`] module"]
#[doc(alias = "DPHY_CONFIG_11")]
pub type DphyConfig11 = crate::Reg<dphy_config_11::DphyConfig11Spec>;
#[doc = "DPHY Analog Control 0"]
pub mod dphy_config_11;
#[doc = "DPHY_CONFIG_12 (rw) register accessor: DPHY Analog Control 1\n\nYou can [`read`](crate::Reg::read) this register and get [`dphy_config_12::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dphy_config_12::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dphy_config_12`] module"]
#[doc(alias = "DPHY_CONFIG_12")]
pub type DphyConfig12 = crate::Reg<dphy_config_12::DphyConfig12Spec>;
#[doc = "DPHY Analog Control 1"]
pub mod dphy_config_12;
#[doc = "DPHY_CONFIG_13 (r) register accessor: DPHY Digital Test Bus Readout\n\nYou can [`read`](crate::Reg::read) this register and get [`dphy_config_13::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dphy_config_13`] module"]
#[doc(alias = "DPHY_CONFIG_13")]
pub type DphyConfig13 = crate::Reg<dphy_config_13::DphyConfig13Spec>;
#[doc = "DPHY Digital Test Bus Readout"]
pub mod dphy_config_13;
#[doc = "DPHY_CONFIG_14 (rw) register accessor: DPHY Pattern Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`dphy_config_14::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dphy_config_14::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dphy_config_14`] module"]
#[doc(alias = "DPHY_CONFIG_14")]
pub type DphyConfig14 = crate::Reg<dphy_config_14::DphyConfig14Spec>;
#[doc = "DPHY Pattern Configuration"]
pub mod dphy_config_14;
#[doc = "DPHY_CONFIG_15 (rw) register accessor: DPHY Power and Reset Control\n\nYou can [`read`](crate::Reg::read) this register and get [`dphy_config_15::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dphy_config_15::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dphy_config_15`] module"]
#[doc(alias = "DPHY_CONFIG_15")]
pub type DphyConfig15 = crate::Reg<dphy_config_15::DphyConfig15Spec>;
#[doc = "DPHY Power and Reset Control"]
pub mod dphy_config_15;
#[doc = "DPHY_CONFIG_16 (rw) register accessor: DPHY Clock Inversion Control\n\nYou can [`read`](crate::Reg::read) this register and get [`dphy_config_16::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dphy_config_16::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dphy_config_16`] module"]
#[doc(alias = "DPHY_CONFIG_16")]
pub type DphyConfig16 = crate::Reg<dphy_config_16::DphyConfig16Spec>;
#[doc = "DPHY Clock Inversion Control"]
pub mod dphy_config_16;
#[doc = "DUMMY_REG (rw) register accessor: DSI Dummy Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dummy_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dummy_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dummy_reg`] module"]
#[doc(alias = "DUMMY_REG")]
pub type DummyReg = crate::Reg<dummy_reg::DummyRegSpec>;
#[doc = "DSI Dummy Register"]
pub mod dummy_reg;
