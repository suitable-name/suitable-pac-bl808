#[doc = "Register `DPHY_CONFIG_4` reader"]
pub type R = crate::R<DphyConfig4Spec>;
#[doc = "Register `DPHY_CONFIG_4` writer"]
pub type W = crate::W<DphyConfig4Spec>;
#[doc = "Field `reg_csi_dc_tp_out_en` reader - DC test pattern output enable"]
pub type RegCsiDcTpOutEnR = crate::BitReader;
#[doc = "Field `reg_csi_dc_tp_out_en` writer - DC test pattern output enable"]
pub type RegCsiDcTpOutEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_csi_pw_avdd1815` reader - CSI analog power supply select (1.8V/1.5V)"]
pub type RegCsiPwAvdd1815R = crate::BitReader;
#[doc = "Field `reg_csi_pw_avdd1815` writer - CSI analog power supply select (1.8V/1.5V)"]
pub type RegCsiPwAvdd1815W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - DC test pattern output enable"]
    #[inline(always)]
    pub fn reg_csi_dc_tp_out_en(&self) -> RegCsiDcTpOutEnR {
        RegCsiDcTpOutEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - CSI analog power supply select (1.8V/1.5V)"]
    #[inline(always)]
    pub fn reg_csi_pw_avdd1815(&self) -> RegCsiPwAvdd1815R {
        RegCsiPwAvdd1815R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DC test pattern output enable"]
    #[inline(always)]
    pub fn reg_csi_dc_tp_out_en(&mut self) -> RegCsiDcTpOutEnW<'_, DphyConfig4Spec> {
        RegCsiDcTpOutEnW::new(self, 0)
    }
    #[doc = "Bit 4 - CSI analog power supply select (1.8V/1.5V)"]
    #[inline(always)]
    pub fn reg_csi_pw_avdd1815(&mut self) -> RegCsiPwAvdd1815W<'_, DphyConfig4Spec> {
        RegCsiPwAvdd1815W::new(self, 4)
    }
}
#[doc = "D-PHY Analog Configuration 4\n\nYou can [`read`](crate::Reg::read) this register and get [`dphy_config_4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dphy_config_4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DphyConfig4Spec;
impl crate::RegisterSpec for DphyConfig4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dphy_config_4::R`](R) reader structure"]
impl crate::Readable for DphyConfig4Spec {}
#[doc = "`write(|w| ..)` method takes [`dphy_config_4::W`](W) writer structure"]
impl crate::Writable for DphyConfig4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DPHY_CONFIG_4 to value 0"]
impl crate::Resettable for DphyConfig4Spec {}
