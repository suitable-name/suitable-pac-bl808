#[doc = "Register `MJPEG_START_ADDR3` reader"]
pub type R = crate::R<MjpegStartAddr3Spec>;
#[doc = "Field `frame_start_addr_3` reader - MJPEG start address for frame 3."]
pub type FrameStartAddr3R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - MJPEG start address for frame 3."]
    #[inline(always)]
    pub fn frame_start_addr_3(&self) -> FrameStartAddr3R {
        FrameStartAddr3R::new(self.bits)
    }
}
#[doc = "MJPEG Start Address 3\n\nYou can [`read`](crate::Reg::read) this register and get [`mjpeg_start_addr3::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MjpegStartAddr3Spec;
impl crate::RegisterSpec for MjpegStartAddr3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mjpeg_start_addr3::R`](R) reader structure"]
impl crate::Readable for MjpegStartAddr3Spec {}
#[doc = "`reset()` method sets MJPEG_START_ADDR3 to value 0"]
impl crate::Resettable for MjpegStartAddr3Spec {}
