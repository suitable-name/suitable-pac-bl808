#[doc = "Register `EF_SW_USAGE_1` reader"]
pub type R = crate::R<EfSwUsage1Spec>;
#[doc = "Register `EF_SW_USAGE_1` writer"]
pub type W = crate::W<EfSwUsage1Spec>;
#[doc = "Field `ef_sw_usage_1` reader - "]
pub type EfSwUsage1R = crate::FieldReader<u32>;
#[doc = "Field `ef_sw_usage_1` writer - "]
pub type EfSwUsage1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn ef_sw_usage_1(&self) -> EfSwUsage1R {
        EfSwUsage1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn ef_sw_usage_1(&mut self) -> EfSwUsage1W<'_, EfSwUsage1Spec> {
        EfSwUsage1W::new(self, 0)
    }
}
#[doc = "Software Usage Slot 1 (eFuse Data)\n\nYou can [`read`](crate::Reg::read) this register and get [`ef_sw_usage_1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ef_sw_usage_1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EfSwUsage1Spec;
impl crate::RegisterSpec for EfSwUsage1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ef_sw_usage_1::R`](R) reader structure"]
impl crate::Readable for EfSwUsage1Spec {}
#[doc = "`write(|w| ..)` method takes [`ef_sw_usage_1::W`](W) writer structure"]
impl crate::Writable for EfSwUsage1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EF_SW_USAGE_1 to value 0"]
impl crate::Resettable for EfSwUsage1Spec {}
