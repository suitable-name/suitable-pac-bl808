#[doc = "Register `MBIST_CFG0` reader"]
pub type R = crate::R<MbistCfg0Spec>;
#[doc = "Register `MBIST_CFG0` writer"]
pub type W = crate::W<MbistCfg0Spec>;
#[doc = "Field `mbist_mode` reader - "]
pub type MbistModeR = crate::BitReader;
#[doc = "Field `mbist_mode` writer - "]
pub type MbistModeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn mbist_mode(&self) -> MbistModeR {
        MbistModeR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn mbist_mode(&mut self) -> MbistModeW<'_, MbistCfg0Spec> {
        MbistModeW::new(self, 0)
    }
}
#[doc = "MBIST Configuration 0\n\nYou can [`read`](crate::Reg::read) this register and get [`mbist_cfg0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mbist_cfg0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MbistCfg0Spec;
impl crate::RegisterSpec for MbistCfg0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mbist_cfg0::R`](R) reader structure"]
impl crate::Readable for MbistCfg0Spec {}
#[doc = "`write(|w| ..)` method takes [`mbist_cfg0::W`](W) writer structure"]
impl crate::Writable for MbistCfg0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MBIST_CFG0 to value 0"]
impl crate::Resettable for MbistCfg0Spec {}
