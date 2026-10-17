#[doc = "Register `MJPEG_BIT_CNT3` reader"]
pub type R = crate::R<MjpegBitCnt3Spec>;
#[doc = "Field `frame_bit_cnt_3` reader - MJPEG bit count for frame 3."]
pub type FrameBitCnt3R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - MJPEG bit count for frame 3."]
    #[inline(always)]
    pub fn frame_bit_cnt_3(&self) -> FrameBitCnt3R {
        FrameBitCnt3R::new(self.bits)
    }
}
#[doc = "MJPEG Bit Count 3\n\nYou can [`read`](crate::Reg::read) this register and get [`mjpeg_bit_cnt3::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MjpegBitCnt3Spec;
impl crate::RegisterSpec for MjpegBitCnt3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mjpeg_bit_cnt3::R`](R) reader structure"]
impl crate::Readable for MjpegBitCnt3Spec {}
#[doc = "`reset()` method sets MJPEG_BIT_CNT3 to value 0"]
impl crate::Resettable for MjpegBitCnt3Spec {}
