#[doc = "Register `MIPI_PLL_CFG5` reader"]
pub type R = crate::R<MipiPllCfg5Spec>;
#[doc = "Register `MIPI_PLL_CFG5` writer"]
pub type W = crate::W<MipiPllCfg5Spec>;
#[doc = "Field `mipipll_vco_speed` reader - "]
pub type MipipllVcoSpeedR = crate::FieldReader;
#[doc = "Field `mipipll_vco_speed` writer - "]
pub type MipipllVcoSpeedW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `mipipll_vco_vdd_ctrl` reader - "]
pub type MipipllVcoVddCtrlR = crate::FieldReader;
#[doc = "Field `mipipll_vco_vdd_ctrl` writer - "]
pub type MipipllVcoVddCtrlW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `mipipll_vco_vdd_ctrl_extra` reader - "]
pub type MipipllVcoVddCtrlExtraR = crate::BitReader;
#[doc = "Field `mipipll_vco_vdd_ctrl_extra` writer - "]
pub type MipipllVcoVddCtrlExtraW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `mipipll_vco_postdiv_sel` reader - "]
pub type MipipllVcoPostdivSelR = crate::FieldReader;
#[doc = "Field `mipipll_vco_postdiv_sel` writer - "]
pub type MipipllVcoPostdivSelW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `mipipll_vco_postdiv_clk_en` reader - "]
pub type MipipllVcoPostdivClkEnR = crate::BitReader;
#[doc = "Field `mipipll_vco_postdiv_clk_en` writer - "]
pub type MipipllVcoPostdivClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn mipipll_vco_speed(&self) -> MipipllVcoSpeedR {
        MipipllVcoSpeedR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:4"]
    #[inline(always)]
    pub fn mipipll_vco_vdd_ctrl(&self) -> MipipllVcoVddCtrlR {
        MipipllVcoVddCtrlR::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn mipipll_vco_vdd_ctrl_extra(&self) -> MipipllVcoVddCtrlExtraR {
        MipipllVcoVddCtrlExtraR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 7:9"]
    #[inline(always)]
    pub fn mipipll_vco_postdiv_sel(&self) -> MipipllVcoPostdivSelR {
        MipipllVcoPostdivSelR::new(((self.bits >> 7) & 7) as u8)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn mipipll_vco_postdiv_clk_en(&self) -> MipipllVcoPostdivClkEnR {
        MipipllVcoPostdivClkEnR::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn mipipll_vco_speed(&mut self) -> MipipllVcoSpeedW<'_, MipiPllCfg5Spec> {
        MipipllVcoSpeedW::new(self, 0)
    }
    #[doc = "Bits 3:4"]
    #[inline(always)]
    pub fn mipipll_vco_vdd_ctrl(&mut self) -> MipipllVcoVddCtrlW<'_, MipiPllCfg5Spec> {
        MipipllVcoVddCtrlW::new(self, 3)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn mipipll_vco_vdd_ctrl_extra(&mut self) -> MipipllVcoVddCtrlExtraW<'_, MipiPllCfg5Spec> {
        MipipllVcoVddCtrlExtraW::new(self, 5)
    }
    #[doc = "Bits 7:9"]
    #[inline(always)]
    pub fn mipipll_vco_postdiv_sel(&mut self) -> MipipllVcoPostdivSelW<'_, MipiPllCfg5Spec> {
        MipipllVcoPostdivSelW::new(self, 7)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn mipipll_vco_postdiv_clk_en(&mut self) -> MipipllVcoPostdivClkEnW<'_, MipiPllCfg5Spec> {
        MipipllVcoPostdivClkEnW::new(self, 10)
    }
}
#[doc = "MIPI PLL Configuration 5 - VCO\n\nYou can [`read`](crate::Reg::read) this register and get [`mipi_pll_cfg5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mipi_pll_cfg5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MipiPllCfg5Spec;
impl crate::RegisterSpec for MipiPllCfg5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mipi_pll_cfg5::R`](R) reader structure"]
impl crate::Readable for MipiPllCfg5Spec {}
#[doc = "`write(|w| ..)` method takes [`mipi_pll_cfg5::W`](W) writer structure"]
impl crate::Writable for MipiPllCfg5Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MIPI_PLL_CFG5 to value 0"]
impl crate::Resettable for MipiPllCfg5Spec {}
