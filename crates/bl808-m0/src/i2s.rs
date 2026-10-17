#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    i2s_config: I2sConfig,
    i2s_int_sts: I2sIntSts,
    _reserved2: [u8; 0x08],
    i2s_bclk_config: I2sBclkConfig,
    _reserved3: [u8; 0x6c],
    i2s_fifo_config_0: I2sFifoConfig0,
    i2s_fifo_config_1: I2sFifoConfig1,
    i2s_fifo_wdata: I2sFifoWdata,
    i2s_fifo_rdata: I2sFifoRdata,
    _reserved7: [u8; 0x6c],
    i2s_io_config: I2sIoConfig,
}
impl RegisterBlock {
    #[doc = "0x00 - I2S Configuration Register"]
    #[inline(always)]
    pub const fn i2s_config(&self) -> &I2sConfig {
        &self.i2s_config
    }
    #[doc = "0x04 - I2S Interrupt Status Register"]
    #[inline(always)]
    pub const fn i2s_int_sts(&self) -> &I2sIntSts {
        &self.i2s_int_sts
    }
    #[doc = "0x10 - BCLK Divider Configuration"]
    #[inline(always)]
    pub const fn i2s_bclk_config(&self) -> &I2sBclkConfig {
        &self.i2s_bclk_config
    }
    #[doc = "0x80 - I2S FIFO Configuration Register 0"]
    #[inline(always)]
    pub const fn i2s_fifo_config_0(&self) -> &I2sFifoConfig0 {
        &self.i2s_fifo_config_0
    }
    #[doc = "0x84 - I2S FIFO Status and Threshold Register"]
    #[inline(always)]
    pub const fn i2s_fifo_config_1(&self) -> &I2sFifoConfig1 {
        &self.i2s_fifo_config_1
    }
    #[doc = "0x88 - I2S FIFO Write Data"]
    #[inline(always)]
    pub const fn i2s_fifo_wdata(&self) -> &I2sFifoWdata {
        &self.i2s_fifo_wdata
    }
    #[doc = "0x8c - I2S FIFO Read Data"]
    #[inline(always)]
    pub const fn i2s_fifo_rdata(&self) -> &I2sFifoRdata {
        &self.i2s_fifo_rdata
    }
    #[doc = "0xfc - I2S Pin Configuration Register"]
    #[inline(always)]
    pub const fn i2s_io_config(&self) -> &I2sIoConfig {
        &self.i2s_io_config
    }
}
#[doc = "I2S_CONFIG (rw) register accessor: I2S Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`i2s_config::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2s_config::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2s_config`] module"]
#[doc(alias = "I2S_CONFIG")]
pub type I2sConfig = crate::Reg<i2s_config::I2sConfigSpec>;
#[doc = "I2S Configuration Register"]
pub mod i2s_config;
#[doc = "I2S_INT_STS (rw) register accessor: I2S Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`i2s_int_sts::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2s_int_sts::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2s_int_sts`] module"]
#[doc(alias = "I2S_INT_STS")]
pub type I2sIntSts = crate::Reg<i2s_int_sts::I2sIntStsSpec>;
#[doc = "I2S Interrupt Status Register"]
pub mod i2s_int_sts;
#[doc = "I2S_BCLK_CONFIG (rw) register accessor: BCLK Divider Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`i2s_bclk_config::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2s_bclk_config::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2s_bclk_config`] module"]
#[doc(alias = "I2S_BCLK_CONFIG")]
pub type I2sBclkConfig = crate::Reg<i2s_bclk_config::I2sBclkConfigSpec>;
#[doc = "BCLK Divider Configuration"]
pub mod i2s_bclk_config;
#[doc = "I2S_FIFO_CONFIG_0 (rw) register accessor: I2S FIFO Configuration Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`i2s_fifo_config_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2s_fifo_config_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2s_fifo_config_0`] module"]
#[doc(alias = "I2S_FIFO_CONFIG_0")]
pub type I2sFifoConfig0 = crate::Reg<i2s_fifo_config_0::I2sFifoConfig0Spec>;
#[doc = "I2S FIFO Configuration Register 0"]
pub mod i2s_fifo_config_0;
#[doc = "I2S_FIFO_CONFIG_1 (rw) register accessor: I2S FIFO Status and Threshold Register\n\nYou can [`read`](crate::Reg::read) this register and get [`i2s_fifo_config_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2s_fifo_config_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2s_fifo_config_1`] module"]
#[doc(alias = "I2S_FIFO_CONFIG_1")]
pub type I2sFifoConfig1 = crate::Reg<i2s_fifo_config_1::I2sFifoConfig1Spec>;
#[doc = "I2S FIFO Status and Threshold Register"]
pub mod i2s_fifo_config_1;
#[doc = "I2S_FIFO_WDATA (w) register accessor: I2S FIFO Write Data\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2s_fifo_wdata::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2s_fifo_wdata`] module"]
#[doc(alias = "I2S_FIFO_WDATA")]
pub type I2sFifoWdata = crate::Reg<i2s_fifo_wdata::I2sFifoWdataSpec>;
#[doc = "I2S FIFO Write Data"]
pub mod i2s_fifo_wdata;
#[doc = "I2S_FIFO_RDATA (r) register accessor: I2S FIFO Read Data\n\nYou can [`read`](crate::Reg::read) this register and get [`i2s_fifo_rdata::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2s_fifo_rdata`] module"]
#[doc(alias = "I2S_FIFO_RDATA")]
pub type I2sFifoRdata = crate::Reg<i2s_fifo_rdata::I2sFifoRdataSpec>;
#[doc = "I2S FIFO Read Data"]
pub mod i2s_fifo_rdata;
#[doc = "I2S_IO_CONFIG (rw) register accessor: I2S Pin Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`i2s_io_config::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2s_io_config::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2s_io_config`] module"]
#[doc(alias = "I2S_IO_CONFIG")]
pub type I2sIoConfig = crate::Reg<i2s_io_config::I2sIoConfigSpec>;
#[doc = "I2S Pin Configuration Register"]
pub mod i2s_io_config;
