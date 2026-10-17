#[doc = "Register `MJPEG_UV_FRAME_READ_STATUS_1` reader"]
pub type R = crate::R<MjpegUvFrameReadStatus1Spec>;
#[doc = "Field `uv_mem_hblk_r` reader - UV memory horizontal block count (read pointer)."]
pub type UvMemHblkRR = crate::FieldReader<u16>;
#[doc = "Field `uv_frm_hblk_r` reader - UV frame horizontal block count (read pointer)."]
pub type UvFrmHblkRR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:12 - UV memory horizontal block count (read pointer)."]
    #[inline(always)]
    pub fn uv_mem_hblk_r(&self) -> UvMemHblkRR {
        UvMemHblkRR::new((self.bits & 0x1fff) as u16)
    }
    #[doc = "Bits 16:28 - UV frame horizontal block count (read pointer)."]
    #[inline(always)]
    pub fn uv_frm_hblk_r(&self) -> UvFrmHblkRR {
        UvFrmHblkRR::new(((self.bits >> 16) & 0x1fff) as u16)
    }
}
#[doc = "UV Component Frame Read Status 1: Memory and Frame Horizontal Block Count (read pointer)\n\nYou can [`read`](crate::Reg::read) this register and get [`mjpeg_uv_frame_read_status_1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MjpegUvFrameReadStatus1Spec;
impl crate::RegisterSpec for MjpegUvFrameReadStatus1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mjpeg_uv_frame_read_status_1::R`](R) reader structure"]
impl crate::Readable for MjpegUvFrameReadStatus1Spec {}
#[doc = "`reset()` method sets MJPEG_UV_FRAME_READ_STATUS_1 to value 0"]
impl crate::Resettable for MjpegUvFrameReadStatus1Spec {}
