#[doc = "Register `MJPEG_BIT_CNT1` reader"]
pub type R = crate::R<MjpegBitCnt1Spec>;
#[doc = "Field `frame_bit_cnt_1` reader - MJPEG bit count for frame 1."]
pub type FrameBitCnt1R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - MJPEG bit count for frame 1."]
    #[inline(always)]
    pub fn frame_bit_cnt_1(&self) -> FrameBitCnt1R {
        FrameBitCnt1R::new(self.bits)
    }
}
#[doc = "MJPEG Bit Count 1\n\nYou can [`read`](crate::Reg::read) this register and get [`mjpeg_bit_cnt1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MjpegBitCnt1Spec;
impl crate::RegisterSpec for MjpegBitCnt1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mjpeg_bit_cnt1::R`](R) reader structure"]
impl crate::Readable for MjpegBitCnt1Spec {}
#[doc = "`reset()` method sets MJPEG_BIT_CNT1 to value 0"]
impl crate::Resettable for MjpegBitCnt1Spec {}
