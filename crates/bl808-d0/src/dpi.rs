#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    dpi_config: DpiConfig,
    dpi_fifo_config: DpiFifoConfig,
    _reserved2: [u8; 0x08],
    dpi_h_timing_0: DpiHTiming0,
    dpi_h_timing_1: DpiHTiming1,
    dpi_v_timing_0: DpiVTiming0,
    dpi_v_timing_1: DpiVTiming1,
    dpi_int_sts: DpiIntSts,
}
impl RegisterBlock {
    #[doc = "0x00 - DPI Global Configuration Register"]
    #[inline(always)]
    pub const fn dpi_config(&self) -> &DpiConfig {
        &self.dpi_config
    }
    #[doc = "0x04 - FIFO and Pixel Format Configuration"]
    #[inline(always)]
    pub const fn dpi_fifo_config(&self) -> &DpiFifoConfig {
        &self.dpi_fifo_config
    }
    #[doc = "0x10 - Horizontal Sync and Back Porch Timing"]
    #[inline(always)]
    pub const fn dpi_h_timing_0(&self) -> &DpiHTiming0 {
        &self.dpi_h_timing_0
    }
    #[doc = "0x14 - Horizontal Active and Front Porch Timing"]
    #[inline(always)]
    pub const fn dpi_h_timing_1(&self) -> &DpiHTiming1 {
        &self.dpi_h_timing_1
    }
    #[doc = "0x18 - Vertical Sync and Back Porch Timing"]
    #[inline(always)]
    pub const fn dpi_v_timing_0(&self) -> &DpiVTiming0 {
        &self.dpi_v_timing_0
    }
    #[doc = "0x1c - Vertical Active and Front Porch Timing"]
    #[inline(always)]
    pub const fn dpi_v_timing_1(&self) -> &DpiVTiming1 {
        &self.dpi_v_timing_1
    }
    #[doc = "0x20 - DPI Interrupt Status Register"]
    #[inline(always)]
    pub const fn dpi_int_sts(&self) -> &DpiIntSts {
        &self.dpi_int_sts
    }
}
#[doc = "DPI_CONFIG (rw) register accessor: DPI Global Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dpi_config::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dpi_config::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dpi_config`] module"]
#[doc(alias = "DPI_CONFIG")]
pub type DpiConfig = crate::Reg<dpi_config::DpiConfigSpec>;
#[doc = "DPI Global Configuration Register"]
pub mod dpi_config;
#[doc = "DPI_FIFO_CONFIG (rw) register accessor: FIFO and Pixel Format Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`dpi_fifo_config::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dpi_fifo_config::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dpi_fifo_config`] module"]
#[doc(alias = "DPI_FIFO_CONFIG")]
pub type DpiFifoConfig = crate::Reg<dpi_fifo_config::DpiFifoConfigSpec>;
#[doc = "FIFO and Pixel Format Configuration"]
pub mod dpi_fifo_config;
#[doc = "DPI_H_TIMING_0 (rw) register accessor: Horizontal Sync and Back Porch Timing\n\nYou can [`read`](crate::Reg::read) this register and get [`dpi_h_timing_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dpi_h_timing_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dpi_h_timing_0`] module"]
#[doc(alias = "DPI_H_TIMING_0")]
pub type DpiHTiming0 = crate::Reg<dpi_h_timing_0::DpiHTiming0Spec>;
#[doc = "Horizontal Sync and Back Porch Timing"]
pub mod dpi_h_timing_0;
#[doc = "DPI_H_TIMING_1 (rw) register accessor: Horizontal Active and Front Porch Timing\n\nYou can [`read`](crate::Reg::read) this register and get [`dpi_h_timing_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dpi_h_timing_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dpi_h_timing_1`] module"]
#[doc(alias = "DPI_H_TIMING_1")]
pub type DpiHTiming1 = crate::Reg<dpi_h_timing_1::DpiHTiming1Spec>;
#[doc = "Horizontal Active and Front Porch Timing"]
pub mod dpi_h_timing_1;
#[doc = "DPI_V_TIMING_0 (rw) register accessor: Vertical Sync and Back Porch Timing\n\nYou can [`read`](crate::Reg::read) this register and get [`dpi_v_timing_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dpi_v_timing_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dpi_v_timing_0`] module"]
#[doc(alias = "DPI_V_TIMING_0")]
pub type DpiVTiming0 = crate::Reg<dpi_v_timing_0::DpiVTiming0Spec>;
#[doc = "Vertical Sync and Back Porch Timing"]
pub mod dpi_v_timing_0;
#[doc = "DPI_V_TIMING_1 (rw) register accessor: Vertical Active and Front Porch Timing\n\nYou can [`read`](crate::Reg::read) this register and get [`dpi_v_timing_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dpi_v_timing_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dpi_v_timing_1`] module"]
#[doc(alias = "DPI_V_TIMING_1")]
pub type DpiVTiming1 = crate::Reg<dpi_v_timing_1::DpiVTiming1Spec>;
#[doc = "Vertical Active and Front Porch Timing"]
pub mod dpi_v_timing_1;
#[doc = "DPI_INT_STS (rw) register accessor: DPI Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dpi_int_sts::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dpi_int_sts::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dpi_int_sts`] module"]
#[doc(alias = "DPI_INT_STS")]
pub type DpiIntSts = crate::Reg<dpi_int_sts::DpiIntStsSpec>;
#[doc = "DPI Interrupt Status Register"]
pub mod dpi_int_sts;
