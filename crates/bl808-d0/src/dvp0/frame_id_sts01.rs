#[doc = "Register `FRAME_ID_STS01` reader"]
pub type R = crate::R<FrameIdSts01Spec>;
#[doc = "Field `frame_id_0` reader - Frame ID for buffer 0"]
pub type FrameId0R = crate::FieldReader<u16>;
#[doc = "Field `frame_id_1` reader - Frame ID for buffer 1"]
pub type FrameId1R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Frame ID for buffer 0"]
    #[inline(always)]
    pub fn frame_id_0(&self) -> FrameId0R {
        FrameId0R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Frame ID for buffer 1"]
    #[inline(always)]
    pub fn frame_id_1(&self) -> FrameId1R {
        FrameId1R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "Frame ID Status for Buffers 0 and 1\n\nYou can [`read`](crate::Reg::read) this register and get [`frame_id_sts01::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FrameIdSts01Spec;
impl crate::RegisterSpec for FrameIdSts01Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`frame_id_sts01::R`](R) reader structure"]
impl crate::Readable for FrameIdSts01Spec {}
#[doc = "`reset()` method sets FRAME_ID_STS01 to value 0"]
impl crate::Resettable for FrameIdSts01Spec {}
