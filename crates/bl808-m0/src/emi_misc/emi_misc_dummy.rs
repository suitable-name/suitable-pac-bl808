#[doc = "Register `EMI_MISC_DUMMY` reader"]
pub type R = crate::R<EmiMiscDummySpec>;
#[doc = "Register `EMI_MISC_DUMMY` writer"]
pub type W = crate::W<EmiMiscDummySpec>;
#[doc = "Field `dummy_reg` reader - "]
pub type DummyRegR = crate::FieldReader<u32>;
#[doc = "Field `dummy_reg` writer - "]
pub type DummyRegW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn dummy_reg(&self) -> DummyRegR {
        DummyRegR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn dummy_reg(&mut self) -> DummyRegW<'_, EmiMiscDummySpec> {
        DummyRegW::new(self, 0)
    }
}
#[doc = "EMI Miscellaneous Dummy Register\n\nYou can [`read`](crate::Reg::read) this register and get [`emi_misc_dummy::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`emi_misc_dummy::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EmiMiscDummySpec;
impl crate::RegisterSpec for EmiMiscDummySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`emi_misc_dummy::R`](R) reader structure"]
impl crate::Readable for EmiMiscDummySpec {}
#[doc = "`write(|w| ..)` method takes [`emi_misc_dummy::W`](W) writer structure"]
impl crate::Writable for EmiMiscDummySpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EMI_MISC_DUMMY to value 0xffff_0000"]
impl crate::Resettable for EmiMiscDummySpec {
    const RESET_VALUE: u32 = 0xffff_0000;
}
