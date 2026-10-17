#[doc = "Register `MJPEG_BIT_CNT2` reader"]
pub type R = crate::R<MjpegBitCnt2Spec>;
#[doc = "Field `frame_bit_cnt_2` reader - MJPEG bit count for frame 2."]
pub type FrameBitCnt2R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - MJPEG bit count for frame 2."]
    #[inline(always)]
    pub fn frame_bit_cnt_2(&self) -> FrameBitCnt2R {
        FrameBitCnt2R::new(self.bits)
    }
}
#[doc = "MJPEG Bit Count 2\n\nYou can [`read`](crate::Reg::read) this register and get [`mjpeg_bit_cnt2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MjpegBitCnt2Spec;
impl crate::RegisterSpec for MjpegBitCnt2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mjpeg_bit_cnt2::R`](R) reader structure"]
impl crate::Readable for MjpegBitCnt2Spec {}
#[doc = "`reset()` method sets MJPEG_BIT_CNT2 to value 0"]
impl crate::Resettable for MjpegBitCnt2Spec {}
