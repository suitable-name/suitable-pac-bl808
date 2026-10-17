#[doc = "Register `UHS_PLL_CFG5` reader"]
pub type R = crate::R<UhsPllCfg5Spec>;
#[doc = "Register `UHS_PLL_CFG5` writer"]
pub type W = crate::W<UhsPllCfg5Spec>;
#[doc = "Field `uhspll_vco_speed` reader - "]
pub type UhspllVcoSpeedR = crate::FieldReader;
#[doc = "Field `uhspll_vco_speed` writer - "]
pub type UhspllVcoSpeedW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `uhspll_vco_vdd_ctrl` reader - "]
pub type UhspllVcoVddCtrlR = crate::FieldReader;
#[doc = "Field `uhspll_vco_vdd_ctrl` writer - "]
pub type UhspllVcoVddCtrlW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `uhspll_vco_vdd_ctrl_extra` reader - "]
pub type UhspllVcoVddCtrlExtraR = crate::BitReader;
#[doc = "Field `uhspll_vco_vdd_ctrl_extra` writer - "]
pub type UhspllVcoVddCtrlExtraW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `uhspll_vco_postdiv_sel` reader - "]
pub type UhspllVcoPostdivSelR = crate::FieldReader;
#[doc = "Field `uhspll_vco_postdiv_sel` writer - "]
pub type UhspllVcoPostdivSelW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `uhspll_vco_postdiv_clk_en` reader - "]
pub type UhspllVcoPostdivClkEnR = crate::BitReader;
#[doc = "Field `uhspll_vco_postdiv_clk_en` writer - "]
pub type UhspllVcoPostdivClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn uhspll_vco_speed(&self) -> UhspllVcoSpeedR {
        UhspllVcoSpeedR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:4"]
    #[inline(always)]
    pub fn uhspll_vco_vdd_ctrl(&self) -> UhspllVcoVddCtrlR {
        UhspllVcoVddCtrlR::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn uhspll_vco_vdd_ctrl_extra(&self) -> UhspllVcoVddCtrlExtraR {
        UhspllVcoVddCtrlExtraR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 7:9"]
    #[inline(always)]
    pub fn uhspll_vco_postdiv_sel(&self) -> UhspllVcoPostdivSelR {
        UhspllVcoPostdivSelR::new(((self.bits >> 7) & 7) as u8)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn uhspll_vco_postdiv_clk_en(&self) -> UhspllVcoPostdivClkEnR {
        UhspllVcoPostdivClkEnR::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn uhspll_vco_speed(&mut self) -> UhspllVcoSpeedW<'_, UhsPllCfg5Spec> {
        UhspllVcoSpeedW::new(self, 0)
    }
    #[doc = "Bits 3:4"]
    #[inline(always)]
    pub fn uhspll_vco_vdd_ctrl(&mut self) -> UhspllVcoVddCtrlW<'_, UhsPllCfg5Spec> {
        UhspllVcoVddCtrlW::new(self, 3)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn uhspll_vco_vdd_ctrl_extra(&mut self) -> UhspllVcoVddCtrlExtraW<'_, UhsPllCfg5Spec> {
        UhspllVcoVddCtrlExtraW::new(self, 5)
    }
    #[doc = "Bits 7:9"]
    #[inline(always)]
    pub fn uhspll_vco_postdiv_sel(&mut self) -> UhspllVcoPostdivSelW<'_, UhsPllCfg5Spec> {
        UhspllVcoPostdivSelW::new(self, 7)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn uhspll_vco_postdiv_clk_en(&mut self) -> UhspllVcoPostdivClkEnW<'_, UhsPllCfg5Spec> {
        UhspllVcoPostdivClkEnW::new(self, 10)
    }
}
#[doc = "UHS PLL Configuration 5 - VCO\n\nYou can [`read`](crate::Reg::read) this register and get [`uhs_pll_cfg5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uhs_pll_cfg5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UhsPllCfg5Spec;
impl crate::RegisterSpec for UhsPllCfg5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uhs_pll_cfg5::R`](R) reader structure"]
impl crate::Readable for UhsPllCfg5Spec {}
#[doc = "`write(|w| ..)` method takes [`uhs_pll_cfg5::W`](W) writer structure"]
impl crate::Writable for UhsPllCfg5Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UHS_PLL_CFG5 to value 0"]
impl crate::Resettable for UhsPllCfg5Spec {}
