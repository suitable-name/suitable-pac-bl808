#[doc = "Register `MJPEG_FRAME_ID_32` reader"]
pub type R = crate::R<MjpegFrameId32Spec>;
#[doc = "Field `frame_id_2` reader - "]
pub type FrameId2R = crate::FieldReader<u16>;
#[doc = "Field `frame_id_3` reader - "]
pub type FrameId3R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn frame_id_2(&self) -> FrameId2R {
        FrameId2R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn frame_id_3(&self) -> FrameId3R {
        FrameId3R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "MJPEG Frame ID Status 2 and 3\n\nYou can [`read`](crate::Reg::read) this register and get [`mjpeg_frame_id_32::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MjpegFrameId32Spec;
impl crate::RegisterSpec for MjpegFrameId32Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mjpeg_frame_id_32::R`](R) reader structure"]
impl crate::Readable for MjpegFrameId32Spec {}
#[doc = "`reset()` method sets MJPEG_FRAME_ID_32 to value 0"]
impl crate::Resettable for MjpegFrameId32Spec {}
