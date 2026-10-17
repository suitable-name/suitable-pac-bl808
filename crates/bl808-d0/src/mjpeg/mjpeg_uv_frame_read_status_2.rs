#[doc = "Register `MJPEG_UV_FRAME_READ_STATUS_2` reader"]
pub type R = crate::R<MjpegUvFrameReadStatus2Spec>;
#[doc = "Field `uv_wblk_r` reader - UV write-block read counter."]
pub type UvWblkRR = crate::FieldReader<u16>;
#[doc = "Field `uv_mem_rnd_r` reader - UV memory round counter (read pointer)."]
pub type UvMemRndRR = crate::FieldReader;
#[doc = "Field `uv_frm_cnt_r` reader - UV frame count (read pointer)."]
pub type UvFrmCntRR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:12 - UV write-block read counter."]
    #[inline(always)]
    pub fn uv_wblk_r(&self) -> UvWblkRR {
        UvWblkRR::new((self.bits & 0x1fff) as u16)
    }
    #[doc = "Bits 16:23 - UV memory round counter (read pointer)."]
    #[inline(always)]
    pub fn uv_mem_rnd_r(&self) -> UvMemRndRR {
        UvMemRndRR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - UV frame count (read pointer)."]
    #[inline(always)]
    pub fn uv_frm_cnt_r(&self) -> UvFrmCntRR {
        UvFrmCntRR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "UV Component Frame Read Status 2: Block and Round Counters\n\nYou can [`read`](crate::Reg::read) this register and get [`mjpeg_uv_frame_read_status_2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MjpegUvFrameReadStatus2Spec;
impl crate::RegisterSpec for MjpegUvFrameReadStatus2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mjpeg_uv_frame_read_status_2::R`](R) reader structure"]
impl crate::Readable for MjpegUvFrameReadStatus2Spec {}
#[doc = "`reset()` method sets MJPEG_UV_FRAME_READ_STATUS_2 to value 0"]
impl crate::Resettable for MjpegUvFrameReadStatus2Spec {}
