#[doc = "Register `WIFI_PLL_CFG4` reader"]
pub type R = crate::R<WifiPllCfg4Spec>;
#[doc = "Register `WIFI_PLL_CFG4` writer"]
pub type W = crate::W<WifiPllCfg4Spec>;
#[doc = "Field `wifipll_sel_sample_clk` reader - "]
pub type WifipllSelSampleClkR = crate::FieldReader;
#[doc = "Field `wifipll_sel_sample_clk` writer - "]
pub type WifipllSelSampleClkW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `wifipll_sel_fb_clk` reader - "]
pub type WifipllSelFbClkR = crate::FieldReader;
#[doc = "Field `wifipll_sel_fb_clk` writer - "]
pub type WifipllSelFbClkW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `wifipll_sdmclk_sel` reader - "]
pub type WifipllSdmclkSelR = crate::BitReader;
#[doc = "Field `wifipll_sdmclk_sel` writer - "]
pub type WifipllSdmclkSelW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn wifipll_sel_sample_clk(&self) -> WifipllSelSampleClkR {
        WifipllSelSampleClkR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn wifipll_sel_fb_clk(&self) -> WifipllSelFbClkR {
        WifipllSelFbClkR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn wifipll_sdmclk_sel(&self) -> WifipllSdmclkSelR {
        WifipllSdmclkSelR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn wifipll_sel_sample_clk(&mut self) -> WifipllSelSampleClkW<'_, WifiPllCfg4Spec> {
        WifipllSelSampleClkW::new(self, 0)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn wifipll_sel_fb_clk(&mut self) -> WifipllSelFbClkW<'_, WifiPllCfg4Spec> {
        WifipllSelFbClkW::new(self, 4)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn wifipll_sdmclk_sel(&mut self) -> WifipllSdmclkSelW<'_, WifiPllCfg4Spec> {
        WifipllSdmclkSelW::new(self, 8)
    }
}
#[doc = "WIFI PLL Configuration 4 - Sample Clock\n\nYou can [`read`](crate::Reg::read) this register and get [`wifi_pll_cfg4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wifi_pll_cfg4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WifiPllCfg4Spec;
impl crate::RegisterSpec for WifiPllCfg4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wifi_pll_cfg4::R`](R) reader structure"]
impl crate::Readable for WifiPllCfg4Spec {}
#[doc = "`write(|w| ..)` method takes [`wifi_pll_cfg4::W`](W) writer structure"]
impl crate::Writable for WifiPllCfg4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets WIFI_PLL_CFG4 to value 0"]
impl crate::Resettable for WifiPllCfg4Spec {}
