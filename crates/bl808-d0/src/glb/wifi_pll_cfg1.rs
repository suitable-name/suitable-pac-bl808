#[doc = "Register `WIFI_PLL_CFG1` reader"]
pub type R = crate::R<WifiPllCfg1Spec>;
#[doc = "Register `WIFI_PLL_CFG1` writer"]
pub type W = crate::W<WifiPllCfg1Spec>;
#[doc = "Field `wifipll_postdiv` reader - "]
pub type WifipllPostdivR = crate::FieldReader;
#[doc = "Field `wifipll_postdiv` writer - "]
pub type WifipllPostdivW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `wifipll_refdiv_ratio` reader - "]
pub type WifipllRefdivRatioR = crate::FieldReader;
#[doc = "Field `wifipll_refdiv_ratio` writer - "]
pub type WifipllRefdivRatioW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `wifipll_refclk_sel` reader - "]
pub type WifipllRefclkSelR = crate::FieldReader;
#[doc = "Field `wifipll_refclk_sel` writer - "]
pub type WifipllRefclkSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `wifipll_vg11_sel` reader - "]
pub type WifipllVg11SelR = crate::FieldReader;
#[doc = "Field `wifipll_vg11_sel` writer - "]
pub type WifipllVg11SelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `wifipll_vg13_sel` reader - "]
pub type WifipllVg13SelR = crate::FieldReader;
#[doc = "Field `wifipll_vg13_sel` writer - "]
pub type WifipllVg13SelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:6"]
    #[inline(always)]
    pub fn wifipll_postdiv(&self) -> WifipllPostdivR {
        WifipllPostdivR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn wifipll_refdiv_ratio(&self) -> WifipllRefdivRatioR {
        WifipllRefdivRatioR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn wifipll_refclk_sel(&self) -> WifipllRefclkSelR {
        WifipllRefclkSelR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    pub fn wifipll_vg11_sel(&self) -> WifipllVg11SelR {
        WifipllVg11SelR::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn wifipll_vg13_sel(&self) -> WifipllVg13SelR {
        WifipllVg13SelR::new(((self.bits >> 24) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6"]
    #[inline(always)]
    pub fn wifipll_postdiv(&mut self) -> WifipllPostdivW<'_, WifiPllCfg1Spec> {
        WifipllPostdivW::new(self, 0)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn wifipll_refdiv_ratio(&mut self) -> WifipllRefdivRatioW<'_, WifiPllCfg1Spec> {
        WifipllRefdivRatioW::new(self, 8)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn wifipll_refclk_sel(&mut self) -> WifipllRefclkSelW<'_, WifiPllCfg1Spec> {
        WifipllRefclkSelW::new(self, 16)
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    pub fn wifipll_vg11_sel(&mut self) -> WifipllVg11SelW<'_, WifiPllCfg1Spec> {
        WifipllVg11SelW::new(self, 20)
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn wifipll_vg13_sel(&mut self) -> WifipllVg13SelW<'_, WifiPllCfg1Spec> {
        WifipllVg13SelW::new(self, 24)
    }
}
#[doc = "WIFI PLL Configuration 1 - Divider and Clock Select\n\nYou can [`read`](crate::Reg::read) this register and get [`wifi_pll_cfg1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wifi_pll_cfg1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WifiPllCfg1Spec;
impl crate::RegisterSpec for WifiPllCfg1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wifi_pll_cfg1::R`](R) reader structure"]
impl crate::Readable for WifiPllCfg1Spec {}
#[doc = "`write(|w| ..)` method takes [`wifi_pll_cfg1::W`](W) writer structure"]
impl crate::Writable for WifiPllCfg1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets WIFI_PLL_CFG1 to value 0"]
impl crate::Resettable for WifiPllCfg1Spec {}
