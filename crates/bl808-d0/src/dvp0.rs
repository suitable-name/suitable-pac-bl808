#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    dvp2axi_configue: Dvp2axiConfigue,
    dvp2axi_addr_start: Dvp2axiAddrStart,
    dvp2axi_mem_bcnt: Dvp2axiMemBcnt,
    dvp_status_and_error: DvpStatusAndError,
    dvp2axi_frame_bcnt: Dvp2axiFrameBcnt,
    dvp_frame_fifo_pop: DvpFrameFifoPop,
    dvp2axi_frame_vld: Dvp2axiFrameVld,
    dvp2axi_frame_period: Dvp2axiFramePeriod,
    dvp2axi_misc: Dvp2axiMisc,
    _reserved9: [u8; 0x0c],
    dvp2axi_hsync_crop: Dvp2axiHsyncCrop,
    dvp2axi_vsync_crop: Dvp2axiVsyncCrop,
    dvp2axi_fram_exm: Dvp2axiFramExm,
    _reserved12: [u8; 0x04],
    frame_start_addr0: FrameStartAddr0,
    _reserved13: [u8; 0x04],
    frame_start_addr1: FrameStartAddr1,
    _reserved14: [u8; 0x04],
    frame_start_addr2: FrameStartAddr2,
    _reserved15: [u8; 0x04],
    frame_start_addr3: FrameStartAddr3,
    _reserved16: [u8; 0x04],
    frame_id_sts01: FrameIdSts01,
    frame_id_sts23: FrameIdSts23,
    _reserved18: [u8; 0x88],
    dvp_debug: DvpDebug,
    _reserved19: [u8; 0x08],
    dvp_dummy_reg: DvpDummyReg,
}
impl RegisterBlock {
    #[doc = "0x00 - DVP to AXI Configuration Register"]
    #[inline(always)]
    pub const fn dvp2axi_configue(&self) -> &Dvp2axiConfigue {
        &self.dvp2axi_configue
    }
    #[doc = "0x04 - AXI Start Address"]
    #[inline(always)]
    pub const fn dvp2axi_addr_start(&self) -> &Dvp2axiAddrStart {
        &self.dvp2axi_addr_start
    }
    #[doc = "0x08 - AXI Burst Count"]
    #[inline(always)]
    pub const fn dvp2axi_mem_bcnt(&self) -> &Dvp2axiMemBcnt {
        &self.dvp2axi_mem_bcnt
    }
    #[doc = "0x0c - DVP Status and Error Register"]
    #[inline(always)]
    pub const fn dvp_status_and_error(&self) -> &DvpStatusAndError {
        &self.dvp_status_and_error
    }
    #[doc = "0x10 - Single Frame Byte Count"]
    #[inline(always)]
    pub const fn dvp2axi_frame_bcnt(&self) -> &Dvp2axiFrameBcnt {
        &self.dvp2axi_frame_bcnt
    }
    #[doc = "0x14 - FIFO Pop and Interrupt Clear"]
    #[inline(always)]
    pub const fn dvp_frame_fifo_pop(&self) -> &DvpFrameFifoPop {
        &self.dvp_frame_fifo_pop
    }
    #[doc = "0x18 - Frame Valid Period"]
    #[inline(always)]
    pub const fn dvp2axi_frame_vld(&self) -> &Dvp2axiFrameVld {
        &self.dvp2axi_frame_vld
    }
    #[doc = "0x1c - Frame Period"]
    #[inline(always)]
    pub const fn dvp2axi_frame_period(&self) -> &Dvp2axiFramePeriod {
        &self.dvp2axi_frame_period
    }
    #[doc = "0x20 - Miscellaneous Configuration"]
    #[inline(always)]
    pub const fn dvp2axi_misc(&self) -> &Dvp2axiMisc {
        &self.dvp2axi_misc
    }
    #[doc = "0x30 - HSYNC Active Region Crop"]
    #[inline(always)]
    pub const fn dvp2axi_hsync_crop(&self) -> &Dvp2axiHsyncCrop {
        &self.dvp2axi_hsync_crop
    }
    #[doc = "0x34 - VSYNC Active Region Crop"]
    #[inline(always)]
    pub const fn dvp2axi_vsync_crop(&self) -> &Dvp2axiVsyncCrop {
        &self.dvp2axi_vsync_crop
    }
    #[doc = "0x38 - Frame Examination (Total H/V Count)"]
    #[inline(always)]
    pub const fn dvp2axi_fram_exm(&self) -> &Dvp2axiFramExm {
        &self.dvp2axi_fram_exm
    }
    #[doc = "0x40 - Frame Buffer 0 Start Address Status"]
    #[inline(always)]
    pub const fn frame_start_addr0(&self) -> &FrameStartAddr0 {
        &self.frame_start_addr0
    }
    #[doc = "0x48 - Frame Buffer 1 Start Address Status"]
    #[inline(always)]
    pub const fn frame_start_addr1(&self) -> &FrameStartAddr1 {
        &self.frame_start_addr1
    }
    #[doc = "0x50 - Frame Buffer 2 Start Address Status"]
    #[inline(always)]
    pub const fn frame_start_addr2(&self) -> &FrameStartAddr2 {
        &self.frame_start_addr2
    }
    #[doc = "0x58 - Frame Buffer 3 Start Address Status"]
    #[inline(always)]
    pub const fn frame_start_addr3(&self) -> &FrameStartAddr3 {
        &self.frame_start_addr3
    }
    #[doc = "0x60 - Frame ID Status for Buffers 0 and 1"]
    #[inline(always)]
    pub const fn frame_id_sts01(&self) -> &FrameIdSts01 {
        &self.frame_id_sts01
    }
    #[doc = "0x64 - Frame ID Status for Buffers 2 and 3"]
    #[inline(always)]
    pub const fn frame_id_sts23(&self) -> &FrameIdSts23 {
        &self.frame_id_sts23
    }
    #[doc = "0xf0 - DVP Debug Configuration"]
    #[inline(always)]
    pub const fn dvp_debug(&self) -> &DvpDebug {
        &self.dvp_debug
    }
    #[doc = "0xfc - DVP Dummy Register"]
    #[inline(always)]
    pub const fn dvp_dummy_reg(&self) -> &DvpDummyReg {
        &self.dvp_dummy_reg
    }
}
#[doc = "DVP2AXI_CONFIGUE (rw) register accessor: DVP to AXI Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dvp2axi_configue::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dvp2axi_configue::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dvp2axi_configue`] module"]
#[doc(alias = "DVP2AXI_CONFIGUE")]
pub type Dvp2axiConfigue = crate::Reg<dvp2axi_configue::Dvp2axiConfigueSpec>;
#[doc = "DVP to AXI Configuration Register"]
pub mod dvp2axi_configue;
#[doc = "DVP2AXI_ADDR_START (rw) register accessor: AXI Start Address\n\nYou can [`read`](crate::Reg::read) this register and get [`dvp2axi_addr_start::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dvp2axi_addr_start::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dvp2axi_addr_start`] module"]
#[doc(alias = "DVP2AXI_ADDR_START")]
pub type Dvp2axiAddrStart = crate::Reg<dvp2axi_addr_start::Dvp2axiAddrStartSpec>;
#[doc = "AXI Start Address"]
pub mod dvp2axi_addr_start;
#[doc = "DVP2AXI_MEM_BCNT (rw) register accessor: AXI Burst Count\n\nYou can [`read`](crate::Reg::read) this register and get [`dvp2axi_mem_bcnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dvp2axi_mem_bcnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dvp2axi_mem_bcnt`] module"]
#[doc(alias = "DVP2AXI_MEM_BCNT")]
pub type Dvp2axiMemBcnt = crate::Reg<dvp2axi_mem_bcnt::Dvp2axiMemBcntSpec>;
#[doc = "AXI Burst Count"]
pub mod dvp2axi_mem_bcnt;
#[doc = "DVP_STATUS_AND_ERROR (rw) register accessor: DVP Status and Error Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dvp_status_and_error::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dvp_status_and_error::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dvp_status_and_error`] module"]
#[doc(alias = "DVP_STATUS_AND_ERROR")]
pub type DvpStatusAndError = crate::Reg<dvp_status_and_error::DvpStatusAndErrorSpec>;
#[doc = "DVP Status and Error Register"]
pub mod dvp_status_and_error;
#[doc = "DVP2AXI_FRAME_BCNT (rw) register accessor: Single Frame Byte Count\n\nYou can [`read`](crate::Reg::read) this register and get [`dvp2axi_frame_bcnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dvp2axi_frame_bcnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dvp2axi_frame_bcnt`] module"]
#[doc(alias = "DVP2AXI_FRAME_BCNT")]
pub type Dvp2axiFrameBcnt = crate::Reg<dvp2axi_frame_bcnt::Dvp2axiFrameBcntSpec>;
#[doc = "Single Frame Byte Count"]
pub mod dvp2axi_frame_bcnt;
#[doc = "DVP_FRAME_FIFO_POP (w) register accessor: FIFO Pop and Interrupt Clear\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dvp_frame_fifo_pop::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dvp_frame_fifo_pop`] module"]
#[doc(alias = "DVP_FRAME_FIFO_POP")]
pub type DvpFrameFifoPop = crate::Reg<dvp_frame_fifo_pop::DvpFrameFifoPopSpec>;
#[doc = "FIFO Pop and Interrupt Clear"]
pub mod dvp_frame_fifo_pop;
#[doc = "DVP2AXI_FRAME_VLD (rw) register accessor: Frame Valid Period\n\nYou can [`read`](crate::Reg::read) this register and get [`dvp2axi_frame_vld::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dvp2axi_frame_vld::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dvp2axi_frame_vld`] module"]
#[doc(alias = "DVP2AXI_FRAME_VLD")]
pub type Dvp2axiFrameVld = crate::Reg<dvp2axi_frame_vld::Dvp2axiFrameVldSpec>;
#[doc = "Frame Valid Period"]
pub mod dvp2axi_frame_vld;
#[doc = "DVP2AXI_FRAME_PERIOD (rw) register accessor: Frame Period\n\nYou can [`read`](crate::Reg::read) this register and get [`dvp2axi_frame_period::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dvp2axi_frame_period::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dvp2axi_frame_period`] module"]
#[doc(alias = "DVP2AXI_FRAME_PERIOD")]
pub type Dvp2axiFramePeriod = crate::Reg<dvp2axi_frame_period::Dvp2axiFramePeriodSpec>;
#[doc = "Frame Period"]
pub mod dvp2axi_frame_period;
#[doc = "DVP2AXI_MISC (rw) register accessor: Miscellaneous Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`dvp2axi_misc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dvp2axi_misc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dvp2axi_misc`] module"]
#[doc(alias = "DVP2AXI_MISC")]
pub type Dvp2axiMisc = crate::Reg<dvp2axi_misc::Dvp2axiMiscSpec>;
#[doc = "Miscellaneous Configuration"]
pub mod dvp2axi_misc;
#[doc = "DVP2AXI_HSYNC_CROP (rw) register accessor: HSYNC Active Region Crop\n\nYou can [`read`](crate::Reg::read) this register and get [`dvp2axi_hsync_crop::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dvp2axi_hsync_crop::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dvp2axi_hsync_crop`] module"]
#[doc(alias = "DVP2AXI_HSYNC_CROP")]
pub type Dvp2axiHsyncCrop = crate::Reg<dvp2axi_hsync_crop::Dvp2axiHsyncCropSpec>;
#[doc = "HSYNC Active Region Crop"]
pub mod dvp2axi_hsync_crop;
#[doc = "DVP2AXI_VSYNC_CROP (rw) register accessor: VSYNC Active Region Crop\n\nYou can [`read`](crate::Reg::read) this register and get [`dvp2axi_vsync_crop::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dvp2axi_vsync_crop::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dvp2axi_vsync_crop`] module"]
#[doc(alias = "DVP2AXI_VSYNC_CROP")]
pub type Dvp2axiVsyncCrop = crate::Reg<dvp2axi_vsync_crop::Dvp2axiVsyncCropSpec>;
#[doc = "VSYNC Active Region Crop"]
pub mod dvp2axi_vsync_crop;
#[doc = "DVP2AXI_FRAM_EXM (rw) register accessor: Frame Examination (Total H/V Count)\n\nYou can [`read`](crate::Reg::read) this register and get [`dvp2axi_fram_exm::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dvp2axi_fram_exm::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dvp2axi_fram_exm`] module"]
#[doc(alias = "DVP2AXI_FRAM_EXM")]
pub type Dvp2axiFramExm = crate::Reg<dvp2axi_fram_exm::Dvp2axiFramExmSpec>;
#[doc = "Frame Examination (Total H/V Count)"]
pub mod dvp2axi_fram_exm;
#[doc = "FRAME_START_ADDR0 (r) register accessor: Frame Buffer 0 Start Address Status\n\nYou can [`read`](crate::Reg::read) this register and get [`frame_start_addr0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@frame_start_addr0`] module"]
#[doc(alias = "FRAME_START_ADDR0")]
pub type FrameStartAddr0 = crate::Reg<frame_start_addr0::FrameStartAddr0Spec>;
#[doc = "Frame Buffer 0 Start Address Status"]
pub mod frame_start_addr0;
#[doc = "FRAME_START_ADDR1 (r) register accessor: Frame Buffer 1 Start Address Status\n\nYou can [`read`](crate::Reg::read) this register and get [`frame_start_addr1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@frame_start_addr1`] module"]
#[doc(alias = "FRAME_START_ADDR1")]
pub type FrameStartAddr1 = crate::Reg<frame_start_addr1::FrameStartAddr1Spec>;
#[doc = "Frame Buffer 1 Start Address Status"]
pub mod frame_start_addr1;
#[doc = "FRAME_START_ADDR2 (r) register accessor: Frame Buffer 2 Start Address Status\n\nYou can [`read`](crate::Reg::read) this register and get [`frame_start_addr2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@frame_start_addr2`] module"]
#[doc(alias = "FRAME_START_ADDR2")]
pub type FrameStartAddr2 = crate::Reg<frame_start_addr2::FrameStartAddr2Spec>;
#[doc = "Frame Buffer 2 Start Address Status"]
pub mod frame_start_addr2;
#[doc = "FRAME_START_ADDR3 (r) register accessor: Frame Buffer 3 Start Address Status\n\nYou can [`read`](crate::Reg::read) this register and get [`frame_start_addr3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@frame_start_addr3`] module"]
#[doc(alias = "FRAME_START_ADDR3")]
pub type FrameStartAddr3 = crate::Reg<frame_start_addr3::FrameStartAddr3Spec>;
#[doc = "Frame Buffer 3 Start Address Status"]
pub mod frame_start_addr3;
#[doc = "FRAME_ID_STS01 (r) register accessor: Frame ID Status for Buffers 0 and 1\n\nYou can [`read`](crate::Reg::read) this register and get [`frame_id_sts01::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@frame_id_sts01`] module"]
#[doc(alias = "FRAME_ID_STS01")]
pub type FrameIdSts01 = crate::Reg<frame_id_sts01::FrameIdSts01Spec>;
#[doc = "Frame ID Status for Buffers 0 and 1"]
pub mod frame_id_sts01;
#[doc = "FRAME_ID_STS23 (r) register accessor: Frame ID Status for Buffers 2 and 3\n\nYou can [`read`](crate::Reg::read) this register and get [`frame_id_sts23::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@frame_id_sts23`] module"]
#[doc(alias = "FRAME_ID_STS23")]
pub type FrameIdSts23 = crate::Reg<frame_id_sts23::FrameIdSts23Spec>;
#[doc = "Frame ID Status for Buffers 2 and 3"]
pub mod frame_id_sts23;
#[doc = "DVP_DEBUG (rw) register accessor: DVP Debug Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`dvp_debug::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dvp_debug::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dvp_debug`] module"]
#[doc(alias = "DVP_DEBUG")]
pub type DvpDebug = crate::Reg<dvp_debug::DvpDebugSpec>;
#[doc = "DVP Debug Configuration"]
pub mod dvp_debug;
#[doc = "DVP_DUMMY_REG (r) register accessor: DVP Dummy Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dvp_dummy_reg::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dvp_dummy_reg`] module"]
#[doc(alias = "DVP_DUMMY_REG")]
pub type DvpDummyReg = crate::Reg<dvp_dummy_reg::DvpDummyRegSpec>;
#[doc = "DVP Dummy Register"]
pub mod dvp_dummy_reg;
