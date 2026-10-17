#[doc = "Register `CORE_CFG19` reader"]
pub type R = crate::R<CoreCfg19Spec>;
#[doc = "Register `CORE_CFG19` writer"]
pub type W = crate::W<CoreCfg19Spec>;
#[doc = "Field `np_int_mask1` reader - "]
pub type NpIntMask1R = crate::FieldReader<u32>;
#[doc = "Field `np_int_mask1` writer - "]
pub type NpIntMask1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn np_int_mask1(&self) -> NpIntMask1R {
        NpIntMask1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn np_int_mask1(&mut self) -> NpIntMask1W<'_, CoreCfg19Spec> {
        NpIntMask1W::new(self, 0)
    }
}
#[doc = "New Peripheral Interrupt Mask 1\n\nYou can [`read`](crate::Reg::read) this register and get [`core_cfg19::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`core_cfg19::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CoreCfg19Spec;
impl crate::RegisterSpec for CoreCfg19Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`core_cfg19::R`](R) reader structure"]
impl crate::Readable for CoreCfg19Spec {}
#[doc = "`write(|w| ..)` method takes [`core_cfg19::W`](W) writer structure"]
impl crate::Writable for CoreCfg19Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CORE_CFG19 to value 0"]
impl crate::Resettable for CoreCfg19Spec {}
