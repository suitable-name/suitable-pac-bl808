#[doc = "Register `MJPEG_SWAP_BIT_CNT` reader"]
pub type R = crate::R<MjpegSwapBitCntSpec>;
#[doc = "Field `frame_swap_end_bit_cnt` reader - Bit count of the currently completed frame in the swap buffer."]
pub type FrameSwapEndBitCntR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Bit count of the currently completed frame in the swap buffer."]
    #[inline(always)]
    pub fn frame_swap_end_bit_cnt(&self) -> FrameSwapEndBitCntR {
        FrameSwapEndBitCntR::new(self.bits)
    }
}
#[doc = "Swap Buffer Bit Counter\n\nYou can [`read`](crate::Reg::read) this register and get [`mjpeg_swap_bit_cnt::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MjpegSwapBitCntSpec;
impl crate::RegisterSpec for MjpegSwapBitCntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mjpeg_swap_bit_cnt::R`](R) reader structure"]
impl crate::Readable for MjpegSwapBitCntSpec {}
#[doc = "`reset()` method sets MJPEG_SWAP_BIT_CNT to value 0"]
impl crate::Resettable for MjpegSwapBitCntSpec {}
