#[doc = "Register `WIFI_PLL_CFG7` reader"]
pub type R = crate::R<WifiPllCfg7Spec>;
#[doc = "Register `WIFI_PLL_CFG7` writer"]
pub type W = crate::W<WifiPllCfg7Spec>;
#[doc = "Field `wifipll_sdm_order_sel` reader - "]
pub type WifipllSdmOrderSelR = crate::FieldReader;
#[doc = "Field `wifipll_sdm_order_sel` writer - "]
pub type WifipllSdmOrderSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `wifipll_sdm_noi_prbs_sel` reader - "]
pub type WifipllSdmNoiPrbsSelR = crate::FieldReader;
#[doc = "Field `wifipll_sdm_noi_prbs_sel` writer - "]
pub type WifipllSdmNoiPrbsSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `wifipll_sdm_noi_prbs_en` reader - "]
pub type WifipllSdmNoiPrbsEnR = crate::BitReader;
#[doc = "Field `wifipll_sdm_noi_prbs_en` writer - "]
pub type WifipllSdmNoiPrbsEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `wifipll_sdm_sig_prbs_sel` reader - "]
pub type WifipllSdmSigPrbsSelR = crate::FieldReader;
#[doc = "Field `wifipll_sdm_sig_prbs_sel` writer - "]
pub type WifipllSdmSigPrbsSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `wifipll_sdm_sig_dith_sel` reader - "]
pub type WifipllSdmSigDithSelR = crate::FieldReader;
#[doc = "Field `wifipll_sdm_sig_dith_sel` writer - "]
pub type WifipllSdmSigDithSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn wifipll_sdm_order_sel(&self) -> WifipllSdmOrderSelR {
        WifipllSdmOrderSelR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn wifipll_sdm_noi_prbs_sel(&self) -> WifipllSdmNoiPrbsSelR {
        WifipllSdmNoiPrbsSelR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn wifipll_sdm_noi_prbs_en(&self) -> WifipllSdmNoiPrbsEnR {
        WifipllSdmNoiPrbsEnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn wifipll_sdm_sig_prbs_sel(&self) -> WifipllSdmSigPrbsSelR {
        WifipllSdmSigPrbsSelR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn wifipll_sdm_sig_dith_sel(&self) -> WifipllSdmSigDithSelR {
        WifipllSdmSigDithSelR::new(((self.bits >> 16) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn wifipll_sdm_order_sel(&mut self) -> WifipllSdmOrderSelW<'_, WifiPllCfg7Spec> {
        WifipllSdmOrderSelW::new(self, 0)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn wifipll_sdm_noi_prbs_sel(&mut self) -> WifipllSdmNoiPrbsSelW<'_, WifiPllCfg7Spec> {
        WifipllSdmNoiPrbsSelW::new(self, 4)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn wifipll_sdm_noi_prbs_en(&mut self) -> WifipllSdmNoiPrbsEnW<'_, WifiPllCfg7Spec> {
        WifipllSdmNoiPrbsEnW::new(self, 8)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn wifipll_sdm_sig_prbs_sel(&mut self) -> WifipllSdmSigPrbsSelW<'_, WifiPllCfg7Spec> {
        WifipllSdmSigPrbsSelW::new(self, 12)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn wifipll_sdm_sig_dith_sel(&mut self) -> WifipllSdmSigDithSelW<'_, WifiPllCfg7Spec> {
        WifipllSdmSigDithSelW::new(self, 16)
    }
}
#[doc = "WIFI PLL Configuration 7 - SDM Dither\n\nYou can [`read`](crate::Reg::read) this register and get [`wifi_pll_cfg7::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wifi_pll_cfg7::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WifiPllCfg7Spec;
impl crate::RegisterSpec for WifiPllCfg7Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wifi_pll_cfg7::R`](R) reader structure"]
impl crate::Readable for WifiPllCfg7Spec {}
#[doc = "`write(|w| ..)` method takes [`wifi_pll_cfg7::W`](W) writer structure"]
impl crate::Writable for WifiPllCfg7Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets WIFI_PLL_CFG7 to value 0"]
impl crate::Resettable for WifiPllCfg7Spec {}
