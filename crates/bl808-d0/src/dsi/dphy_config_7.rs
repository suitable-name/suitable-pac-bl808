#[doc = "Register `DPHY_CONFIG_7` reader"]
pub type R = crate::R<DphyConfig7Spec>;
#[doc = "Register `DPHY_CONFIG_7` writer"]
pub type W = crate::W<DphyConfig7Spec>;
#[doc = "Field `reg_ana_test_en` reader - "]
pub type RegAnaTestEnR = crate::BitReader;
#[doc = "Field `reg_ana_test_en` writer - "]
pub type RegAnaTestEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_pt_en` reader - "]
pub type RegPtEnR = crate::BitReader;
#[doc = "Field `reg_pt_en` writer - "]
pub type RegPtEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_pt_lp_mode` reader - "]
pub type RegPtLpModeR = crate::BitReader;
#[doc = "Field `reg_pt_lp_mode` writer - "]
pub type RegPtLpModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_pt_prbs_or_jitt` reader - "]
pub type RegPtPrbsOrJittR = crate::BitReader;
#[doc = "Field `reg_pt_prbs_or_jitt` writer - "]
pub type RegPtPrbsOrJittW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_force_hiz_lp` reader - "]
pub type RegForceHizLpR = crate::FieldReader;
#[doc = "Field `reg_force_hiz_lp` writer - "]
pub type RegForceHizLpW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `reg_force_hiz_hs` reader - "]
pub type RegForceHizHsR = crate::FieldReader;
#[doc = "Field `reg_force_hiz_hs` writer - "]
pub type RegForceHizHsW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `reg_force_clk_hiz_lp` reader - "]
pub type RegForceClkHizLpR = crate::BitReader;
#[doc = "Field `reg_force_clk_hiz_lp` writer - "]
pub type RegForceClkHizLpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_force_clk_hiz_hs` reader - "]
pub type RegForceClkHizHsR = crate::BitReader;
#[doc = "Field `reg_force_clk_hiz_hs` writer - "]
pub type RegForceClkHizHsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_ana_lp_txen_clklane_esc` reader - "]
pub type RegAnaLpTxenClklaneEscR = crate::BitReader;
#[doc = "Field `reg_ana_lp_txen_clklane_esc` writer - "]
pub type RegAnaLpTxenClklaneEscW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_ana_lp_dptxdata_clklane_esc` reader - "]
pub type RegAnaLpDptxdataClklaneEscR = crate::BitReader;
#[doc = "Field `reg_ana_lp_dptxdata_clklane_esc` writer - "]
pub type RegAnaLpDptxdataClklaneEscW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_ana_lp_dntxdata_clklane_esc` reader - "]
pub type RegAnaLpDntxdataClklaneEscR = crate::BitReader;
#[doc = "Field `reg_ana_lp_dntxdata_clklane_esc` writer - "]
pub type RegAnaLpDntxdataClklaneEscW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_ana_hs_txen_clklane_esc` reader - "]
pub type RegAnaHsTxenClklaneEscR = crate::BitReader;
#[doc = "Field `reg_ana_hs_txen_clklane_esc` writer - "]
pub type RegAnaHsTxenClklaneEscW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_ana_clk_en_esc` reader - "]
pub type RegAnaClkEnEscR = crate::BitReader;
#[doc = "Field `reg_ana_clk_en_esc` writer - "]
pub type RegAnaClkEnEscW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_ana_byte_rev` reader - "]
pub type RegAnaByteRevR = crate::BitReader;
#[doc = "Field `reg_ana_byte_rev` writer - "]
pub type RegAnaByteRevW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn reg_ana_test_en(&self) -> RegAnaTestEnR {
        RegAnaTestEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn reg_pt_en(&self) -> RegPtEnR {
        RegPtEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn reg_pt_lp_mode(&self) -> RegPtLpModeR {
        RegPtLpModeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn reg_pt_prbs_or_jitt(&self) -> RegPtPrbsOrJittR {
        RegPtPrbsOrJittR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn reg_force_hiz_lp(&self) -> RegForceHizLpR {
        RegForceHizLpR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn reg_force_hiz_hs(&self) -> RegForceHizHsR {
        RegForceHizHsR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn reg_force_clk_hiz_lp(&self) -> RegForceClkHizLpR {
        RegForceClkHizLpR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn reg_force_clk_hiz_hs(&self) -> RegForceClkHizHsR {
        RegForceClkHizHsR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn reg_ana_lp_txen_clklane_esc(&self) -> RegAnaLpTxenClklaneEscR {
        RegAnaLpTxenClklaneEscR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn reg_ana_lp_dptxdata_clklane_esc(&self) -> RegAnaLpDptxdataClklaneEscR {
        RegAnaLpDptxdataClklaneEscR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn reg_ana_lp_dntxdata_clklane_esc(&self) -> RegAnaLpDntxdataClklaneEscR {
        RegAnaLpDntxdataClklaneEscR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn reg_ana_hs_txen_clklane_esc(&self) -> RegAnaHsTxenClklaneEscR {
        RegAnaHsTxenClklaneEscR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn reg_ana_clk_en_esc(&self) -> RegAnaClkEnEscR {
        RegAnaClkEnEscR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn reg_ana_byte_rev(&self) -> RegAnaByteRevR {
        RegAnaByteRevR::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn reg_ana_test_en(&mut self) -> RegAnaTestEnW<'_, DphyConfig7Spec> {
        RegAnaTestEnW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn reg_pt_en(&mut self) -> RegPtEnW<'_, DphyConfig7Spec> {
        RegPtEnW::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn reg_pt_lp_mode(&mut self) -> RegPtLpModeW<'_, DphyConfig7Spec> {
        RegPtLpModeW::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn reg_pt_prbs_or_jitt(&mut self) -> RegPtPrbsOrJittW<'_, DphyConfig7Spec> {
        RegPtPrbsOrJittW::new(self, 3)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn reg_force_hiz_lp(&mut self) -> RegForceHizLpW<'_, DphyConfig7Spec> {
        RegForceHizLpW::new(self, 4)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn reg_force_hiz_hs(&mut self) -> RegForceHizHsW<'_, DphyConfig7Spec> {
        RegForceHizHsW::new(self, 8)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn reg_force_clk_hiz_lp(&mut self) -> RegForceClkHizLpW<'_, DphyConfig7Spec> {
        RegForceClkHizLpW::new(self, 12)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn reg_force_clk_hiz_hs(&mut self) -> RegForceClkHizHsW<'_, DphyConfig7Spec> {
        RegForceClkHizHsW::new(self, 13)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn reg_ana_lp_txen_clklane_esc(&mut self) -> RegAnaLpTxenClklaneEscW<'_, DphyConfig7Spec> {
        RegAnaLpTxenClklaneEscW::new(self, 14)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn reg_ana_lp_dptxdata_clklane_esc(
        &mut self,
    ) -> RegAnaLpDptxdataClklaneEscW<'_, DphyConfig7Spec> {
        RegAnaLpDptxdataClklaneEscW::new(self, 15)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn reg_ana_lp_dntxdata_clklane_esc(
        &mut self,
    ) -> RegAnaLpDntxdataClklaneEscW<'_, DphyConfig7Spec> {
        RegAnaLpDntxdataClklaneEscW::new(self, 16)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn reg_ana_hs_txen_clklane_esc(&mut self) -> RegAnaHsTxenClklaneEscW<'_, DphyConfig7Spec> {
        RegAnaHsTxenClklaneEscW::new(self, 17)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn reg_ana_clk_en_esc(&mut self) -> RegAnaClkEnEscW<'_, DphyConfig7Spec> {
        RegAnaClkEnEscW::new(self, 18)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn reg_ana_byte_rev(&mut self) -> RegAnaByteRevW<'_, DphyConfig7Spec> {
        RegAnaByteRevW::new(self, 19)
    }
}
#[doc = "DPHY Analog and Force Control\n\nYou can [`read`](crate::Reg::read) this register and get [`dphy_config_7::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dphy_config_7::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DphyConfig7Spec;
impl crate::RegisterSpec for DphyConfig7Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dphy_config_7::R`](R) reader structure"]
impl crate::Readable for DphyConfig7Spec {}
#[doc = "`write(|w| ..)` method takes [`dphy_config_7::W`](W) writer structure"]
impl crate::Writable for DphyConfig7Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DPHY_CONFIG_7 to value 0"]
impl crate::Resettable for DphyConfig7Spec {}
