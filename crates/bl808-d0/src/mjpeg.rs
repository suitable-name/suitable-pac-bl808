#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    mjpeg_control_1: MjpegControl1,
    mjpeg_control_2: MjpegControl2,
    mjpeg_yy_frame_addr: MjpegYyFrameAddr,
    mjpeg_uv_frame_addr: MjpegUvFrameAddr,
    mjpeg_yuv_mem: MjpegYuvMem,
    jpeg_frame_addr: JpegFrameAddr,
    jpeg_store_memory: JpegStoreMemory,
    mjpeg_control_3: MjpegControl3,
    mjpeg_frame_fifo_pop: MjpegFrameFifoPop,
    mjpeg_frame_size: MjpegFrameSize,
    mjpeg_header_byte: MjpegHeaderByte,
    _reserved11: [u8; 0x04],
    mjpeg_swap_mode: MjpegSwapMode,
    mjpeg_swap_bit_cnt: MjpegSwapBitCnt,
    mjpeg_yuv_mem_sw: MjpegYuvMemSw,
    _reserved14: [u8; 0x04],
    mjpeg_y_frame_read_status_1: MjpegYFrameReadStatus1,
    mjpeg_y_frame_read_status_2: MjpegYFrameReadStatus2,
    mjpeg_y_frame_write_status: MjpegYFrameWriteStatus,
    mjpeg_uv_frame_read_status_1: MjpegUvFrameReadStatus1,
    mjpeg_uv_frame_read_status_2: MjpegUvFrameReadStatus2,
    mjpeg_uv_frame_write_status: MjpegUvFrameWriteStatus,
    mjpeg_frame_w_hblk_status: MjpegFrameWHblkStatus,
    _reserved21: [u8; 0x24],
    mjpeg_start_addr0: MjpegStartAddr0,
    mjpeg_bit_cnt0: MjpegBitCnt0,
    mjpeg_start_addr1: MjpegStartAddr1,
    mjpeg_bit_cnt1: MjpegBitCnt1,
    mjpeg_start_addr2: MjpegStartAddr2,
    mjpeg_bit_cnt2: MjpegBitCnt2,
    mjpeg_start_addr3: MjpegStartAddr3,
    mjpeg_bit_cnt3: MjpegBitCnt3,
    _reserved29: [u8; 0x60],
    mjpeg_q_enc: MjpegQEnc,
    _reserved30: [u8; 0x0c],
    mjpeg_frame_id_10: MjpegFrameId10,
    mjpeg_frame_id_32: MjpegFrameId32,
    _reserved32: [u8; 0xd8],
    mjpeg_debug: MjpegDebug,
    _reserved33: [u8; 0x08],
    mjpeg_dummy_reg: MjpegDummyReg,
    _reserved34: [u8; 0x0200],
    mjpeg_q_param_: [MjpegQParam_; 64],
}
impl RegisterBlock {
    #[doc = "0x00 - MJPEG Control Register 1: Enable, Bit Order, and Burst Length"]
    #[inline(always)]
    pub const fn mjpeg_control_1(&self) -> &MjpegControl1 {
        &self.mjpeg_control_1
    }
    #[doc = "0x04 - MJPEG Control Register 2: SW Mode, Kick, and DVP Select"]
    #[inline(always)]
    pub const fn mjpeg_control_2(&self) -> &MjpegControl2 {
        &self.mjpeg_control_2
    }
    #[doc = "0x08 - Y Component Input Frame Buffer Start Address"]
    #[inline(always)]
    pub const fn mjpeg_yy_frame_addr(&self) -> &MjpegYyFrameAddr {
        &self.mjpeg_yy_frame_addr
    }
    #[doc = "0x0c - UV Component Input Frame Buffer Start Address"]
    #[inline(always)]
    pub const fn mjpeg_uv_frame_addr(&self) -> &MjpegUvFrameAddr {
        &self.mjpeg_uv_frame_addr
    }
    #[doc = "0x10 - YUV Memory Block Count Configuration"]
    #[inline(always)]
    pub const fn mjpeg_yuv_mem(&self) -> &MjpegYuvMem {
        &self.mjpeg_yuv_mem
    }
    #[doc = "0x14 - Output JPEG Frame Buffer Start Address"]
    #[inline(always)]
    pub const fn jpeg_frame_addr(&self) -> &JpegFrameAddr {
        &self.jpeg_frame_addr
    }
    #[doc = "0x18 - JPEG Output Burst Count Configuration"]
    #[inline(always)]
    pub const fn jpeg_store_memory(&self) -> &JpegStoreMemory {
        &self.jpeg_store_memory
    }
    #[doc = "0x1c - MJPEG Interrupt, Status, and Frame Count Register"]
    #[inline(always)]
    pub const fn mjpeg_control_3(&self) -> &MjpegControl3 {
        &self.mjpeg_control_3
    }
    #[doc = "0x20 - Interrupt Clear and FIFO Pop Register"]
    #[inline(always)]
    pub const fn mjpeg_frame_fifo_pop(&self) -> &MjpegFrameFifoPop {
        &self.mjpeg_frame_fifo_pop
    }
    #[doc = "0x24 - MJPEG Resolution Configuration (Blocks)"]
    #[inline(always)]
    pub const fn mjpeg_frame_size(&self) -> &MjpegFrameSize {
        &self.mjpeg_frame_size
    }
    #[doc = "0x28 - MJPEG Header Byte Count and Component Order"]
    #[inline(always)]
    pub const fn mjpeg_header_byte(&self) -> &MjpegHeaderByte {
        &self.mjpeg_header_byte
    }
    #[doc = "0x30 - Buffer Swap Configuration and Status"]
    #[inline(always)]
    pub const fn mjpeg_swap_mode(&self) -> &MjpegSwapMode {
        &self.mjpeg_swap_mode
    }
    #[doc = "0x34 - Swap Buffer Bit Counter"]
    #[inline(always)]
    pub const fn mjpeg_swap_bit_cnt(&self) -> &MjpegSwapBitCnt {
        &self.mjpeg_swap_bit_cnt
    }
    #[doc = "0x38 - Software Kick Row Count"]
    #[inline(always)]
    pub const fn mjpeg_yuv_mem_sw(&self) -> &MjpegYuvMemSw {
        &self.mjpeg_yuv_mem_sw
    }
    #[doc = "0x40 - Y Component Frame Read Status 1: Memory and Frame Horizontal Block Count (read pointer)"]
    #[inline(always)]
    pub const fn mjpeg_y_frame_read_status_1(&self) -> &MjpegYFrameReadStatus1 {
        &self.mjpeg_y_frame_read_status_1
    }
    #[doc = "0x44 - Y Component Frame Read Status 2: Block and Round Counters"]
    #[inline(always)]
    pub const fn mjpeg_y_frame_read_status_2(&self) -> &MjpegYFrameReadStatus2 {
        &self.mjpeg_y_frame_read_status_2
    }
    #[doc = "0x48 - Y Component Frame Write Status: Block and Round Counters"]
    #[inline(always)]
    pub const fn mjpeg_y_frame_write_status(&self) -> &MjpegYFrameWriteStatus {
        &self.mjpeg_y_frame_write_status
    }
    #[doc = "0x4c - UV Component Frame Read Status 1: Memory and Frame Horizontal Block Count (read pointer)"]
    #[inline(always)]
    pub const fn mjpeg_uv_frame_read_status_1(&self) -> &MjpegUvFrameReadStatus1 {
        &self.mjpeg_uv_frame_read_status_1
    }
    #[doc = "0x50 - UV Component Frame Read Status 2: Block and Round Counters"]
    #[inline(always)]
    pub const fn mjpeg_uv_frame_read_status_2(&self) -> &MjpegUvFrameReadStatus2 {
        &self.mjpeg_uv_frame_read_status_2
    }
    #[doc = "0x54 - UV Component Frame Write Status: Block and Round Counters"]
    #[inline(always)]
    pub const fn mjpeg_uv_frame_write_status(&self) -> &MjpegUvFrameWriteStatus {
        &self.mjpeg_uv_frame_write_status
    }
    #[doc = "0x58 - Frame Write Horizontal Block Status: Y and UV write horizontal block counters"]
    #[inline(always)]
    pub const fn mjpeg_frame_w_hblk_status(&self) -> &MjpegFrameWHblkStatus {
        &self.mjpeg_frame_w_hblk_status
    }
    #[doc = "0x80 - MJPEG Start Address 0"]
    #[inline(always)]
    pub const fn mjpeg_start_addr0(&self) -> &MjpegStartAddr0 {
        &self.mjpeg_start_addr0
    }
    #[doc = "0x84 - MJPEG Bit Count 0"]
    #[inline(always)]
    pub const fn mjpeg_bit_cnt0(&self) -> &MjpegBitCnt0 {
        &self.mjpeg_bit_cnt0
    }
    #[doc = "0x88 - MJPEG Start Address 1"]
    #[inline(always)]
    pub const fn mjpeg_start_addr1(&self) -> &MjpegStartAddr1 {
        &self.mjpeg_start_addr1
    }
    #[doc = "0x8c - MJPEG Bit Count 1"]
    #[inline(always)]
    pub const fn mjpeg_bit_cnt1(&self) -> &MjpegBitCnt1 {
        &self.mjpeg_bit_cnt1
    }
    #[doc = "0x90 - MJPEG Start Address 2"]
    #[inline(always)]
    pub const fn mjpeg_start_addr2(&self) -> &MjpegStartAddr2 {
        &self.mjpeg_start_addr2
    }
    #[doc = "0x94 - MJPEG Bit Count 2"]
    #[inline(always)]
    pub const fn mjpeg_bit_cnt2(&self) -> &MjpegBitCnt2 {
        &self.mjpeg_bit_cnt2
    }
    #[doc = "0x98 - MJPEG Start Address 3"]
    #[inline(always)]
    pub const fn mjpeg_start_addr3(&self) -> &MjpegStartAddr3 {
        &self.mjpeg_start_addr3
    }
    #[doc = "0x9c - MJPEG Bit Count 3"]
    #[inline(always)]
    pub const fn mjpeg_bit_cnt3(&self) -> &MjpegBitCnt3 {
        &self.mjpeg_bit_cnt3
    }
    #[doc = "0x100 - Quantization Table Encoding Status and Control"]
    #[inline(always)]
    pub const fn mjpeg_q_enc(&self) -> &MjpegQEnc {
        &self.mjpeg_q_enc
    }
    #[doc = "0x110 - MJPEG Frame ID Status 0 and 1"]
    #[inline(always)]
    pub const fn mjpeg_frame_id_10(&self) -> &MjpegFrameId10 {
        &self.mjpeg_frame_id_10
    }
    #[doc = "0x114 - MJPEG Frame ID Status 2 and 3"]
    #[inline(always)]
    pub const fn mjpeg_frame_id_32(&self) -> &MjpegFrameId32 {
        &self.mjpeg_frame_id_32
    }
    #[doc = "0x1f0 - MJPEG Debug Configuration"]
    #[inline(always)]
    pub const fn mjpeg_debug(&self) -> &MjpegDebug {
        &self.mjpeg_debug
    }
    #[doc = "0x1fc - MJPEG Dummy Register"]
    #[inline(always)]
    pub const fn mjpeg_dummy_reg(&self) -> &MjpegDummyReg {
        &self.mjpeg_dummy_reg
    }
    #[doc = "0x400..0x500 - Quantization Table Parameter Register (index %s)"]
    #[inline(always)]
    pub const fn mjpeg_q_param_(&self, n: usize) -> &MjpegQParam_ {
        &self.mjpeg_q_param_[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x400..0x500 - Quantization Table Parameter Register (index %s)"]
    #[inline(always)]
    pub fn mjpeg_q_param__iter(&self) -> impl Iterator<Item = &MjpegQParam_> {
        self.mjpeg_q_param_.iter()
    }
}
#[doc = "MJPEG_CONTROL_1 (rw) register accessor: MJPEG Control Register 1: Enable, Bit Order, and Burst Length\n\nYou can [`read`](crate::Reg::read) this register and get [`mjpeg_control_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mjpeg_control_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mjpeg_control_1`] module"]
#[doc(alias = "MJPEG_CONTROL_1")]
pub type MjpegControl1 = crate::Reg<mjpeg_control_1::MjpegControl1Spec>;
#[doc = "MJPEG Control Register 1: Enable, Bit Order, and Burst Length"]
pub mod mjpeg_control_1;
#[doc = "MJPEG_CONTROL_2 (rw) register accessor: MJPEG Control Register 2: SW Mode, Kick, and DVP Select\n\nYou can [`read`](crate::Reg::read) this register and get [`mjpeg_control_2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mjpeg_control_2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mjpeg_control_2`] module"]
#[doc(alias = "MJPEG_CONTROL_2")]
pub type MjpegControl2 = crate::Reg<mjpeg_control_2::MjpegControl2Spec>;
#[doc = "MJPEG Control Register 2: SW Mode, Kick, and DVP Select"]
pub mod mjpeg_control_2;
#[doc = "MJPEG_YY_FRAME_ADDR (rw) register accessor: Y Component Input Frame Buffer Start Address\n\nYou can [`read`](crate::Reg::read) this register and get [`mjpeg_yy_frame_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mjpeg_yy_frame_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mjpeg_yy_frame_addr`] module"]
#[doc(alias = "MJPEG_YY_FRAME_ADDR")]
pub type MjpegYyFrameAddr = crate::Reg<mjpeg_yy_frame_addr::MjpegYyFrameAddrSpec>;
#[doc = "Y Component Input Frame Buffer Start Address"]
pub mod mjpeg_yy_frame_addr;
#[doc = "MJPEG_UV_FRAME_ADDR (rw) register accessor: UV Component Input Frame Buffer Start Address\n\nYou can [`read`](crate::Reg::read) this register and get [`mjpeg_uv_frame_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mjpeg_uv_frame_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mjpeg_uv_frame_addr`] module"]
#[doc(alias = "MJPEG_UV_FRAME_ADDR")]
pub type MjpegUvFrameAddr = crate::Reg<mjpeg_uv_frame_addr::MjpegUvFrameAddrSpec>;
#[doc = "UV Component Input Frame Buffer Start Address"]
pub mod mjpeg_uv_frame_addr;
#[doc = "MJPEG_YUV_MEM (rw) register accessor: YUV Memory Block Count Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`mjpeg_yuv_mem::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mjpeg_yuv_mem::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mjpeg_yuv_mem`] module"]
#[doc(alias = "MJPEG_YUV_MEM")]
pub type MjpegYuvMem = crate::Reg<mjpeg_yuv_mem::MjpegYuvMemSpec>;
#[doc = "YUV Memory Block Count Configuration"]
pub mod mjpeg_yuv_mem;
#[doc = "JPEG_FRAME_ADDR (rw) register accessor: Output JPEG Frame Buffer Start Address\n\nYou can [`read`](crate::Reg::read) this register and get [`jpeg_frame_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`jpeg_frame_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@jpeg_frame_addr`] module"]
#[doc(alias = "JPEG_FRAME_ADDR")]
pub type JpegFrameAddr = crate::Reg<jpeg_frame_addr::JpegFrameAddrSpec>;
#[doc = "Output JPEG Frame Buffer Start Address"]
pub mod jpeg_frame_addr;
#[doc = "JPEG_STORE_MEMORY (rw) register accessor: JPEG Output Burst Count Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`jpeg_store_memory::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`jpeg_store_memory::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@jpeg_store_memory`] module"]
#[doc(alias = "JPEG_STORE_MEMORY")]
pub type JpegStoreMemory = crate::Reg<jpeg_store_memory::JpegStoreMemorySpec>;
#[doc = "JPEG Output Burst Count Configuration"]
pub mod jpeg_store_memory;
#[doc = "MJPEG_CONTROL_3 (rw) register accessor: MJPEG Interrupt, Status, and Frame Count Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mjpeg_control_3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mjpeg_control_3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mjpeg_control_3`] module"]
#[doc(alias = "MJPEG_CONTROL_3")]
pub type MjpegControl3 = crate::Reg<mjpeg_control_3::MjpegControl3Spec>;
#[doc = "MJPEG Interrupt, Status, and Frame Count Register"]
pub mod mjpeg_control_3;
#[doc = "MJPEG_FRAME_FIFO_POP (w) register accessor: Interrupt Clear and FIFO Pop Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mjpeg_frame_fifo_pop::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mjpeg_frame_fifo_pop`] module"]
#[doc(alias = "MJPEG_FRAME_FIFO_POP")]
pub type MjpegFrameFifoPop = crate::Reg<mjpeg_frame_fifo_pop::MjpegFrameFifoPopSpec>;
#[doc = "Interrupt Clear and FIFO Pop Register"]
pub mod mjpeg_frame_fifo_pop;
#[doc = "MJPEG_FRAME_SIZE (rw) register accessor: MJPEG Resolution Configuration (Blocks)\n\nYou can [`read`](crate::Reg::read) this register and get [`mjpeg_frame_size::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mjpeg_frame_size::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mjpeg_frame_size`] module"]
#[doc(alias = "MJPEG_FRAME_SIZE")]
pub type MjpegFrameSize = crate::Reg<mjpeg_frame_size::MjpegFrameSizeSpec>;
#[doc = "MJPEG Resolution Configuration (Blocks)"]
pub mod mjpeg_frame_size;
#[doc = "MJPEG_HEADER_BYTE (rw) register accessor: MJPEG Header Byte Count and Component Order\n\nYou can [`read`](crate::Reg::read) this register and get [`mjpeg_header_byte::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mjpeg_header_byte::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mjpeg_header_byte`] module"]
#[doc(alias = "MJPEG_HEADER_BYTE")]
pub type MjpegHeaderByte = crate::Reg<mjpeg_header_byte::MjpegHeaderByteSpec>;
#[doc = "MJPEG Header Byte Count and Component Order"]
pub mod mjpeg_header_byte;
#[doc = "MJPEG_SWAP_MODE (rw) register accessor: Buffer Swap Configuration and Status\n\nYou can [`read`](crate::Reg::read) this register and get [`mjpeg_swap_mode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mjpeg_swap_mode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mjpeg_swap_mode`] module"]
#[doc(alias = "MJPEG_SWAP_MODE")]
pub type MjpegSwapMode = crate::Reg<mjpeg_swap_mode::MjpegSwapModeSpec>;
#[doc = "Buffer Swap Configuration and Status"]
pub mod mjpeg_swap_mode;
#[doc = "MJPEG_SWAP_BIT_CNT (r) register accessor: Swap Buffer Bit Counter\n\nYou can [`read`](crate::Reg::read) this register and get [`mjpeg_swap_bit_cnt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mjpeg_swap_bit_cnt`] module"]
#[doc(alias = "MJPEG_SWAP_BIT_CNT")]
pub type MjpegSwapBitCnt = crate::Reg<mjpeg_swap_bit_cnt::MjpegSwapBitCntSpec>;
#[doc = "Swap Buffer Bit Counter"]
pub mod mjpeg_swap_bit_cnt;
#[doc = "MJPEG_YUV_MEM_SW (rw) register accessor: Software Kick Row Count\n\nYou can [`read`](crate::Reg::read) this register and get [`mjpeg_yuv_mem_sw::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mjpeg_yuv_mem_sw::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mjpeg_yuv_mem_sw`] module"]
#[doc(alias = "MJPEG_YUV_MEM_SW")]
pub type MjpegYuvMemSw = crate::Reg<mjpeg_yuv_mem_sw::MjpegYuvMemSwSpec>;
#[doc = "Software Kick Row Count"]
pub mod mjpeg_yuv_mem_sw;
#[doc = "MJPEG_Y_FRAME_READ_STATUS_1 (r) register accessor: Y Component Frame Read Status 1: Memory and Frame Horizontal Block Count (read pointer)\n\nYou can [`read`](crate::Reg::read) this register and get [`mjpeg_y_frame_read_status_1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mjpeg_y_frame_read_status_1`] module"]
#[doc(alias = "MJPEG_Y_FRAME_READ_STATUS_1")]
pub type MjpegYFrameReadStatus1 =
    crate::Reg<mjpeg_y_frame_read_status_1::MjpegYFrameReadStatus1Spec>;
#[doc = "Y Component Frame Read Status 1: Memory and Frame Horizontal Block Count (read pointer)"]
pub mod mjpeg_y_frame_read_status_1;
#[doc = "MJPEG_Y_FRAME_READ_STATUS_2 (r) register accessor: Y Component Frame Read Status 2: Block and Round Counters\n\nYou can [`read`](crate::Reg::read) this register and get [`mjpeg_y_frame_read_status_2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mjpeg_y_frame_read_status_2`] module"]
#[doc(alias = "MJPEG_Y_FRAME_READ_STATUS_2")]
pub type MjpegYFrameReadStatus2 =
    crate::Reg<mjpeg_y_frame_read_status_2::MjpegYFrameReadStatus2Spec>;
#[doc = "Y Component Frame Read Status 2: Block and Round Counters"]
pub mod mjpeg_y_frame_read_status_2;
#[doc = "MJPEG_Y_FRAME_WRITE_STATUS (r) register accessor: Y Component Frame Write Status: Block and Round Counters\n\nYou can [`read`](crate::Reg::read) this register and get [`mjpeg_y_frame_write_status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mjpeg_y_frame_write_status`] module"]
#[doc(alias = "MJPEG_Y_FRAME_WRITE_STATUS")]
pub type MjpegYFrameWriteStatus =
    crate::Reg<mjpeg_y_frame_write_status::MjpegYFrameWriteStatusSpec>;
#[doc = "Y Component Frame Write Status: Block and Round Counters"]
pub mod mjpeg_y_frame_write_status;
#[doc = "MJPEG_UV_FRAME_READ_STATUS_1 (r) register accessor: UV Component Frame Read Status 1: Memory and Frame Horizontal Block Count (read pointer)\n\nYou can [`read`](crate::Reg::read) this register and get [`mjpeg_uv_frame_read_status_1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mjpeg_uv_frame_read_status_1`] module"]
#[doc(alias = "MJPEG_UV_FRAME_READ_STATUS_1")]
pub type MjpegUvFrameReadStatus1 =
    crate::Reg<mjpeg_uv_frame_read_status_1::MjpegUvFrameReadStatus1Spec>;
#[doc = "UV Component Frame Read Status 1: Memory and Frame Horizontal Block Count (read pointer)"]
pub mod mjpeg_uv_frame_read_status_1;
#[doc = "MJPEG_UV_FRAME_READ_STATUS_2 (r) register accessor: UV Component Frame Read Status 2: Block and Round Counters\n\nYou can [`read`](crate::Reg::read) this register and get [`mjpeg_uv_frame_read_status_2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mjpeg_uv_frame_read_status_2`] module"]
#[doc(alias = "MJPEG_UV_FRAME_READ_STATUS_2")]
pub type MjpegUvFrameReadStatus2 =
    crate::Reg<mjpeg_uv_frame_read_status_2::MjpegUvFrameReadStatus2Spec>;
#[doc = "UV Component Frame Read Status 2: Block and Round Counters"]
pub mod mjpeg_uv_frame_read_status_2;
#[doc = "MJPEG_UV_FRAME_WRITE_STATUS (r) register accessor: UV Component Frame Write Status: Block and Round Counters\n\nYou can [`read`](crate::Reg::read) this register and get [`mjpeg_uv_frame_write_status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mjpeg_uv_frame_write_status`] module"]
#[doc(alias = "MJPEG_UV_FRAME_WRITE_STATUS")]
pub type MjpegUvFrameWriteStatus =
    crate::Reg<mjpeg_uv_frame_write_status::MjpegUvFrameWriteStatusSpec>;
#[doc = "UV Component Frame Write Status: Block and Round Counters"]
pub mod mjpeg_uv_frame_write_status;
#[doc = "MJPEG_FRAME_W_HBLK_STATUS (r) register accessor: Frame Write Horizontal Block Status: Y and UV write horizontal block counters\n\nYou can [`read`](crate::Reg::read) this register and get [`mjpeg_frame_w_hblk_status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mjpeg_frame_w_hblk_status`] module"]
#[doc(alias = "MJPEG_FRAME_W_HBLK_STATUS")]
pub type MjpegFrameWHblkStatus = crate::Reg<mjpeg_frame_w_hblk_status::MjpegFrameWHblkStatusSpec>;
#[doc = "Frame Write Horizontal Block Status: Y and UV write horizontal block counters"]
pub mod mjpeg_frame_w_hblk_status;
#[doc = "MJPEG_START_ADDR0 (r) register accessor: MJPEG Start Address 0\n\nYou can [`read`](crate::Reg::read) this register and get [`mjpeg_start_addr0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mjpeg_start_addr0`] module"]
#[doc(alias = "MJPEG_START_ADDR0")]
pub type MjpegStartAddr0 = crate::Reg<mjpeg_start_addr0::MjpegStartAddr0Spec>;
#[doc = "MJPEG Start Address 0"]
pub mod mjpeg_start_addr0;
#[doc = "MJPEG_BIT_CNT0 (r) register accessor: MJPEG Bit Count 0\n\nYou can [`read`](crate::Reg::read) this register and get [`mjpeg_bit_cnt0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mjpeg_bit_cnt0`] module"]
#[doc(alias = "MJPEG_BIT_CNT0")]
pub type MjpegBitCnt0 = crate::Reg<mjpeg_bit_cnt0::MjpegBitCnt0Spec>;
#[doc = "MJPEG Bit Count 0"]
pub mod mjpeg_bit_cnt0;
#[doc = "MJPEG_START_ADDR1 (r) register accessor: MJPEG Start Address 1\n\nYou can [`read`](crate::Reg::read) this register and get [`mjpeg_start_addr1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mjpeg_start_addr1`] module"]
#[doc(alias = "MJPEG_START_ADDR1")]
pub type MjpegStartAddr1 = crate::Reg<mjpeg_start_addr1::MjpegStartAddr1Spec>;
#[doc = "MJPEG Start Address 1"]
pub mod mjpeg_start_addr1;
#[doc = "MJPEG_BIT_CNT1 (r) register accessor: MJPEG Bit Count 1\n\nYou can [`read`](crate::Reg::read) this register and get [`mjpeg_bit_cnt1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mjpeg_bit_cnt1`] module"]
#[doc(alias = "MJPEG_BIT_CNT1")]
pub type MjpegBitCnt1 = crate::Reg<mjpeg_bit_cnt1::MjpegBitCnt1Spec>;
#[doc = "MJPEG Bit Count 1"]
pub mod mjpeg_bit_cnt1;
#[doc = "MJPEG_START_ADDR2 (r) register accessor: MJPEG Start Address 2\n\nYou can [`read`](crate::Reg::read) this register and get [`mjpeg_start_addr2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mjpeg_start_addr2`] module"]
#[doc(alias = "MJPEG_START_ADDR2")]
pub type MjpegStartAddr2 = crate::Reg<mjpeg_start_addr2::MjpegStartAddr2Spec>;
#[doc = "MJPEG Start Address 2"]
pub mod mjpeg_start_addr2;
#[doc = "MJPEG_BIT_CNT2 (r) register accessor: MJPEG Bit Count 2\n\nYou can [`read`](crate::Reg::read) this register and get [`mjpeg_bit_cnt2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mjpeg_bit_cnt2`] module"]
#[doc(alias = "MJPEG_BIT_CNT2")]
pub type MjpegBitCnt2 = crate::Reg<mjpeg_bit_cnt2::MjpegBitCnt2Spec>;
#[doc = "MJPEG Bit Count 2"]
pub mod mjpeg_bit_cnt2;
#[doc = "MJPEG_START_ADDR3 (r) register accessor: MJPEG Start Address 3\n\nYou can [`read`](crate::Reg::read) this register and get [`mjpeg_start_addr3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mjpeg_start_addr3`] module"]
#[doc(alias = "MJPEG_START_ADDR3")]
pub type MjpegStartAddr3 = crate::Reg<mjpeg_start_addr3::MjpegStartAddr3Spec>;
#[doc = "MJPEG Start Address 3"]
pub mod mjpeg_start_addr3;
#[doc = "MJPEG_BIT_CNT3 (r) register accessor: MJPEG Bit Count 3\n\nYou can [`read`](crate::Reg::read) this register and get [`mjpeg_bit_cnt3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mjpeg_bit_cnt3`] module"]
#[doc(alias = "MJPEG_BIT_CNT3")]
pub type MjpegBitCnt3 = crate::Reg<mjpeg_bit_cnt3::MjpegBitCnt3Spec>;
#[doc = "MJPEG Bit Count 3"]
pub mod mjpeg_bit_cnt3;
#[doc = "MJPEG_Q_ENC (rw) register accessor: Quantization Table Encoding Status and Control\n\nYou can [`read`](crate::Reg::read) this register and get [`mjpeg_q_enc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mjpeg_q_enc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mjpeg_q_enc`] module"]
#[doc(alias = "MJPEG_Q_ENC")]
pub type MjpegQEnc = crate::Reg<mjpeg_q_enc::MjpegQEncSpec>;
#[doc = "Quantization Table Encoding Status and Control"]
pub mod mjpeg_q_enc;
#[doc = "MJPEG_FRAME_ID_10 (r) register accessor: MJPEG Frame ID Status 0 and 1\n\nYou can [`read`](crate::Reg::read) this register and get [`mjpeg_frame_id_10::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mjpeg_frame_id_10`] module"]
#[doc(alias = "MJPEG_FRAME_ID_10")]
pub type MjpegFrameId10 = crate::Reg<mjpeg_frame_id_10::MjpegFrameId10Spec>;
#[doc = "MJPEG Frame ID Status 0 and 1"]
pub mod mjpeg_frame_id_10;
#[doc = "MJPEG_FRAME_ID_32 (r) register accessor: MJPEG Frame ID Status 2 and 3\n\nYou can [`read`](crate::Reg::read) this register and get [`mjpeg_frame_id_32::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mjpeg_frame_id_32`] module"]
#[doc(alias = "MJPEG_FRAME_ID_32")]
pub type MjpegFrameId32 = crate::Reg<mjpeg_frame_id_32::MjpegFrameId32Spec>;
#[doc = "MJPEG Frame ID Status 2 and 3"]
pub mod mjpeg_frame_id_32;
#[doc = "MJPEG_DEBUG (rw) register accessor: MJPEG Debug Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`mjpeg_debug::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mjpeg_debug::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mjpeg_debug`] module"]
#[doc(alias = "MJPEG_DEBUG")]
pub type MjpegDebug = crate::Reg<mjpeg_debug::MjpegDebugSpec>;
#[doc = "MJPEG Debug Configuration"]
pub mod mjpeg_debug;
#[doc = "MJPEG_DUMMY_REG (rw) register accessor: MJPEG Dummy Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mjpeg_dummy_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mjpeg_dummy_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mjpeg_dummy_reg`] module"]
#[doc(alias = "MJPEG_DUMMY_REG")]
pub type MjpegDummyReg = crate::Reg<mjpeg_dummy_reg::MjpegDummyRegSpec>;
#[doc = "MJPEG Dummy Register"]
pub mod mjpeg_dummy_reg;
#[doc = "MJPEG_Q_PARAM_ (rw) register accessor: Quantization Table Parameter Register (index %s)\n\nYou can [`read`](crate::Reg::read) this register and get [`mjpeg_q_param_::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mjpeg_q_param_::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mjpeg_q_param_`] module"]
#[doc(alias = "MJPEG_Q_PARAM_")]
pub type MjpegQParam_ = crate::Reg<mjpeg_q_param_::MjpegQParam_Spec>;
#[doc = "Quantization Table Parameter Register (index %s)"]
pub mod mjpeg_q_param_;
