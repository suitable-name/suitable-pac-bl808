#[doc = "Register `FRAME_START_ADDR2` reader"]
pub type R = crate::R<FrameStartAddr2Spec>;
#[doc = "Field `frame_start_addr_2` reader - "]
pub type FrameStartAddr2R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn frame_start_addr_2(&self) -> FrameStartAddr2R {
        FrameStartAddr2R::new(self.bits)
    }
}
#[doc = "Frame Buffer 2 Start Address Status\n\nYou can [`read`](crate::Reg::read) this register and get [`frame_start_addr2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FrameStartAddr2Spec;
impl crate::RegisterSpec for FrameStartAddr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`frame_start_addr2::R`](R) reader structure"]
impl crate::Readable for FrameStartAddr2Spec {}
#[doc = "`reset()` method sets FRAME_START_ADDR2 to value 0"]
impl crate::Resettable for FrameStartAddr2Spec {}
