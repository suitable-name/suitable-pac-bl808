#[doc = "Register `CORE_CFG20` reader"]
pub type R = crate::R<CoreCfg20Spec>;
#[doc = "Register `CORE_CFG20` writer"]
pub type W = crate::W<CoreCfg20Spec>;
#[doc = "Field `np_int_clr0` writer - "]
pub type NpIntClr0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn np_int_clr0(&mut self) -> NpIntClr0W<'_, CoreCfg20Spec> {
        NpIntClr0W::new(self, 0)
    }
}
#[doc = "New Peripheral Interrupt Clear 0\n\nYou can [`read`](crate::Reg::read) this register and get [`core_cfg20::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`core_cfg20::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CoreCfg20Spec;
impl crate::RegisterSpec for CoreCfg20Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`core_cfg20::R`](R) reader structure"]
impl crate::Readable for CoreCfg20Spec {}
#[doc = "`write(|w| ..)` method takes [`core_cfg20::W`](W) writer structure"]
impl crate::Writable for CoreCfg20Spec {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0xffff_ffff;
}
#[doc = "`reset()` method sets CORE_CFG20 to value 0"]
impl crate::Resettable for CoreCfg20Spec {}
