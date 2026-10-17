#[doc = "Register `DPHY_CONFIG_3` reader"]
pub type R = crate::R<DphyConfig3Spec>;
#[doc = "Register `DPHY_CONFIG_3` writer"]
pub type W = crate::W<DphyConfig3Spec>;
#[doc = "Field `reg_csi_ana_1` reader - CSI Analog Register 1"]
pub type RegCsiAna1R = crate::FieldReader<u16>;
#[doc = "Field `reg_csi_ana_1` writer - CSI Analog Register 1"]
pub type RegCsiAna1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `reg_csi_ana_0` reader - CSI Analog Register 0"]
pub type RegCsiAna0R = crate::FieldReader<u16>;
#[doc = "Field `reg_csi_ana_0` writer - CSI Analog Register 0"]
pub type RegCsiAna0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - CSI Analog Register 1"]
    #[inline(always)]
    pub fn reg_csi_ana_1(&self) -> RegCsiAna1R {
        RegCsiAna1R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - CSI Analog Register 0"]
    #[inline(always)]
    pub fn reg_csi_ana_0(&self) -> RegCsiAna0R {
        RegCsiAna0R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - CSI Analog Register 1"]
    #[inline(always)]
    pub fn reg_csi_ana_1(&mut self) -> RegCsiAna1W<'_, DphyConfig3Spec> {
        RegCsiAna1W::new(self, 0)
    }
    #[doc = "Bits 16:31 - CSI Analog Register 0"]
    #[inline(always)]
    pub fn reg_csi_ana_0(&mut self) -> RegCsiAna0W<'_, DphyConfig3Spec> {
        RegCsiAna0W::new(self, 16)
    }
}
#[doc = "D-PHY Analog Configuration 3\n\nYou can [`read`](crate::Reg::read) this register and get [`dphy_config_3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dphy_config_3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DphyConfig3Spec;
impl crate::RegisterSpec for DphyConfig3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dphy_config_3::R`](R) reader structure"]
impl crate::Readable for DphyConfig3Spec {}
#[doc = "`write(|w| ..)` method takes [`dphy_config_3::W`](W) writer structure"]
impl crate::Writable for DphyConfig3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DPHY_CONFIG_3 to value 0"]
impl crate::Resettable for DphyConfig3Spec {}
