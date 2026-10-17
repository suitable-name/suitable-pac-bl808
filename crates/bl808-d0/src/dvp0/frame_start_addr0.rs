#[doc = "Register `FRAME_START_ADDR0` reader"]
pub type R = crate::R<FrameStartAddr0Spec>;
#[doc = "Field `frame_start_addr_0` reader - "]
pub type FrameStartAddr0R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn frame_start_addr_0(&self) -> FrameStartAddr0R {
        FrameStartAddr0R::new(self.bits)
    }
}
#[doc = "Frame Buffer 0 Start Address Status\n\nYou can [`read`](crate::Reg::read) this register and get [`frame_start_addr0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FrameStartAddr0Spec;
impl crate::RegisterSpec for FrameStartAddr0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`frame_start_addr0::R`](R) reader structure"]
impl crate::Readable for FrameStartAddr0Spec {}
#[doc = "`reset()` method sets FRAME_START_ADDR0 to value 0"]
impl crate::Resettable for FrameStartAddr0Spec {}
