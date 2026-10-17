#[doc = "Register `SE_PKA_0_RW_BURST` reader"]
pub type R = crate::R<SePka0RwBurstSpec>;
#[doc = "Register `SE_PKA_0_RW_BURST` writer"]
pub type W = crate::W<SePka0RwBurstSpec>;
#[doc = "Field `se_pka_0_rw_burst` reader - "]
pub type SePka0RwBurstR = crate::FieldReader<u32>;
#[doc = "Field `se_pka_0_rw_burst` writer - "]
pub type SePka0RwBurstW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn se_pka_0_rw_burst(&self) -> SePka0RwBurstR {
        SePka0RwBurstR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn se_pka_0_rw_burst(&mut self) -> SePka0RwBurstW<'_, SePka0RwBurstSpec> {
        SePka0RwBurstW::new(self, 0)
    }
}
#[doc = "PKA Register Burst Read/Write\n\nYou can [`read`](crate::Reg::read) this register and get [`se_pka_0_rw_burst::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`se_pka_0_rw_burst::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SePka0RwBurstSpec;
impl crate::RegisterSpec for SePka0RwBurstSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`se_pka_0_rw_burst::R`](R) reader structure"]
impl crate::Readable for SePka0RwBurstSpec {}
#[doc = "`write(|w| ..)` method takes [`se_pka_0_rw_burst::W`](W) writer structure"]
impl crate::Writable for SePka0RwBurstSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SE_PKA_0_RW_BURST to value 0"]
impl crate::Resettable for SePka0RwBurstSpec {}
