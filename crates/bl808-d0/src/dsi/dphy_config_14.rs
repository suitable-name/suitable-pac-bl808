#[doc = "Register `DPHY_CONFIG_14` reader"]
pub type R = crate::R<DphyConfig14Spec>;
#[doc = "Register `DPHY_CONFIG_14` writer"]
pub type W = crate::W<DphyConfig14Spec>;
#[doc = "Field `reg_pt_free_rep_pat` reader - "]
pub type RegPtFreeRepPatR = crate::FieldReader<u32>;
#[doc = "Field `reg_pt_free_rep_pat` writer - "]
pub type RegPtFreeRepPatW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn reg_pt_free_rep_pat(&self) -> RegPtFreeRepPatR {
        RegPtFreeRepPatR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn reg_pt_free_rep_pat(&mut self) -> RegPtFreeRepPatW<'_, DphyConfig14Spec> {
        RegPtFreeRepPatW::new(self, 0)
    }
}
#[doc = "DPHY Pattern Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`dphy_config_14::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dphy_config_14::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DphyConfig14Spec;
impl crate::RegisterSpec for DphyConfig14Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dphy_config_14::R`](R) reader structure"]
impl crate::Readable for DphyConfig14Spec {}
#[doc = "`write(|w| ..)` method takes [`dphy_config_14::W`](W) writer structure"]
impl crate::Writable for DphyConfig14Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DPHY_CONFIG_14 to value 0x8765_4321"]
impl crate::Resettable for DphyConfig14Spec {
    const RESET_VALUE: u32 = 0x8765_4321;
}
