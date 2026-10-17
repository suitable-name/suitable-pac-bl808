#[doc = "Register `DPHY_CONFIG_15` reader"]
pub type R = crate::R<DphyConfig15Spec>;
#[doc = "Register `DPHY_CONFIG_15` writer"]
pub type W = crate::W<DphyConfig15Spec>;
#[doc = "Field `reg_dsi_dc_tp_out_en` reader - "]
pub type RegDsiDcTpOutEnR = crate::BitReader;
#[doc = "Field `reg_dsi_dc_tp_out_en` writer - "]
pub type RegDsiDcTpOutEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_dsi_pw_avdd1815` reader - "]
pub type RegDsiPwAvdd1815R = crate::BitReader;
#[doc = "Field `reg_dsi_pw_avdd1815` writer - "]
pub type RegDsiPwAvdd1815W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_dphy_pu_ldo11` reader - "]
pub type RegDphyPuLdo11R = crate::BitReader;
#[doc = "Field `reg_dphy_pu_ldo11` writer - "]
pub type RegDphyPuLdo11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_dphy_short_ldo11` reader - "]
pub type RegDphyShortLdo11R = crate::BitReader;
#[doc = "Field `reg_dphy_short_ldo11` writer - "]
pub type RegDphyShortLdo11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_dphy_ldo11_rfb_sw` reader - "]
pub type RegDphyLdo11RfbSwR = crate::FieldReader;
#[doc = "Field `reg_dphy_ldo11_rfb_sw` writer - "]
pub type RegDphyLdo11RfbSwW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `reg_ten_dsi_ldo` reader - "]
pub type RegTenDsiLdoR = crate::BitReader;
#[doc = "Field `reg_ten_dsi_ldo` writer - "]
pub type RegTenDsiLdoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_mipi_ldo_fast` reader - "]
pub type RegMipiLdoFastR = crate::BitReader;
#[doc = "Field `reg_mipi_ldo_fast` writer - "]
pub type RegMipiLdoFastW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_dsi_rst_n_pre` reader - "]
pub type RegDsiRstNPreR = crate::BitReader;
#[doc = "Field `reg_dsi_rst_n_pre` writer - "]
pub type RegDsiRstNPreW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_csi_rst_n_pre` reader - "]
pub type RegCsiRstNPreR = crate::BitReader;
#[doc = "Field `reg_csi_rst_n_pre` writer - "]
pub type RegCsiRstNPreW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn reg_dsi_dc_tp_out_en(&self) -> RegDsiDcTpOutEnR {
        RegDsiDcTpOutEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn reg_dsi_pw_avdd1815(&self) -> RegDsiPwAvdd1815R {
        RegDsiPwAvdd1815R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn reg_dphy_pu_ldo11(&self) -> RegDphyPuLdo11R {
        RegDphyPuLdo11R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn reg_dphy_short_ldo11(&self) -> RegDphyShortLdo11R {
        RegDphyShortLdo11R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 16:18"]
    #[inline(always)]
    pub fn reg_dphy_ldo11_rfb_sw(&self) -> RegDphyLdo11RfbSwR {
        RegDphyLdo11RfbSwR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn reg_ten_dsi_ldo(&self) -> RegTenDsiLdoR {
        RegTenDsiLdoR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn reg_mipi_ldo_fast(&self) -> RegMipiLdoFastR {
        RegMipiLdoFastR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn reg_dsi_rst_n_pre(&self) -> RegDsiRstNPreR {
        RegDsiRstNPreR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn reg_csi_rst_n_pre(&self) -> RegCsiRstNPreR {
        RegCsiRstNPreR::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn reg_dsi_dc_tp_out_en(&mut self) -> RegDsiDcTpOutEnW<'_, DphyConfig15Spec> {
        RegDsiDcTpOutEnW::new(self, 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn reg_dsi_pw_avdd1815(&mut self) -> RegDsiPwAvdd1815W<'_, DphyConfig15Spec> {
        RegDsiPwAvdd1815W::new(self, 4)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn reg_dphy_pu_ldo11(&mut self) -> RegDphyPuLdo11W<'_, DphyConfig15Spec> {
        RegDphyPuLdo11W::new(self, 8)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn reg_dphy_short_ldo11(&mut self) -> RegDphyShortLdo11W<'_, DphyConfig15Spec> {
        RegDphyShortLdo11W::new(self, 12)
    }
    #[doc = "Bits 16:18"]
    #[inline(always)]
    pub fn reg_dphy_ldo11_rfb_sw(&mut self) -> RegDphyLdo11RfbSwW<'_, DphyConfig15Spec> {
        RegDphyLdo11RfbSwW::new(self, 16)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn reg_ten_dsi_ldo(&mut self) -> RegTenDsiLdoW<'_, DphyConfig15Spec> {
        RegTenDsiLdoW::new(self, 20)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn reg_mipi_ldo_fast(&mut self) -> RegMipiLdoFastW<'_, DphyConfig15Spec> {
        RegMipiLdoFastW::new(self, 24)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn reg_dsi_rst_n_pre(&mut self) -> RegDsiRstNPreW<'_, DphyConfig15Spec> {
        RegDsiRstNPreW::new(self, 28)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn reg_csi_rst_n_pre(&mut self) -> RegCsiRstNPreW<'_, DphyConfig15Spec> {
        RegCsiRstNPreW::new(self, 29)
    }
}
#[doc = "DPHY Power and Reset Control\n\nYou can [`read`](crate::Reg::read) this register and get [`dphy_config_15::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dphy_config_15::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DphyConfig15Spec;
impl crate::RegisterSpec for DphyConfig15Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dphy_config_15::R`](R) reader structure"]
impl crate::Readable for DphyConfig15Spec {}
#[doc = "`write(|w| ..)` method takes [`dphy_config_15::W`](W) writer structure"]
impl crate::Writable for DphyConfig15Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DPHY_CONFIG_15 to value 0x0004_0100"]
impl crate::Resettable for DphyConfig15Spec {
    const RESET_VALUE: u32 = 0x0004_0100;
}
