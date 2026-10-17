#[doc = "Register `PSRAM_DUMMY_REG` reader"]
pub type R = crate::R<PsramDummyRegSpec>;
#[doc = "Register `PSRAM_DUMMY_REG` writer"]
pub type W = crate::W<PsramDummyRegSpec>;
#[doc = "Field `reg_psram_dummy_reg` reader - Dummy register value"]
pub type RegPsramDummyRegR = crate::FieldReader<u32>;
#[doc = "Field `reg_psram_dummy_reg` writer - Dummy register value"]
pub type RegPsramDummyRegW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Dummy register value"]
    #[inline(always)]
    pub fn reg_psram_dummy_reg(&self) -> RegPsramDummyRegR {
        RegPsramDummyRegR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Dummy register value"]
    #[inline(always)]
    pub fn reg_psram_dummy_reg(&mut self) -> RegPsramDummyRegW<'_, PsramDummyRegSpec> {
        RegPsramDummyRegW::new(self, 0)
    }
}
#[doc = "PSRAM Dummy Register\n\nYou can [`read`](crate::Reg::read) this register and get [`psram_dummy_reg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psram_dummy_reg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PsramDummyRegSpec;
impl crate::RegisterSpec for PsramDummyRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`psram_dummy_reg::R`](R) reader structure"]
impl crate::Readable for PsramDummyRegSpec {}
#[doc = "`write(|w| ..)` method takes [`psram_dummy_reg::W`](W) writer structure"]
impl crate::Writable for PsramDummyRegSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PSRAM_DUMMY_REG to value 0xffff_0000"]
impl crate::Resettable for PsramDummyRegSpec {
    const RESET_VALUE: u32 = 0xffff_0000;
}
