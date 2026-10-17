#[doc = "Register `SE_PKA_0_RW` reader"]
pub type R = crate::R<SePka0RwSpec>;
#[doc = "Register `SE_PKA_0_RW` writer"]
pub type W = crate::W<SePka0RwSpec>;
#[doc = "Field `se_pka_0_rw` reader - "]
pub type SePka0RwR = crate::FieldReader<u32>;
#[doc = "Field `se_pka_0_rw` writer - "]
pub type SePka0RwW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn se_pka_0_rw(&self) -> SePka0RwR {
        SePka0RwR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn se_pka_0_rw(&mut self) -> SePka0RwW<'_, SePka0RwSpec> {
        SePka0RwW::new(self, 0)
    }
}
#[doc = "PKA Register Read/Write\n\nYou can [`read`](crate::Reg::read) this register and get [`se_pka_0_rw::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`se_pka_0_rw::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SePka0RwSpec;
impl crate::RegisterSpec for SePka0RwSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`se_pka_0_rw::R`](R) reader structure"]
impl crate::Readable for SePka0RwSpec {}
#[doc = "`write(|w| ..)` method takes [`se_pka_0_rw::W`](W) writer structure"]
impl crate::Writable for SePka0RwSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SE_PKA_0_RW to value 0"]
impl crate::Resettable for SePka0RwSpec {}
