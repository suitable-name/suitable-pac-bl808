#[doc = "Register `MJPEG_START_ADDR2` reader"]
pub type R = crate::R<MjpegStartAddr2Spec>;
#[doc = "Field `frame_start_addr_2` reader - MJPEG start address for frame 2."]
pub type FrameStartAddr2R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - MJPEG start address for frame 2."]
    #[inline(always)]
    pub fn frame_start_addr_2(&self) -> FrameStartAddr2R {
        FrameStartAddr2R::new(self.bits)
    }
}
#[doc = "MJPEG Start Address 2\n\nYou can [`read`](crate::Reg::read) this register and get [`mjpeg_start_addr2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MjpegStartAddr2Spec;
impl crate::RegisterSpec for MjpegStartAddr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mjpeg_start_addr2::R`](R) reader structure"]
impl crate::Readable for MjpegStartAddr2Spec {}
#[doc = "`reset()` method sets MJPEG_START_ADDR2 to value 0"]
impl crate::Resettable for MjpegStartAddr2Spec {}
