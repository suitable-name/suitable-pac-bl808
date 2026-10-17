#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    config: Config,
    frame_size_h: FrameSizeH,
    frame_size_v: FrameSizeV,
    frame_size_cea_861: FrameSizeCea861,
    pix_data_range: PixDataRange,
    pix_data_step: PixDataStep,
    _reserved6: [u8; 0x08],
    axi2dvp_setting: Axi2dvpSetting,
    axi2dvp_start_addr_by: Axi2dvpStartAddrBy,
    axi2dvp_burst_cnt: Axi2dvpBurstCnt,
    axi2dvp_status: Axi2dvpStatus,
    axi2dvp_swap_addr_by: Axi2dvpSwapAddrBy,
    axi2dvp_prefetch: Axi2dvpPrefetch,
    snsr2dvp_wait_pos: Snsr2dvpWaitPos,
    _reserved13: [u8; 0x04],
    axi2dvp_start_addr_uv: Axi2dvpStartAddrUv,
    axi2dvp_swap_addr_uv: Axi2dvpSwapAddrUv,
}
impl RegisterBlock {
    #[doc = "0x00 - DVP Timing Source Configuration"]
    #[inline(always)]
    pub const fn config(&self) -> &Config {
        &self.config
    }
    #[doc = "0x04 - Horizontal Frame Size Configuration"]
    #[inline(always)]
    pub const fn frame_size_h(&self) -> &FrameSizeH {
        &self.frame_size_h
    }
    #[doc = "0x08 - Vertical Frame Size Configuration"]
    #[inline(always)]
    pub const fn frame_size_v(&self) -> &FrameSizeV {
        &self.frame_size_v
    }
    #[doc = "0x0c - CEA-861 Timing Configuration"]
    #[inline(always)]
    pub const fn frame_size_cea_861(&self) -> &FrameSizeCea861 {
        &self.frame_size_cea_861
    }
    #[doc = "0x10 - Pixel Data Range (Clipping) Configuration"]
    #[inline(always)]
    pub const fn pix_data_range(&self) -> &PixDataRange {
        &self.pix_data_range
    }
    #[doc = "0x14 - Pixel Data Step/Stride Configuration"]
    #[inline(always)]
    pub const fn pix_data_step(&self) -> &PixDataStep {
        &self.pix_data_step
    }
    #[doc = "0x20 - AXI to DVP Interface Setting"]
    #[inline(always)]
    pub const fn axi2dvp_setting(&self) -> &Axi2dvpSetting {
        &self.axi2dvp_setting
    }
    #[doc = "0x24 - AXI to DVP Start Address (luma/BY plane)"]
    #[inline(always)]
    pub const fn axi2dvp_start_addr_by(&self) -> &Axi2dvpStartAddrBy {
        &self.axi2dvp_start_addr_by
    }
    #[doc = "0x28 - AXI to DVP Total Burst Count"]
    #[inline(always)]
    pub const fn axi2dvp_burst_cnt(&self) -> &Axi2dvpBurstCnt {
        &self.axi2dvp_burst_cnt
    }
    #[doc = "0x2c - AXI to DVP Status (luma/BY channel bits\\[15:0\\], chroma/UV channel bits\\[31:16\\])"]
    #[inline(always)]
    pub const fn axi2dvp_status(&self) -> &Axi2dvpStatus {
        &self.axi2dvp_status
    }
    #[doc = "0x30 - AXI to DVP Swap Address (luma/BY plane, for double buffering)"]
    #[inline(always)]
    pub const fn axi2dvp_swap_addr_by(&self) -> &Axi2dvpSwapAddrBy {
        &self.axi2dvp_swap_addr_by
    }
    #[doc = "0x34 - AXI to DVP Vertical Prefetch Setting"]
    #[inline(always)]
    pub const fn axi2dvp_prefetch(&self) -> &Axi2dvpPrefetch {
        &self.axi2dvp_prefetch
    }
    #[doc = "0x38 - Sensor to DVP FIFO Threshold/Wait Position"]
    #[inline(always)]
    pub const fn snsr2dvp_wait_pos(&self) -> &Snsr2dvpWaitPos {
        &self.snsr2dvp_wait_pos
    }
    #[doc = "0x40 - AXI to DVP Start Address for Chroma (UV) plane"]
    #[inline(always)]
    pub const fn axi2dvp_start_addr_uv(&self) -> &Axi2dvpStartAddrUv {
        &self.axi2dvp_start_addr_uv
    }
    #[doc = "0x44 - AXI to DVP Swap Address for Chroma (UV) plane (double buffering)"]
    #[inline(always)]
    pub const fn axi2dvp_swap_addr_uv(&self) -> &Axi2dvpSwapAddrUv {
        &self.axi2dvp_swap_addr_uv
    }
}
#[doc = "CONFIG (rw) register accessor: DVP Timing Source Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`config::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`config::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@config`] module"]
#[doc(alias = "CONFIG")]
pub type Config = crate::Reg<config::ConfigSpec>;
#[doc = "DVP Timing Source Configuration"]
pub mod config;
#[doc = "FRAME_SIZE_H (rw) register accessor: Horizontal Frame Size Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`frame_size_h::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`frame_size_h::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@frame_size_h`] module"]
#[doc(alias = "FRAME_SIZE_H")]
pub type FrameSizeH = crate::Reg<frame_size_h::FrameSizeHSpec>;
#[doc = "Horizontal Frame Size Configuration"]
pub mod frame_size_h;
#[doc = "FRAME_SIZE_V (rw) register accessor: Vertical Frame Size Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`frame_size_v::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`frame_size_v::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@frame_size_v`] module"]
#[doc(alias = "FRAME_SIZE_V")]
pub type FrameSizeV = crate::Reg<frame_size_v::FrameSizeVSpec>;
#[doc = "Vertical Frame Size Configuration"]
pub mod frame_size_v;
#[doc = "FRAME_SIZE_CEA_861 (rw) register accessor: CEA-861 Timing Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`frame_size_cea_861::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`frame_size_cea_861::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@frame_size_cea_861`] module"]
#[doc(alias = "FRAME_SIZE_CEA_861")]
pub type FrameSizeCea861 = crate::Reg<frame_size_cea_861::FrameSizeCea861Spec>;
#[doc = "CEA-861 Timing Configuration"]
pub mod frame_size_cea_861;
#[doc = "PIX_DATA_RANGE (rw) register accessor: Pixel Data Range (Clipping) Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`pix_data_range::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pix_data_range::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pix_data_range`] module"]
#[doc(alias = "PIX_DATA_RANGE")]
pub type PixDataRange = crate::Reg<pix_data_range::PixDataRangeSpec>;
#[doc = "Pixel Data Range (Clipping) Configuration"]
pub mod pix_data_range;
#[doc = "PIX_DATA_STEP (rw) register accessor: Pixel Data Step/Stride Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`pix_data_step::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pix_data_step::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pix_data_step`] module"]
#[doc(alias = "PIX_DATA_STEP")]
pub type PixDataStep = crate::Reg<pix_data_step::PixDataStepSpec>;
#[doc = "Pixel Data Step/Stride Configuration"]
pub mod pix_data_step;
#[doc = "AXI2DVP_SETTING (rw) register accessor: AXI to DVP Interface Setting\n\nYou can [`read`](crate::Reg::read) this register and get [`axi2dvp_setting::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`axi2dvp_setting::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@axi2dvp_setting`] module"]
#[doc(alias = "AXI2DVP_SETTING")]
pub type Axi2dvpSetting = crate::Reg<axi2dvp_setting::Axi2dvpSettingSpec>;
#[doc = "AXI to DVP Interface Setting"]
pub mod axi2dvp_setting;
#[doc = "AXI2DVP_START_ADDR_BY (rw) register accessor: AXI to DVP Start Address (luma/BY plane)\n\nYou can [`read`](crate::Reg::read) this register and get [`axi2dvp_start_addr_by::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`axi2dvp_start_addr_by::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@axi2dvp_start_addr_by`] module"]
#[doc(alias = "AXI2DVP_START_ADDR_BY")]
pub type Axi2dvpStartAddrBy = crate::Reg<axi2dvp_start_addr_by::Axi2dvpStartAddrBySpec>;
#[doc = "AXI to DVP Start Address (luma/BY plane)"]
pub mod axi2dvp_start_addr_by;
#[doc = "AXI2DVP_BURST_CNT (rw) register accessor: AXI to DVP Total Burst Count\n\nYou can [`read`](crate::Reg::read) this register and get [`axi2dvp_burst_cnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`axi2dvp_burst_cnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@axi2dvp_burst_cnt`] module"]
#[doc(alias = "AXI2DVP_BURST_CNT")]
pub type Axi2dvpBurstCnt = crate::Reg<axi2dvp_burst_cnt::Axi2dvpBurstCntSpec>;
#[doc = "AXI to DVP Total Burst Count"]
pub mod axi2dvp_burst_cnt;
#[doc = "AXI2DVP_STATUS (r) register accessor: AXI to DVP Status (luma/BY channel bits\\[15:0\\], chroma/UV channel bits\\[31:16\\])\n\nYou can [`read`](crate::Reg::read) this register and get [`axi2dvp_status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@axi2dvp_status`] module"]
#[doc(alias = "AXI2DVP_STATUS")]
pub type Axi2dvpStatus = crate::Reg<axi2dvp_status::Axi2dvpStatusSpec>;
#[doc = "AXI to DVP Status (luma/BY channel bits\\[15:0\\], chroma/UV channel bits\\[31:16\\])"]
pub mod axi2dvp_status;
#[doc = "AXI2DVP_SWAP_ADDR_BY (rw) register accessor: AXI to DVP Swap Address (luma/BY plane, for double buffering)\n\nYou can [`read`](crate::Reg::read) this register and get [`axi2dvp_swap_addr_by::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`axi2dvp_swap_addr_by::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@axi2dvp_swap_addr_by`] module"]
#[doc(alias = "AXI2DVP_SWAP_ADDR_BY")]
pub type Axi2dvpSwapAddrBy = crate::Reg<axi2dvp_swap_addr_by::Axi2dvpSwapAddrBySpec>;
#[doc = "AXI to DVP Swap Address (luma/BY plane, for double buffering)"]
pub mod axi2dvp_swap_addr_by;
#[doc = "AXI2DVP_PREFETCH (rw) register accessor: AXI to DVP Vertical Prefetch Setting\n\nYou can [`read`](crate::Reg::read) this register and get [`axi2dvp_prefetch::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`axi2dvp_prefetch::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@axi2dvp_prefetch`] module"]
#[doc(alias = "AXI2DVP_PREFETCH")]
pub type Axi2dvpPrefetch = crate::Reg<axi2dvp_prefetch::Axi2dvpPrefetchSpec>;
#[doc = "AXI to DVP Vertical Prefetch Setting"]
pub mod axi2dvp_prefetch;
#[doc = "SNSR2DVP_WAIT_POS (rw) register accessor: Sensor to DVP FIFO Threshold/Wait Position\n\nYou can [`read`](crate::Reg::read) this register and get [`snsr2dvp_wait_pos::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`snsr2dvp_wait_pos::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@snsr2dvp_wait_pos`] module"]
#[doc(alias = "SNSR2DVP_WAIT_POS")]
pub type Snsr2dvpWaitPos = crate::Reg<snsr2dvp_wait_pos::Snsr2dvpWaitPosSpec>;
#[doc = "Sensor to DVP FIFO Threshold/Wait Position"]
pub mod snsr2dvp_wait_pos;
#[doc = "AXI2DVP_START_ADDR_UV (rw) register accessor: AXI to DVP Start Address for Chroma (UV) plane\n\nYou can [`read`](crate::Reg::read) this register and get [`axi2dvp_start_addr_uv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`axi2dvp_start_addr_uv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@axi2dvp_start_addr_uv`] module"]
#[doc(alias = "AXI2DVP_START_ADDR_UV")]
pub type Axi2dvpStartAddrUv = crate::Reg<axi2dvp_start_addr_uv::Axi2dvpStartAddrUvSpec>;
#[doc = "AXI to DVP Start Address for Chroma (UV) plane"]
pub mod axi2dvp_start_addr_uv;
#[doc = "AXI2DVP_SWAP_ADDR_UV (rw) register accessor: AXI to DVP Swap Address for Chroma (UV) plane (double buffering)\n\nYou can [`read`](crate::Reg::read) this register and get [`axi2dvp_swap_addr_uv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`axi2dvp_swap_addr_uv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@axi2dvp_swap_addr_uv`] module"]
#[doc(alias = "AXI2DVP_SWAP_ADDR_UV")]
pub type Axi2dvpSwapAddrUv = crate::Reg<axi2dvp_swap_addr_uv::Axi2dvpSwapAddrUvSpec>;
#[doc = "AXI to DVP Swap Address for Chroma (UV) plane (double buffering)"]
pub mod axi2dvp_swap_addr_uv;
