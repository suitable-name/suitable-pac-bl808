#[doc = "Register `MJPEG_FRAME_ID_10` reader"]
pub type R = crate::R<MjpegFrameId10Spec>;
#[doc = "Field `frame_id_0` reader - "]
pub type FrameId0R = crate::FieldReader<u16>;
#[doc = "Field `frame_id_1` reader - "]
pub type FrameId1R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn frame_id_0(&self) -> FrameId0R {
        FrameId0R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn frame_id_1(&self) -> FrameId1R {
        FrameId1R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "MJPEG Frame ID Status 0 and 1\n\nYou can [`read`](crate::Reg::read) this register and get [`mjpeg_frame_id_10::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MjpegFrameId10Spec;
impl crate::RegisterSpec for MjpegFrameId10Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mjpeg_frame_id_10::R`](R) reader structure"]
impl crate::Readable for MjpegFrameId10Spec {}
#[doc = "`reset()` method sets MJPEG_FRAME_ID_10 to value 0"]
impl crate::Resettable for MjpegFrameId10Spec {}
