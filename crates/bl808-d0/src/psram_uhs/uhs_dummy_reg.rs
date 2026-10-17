#[doc = "Register `UHS_DUMMY_REG` reader"]
pub type R = crate::R<UhsDummyRegSpec>;
#[doc = "Register `UHS_DUMMY_REG` writer"]
pub type W = crate::W<UhsDummyRegSpec>;
#[doc = "Field `reg_psram_dummy_reg` reader - Dummy bits"]
pub type RegPsramDummyRegR = crate::FieldReader<u32>;
#[doc = "Field `reg_psram_dummy_reg` writer - Dummy bits"]
pub type RegPsramDummyRegW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Dummy bits"]
    #[inline(always)]
    pub fn reg_psram_dummy_reg(&self) -> RegPsramDummyRegR {
        RegPsramDummyRegR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Dummy bits"]
    #[inline(always)]
    pub fn reg_psram_dummy_reg(&mut self) -> RegPsramDummyRegW<'_, UhsDummyRegSpec> {
        RegPsramDummyRegW::new(self, 0)
    }
}
#[doc = "UHS Dummy Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`uhs_dummy_reg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uhs_dummy_reg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UhsDummyRegSpec;
impl crate::RegisterSpec for UhsDummyRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uhs_dummy_reg::R`](R) reader structure"]
impl crate::Readable for UhsDummyRegSpec {}
#[doc = "`write(|w| ..)` method takes [`uhs_dummy_reg::W`](W) writer structure"]
impl crate::Writable for UhsDummyRegSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UHS_DUMMY_REG to value 0xffff_0000"]
impl crate::Resettable for UhsDummyRegSpec {
    const RESET_VALUE: u32 = 0xffff_0000;
}
