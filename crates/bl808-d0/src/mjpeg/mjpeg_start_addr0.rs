#[doc = "Register `MJPEG_START_ADDR0` reader"]
pub type R = crate::R<MjpegStartAddr0Spec>;
#[doc = "Field `frame_start_addr_0` reader - MJPEG start address for frame 0."]
pub type FrameStartAddr0R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - MJPEG start address for frame 0."]
    #[inline(always)]
    pub fn frame_start_addr_0(&self) -> FrameStartAddr0R {
        FrameStartAddr0R::new(self.bits)
    }
}
#[doc = "MJPEG Start Address 0\n\nYou can [`read`](crate::Reg::read) this register and get [`mjpeg_start_addr0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MjpegStartAddr0Spec;
impl crate::RegisterSpec for MjpegStartAddr0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mjpeg_start_addr0::R`](R) reader structure"]
impl crate::Readable for MjpegStartAddr0Spec {}
#[doc = "`reset()` method sets MJPEG_START_ADDR0 to value 0"]
impl crate::Resettable for MjpegStartAddr0Spec {}
