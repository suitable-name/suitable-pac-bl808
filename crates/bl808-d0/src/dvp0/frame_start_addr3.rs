#[doc = "Register `FRAME_START_ADDR3` reader"]
pub type R = crate::R<FrameStartAddr3Spec>;
#[doc = "Field `frame_start_addr_3` reader - "]
pub type FrameStartAddr3R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn frame_start_addr_3(&self) -> FrameStartAddr3R {
        FrameStartAddr3R::new(self.bits)
    }
}
#[doc = "Frame Buffer 3 Start Address Status\n\nYou can [`read`](crate::Reg::read) this register and get [`frame_start_addr3::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FrameStartAddr3Spec;
impl crate::RegisterSpec for FrameStartAddr3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`frame_start_addr3::R`](R) reader structure"]
impl crate::Readable for FrameStartAddr3Spec {}
#[doc = "`reset()` method sets FRAME_START_ADDR3 to value 0"]
impl crate::Resettable for FrameStartAddr3Spec {}
