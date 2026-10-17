#[doc = "Register `DPHY_CONFIG_12` reader"]
pub type R = crate::R<DphyConfig12Spec>;
#[doc = "Register `DPHY_CONFIG_12` writer"]
pub type W = crate::W<DphyConfig12Spec>;
#[doc = "Field `reg_dsi_ana_2` reader - "]
pub type RegDsiAna2R = crate::FieldReader<u16>;
#[doc = "Field `reg_dsi_ana_2` writer - "]
pub type RegDsiAna2W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn reg_dsi_ana_2(&self) -> RegDsiAna2R {
        RegDsiAna2R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn reg_dsi_ana_2(&mut self) -> RegDsiAna2W<'_, DphyConfig12Spec> {
        RegDsiAna2W::new(self, 0)
    }
}
#[doc = "DPHY Analog Control 1\n\nYou can [`read`](crate::Reg::read) this register and get [`dphy_config_12::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dphy_config_12::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DphyConfig12Spec;
impl crate::RegisterSpec for DphyConfig12Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dphy_config_12::R`](R) reader structure"]
impl crate::Readable for DphyConfig12Spec {}
#[doc = "`write(|w| ..)` method takes [`dphy_config_12::W`](W) writer structure"]
impl crate::Writable for DphyConfig12Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DPHY_CONFIG_12 to value 0"]
impl crate::Resettable for DphyConfig12Spec {}
