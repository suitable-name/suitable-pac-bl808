#[doc = "Register `MJPEG_Y_FRAME_READ_STATUS_2` reader"]
pub type R = crate::R<MjpegYFrameReadStatus2Spec>;
#[doc = "Field `yy_wblk_r` reader - Y write-block read counter."]
pub type YyWblkRR = crate::FieldReader<u16>;
#[doc = "Field `yy_mem_rnd_r` reader - Y memory round counter (read pointer)."]
pub type YyMemRndRR = crate::FieldReader;
#[doc = "Field `yy_frm_cnt_r` reader - Y frame count (read pointer)."]
pub type YyFrmCntRR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:12 - Y write-block read counter."]
    #[inline(always)]
    pub fn yy_wblk_r(&self) -> YyWblkRR {
        YyWblkRR::new((self.bits & 0x1fff) as u16)
    }
    #[doc = "Bits 16:23 - Y memory round counter (read pointer)."]
    #[inline(always)]
    pub fn yy_mem_rnd_r(&self) -> YyMemRndRR {
        YyMemRndRR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Y frame count (read pointer)."]
    #[inline(always)]
    pub fn yy_frm_cnt_r(&self) -> YyFrmCntRR {
        YyFrmCntRR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "Y Component Frame Read Status 2: Block and Round Counters\n\nYou can [`read`](crate::Reg::read) this register and get [`mjpeg_y_frame_read_status_2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MjpegYFrameReadStatus2Spec;
impl crate::RegisterSpec for MjpegYFrameReadStatus2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mjpeg_y_frame_read_status_2::R`](R) reader structure"]
impl crate::Readable for MjpegYFrameReadStatus2Spec {}
#[doc = "`reset()` method sets MJPEG_Y_FRAME_READ_STATUS_2 to value 0"]
impl crate::Resettable for MjpegYFrameReadStatus2Spec {}
