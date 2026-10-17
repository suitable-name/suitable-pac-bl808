#[doc = "Register `MJPEG_FRAME_W_HBLK_STATUS` reader"]
pub type R = crate::R<MjpegFrameWHblkStatusSpec>;
#[doc = "Field `yy_frm_hblk_w` reader - Y frame horizontal block count (write pointer)."]
pub type YyFrmHblkWR = crate::FieldReader<u16>;
#[doc = "Field `uv_frm_hblk_w` reader - UV frame horizontal block count (write pointer)."]
pub type UvFrmHblkWR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:12 - Y frame horizontal block count (write pointer)."]
    #[inline(always)]
    pub fn yy_frm_hblk_w(&self) -> YyFrmHblkWR {
        YyFrmHblkWR::new((self.bits & 0x1fff) as u16)
    }
    #[doc = "Bits 16:28 - UV frame horizontal block count (write pointer)."]
    #[inline(always)]
    pub fn uv_frm_hblk_w(&self) -> UvFrmHblkWR {
        UvFrmHblkWR::new(((self.bits >> 16) & 0x1fff) as u16)
    }
}
#[doc = "Frame Write Horizontal Block Status: Y and UV write horizontal block counters\n\nYou can [`read`](crate::Reg::read) this register and get [`mjpeg_frame_w_hblk_status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MjpegFrameWHblkStatusSpec;
impl crate::RegisterSpec for MjpegFrameWHblkStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mjpeg_frame_w_hblk_status::R`](R) reader structure"]
impl crate::Readable for MjpegFrameWHblkStatusSpec {}
#[doc = "`reset()` method sets MJPEG_FRAME_W_HBLK_STATUS to value 0"]
impl crate::Resettable for MjpegFrameWHblkStatusSpec {}
