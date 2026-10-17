#[doc = "Register `FRAME_ID_STS23` reader"]
pub type R = crate::R<FrameIdSts23Spec>;
#[doc = "Field `frame_id_2` reader - Frame ID for buffer 2"]
pub type FrameId2R = crate::FieldReader<u16>;
#[doc = "Field `frame_id_3` reader - Frame ID for buffer 3"]
pub type FrameId3R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Frame ID for buffer 2"]
    #[inline(always)]
    pub fn frame_id_2(&self) -> FrameId2R {
        FrameId2R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Frame ID for buffer 3"]
    #[inline(always)]
    pub fn frame_id_3(&self) -> FrameId3R {
        FrameId3R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "Frame ID Status for Buffers 2 and 3\n\nYou can [`read`](crate::Reg::read) this register and get [`frame_id_sts23::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FrameIdSts23Spec;
impl crate::RegisterSpec for FrameIdSts23Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`frame_id_sts23::R`](R) reader structure"]
impl crate::Readable for FrameIdSts23Spec {}
#[doc = "`reset()` method sets FRAME_ID_STS23 to value 0"]
impl crate::Resettable for FrameIdSts23Spec {}
