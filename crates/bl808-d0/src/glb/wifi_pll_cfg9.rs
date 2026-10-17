#[doc = "Register `WIFI_PLL_CFG9` reader"]
pub type R = crate::R<WifiPllCfg9Spec>;
#[doc = "Register `WIFI_PLL_CFG9` writer"]
pub type W = crate::W<WifiPllCfg9Spec>;
#[doc = "Field `wifipll_dc_tp_out_en` reader - "]
pub type WifipllDcTpOutEnR = crate::BitReader;
#[doc = "Field `wifipll_dc_tp_out_en` writer - "]
pub type WifipllDcTpOutEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ten_wifipll` reader - "]
pub type TenWifipllR = crate::BitReader;
#[doc = "Field `ten_wifipll` writer - "]
pub type TenWifipllW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ten_wifipll_sfreg` reader - "]
pub type TenWifipllSfregR = crate::BitReader;
#[doc = "Field `ten_wifipll_sfreg` writer - "]
pub type TenWifipllSfregW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dten_wifipll_fin` reader - "]
pub type DtenWifipllFinR = crate::BitReader;
#[doc = "Field `dten_wifipll_fin` writer - "]
pub type DtenWifipllFinW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dten_wifipll_fref` reader - "]
pub type DtenWifipllFrefR = crate::BitReader;
#[doc = "Field `dten_wifipll_fref` writer - "]
pub type DtenWifipllFrefW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dten_wifipll_fsdm` reader - "]
pub type DtenWifipllFsdmR = crate::BitReader;
#[doc = "Field `dten_wifipll_fsdm` writer - "]
pub type DtenWifipllFsdmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dten_wifipll_div30` reader - "]
pub type DtenWifipllDiv30R = crate::BitReader;
#[doc = "Field `dten_wifipll_div30` writer - "]
pub type DtenWifipllDiv30W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dten_wifipll_div10` reader - "]
pub type DtenWifipllDiv10R = crate::BitReader;
#[doc = "Field `dten_wifipll_div10` writer - "]
pub type DtenWifipllDiv10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dten_wifipll_postdiv_clk` reader - "]
pub type DtenWifipllPostdivClkR = crate::BitReader;
#[doc = "Field `dten_wifipll_postdiv_clk` writer - "]
pub type DtenWifipllPostdivClkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `usbpll_dtest_pclk_en` reader - "]
pub type UsbpllDtestPclkEnR = crate::BitReader;
#[doc = "Field `usbpll_dtest_pclk_en` writer - "]
pub type UsbpllDtestPclkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `usbpll_dtest_clkout_en` reader - "]
pub type UsbpllDtestClkoutEnR = crate::BitReader;
#[doc = "Field `usbpll_dtest_clkout_en` writer - "]
pub type UsbpllDtestClkoutEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dtest_wifipll_pulldown` reader - "]
pub type DtestWifipllPulldownR = crate::BitReader;
#[doc = "Field `dtest_wifipll_pulldown` writer - "]
pub type DtestWifipllPulldownW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn wifipll_dc_tp_out_en(&self) -> WifipllDcTpOutEnR {
        WifipllDcTpOutEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn ten_wifipll(&self) -> TenWifipllR {
        TenWifipllR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn ten_wifipll_sfreg(&self) -> TenWifipllSfregR {
        TenWifipllSfregR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn dten_wifipll_fin(&self) -> DtenWifipllFinR {
        DtenWifipllFinR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn dten_wifipll_fref(&self) -> DtenWifipllFrefR {
        DtenWifipllFrefR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn dten_wifipll_fsdm(&self) -> DtenWifipllFsdmR {
        DtenWifipllFsdmR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn dten_wifipll_div30(&self) -> DtenWifipllDiv30R {
        DtenWifipllDiv30R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn dten_wifipll_div10(&self) -> DtenWifipllDiv10R {
        DtenWifipllDiv10R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn dten_wifipll_postdiv_clk(&self) -> DtenWifipllPostdivClkR {
        DtenWifipllPostdivClkR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn usbpll_dtest_pclk_en(&self) -> UsbpllDtestPclkEnR {
        UsbpllDtestPclkEnR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn usbpll_dtest_clkout_en(&self) -> UsbpllDtestClkoutEnR {
        UsbpllDtestClkoutEnR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn dtest_wifipll_pulldown(&self) -> DtestWifipllPulldownR {
        DtestWifipllPulldownR::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn wifipll_dc_tp_out_en(&mut self) -> WifipllDcTpOutEnW<'_, WifiPllCfg9Spec> {
        WifipllDcTpOutEnW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn ten_wifipll(&mut self) -> TenWifipllW<'_, WifiPllCfg9Spec> {
        TenWifipllW::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn ten_wifipll_sfreg(&mut self) -> TenWifipllSfregW<'_, WifiPllCfg9Spec> {
        TenWifipllSfregW::new(self, 2)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn dten_wifipll_fin(&mut self) -> DtenWifipllFinW<'_, WifiPllCfg9Spec> {
        DtenWifipllFinW::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn dten_wifipll_fref(&mut self) -> DtenWifipllFrefW<'_, WifiPllCfg9Spec> {
        DtenWifipllFrefW::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn dten_wifipll_fsdm(&mut self) -> DtenWifipllFsdmW<'_, WifiPllCfg9Spec> {
        DtenWifipllFsdmW::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn dten_wifipll_div30(&mut self) -> DtenWifipllDiv30W<'_, WifiPllCfg9Spec> {
        DtenWifipllDiv30W::new(self, 7)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn dten_wifipll_div10(&mut self) -> DtenWifipllDiv10W<'_, WifiPllCfg9Spec> {
        DtenWifipllDiv10W::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn dten_wifipll_postdiv_clk(&mut self) -> DtenWifipllPostdivClkW<'_, WifiPllCfg9Spec> {
        DtenWifipllPostdivClkW::new(self, 9)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn usbpll_dtest_pclk_en(&mut self) -> UsbpllDtestPclkEnW<'_, WifiPllCfg9Spec> {
        UsbpllDtestPclkEnW::new(self, 10)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn usbpll_dtest_clkout_en(&mut self) -> UsbpllDtestClkoutEnW<'_, WifiPllCfg9Spec> {
        UsbpllDtestClkoutEnW::new(self, 11)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn dtest_wifipll_pulldown(&mut self) -> DtestWifipllPulldownW<'_, WifiPllCfg9Spec> {
        DtestWifipllPulldownW::new(self, 12)
    }
}
#[doc = "WIFI PLL Configuration 9 - Test and Debug\n\nYou can [`read`](crate::Reg::read) this register and get [`wifi_pll_cfg9::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wifi_pll_cfg9::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WifiPllCfg9Spec;
impl crate::RegisterSpec for WifiPllCfg9Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wifi_pll_cfg9::R`](R) reader structure"]
impl crate::Readable for WifiPllCfg9Spec {}
#[doc = "`write(|w| ..)` method takes [`wifi_pll_cfg9::W`](W) writer structure"]
impl crate::Writable for WifiPllCfg9Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets WIFI_PLL_CFG9 to value 0"]
impl crate::Resettable for WifiPllCfg9Spec {}
