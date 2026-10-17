#[doc = "Register `MJPEG_DUMMY_REG` reader"]
pub type R = crate::R<MjpegDummyRegSpec>;
#[doc = "Register `MJPEG_DUMMY_REG` writer"]
pub type W = crate::W<MjpegDummyRegSpec>;
#[doc = "Field `mjpeg_dummy_reg` reader - General purpose dummy bits."]
pub type MjpegDummyRegR = crate::FieldReader<u32>;
#[doc = "Field `mjpeg_dummy_reg` writer - General purpose dummy bits."]
pub type MjpegDummyRegW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - General purpose dummy bits."]
    #[inline(always)]
    pub fn mjpeg_dummy_reg(&self) -> MjpegDummyRegR {
        MjpegDummyRegR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - General purpose dummy bits."]
    #[inline(always)]
    pub fn mjpeg_dummy_reg(&mut self) -> MjpegDummyRegW<'_, MjpegDummyRegSpec> {
        MjpegDummyRegW::new(self, 0)
    }
}
#[doc = "MJPEG Dummy Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mjpeg_dummy_reg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mjpeg_dummy_reg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MjpegDummyRegSpec;
impl crate::RegisterSpec for MjpegDummyRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mjpeg_dummy_reg::R`](R) reader structure"]
impl crate::Readable for MjpegDummyRegSpec {}
#[doc = "`write(|w| ..)` method takes [`mjpeg_dummy_reg::W`](W) writer structure"]
impl crate::Writable for MjpegDummyRegSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MJPEG_DUMMY_REG to value 0"]
impl crate::Resettable for MjpegDummyRegSpec {}
