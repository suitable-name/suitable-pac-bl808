#[doc = "Register `CORE_CFG21` reader"]
pub type R = crate::R<CoreCfg21Spec>;
#[doc = "Register `CORE_CFG21` writer"]
pub type W = crate::W<CoreCfg21Spec>;
#[doc = "Field `np_int_clr1` writer - "]
pub type NpIntClr1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn np_int_clr1(&mut self) -> NpIntClr1W<'_, CoreCfg21Spec> {
        NpIntClr1W::new(self, 0)
    }
}
#[doc = "New Peripheral Interrupt Clear 1\n\nYou can [`read`](crate::Reg::read) this register and get [`core_cfg21::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`core_cfg21::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CoreCfg21Spec;
impl crate::RegisterSpec for CoreCfg21Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`core_cfg21::R`](R) reader structure"]
impl crate::Readable for CoreCfg21Spec {}
#[doc = "`write(|w| ..)` method takes [`core_cfg21::W`](W) writer structure"]
impl crate::Writable for CoreCfg21Spec {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0xffff_ffff;
}
#[doc = "`reset()` method sets CORE_CFG21 to value 0"]
impl crate::Resettable for CoreCfg21Spec {}
