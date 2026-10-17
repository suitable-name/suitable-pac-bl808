#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    vdo_h264_encoder_ctrl: VdoH264EncoderCtrl,
    vdo_h264_frame_size: VdoH264FrameSize,
    vdo_h264_frame_base_addr_y: VdoH264FrameBaseAddrY,
    vdo_h264_frame_base_addr_uv: VdoH264FrameBaseAddrUv,
    vdo_h264_bs_base_addr: VdoH264BsBaseAddr,
    vdo_h264_lb_base_addr: VdoH264LbBaseAddr,
    vdo_h264_bs_size: VdoH264BsSize,
    vdo_h264_bs_full_thr: VdoH264BsFullThr,
    _reserved8: [u8; 0x04],
    vdo_core_reg0: VdoCoreReg0,
    _reserved9: [u8; 0x08],
    vdo_core_reg3: VdoCoreReg3,
    _reserved10: [u8; 0x04],
    vdo_core_reg5: VdoCoreReg5,
    _reserved11: [u8; 0x18],
    vdo_core_reg12: VdoCoreReg12,
    _reserved12: [u8; 0x28],
    vdo_core_reg23: VdoCoreReg23,
    _reserved13: [u8; 0x7c],
    vdo_int: VdoInt,
    vdo_int_clr: VdoIntClr,
    vdo_int_mask: VdoIntMask,
    vdo_total_bytecount: VdoTotalBytecount,
    vdo_frame_bytecount: VdoFrameBytecount,
    _reserved18: [u8; 0x38],
    vdo_h264_s_frame_size: VdoH264SFrameSize,
    vdo_h264_s_frame_base_addr_y: VdoH264SFrameBaseAddrY,
    vdo_h264_s_frame_base_addr_uv: VdoH264SFrameBaseAddrUv,
    _reserved21: [u8; 0x08],
    vdo_h264_s_bs_base_addr: VdoH264SBsBaseAddr,
    _reserved22: [u8; 0x04],
    vdo_h264_s_bs_size: VdoH264SBsSize,
    _reserved23: [u8; 0x0c],
    vdo_s_total_bytecount: VdoSTotalBytecount,
    vdo_s_frame_bytecount: VdoSFrameBytecount,
    _reserved25: [u8; 0x04],
    vdo_src_sta_clr: VdoSrcStaClr,
    vdo_src_ctrl: VdoSrcCtrl,
    vdo_h264_refy0_base_addr: VdoH264Refy0BaseAddr,
    vdo_h264_refu0_base_addr: VdoH264Refu0BaseAddr,
    vdo_h264_refv0_base_addr: VdoH264Refv0BaseAddr,
    vdo_h264_refy1_base_addr: VdoH264Refy1BaseAddr,
    vdo_h264_refu1_base_addr: VdoH264Refu1BaseAddr,
    vdo_h264_refv1_base_addr: VdoH264Refv1BaseAddr,
    vdo_h264_ext_base_addr: VdoH264ExtBaseAddr,
    vdo_h264_s_refy0_base_addr: VdoH264SRefy0BaseAddr,
    vdo_h264_s_refu0_base_addr: VdoH264SRefu0BaseAddr,
    vdo_h264_s_refv0_base_addr: VdoH264SRefv0BaseAddr,
    vdo_h264_s_refy1_base_addr: VdoH264SRefy1BaseAddr,
    vdo_h264_s_refu1_base_addr: VdoH264SRefu1BaseAddr,
    vdo_h264_s_refv1_base_addr: VdoH264SRefv1BaseAddr,
    vdo_h264_s_ext_base_addr: VdoH264SExtBaseAddr,
    _reserved41: [u8; 0x7c],
    vdo_h264_hwback_mode: VdoH264HwbackMode,
    vdo_h264_roi_mode: VdoH264RoiMode,
    _reserved43: [u8; 0x0c],
    vdo_h264_roi: [VdoH264Roi; 8],
}
impl RegisterBlock {
    #[doc = "0x00 - H264 encoder control register"]
    #[inline(always)]
    pub const fn vdo_h264_encoder_ctrl(&self) -> &VdoH264EncoderCtrl {
        &self.vdo_h264_encoder_ctrl
    }
    #[doc = "0x04 - Main-stream frame resolution (in pixels)"]
    #[inline(always)]
    pub const fn vdo_h264_frame_size(&self) -> &VdoH264FrameSize {
        &self.vdo_h264_frame_size
    }
    #[doc = "0x08 - Main-stream source Y frame buffer base address"]
    #[inline(always)]
    pub const fn vdo_h264_frame_base_addr_y(&self) -> &VdoH264FrameBaseAddrY {
        &self.vdo_h264_frame_base_addr_y
    }
    #[doc = "0x0c - Main-stream source UV frame buffer base address"]
    #[inline(always)]
    pub const fn vdo_h264_frame_base_addr_uv(&self) -> &VdoH264FrameBaseAddrUv {
        &self.vdo_h264_frame_base_addr_uv
    }
    #[doc = "0x10 - Main-stream bitstream output buffer base address"]
    #[inline(always)]
    pub const fn vdo_h264_bs_base_addr(&self) -> &VdoH264BsBaseAddr {
        &self.vdo_h264_bs_base_addr
    }
    #[doc = "0x14 - Main-stream line-buffer base address"]
    #[inline(always)]
    pub const fn vdo_h264_lb_base_addr(&self) -> &VdoH264LbBaseAddr {
        &self.vdo_h264_lb_base_addr
    }
    #[doc = "0x18 - Main-stream bitstream output buffer size (bytes)"]
    #[inline(always)]
    pub const fn vdo_h264_bs_size(&self) -> &VdoH264BsSize {
        &self.vdo_h264_bs_size
    }
    #[doc = "0x1c - Main-stream bitstream buffer full threshold"]
    #[inline(always)]
    pub const fn vdo_h264_bs_full_thr(&self) -> &VdoH264BsFullThr {
        &self.vdo_h264_bs_full_thr
    }
    #[doc = "0x24 - Core control: encoder enable, VUI settings, IDR"]
    #[inline(always)]
    pub const fn vdo_core_reg0(&self) -> &VdoCoreReg0 {
        &self.vdo_core_reg0
    }
    #[doc = "0x30 - Target I-frame QP for both streams"]
    #[inline(always)]
    pub const fn vdo_core_reg3(&self) -> &VdoCoreReg3 {
        &self.vdo_core_reg3
    }
    #[doc = "0x38 - Target P-frame QP for both streams"]
    #[inline(always)]
    pub const fn vdo_core_reg5(&self) -> &VdoCoreReg5 {
        &self.vdo_core_reg5
    }
    #[doc = "0x54 - H.264 profile IDC for both streams (0=Baseline, 1=Main)"]
    #[inline(always)]
    pub const fn vdo_core_reg12(&self) -> &VdoCoreReg12 {
        &self.vdo_core_reg12
    }
    #[doc = "0x80 - I-frame macroblock bit count and NAL/prediction options"]
    #[inline(always)]
    pub const fn vdo_core_reg23(&self) -> &VdoCoreReg23 {
        &self.vdo_core_reg23
    }
    #[doc = "0x100 - Video encoder interrupt status register"]
    #[inline(always)]
    pub const fn vdo_int(&self) -> &VdoInt {
        &self.vdo_int
    }
    #[doc = "0x104 - Video encoder interrupt clear register (write 1 to clear)"]
    #[inline(always)]
    pub const fn vdo_int_clr(&self) -> &VdoIntClr {
        &self.vdo_int_clr
    }
    #[doc = "0x108 - Video encoder interrupt mask register (1=masked)"]
    #[inline(always)]
    pub const fn vdo_int_mask(&self) -> &VdoIntMask {
        &self.vdo_int_mask
    }
    #[doc = "0x10c - Main-stream total bitstream byte count"]
    #[inline(always)]
    pub const fn vdo_total_bytecount(&self) -> &VdoTotalBytecount {
        &self.vdo_total_bytecount
    }
    #[doc = "0x110 - Main-stream per-frame bitstream byte count"]
    #[inline(always)]
    pub const fn vdo_frame_bytecount(&self) -> &VdoFrameBytecount {
        &self.vdo_frame_bytecount
    }
    #[doc = "0x14c - Secondary-stream frame resolution (in pixels)"]
    #[inline(always)]
    pub const fn vdo_h264_s_frame_size(&self) -> &VdoH264SFrameSize {
        &self.vdo_h264_s_frame_size
    }
    #[doc = "0x150 - Secondary-stream source Y frame buffer base address"]
    #[inline(always)]
    pub const fn vdo_h264_s_frame_base_addr_y(&self) -> &VdoH264SFrameBaseAddrY {
        &self.vdo_h264_s_frame_base_addr_y
    }
    #[doc = "0x154 - Secondary-stream source UV frame buffer base address"]
    #[inline(always)]
    pub const fn vdo_h264_s_frame_base_addr_uv(&self) -> &VdoH264SFrameBaseAddrUv {
        &self.vdo_h264_s_frame_base_addr_uv
    }
    #[doc = "0x160 - Secondary-stream bitstream output buffer base address"]
    #[inline(always)]
    pub const fn vdo_h264_s_bs_base_addr(&self) -> &VdoH264SBsBaseAddr {
        &self.vdo_h264_s_bs_base_addr
    }
    #[doc = "0x168 - Secondary-stream bitstream output buffer size (bytes)"]
    #[inline(always)]
    pub const fn vdo_h264_s_bs_size(&self) -> &VdoH264SBsSize {
        &self.vdo_h264_s_bs_size
    }
    #[doc = "0x178 - Secondary-stream total bitstream byte count"]
    #[inline(always)]
    pub const fn vdo_s_total_bytecount(&self) -> &VdoSTotalBytecount {
        &self.vdo_s_total_bytecount
    }
    #[doc = "0x17c - Secondary-stream per-frame bitstream byte count"]
    #[inline(always)]
    pub const fn vdo_s_frame_bytecount(&self) -> &VdoSFrameBytecount {
        &self.vdo_s_frame_bytecount
    }
    #[doc = "0x184 - Source buffer overflow status clear register"]
    #[inline(always)]
    pub const fn vdo_src_sta_clr(&self) -> &VdoSrcStaClr {
        &self.vdo_src_sta_clr
    }
    #[doc = "0x188 - Video source buffer and timing control"]
    #[inline(always)]
    pub const fn vdo_src_ctrl(&self) -> &VdoSrcCtrl {
        &self.vdo_src_ctrl
    }
    #[doc = "0x18c - Main-stream reference frame 0 Y base address"]
    #[inline(always)]
    pub const fn vdo_h264_refy0_base_addr(&self) -> &VdoH264Refy0BaseAddr {
        &self.vdo_h264_refy0_base_addr
    }
    #[doc = "0x190 - Main-stream reference frame 0 U base address"]
    #[inline(always)]
    pub const fn vdo_h264_refu0_base_addr(&self) -> &VdoH264Refu0BaseAddr {
        &self.vdo_h264_refu0_base_addr
    }
    #[doc = "0x194 - Main-stream reference frame 0 V base address"]
    #[inline(always)]
    pub const fn vdo_h264_refv0_base_addr(&self) -> &VdoH264Refv0BaseAddr {
        &self.vdo_h264_refv0_base_addr
    }
    #[doc = "0x198 - Main-stream reference frame 1 Y base address"]
    #[inline(always)]
    pub const fn vdo_h264_refy1_base_addr(&self) -> &VdoH264Refy1BaseAddr {
        &self.vdo_h264_refy1_base_addr
    }
    #[doc = "0x19c - Main-stream reference frame 1 U base address"]
    #[inline(always)]
    pub const fn vdo_h264_refu1_base_addr(&self) -> &VdoH264Refu1BaseAddr {
        &self.vdo_h264_refu1_base_addr
    }
    #[doc = "0x1a0 - Main-stream reference frame 1 V base address"]
    #[inline(always)]
    pub const fn vdo_h264_refv1_base_addr(&self) -> &VdoH264Refv1BaseAddr {
        &self.vdo_h264_refv1_base_addr
    }
    #[doc = "0x1a4 - Main-stream external data base address"]
    #[inline(always)]
    pub const fn vdo_h264_ext_base_addr(&self) -> &VdoH264ExtBaseAddr {
        &self.vdo_h264_ext_base_addr
    }
    #[doc = "0x1a8 - Secondary-stream reference frame 0 Y base address"]
    #[inline(always)]
    pub const fn vdo_h264_s_refy0_base_addr(&self) -> &VdoH264SRefy0BaseAddr {
        &self.vdo_h264_s_refy0_base_addr
    }
    #[doc = "0x1ac - Secondary-stream reference frame 0 U base address"]
    #[inline(always)]
    pub const fn vdo_h264_s_refu0_base_addr(&self) -> &VdoH264SRefu0BaseAddr {
        &self.vdo_h264_s_refu0_base_addr
    }
    #[doc = "0x1b0 - Secondary-stream reference frame 0 V base address"]
    #[inline(always)]
    pub const fn vdo_h264_s_refv0_base_addr(&self) -> &VdoH264SRefv0BaseAddr {
        &self.vdo_h264_s_refv0_base_addr
    }
    #[doc = "0x1b4 - Secondary-stream reference frame 1 Y base address"]
    #[inline(always)]
    pub const fn vdo_h264_s_refy1_base_addr(&self) -> &VdoH264SRefy1BaseAddr {
        &self.vdo_h264_s_refy1_base_addr
    }
    #[doc = "0x1b8 - Secondary-stream reference frame 1 U base address"]
    #[inline(always)]
    pub const fn vdo_h264_s_refu1_base_addr(&self) -> &VdoH264SRefu1BaseAddr {
        &self.vdo_h264_s_refu1_base_addr
    }
    #[doc = "0x1bc - Secondary-stream reference frame 1 V base address"]
    #[inline(always)]
    pub const fn vdo_h264_s_refv1_base_addr(&self) -> &VdoH264SRefv1BaseAddr {
        &self.vdo_h264_s_refv1_base_addr
    }
    #[doc = "0x1c0 - Secondary-stream external data base address"]
    #[inline(always)]
    pub const fn vdo_h264_s_ext_base_addr(&self) -> &VdoH264SExtBaseAddr {
        &self.vdo_h264_s_ext_base_addr
    }
    #[doc = "0x240 - H264 hardware back-write mode control"]
    #[inline(always)]
    pub const fn vdo_h264_hwback_mode(&self) -> &VdoH264HwbackMode {
        &self.vdo_h264_hwback_mode
    }
    #[doc = "0x244 - ROI mode control: enable bits and update trigger"]
    #[inline(always)]
    pub const fn vdo_h264_roi_mode(&self) -> &VdoH264RoiMode {
        &self.vdo_h264_roi_mode
    }
    #[doc = "0x254..0x274 - ROI %s macroblock coordinate bounds (MBX_S/E, MBY_S/E)"]
    #[inline(always)]
    pub const fn vdo_h264_roi(&self, n: usize) -> &VdoH264Roi {
        &self.vdo_h264_roi[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x254..0x274 - ROI %s macroblock coordinate bounds (MBX_S/E, MBY_S/E)"]
    #[inline(always)]
    pub fn vdo_h264_roi_iter(&self) -> impl Iterator<Item = &VdoH264Roi> {
        self.vdo_h264_roi.iter()
    }
}
#[doc = "VDO_H264_ENCODER_CTRL (rw) register accessor: H264 encoder control register\n\nYou can [`read`](crate::Reg::read) this register and get [`vdo_h264_encoder_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vdo_h264_encoder_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vdo_h264_encoder_ctrl`] module"]
#[doc(alias = "VDO_H264_ENCODER_CTRL")]
pub type VdoH264EncoderCtrl = crate::Reg<vdo_h264_encoder_ctrl::VdoH264EncoderCtrlSpec>;
#[doc = "H264 encoder control register"]
pub mod vdo_h264_encoder_ctrl;
#[doc = "VDO_H264_FRAME_SIZE (rw) register accessor: Main-stream frame resolution (in pixels)\n\nYou can [`read`](crate::Reg::read) this register and get [`vdo_h264_frame_size::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vdo_h264_frame_size::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vdo_h264_frame_size`] module"]
#[doc(alias = "VDO_H264_FRAME_SIZE")]
pub type VdoH264FrameSize = crate::Reg<vdo_h264_frame_size::VdoH264FrameSizeSpec>;
#[doc = "Main-stream frame resolution (in pixels)"]
pub mod vdo_h264_frame_size;
#[doc = "VDO_H264_FRAME_BASE_ADDR_Y (rw) register accessor: Main-stream source Y frame buffer base address\n\nYou can [`read`](crate::Reg::read) this register and get [`vdo_h264_frame_base_addr_y::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vdo_h264_frame_base_addr_y::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vdo_h264_frame_base_addr_y`] module"]
#[doc(alias = "VDO_H264_FRAME_BASE_ADDR_Y")]
pub type VdoH264FrameBaseAddrY = crate::Reg<vdo_h264_frame_base_addr_y::VdoH264FrameBaseAddrYSpec>;
#[doc = "Main-stream source Y frame buffer base address"]
pub mod vdo_h264_frame_base_addr_y;
#[doc = "VDO_H264_FRAME_BASE_ADDR_UV (rw) register accessor: Main-stream source UV frame buffer base address\n\nYou can [`read`](crate::Reg::read) this register and get [`vdo_h264_frame_base_addr_uv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vdo_h264_frame_base_addr_uv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vdo_h264_frame_base_addr_uv`] module"]
#[doc(alias = "VDO_H264_FRAME_BASE_ADDR_UV")]
pub type VdoH264FrameBaseAddrUv =
    crate::Reg<vdo_h264_frame_base_addr_uv::VdoH264FrameBaseAddrUvSpec>;
#[doc = "Main-stream source UV frame buffer base address"]
pub mod vdo_h264_frame_base_addr_uv;
#[doc = "VDO_H264_BS_BASE_ADDR (rw) register accessor: Main-stream bitstream output buffer base address\n\nYou can [`read`](crate::Reg::read) this register and get [`vdo_h264_bs_base_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vdo_h264_bs_base_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vdo_h264_bs_base_addr`] module"]
#[doc(alias = "VDO_H264_BS_BASE_ADDR")]
pub type VdoH264BsBaseAddr = crate::Reg<vdo_h264_bs_base_addr::VdoH264BsBaseAddrSpec>;
#[doc = "Main-stream bitstream output buffer base address"]
pub mod vdo_h264_bs_base_addr;
#[doc = "VDO_H264_LB_BASE_ADDR (rw) register accessor: Main-stream line-buffer base address\n\nYou can [`read`](crate::Reg::read) this register and get [`vdo_h264_lb_base_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vdo_h264_lb_base_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vdo_h264_lb_base_addr`] module"]
#[doc(alias = "VDO_H264_LB_BASE_ADDR")]
pub type VdoH264LbBaseAddr = crate::Reg<vdo_h264_lb_base_addr::VdoH264LbBaseAddrSpec>;
#[doc = "Main-stream line-buffer base address"]
pub mod vdo_h264_lb_base_addr;
#[doc = "VDO_H264_BS_SIZE (rw) register accessor: Main-stream bitstream output buffer size (bytes)\n\nYou can [`read`](crate::Reg::read) this register and get [`vdo_h264_bs_size::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vdo_h264_bs_size::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vdo_h264_bs_size`] module"]
#[doc(alias = "VDO_H264_BS_SIZE")]
pub type VdoH264BsSize = crate::Reg<vdo_h264_bs_size::VdoH264BsSizeSpec>;
#[doc = "Main-stream bitstream output buffer size (bytes)"]
pub mod vdo_h264_bs_size;
#[doc = "VDO_H264_BS_FULL_THR (rw) register accessor: Main-stream bitstream buffer full threshold\n\nYou can [`read`](crate::Reg::read) this register and get [`vdo_h264_bs_full_thr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vdo_h264_bs_full_thr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vdo_h264_bs_full_thr`] module"]
#[doc(alias = "VDO_H264_BS_FULL_THR")]
pub type VdoH264BsFullThr = crate::Reg<vdo_h264_bs_full_thr::VdoH264BsFullThrSpec>;
#[doc = "Main-stream bitstream buffer full threshold"]
pub mod vdo_h264_bs_full_thr;
#[doc = "VDO_CORE_REG0 (rw) register accessor: Core control: encoder enable, VUI settings, IDR\n\nYou can [`read`](crate::Reg::read) this register and get [`vdo_core_reg0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vdo_core_reg0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vdo_core_reg0`] module"]
#[doc(alias = "VDO_CORE_REG0")]
pub type VdoCoreReg0 = crate::Reg<vdo_core_reg0::VdoCoreReg0Spec>;
#[doc = "Core control: encoder enable, VUI settings, IDR"]
pub mod vdo_core_reg0;
#[doc = "VDO_CORE_REG3 (rw) register accessor: Target I-frame QP for both streams\n\nYou can [`read`](crate::Reg::read) this register and get [`vdo_core_reg3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vdo_core_reg3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vdo_core_reg3`] module"]
#[doc(alias = "VDO_CORE_REG3")]
pub type VdoCoreReg3 = crate::Reg<vdo_core_reg3::VdoCoreReg3Spec>;
#[doc = "Target I-frame QP for both streams"]
pub mod vdo_core_reg3;
#[doc = "VDO_CORE_REG5 (rw) register accessor: Target P-frame QP for both streams\n\nYou can [`read`](crate::Reg::read) this register and get [`vdo_core_reg5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vdo_core_reg5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vdo_core_reg5`] module"]
#[doc(alias = "VDO_CORE_REG5")]
pub type VdoCoreReg5 = crate::Reg<vdo_core_reg5::VdoCoreReg5Spec>;
#[doc = "Target P-frame QP for both streams"]
pub mod vdo_core_reg5;
#[doc = "VDO_CORE_REG12 (rw) register accessor: H.264 profile IDC for both streams (0=Baseline, 1=Main)\n\nYou can [`read`](crate::Reg::read) this register and get [`vdo_core_reg12::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vdo_core_reg12::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vdo_core_reg12`] module"]
#[doc(alias = "VDO_CORE_REG12")]
pub type VdoCoreReg12 = crate::Reg<vdo_core_reg12::VdoCoreReg12Spec>;
#[doc = "H.264 profile IDC for both streams (0=Baseline, 1=Main)"]
pub mod vdo_core_reg12;
#[doc = "VDO_CORE_REG23 (rw) register accessor: I-frame macroblock bit count and NAL/prediction options\n\nYou can [`read`](crate::Reg::read) this register and get [`vdo_core_reg23::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vdo_core_reg23::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vdo_core_reg23`] module"]
#[doc(alias = "VDO_CORE_REG23")]
pub type VdoCoreReg23 = crate::Reg<vdo_core_reg23::VdoCoreReg23Spec>;
#[doc = "I-frame macroblock bit count and NAL/prediction options"]
pub mod vdo_core_reg23;
#[doc = "VDO_INT (r) register accessor: Video encoder interrupt status register\n\nYou can [`read`](crate::Reg::read) this register and get [`vdo_int::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vdo_int`] module"]
#[doc(alias = "VDO_INT")]
pub type VdoInt = crate::Reg<vdo_int::VdoIntSpec>;
#[doc = "Video encoder interrupt status register"]
pub mod vdo_int;
#[doc = "VDO_INT_CLR (w) register accessor: Video encoder interrupt clear register (write 1 to clear)\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vdo_int_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vdo_int_clr`] module"]
#[doc(alias = "VDO_INT_CLR")]
pub type VdoIntClr = crate::Reg<vdo_int_clr::VdoIntClrSpec>;
#[doc = "Video encoder interrupt clear register (write 1 to clear)"]
pub mod vdo_int_clr;
#[doc = "VDO_INT_MASK (rw) register accessor: Video encoder interrupt mask register (1=masked)\n\nYou can [`read`](crate::Reg::read) this register and get [`vdo_int_mask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vdo_int_mask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vdo_int_mask`] module"]
#[doc(alias = "VDO_INT_MASK")]
pub type VdoIntMask = crate::Reg<vdo_int_mask::VdoIntMaskSpec>;
#[doc = "Video encoder interrupt mask register (1=masked)"]
pub mod vdo_int_mask;
#[doc = "VDO_TOTAL_BYTECOUNT (r) register accessor: Main-stream total bitstream byte count\n\nYou can [`read`](crate::Reg::read) this register and get [`vdo_total_bytecount::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vdo_total_bytecount`] module"]
#[doc(alias = "VDO_TOTAL_BYTECOUNT")]
pub type VdoTotalBytecount = crate::Reg<vdo_total_bytecount::VdoTotalBytecountSpec>;
#[doc = "Main-stream total bitstream byte count"]
pub mod vdo_total_bytecount;
#[doc = "VDO_FRAME_BYTECOUNT (r) register accessor: Main-stream per-frame bitstream byte count\n\nYou can [`read`](crate::Reg::read) this register and get [`vdo_frame_bytecount::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vdo_frame_bytecount`] module"]
#[doc(alias = "VDO_FRAME_BYTECOUNT")]
pub type VdoFrameBytecount = crate::Reg<vdo_frame_bytecount::VdoFrameBytecountSpec>;
#[doc = "Main-stream per-frame bitstream byte count"]
pub mod vdo_frame_bytecount;
#[doc = "VDO_H264_S_FRAME_SIZE (rw) register accessor: Secondary-stream frame resolution (in pixels)\n\nYou can [`read`](crate::Reg::read) this register and get [`vdo_h264_s_frame_size::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vdo_h264_s_frame_size::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vdo_h264_s_frame_size`] module"]
#[doc(alias = "VDO_H264_S_FRAME_SIZE")]
pub type VdoH264SFrameSize = crate::Reg<vdo_h264_s_frame_size::VdoH264SFrameSizeSpec>;
#[doc = "Secondary-stream frame resolution (in pixels)"]
pub mod vdo_h264_s_frame_size;
#[doc = "VDO_H264_S_FRAME_BASE_ADDR_Y (rw) register accessor: Secondary-stream source Y frame buffer base address\n\nYou can [`read`](crate::Reg::read) this register and get [`vdo_h264_s_frame_base_addr_y::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vdo_h264_s_frame_base_addr_y::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vdo_h264_s_frame_base_addr_y`] module"]
#[doc(alias = "VDO_H264_S_FRAME_BASE_ADDR_Y")]
pub type VdoH264SFrameBaseAddrY =
    crate::Reg<vdo_h264_s_frame_base_addr_y::VdoH264SFrameBaseAddrYSpec>;
#[doc = "Secondary-stream source Y frame buffer base address"]
pub mod vdo_h264_s_frame_base_addr_y;
#[doc = "VDO_H264_S_FRAME_BASE_ADDR_UV (rw) register accessor: Secondary-stream source UV frame buffer base address\n\nYou can [`read`](crate::Reg::read) this register and get [`vdo_h264_s_frame_base_addr_uv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vdo_h264_s_frame_base_addr_uv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vdo_h264_s_frame_base_addr_uv`] module"]
#[doc(alias = "VDO_H264_S_FRAME_BASE_ADDR_UV")]
pub type VdoH264SFrameBaseAddrUv =
    crate::Reg<vdo_h264_s_frame_base_addr_uv::VdoH264SFrameBaseAddrUvSpec>;
#[doc = "Secondary-stream source UV frame buffer base address"]
pub mod vdo_h264_s_frame_base_addr_uv;
#[doc = "VDO_H264_S_BS_BASE_ADDR (rw) register accessor: Secondary-stream bitstream output buffer base address\n\nYou can [`read`](crate::Reg::read) this register and get [`vdo_h264_s_bs_base_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vdo_h264_s_bs_base_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vdo_h264_s_bs_base_addr`] module"]
#[doc(alias = "VDO_H264_S_BS_BASE_ADDR")]
pub type VdoH264SBsBaseAddr = crate::Reg<vdo_h264_s_bs_base_addr::VdoH264SBsBaseAddrSpec>;
#[doc = "Secondary-stream bitstream output buffer base address"]
pub mod vdo_h264_s_bs_base_addr;
#[doc = "VDO_H264_S_BS_SIZE (rw) register accessor: Secondary-stream bitstream output buffer size (bytes)\n\nYou can [`read`](crate::Reg::read) this register and get [`vdo_h264_s_bs_size::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vdo_h264_s_bs_size::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vdo_h264_s_bs_size`] module"]
#[doc(alias = "VDO_H264_S_BS_SIZE")]
pub type VdoH264SBsSize = crate::Reg<vdo_h264_s_bs_size::VdoH264SBsSizeSpec>;
#[doc = "Secondary-stream bitstream output buffer size (bytes)"]
pub mod vdo_h264_s_bs_size;
#[doc = "VDO_S_TOTAL_BYTECOUNT (r) register accessor: Secondary-stream total bitstream byte count\n\nYou can [`read`](crate::Reg::read) this register and get [`vdo_s_total_bytecount::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vdo_s_total_bytecount`] module"]
#[doc(alias = "VDO_S_TOTAL_BYTECOUNT")]
pub type VdoSTotalBytecount = crate::Reg<vdo_s_total_bytecount::VdoSTotalBytecountSpec>;
#[doc = "Secondary-stream total bitstream byte count"]
pub mod vdo_s_total_bytecount;
#[doc = "VDO_S_FRAME_BYTECOUNT (r) register accessor: Secondary-stream per-frame bitstream byte count\n\nYou can [`read`](crate::Reg::read) this register and get [`vdo_s_frame_bytecount::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vdo_s_frame_bytecount`] module"]
#[doc(alias = "VDO_S_FRAME_BYTECOUNT")]
pub type VdoSFrameBytecount = crate::Reg<vdo_s_frame_bytecount::VdoSFrameBytecountSpec>;
#[doc = "Secondary-stream per-frame bitstream byte count"]
pub mod vdo_s_frame_bytecount;
#[doc = "VDO_SRC_STA_CLR (w) register accessor: Source buffer overflow status clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vdo_src_sta_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vdo_src_sta_clr`] module"]
#[doc(alias = "VDO_SRC_STA_CLR")]
pub type VdoSrcStaClr = crate::Reg<vdo_src_sta_clr::VdoSrcStaClrSpec>;
#[doc = "Source buffer overflow status clear register"]
pub mod vdo_src_sta_clr;
#[doc = "VDO_SRC_CTRL (rw) register accessor: Video source buffer and timing control\n\nYou can [`read`](crate::Reg::read) this register and get [`vdo_src_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vdo_src_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vdo_src_ctrl`] module"]
#[doc(alias = "VDO_SRC_CTRL")]
pub type VdoSrcCtrl = crate::Reg<vdo_src_ctrl::VdoSrcCtrlSpec>;
#[doc = "Video source buffer and timing control"]
pub mod vdo_src_ctrl;
#[doc = "VDO_H264_REFY0_BASE_ADDR (rw) register accessor: Main-stream reference frame 0 Y base address\n\nYou can [`read`](crate::Reg::read) this register and get [`vdo_h264_refy0_base_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vdo_h264_refy0_base_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vdo_h264_refy0_base_addr`] module"]
#[doc(alias = "VDO_H264_REFY0_BASE_ADDR")]
pub type VdoH264Refy0BaseAddr = crate::Reg<vdo_h264_refy0_base_addr::VdoH264Refy0BaseAddrSpec>;
#[doc = "Main-stream reference frame 0 Y base address"]
pub mod vdo_h264_refy0_base_addr;
#[doc = "VDO_H264_REFU0_BASE_ADDR (rw) register accessor: Main-stream reference frame 0 U base address\n\nYou can [`read`](crate::Reg::read) this register and get [`vdo_h264_refu0_base_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vdo_h264_refu0_base_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vdo_h264_refu0_base_addr`] module"]
#[doc(alias = "VDO_H264_REFU0_BASE_ADDR")]
pub type VdoH264Refu0BaseAddr = crate::Reg<vdo_h264_refu0_base_addr::VdoH264Refu0BaseAddrSpec>;
#[doc = "Main-stream reference frame 0 U base address"]
pub mod vdo_h264_refu0_base_addr;
#[doc = "VDO_H264_REFV0_BASE_ADDR (rw) register accessor: Main-stream reference frame 0 V base address\n\nYou can [`read`](crate::Reg::read) this register and get [`vdo_h264_refv0_base_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vdo_h264_refv0_base_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vdo_h264_refv0_base_addr`] module"]
#[doc(alias = "VDO_H264_REFV0_BASE_ADDR")]
pub type VdoH264Refv0BaseAddr = crate::Reg<vdo_h264_refv0_base_addr::VdoH264Refv0BaseAddrSpec>;
#[doc = "Main-stream reference frame 0 V base address"]
pub mod vdo_h264_refv0_base_addr;
#[doc = "VDO_H264_REFY1_BASE_ADDR (rw) register accessor: Main-stream reference frame 1 Y base address\n\nYou can [`read`](crate::Reg::read) this register and get [`vdo_h264_refy1_base_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vdo_h264_refy1_base_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vdo_h264_refy1_base_addr`] module"]
#[doc(alias = "VDO_H264_REFY1_BASE_ADDR")]
pub type VdoH264Refy1BaseAddr = crate::Reg<vdo_h264_refy1_base_addr::VdoH264Refy1BaseAddrSpec>;
#[doc = "Main-stream reference frame 1 Y base address"]
pub mod vdo_h264_refy1_base_addr;
#[doc = "VDO_H264_REFU1_BASE_ADDR (rw) register accessor: Main-stream reference frame 1 U base address\n\nYou can [`read`](crate::Reg::read) this register and get [`vdo_h264_refu1_base_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vdo_h264_refu1_base_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vdo_h264_refu1_base_addr`] module"]
#[doc(alias = "VDO_H264_REFU1_BASE_ADDR")]
pub type VdoH264Refu1BaseAddr = crate::Reg<vdo_h264_refu1_base_addr::VdoH264Refu1BaseAddrSpec>;
#[doc = "Main-stream reference frame 1 U base address"]
pub mod vdo_h264_refu1_base_addr;
#[doc = "VDO_H264_REFV1_BASE_ADDR (rw) register accessor: Main-stream reference frame 1 V base address\n\nYou can [`read`](crate::Reg::read) this register and get [`vdo_h264_refv1_base_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vdo_h264_refv1_base_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vdo_h264_refv1_base_addr`] module"]
#[doc(alias = "VDO_H264_REFV1_BASE_ADDR")]
pub type VdoH264Refv1BaseAddr = crate::Reg<vdo_h264_refv1_base_addr::VdoH264Refv1BaseAddrSpec>;
#[doc = "Main-stream reference frame 1 V base address"]
pub mod vdo_h264_refv1_base_addr;
#[doc = "VDO_H264_EXT_BASE_ADDR (rw) register accessor: Main-stream external data base address\n\nYou can [`read`](crate::Reg::read) this register and get [`vdo_h264_ext_base_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vdo_h264_ext_base_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vdo_h264_ext_base_addr`] module"]
#[doc(alias = "VDO_H264_EXT_BASE_ADDR")]
pub type VdoH264ExtBaseAddr = crate::Reg<vdo_h264_ext_base_addr::VdoH264ExtBaseAddrSpec>;
#[doc = "Main-stream external data base address"]
pub mod vdo_h264_ext_base_addr;
#[doc = "VDO_H264_S_REFY0_BASE_ADDR (rw) register accessor: Secondary-stream reference frame 0 Y base address\n\nYou can [`read`](crate::Reg::read) this register and get [`vdo_h264_s_refy0_base_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vdo_h264_s_refy0_base_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vdo_h264_s_refy0_base_addr`] module"]
#[doc(alias = "VDO_H264_S_REFY0_BASE_ADDR")]
pub type VdoH264SRefy0BaseAddr = crate::Reg<vdo_h264_s_refy0_base_addr::VdoH264SRefy0BaseAddrSpec>;
#[doc = "Secondary-stream reference frame 0 Y base address"]
pub mod vdo_h264_s_refy0_base_addr;
#[doc = "VDO_H264_S_REFU0_BASE_ADDR (rw) register accessor: Secondary-stream reference frame 0 U base address\n\nYou can [`read`](crate::Reg::read) this register and get [`vdo_h264_s_refu0_base_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vdo_h264_s_refu0_base_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vdo_h264_s_refu0_base_addr`] module"]
#[doc(alias = "VDO_H264_S_REFU0_BASE_ADDR")]
pub type VdoH264SRefu0BaseAddr = crate::Reg<vdo_h264_s_refu0_base_addr::VdoH264SRefu0BaseAddrSpec>;
#[doc = "Secondary-stream reference frame 0 U base address"]
pub mod vdo_h264_s_refu0_base_addr;
#[doc = "VDO_H264_S_REFV0_BASE_ADDR (rw) register accessor: Secondary-stream reference frame 0 V base address\n\nYou can [`read`](crate::Reg::read) this register and get [`vdo_h264_s_refv0_base_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vdo_h264_s_refv0_base_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vdo_h264_s_refv0_base_addr`] module"]
#[doc(alias = "VDO_H264_S_REFV0_BASE_ADDR")]
pub type VdoH264SRefv0BaseAddr = crate::Reg<vdo_h264_s_refv0_base_addr::VdoH264SRefv0BaseAddrSpec>;
#[doc = "Secondary-stream reference frame 0 V base address"]
pub mod vdo_h264_s_refv0_base_addr;
#[doc = "VDO_H264_S_REFY1_BASE_ADDR (rw) register accessor: Secondary-stream reference frame 1 Y base address\n\nYou can [`read`](crate::Reg::read) this register and get [`vdo_h264_s_refy1_base_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vdo_h264_s_refy1_base_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vdo_h264_s_refy1_base_addr`] module"]
#[doc(alias = "VDO_H264_S_REFY1_BASE_ADDR")]
pub type VdoH264SRefy1BaseAddr = crate::Reg<vdo_h264_s_refy1_base_addr::VdoH264SRefy1BaseAddrSpec>;
#[doc = "Secondary-stream reference frame 1 Y base address"]
pub mod vdo_h264_s_refy1_base_addr;
#[doc = "VDO_H264_S_REFU1_BASE_ADDR (rw) register accessor: Secondary-stream reference frame 1 U base address\n\nYou can [`read`](crate::Reg::read) this register and get [`vdo_h264_s_refu1_base_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vdo_h264_s_refu1_base_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vdo_h264_s_refu1_base_addr`] module"]
#[doc(alias = "VDO_H264_S_REFU1_BASE_ADDR")]
pub type VdoH264SRefu1BaseAddr = crate::Reg<vdo_h264_s_refu1_base_addr::VdoH264SRefu1BaseAddrSpec>;
#[doc = "Secondary-stream reference frame 1 U base address"]
pub mod vdo_h264_s_refu1_base_addr;
#[doc = "VDO_H264_S_REFV1_BASE_ADDR (rw) register accessor: Secondary-stream reference frame 1 V base address\n\nYou can [`read`](crate::Reg::read) this register and get [`vdo_h264_s_refv1_base_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vdo_h264_s_refv1_base_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vdo_h264_s_refv1_base_addr`] module"]
#[doc(alias = "VDO_H264_S_REFV1_BASE_ADDR")]
pub type VdoH264SRefv1BaseAddr = crate::Reg<vdo_h264_s_refv1_base_addr::VdoH264SRefv1BaseAddrSpec>;
#[doc = "Secondary-stream reference frame 1 V base address"]
pub mod vdo_h264_s_refv1_base_addr;
#[doc = "VDO_H264_S_EXT_BASE_ADDR (rw) register accessor: Secondary-stream external data base address\n\nYou can [`read`](crate::Reg::read) this register and get [`vdo_h264_s_ext_base_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vdo_h264_s_ext_base_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vdo_h264_s_ext_base_addr`] module"]
#[doc(alias = "VDO_H264_S_EXT_BASE_ADDR")]
pub type VdoH264SExtBaseAddr = crate::Reg<vdo_h264_s_ext_base_addr::VdoH264SExtBaseAddrSpec>;
#[doc = "Secondary-stream external data base address"]
pub mod vdo_h264_s_ext_base_addr;
#[doc = "VDO_H264_HWBACK_MODE (rw) register accessor: H264 hardware back-write mode control\n\nYou can [`read`](crate::Reg::read) this register and get [`vdo_h264_hwback_mode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vdo_h264_hwback_mode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vdo_h264_hwback_mode`] module"]
#[doc(alias = "VDO_H264_HWBACK_MODE")]
pub type VdoH264HwbackMode = crate::Reg<vdo_h264_hwback_mode::VdoH264HwbackModeSpec>;
#[doc = "H264 hardware back-write mode control"]
pub mod vdo_h264_hwback_mode;
#[doc = "VDO_H264_ROI_MODE (rw) register accessor: ROI mode control: enable bits and update trigger\n\nYou can [`read`](crate::Reg::read) this register and get [`vdo_h264_roi_mode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vdo_h264_roi_mode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vdo_h264_roi_mode`] module"]
#[doc(alias = "VDO_H264_ROI_MODE")]
pub type VdoH264RoiMode = crate::Reg<vdo_h264_roi_mode::VdoH264RoiModeSpec>;
#[doc = "ROI mode control: enable bits and update trigger"]
pub mod vdo_h264_roi_mode;
#[doc = "VDO_H264_ROI (rw) register accessor: ROI %s macroblock coordinate bounds (MBX_S/E, MBY_S/E)\n\nYou can [`read`](crate::Reg::read) this register and get [`vdo_h264_roi::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vdo_h264_roi::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vdo_h264_roi`] module"]
#[doc(alias = "VDO_H264_ROI")]
pub type VdoH264Roi = crate::Reg<vdo_h264_roi::VdoH264RoiSpec>;
#[doc = "ROI %s macroblock coordinate bounds (MBX_S/E, MBY_S/E)"]
pub mod vdo_h264_roi;
