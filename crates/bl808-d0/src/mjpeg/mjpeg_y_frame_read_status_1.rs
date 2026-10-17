#[doc = "Register `MJPEG_Y_FRAME_READ_STATUS_1` reader"]
pub type R = crate::R<MjpegYFrameReadStatus1Spec>;
#[doc = "Field `yy_mem_hblk_r` reader - Y memory horizontal block count (read pointer)."]
pub type YyMemHblkRR = crate::FieldReader<u16>;
#[doc = "Field `yy_frm_hblk_r` reader - Y frame horizontal block count (read pointer)."]
pub type YyFrmHblkRR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:12 - Y memory horizontal block count (read pointer)."]
    #[inline(always)]
    pub fn yy_mem_hblk_r(&self) -> YyMemHblkRR {
        YyMemHblkRR::new((self.bits & 0x1fff) as u16)
    }
    #[doc = "Bits 16:28 - Y frame horizontal block count (read pointer)."]
    #[inline(always)]
    pub fn yy_frm_hblk_r(&self) -> YyFrmHblkRR {
        YyFrmHblkRR::new(((self.bits >> 16) & 0x1fff) as u16)
    }
}
#[doc = "Y Component Frame Read Status 1: Memory and Frame Horizontal Block Count (read pointer)\n\nYou can [`read`](crate::Reg::read) this register and get [`mjpeg_y_frame_read_status_1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MjpegYFrameReadStatus1Spec;
impl crate::RegisterSpec for MjpegYFrameReadStatus1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mjpeg_y_frame_read_status_1::R`](R) reader structure"]
impl crate::Readable for MjpegYFrameReadStatus1Spec {}
#[doc = "`reset()` method sets MJPEG_Y_FRAME_READ_STATUS_1 to value 0"]
impl crate::Resettable for MjpegYFrameReadStatus1Spec {}
