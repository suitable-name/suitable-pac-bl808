#[doc = "Register `CORE_CFG18` reader"]
pub type R = crate::R<CoreCfg18Spec>;
#[doc = "Register `CORE_CFG18` writer"]
pub type W = crate::W<CoreCfg18Spec>;
#[doc = "Field `np_int_mask0` reader - "]
pub type NpIntMask0R = crate::FieldReader<u32>;
#[doc = "Field `np_int_mask0` writer - "]
pub type NpIntMask0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn np_int_mask0(&self) -> NpIntMask0R {
        NpIntMask0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn np_int_mask0(&mut self) -> NpIntMask0W<'_, CoreCfg18Spec> {
        NpIntMask0W::new(self, 0)
    }
}
#[doc = "New Peripheral Interrupt Mask 0\n\nYou can [`read`](crate::Reg::read) this register and get [`core_cfg18::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`core_cfg18::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CoreCfg18Spec;
impl crate::RegisterSpec for CoreCfg18Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`core_cfg18::R`](R) reader structure"]
impl crate::Readable for CoreCfg18Spec {}
#[doc = "`write(|w| ..)` method takes [`core_cfg18::W`](W) writer structure"]
impl crate::Writable for CoreCfg18Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CORE_CFG18 to value 0"]
impl crate::Resettable for CoreCfg18Spec {}
