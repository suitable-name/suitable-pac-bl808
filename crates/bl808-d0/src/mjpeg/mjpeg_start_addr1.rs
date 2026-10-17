#[doc = "Register `MJPEG_START_ADDR1` reader"]
pub type R = crate::R<MjpegStartAddr1Spec>;
#[doc = "Field `frame_start_addr_1` reader - MJPEG start address for frame 1."]
pub type FrameStartAddr1R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - MJPEG start address for frame 1."]
    #[inline(always)]
    pub fn frame_start_addr_1(&self) -> FrameStartAddr1R {
        FrameStartAddr1R::new(self.bits)
    }
}
#[doc = "MJPEG Start Address 1\n\nYou can [`read`](crate::Reg::read) this register and get [`mjpeg_start_addr1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MjpegStartAddr1Spec;
impl crate::RegisterSpec for MjpegStartAddr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mjpeg_start_addr1::R`](R) reader structure"]
impl crate::Readable for MjpegStartAddr1Spec {}
#[doc = "`reset()` method sets MJPEG_START_ADDR1 to value 0"]
impl crate::Resettable for MjpegStartAddr1Spec {}
