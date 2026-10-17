#[doc = "Register `WIFI_PLL_CFG6` reader"]
pub type R = crate::R<WifiPllCfg6Spec>;
#[doc = "Register `WIFI_PLL_CFG6` writer"]
pub type W = crate::W<WifiPllCfg6Spec>;
#[doc = "Field `wifipll_sdmin` reader - "]
pub type WifipllSdminR = crate::FieldReader<u32>;
#[doc = "Field `wifipll_sdmin` writer - "]
pub type WifipllSdminW<'a, REG> = crate::FieldWriter<'a, REG, 26, u32>;
#[doc = "Field `wifipll_sdm_bypass` reader - "]
pub type WifipllSdmBypassR = crate::BitReader;
#[doc = "Field `wifipll_sdm_bypass` writer - "]
pub type WifipllSdmBypassW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `wifipll_sdm_bypass_hw` reader - "]
pub type WifipllSdmBypassHwR = crate::BitReader;
#[doc = "Field `wifipll_sdm_bypass_hw` writer - "]
pub type WifipllSdmBypassHwW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `wifipll_sdm_ctrl_hw` reader - "]
pub type WifipllSdmCtrlHwR = crate::BitReader;
#[doc = "Field `wifipll_sdm_ctrl_hw` writer - "]
pub type WifipllSdmCtrlHwW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:25"]
    #[inline(always)]
    pub fn wifipll_sdmin(&self) -> WifipllSdminR {
        WifipllSdminR::new(self.bits & 0x03ff_ffff)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn wifipll_sdm_bypass(&self) -> WifipllSdmBypassR {
        WifipllSdmBypassR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn wifipll_sdm_bypass_hw(&self) -> WifipllSdmBypassHwR {
        WifipllSdmBypassHwR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn wifipll_sdm_ctrl_hw(&self) -> WifipllSdmCtrlHwR {
        WifipllSdmCtrlHwR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:25"]
    #[inline(always)]
    pub fn wifipll_sdmin(&mut self) -> WifipllSdminW<'_, WifiPllCfg6Spec> {
        WifipllSdminW::new(self, 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn wifipll_sdm_bypass(&mut self) -> WifipllSdmBypassW<'_, WifiPllCfg6Spec> {
        WifipllSdmBypassW::new(self, 28)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn wifipll_sdm_bypass_hw(&mut self) -> WifipllSdmBypassHwW<'_, WifiPllCfg6Spec> {
        WifipllSdmBypassHwW::new(self, 29)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn wifipll_sdm_ctrl_hw(&mut self) -> WifipllSdmCtrlHwW<'_, WifiPllCfg6Spec> {
        WifipllSdmCtrlHwW::new(self, 31)
    }
}
#[doc = "WIFI PLL Configuration 6 - SDM Input\n\nYou can [`read`](crate::Reg::read) this register and get [`wifi_pll_cfg6::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wifi_pll_cfg6::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WifiPllCfg6Spec;
impl crate::RegisterSpec for WifiPllCfg6Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wifi_pll_cfg6::R`](R) reader structure"]
impl crate::Readable for WifiPllCfg6Spec {}
#[doc = "`write(|w| ..)` method takes [`wifi_pll_cfg6::W`](W) writer structure"]
impl crate::Writable for WifiPllCfg6Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets WIFI_PLL_CFG6 to value 0"]
impl crate::Resettable for WifiPllCfg6Spec {}
