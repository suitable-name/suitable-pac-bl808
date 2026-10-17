#[doc = "Register `CODEC_MISC_DUMMY` reader"]
pub type R = crate::R<CodecMiscDummySpec>;
#[doc = "Register `CODEC_MISC_DUMMY` writer"]
pub type W = crate::W<CodecMiscDummySpec>;
#[doc = "Field `DUMMY_REG` reader - Dummy data"]
pub type DummyRegR = crate::FieldReader<u32>;
#[doc = "Field `DUMMY_REG` writer - Dummy data"]
pub type DummyRegW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Dummy data"]
    #[inline(always)]
    pub fn dummy_reg(&self) -> DummyRegR {
        DummyRegR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Dummy data"]
    #[inline(always)]
    pub fn dummy_reg(&mut self) -> DummyRegW<'_, CodecMiscDummySpec> {
        DummyRegW::new(self, 0)
    }
}
#[doc = "Dummy Register\n\nYou can [`read`](crate::Reg::read) this register and get [`codec_misc_dummy::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`codec_misc_dummy::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CodecMiscDummySpec;
impl crate::RegisterSpec for CodecMiscDummySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`codec_misc_dummy::R`](R) reader structure"]
impl crate::Readable for CodecMiscDummySpec {}
#[doc = "`write(|w| ..)` method takes [`codec_misc_dummy::W`](W) writer structure"]
impl crate::Writable for CodecMiscDummySpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CODEC_MISC_DUMMY to value 0xffff_0000"]
impl crate::Resettable for CodecMiscDummySpec {
    const RESET_VALUE: u32 = 0xffff_0000;
}
