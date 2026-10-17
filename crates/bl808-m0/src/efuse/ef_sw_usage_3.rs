#[doc = "Register `EF_SW_USAGE_3` reader"]
pub type R = crate::R<EfSwUsage3Spec>;
#[doc = "Register `EF_SW_USAGE_3` writer"]
pub type W = crate::W<EfSwUsage3Spec>;
#[doc = "Field `ef_sw_usage_3` reader - "]
pub type EfSwUsage3R = crate::FieldReader<u32>;
#[doc = "Field `ef_sw_usage_3` writer - "]
pub type EfSwUsage3W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn ef_sw_usage_3(&self) -> EfSwUsage3R {
        EfSwUsage3R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn ef_sw_usage_3(&mut self) -> EfSwUsage3W<'_, EfSwUsage3Spec> {
        EfSwUsage3W::new(self, 0)
    }
}
#[doc = "Software Usage Slot 3 (eFuse Data)\n\nYou can [`read`](crate::Reg::read) this register and get [`ef_sw_usage_3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ef_sw_usage_3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EfSwUsage3Spec;
impl crate::RegisterSpec for EfSwUsage3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ef_sw_usage_3::R`](R) reader structure"]
impl crate::Readable for EfSwUsage3Spec {}
#[doc = "`write(|w| ..)` method takes [`ef_sw_usage_3::W`](W) writer structure"]
impl crate::Writable for EfSwUsage3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EF_SW_USAGE_3 to value 0"]
impl crate::Resettable for EfSwUsage3Spec {}
