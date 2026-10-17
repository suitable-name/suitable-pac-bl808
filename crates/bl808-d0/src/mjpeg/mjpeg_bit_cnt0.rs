#[doc = "Register `MJPEG_BIT_CNT0` reader"]
pub type R = crate::R<MjpegBitCnt0Spec>;
#[doc = "Field `frame_bit_cnt_0` reader - MJPEG bit count for frame 0."]
pub type FrameBitCnt0R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - MJPEG bit count for frame 0."]
    #[inline(always)]
    pub fn frame_bit_cnt_0(&self) -> FrameBitCnt0R {
        FrameBitCnt0R::new(self.bits)
    }
}
#[doc = "MJPEG Bit Count 0\n\nYou can [`read`](crate::Reg::read) this register and get [`mjpeg_bit_cnt0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MjpegBitCnt0Spec;
impl crate::RegisterSpec for MjpegBitCnt0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mjpeg_bit_cnt0::R`](R) reader structure"]
impl crate::Readable for MjpegBitCnt0Spec {}
#[doc = "`reset()` method sets MJPEG_BIT_CNT0 to value 0"]
impl crate::Resettable for MjpegBitCnt0Spec {}
