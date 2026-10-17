#[doc = "Register `EF_SW_USAGE_2` reader"]
pub type R = crate::R<EfSwUsage2Spec>;
#[doc = "Register `EF_SW_USAGE_2` writer"]
pub type W = crate::W<EfSwUsage2Spec>;
#[doc = "Field `ef_sw_usage_2` reader - "]
pub type EfSwUsage2R = crate::FieldReader<u32>;
#[doc = "Field `ef_sw_usage_2` writer - "]
pub type EfSwUsage2W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn ef_sw_usage_2(&self) -> EfSwUsage2R {
        EfSwUsage2R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn ef_sw_usage_2(&mut self) -> EfSwUsage2W<'_, EfSwUsage2Spec> {
        EfSwUsage2W::new(self, 0)
    }
}
#[doc = "Software Usage Slot 2 (eFuse Data)\n\nYou can [`read`](crate::Reg::read) this register and get [`ef_sw_usage_2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ef_sw_usage_2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EfSwUsage2Spec;
impl crate::RegisterSpec for EfSwUsage2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ef_sw_usage_2::R`](R) reader structure"]
impl crate::Readable for EfSwUsage2Spec {}
#[doc = "`write(|w| ..)` method takes [`ef_sw_usage_2::W`](W) writer structure"]
impl crate::Writable for EfSwUsage2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EF_SW_USAGE_2 to value 0"]
impl crate::Resettable for EfSwUsage2Spec {}
