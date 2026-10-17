#[doc = "Register `SE_PKA_0_SEED` reader"]
pub type R = crate::R<SePka0SeedSpec>;
#[doc = "Register `SE_PKA_0_SEED` writer"]
pub type W = crate::W<SePka0SeedSpec>;
#[doc = "Field `se_pka_0_seed` reader - "]
pub type SePka0SeedR = crate::FieldReader<u32>;
#[doc = "Field `se_pka_0_seed` writer - "]
pub type SePka0SeedW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn se_pka_0_seed(&self) -> SePka0SeedR {
        SePka0SeedR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn se_pka_0_seed(&mut self) -> SePka0SeedW<'_, SePka0SeedSpec> {
        SePka0SeedW::new(self, 0)
    }
}
#[doc = "PKA Random Seed\n\nYou can [`read`](crate::Reg::read) this register and get [`se_pka_0_seed::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`se_pka_0_seed::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SePka0SeedSpec;
impl crate::RegisterSpec for SePka0SeedSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`se_pka_0_seed::R`](R) reader structure"]
impl crate::Readable for SePka0SeedSpec {}
#[doc = "`write(|w| ..)` method takes [`se_pka_0_seed::W`](W) writer structure"]
impl crate::Writable for SePka0SeedSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SE_PKA_0_SEED to value 0"]
impl crate::Resettable for SePka0SeedSpec {}
