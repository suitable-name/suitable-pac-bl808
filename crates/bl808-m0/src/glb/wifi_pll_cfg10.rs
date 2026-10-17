#[doc = "Register `WIFI_PLL_CFG10` reader"]
pub type R = crate::R<WifiPllCfg10Spec>;
#[doc = "Register `WIFI_PLL_CFG10` writer"]
pub type W = crate::W<WifiPllCfg10Spec>;
#[doc = "Field `usbpll_ssc_start` reader - "]
pub type UsbpllSscStartR = crate::BitReader;
#[doc = "Field `usbpll_ssc_start` writer - "]
pub type UsbpllSscStartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `usbpll_ssc_start_gate_en` reader - "]
pub type UsbpllSscStartGateEnR = crate::BitReader;
#[doc = "Field `usbpll_ssc_start_gate_en` writer - "]
pub type UsbpllSscStartGateEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `usbpll_ssc_gain` reader - "]
pub type UsbpllSscGainR = crate::FieldReader;
#[doc = "Field `usbpll_ssc_gain` writer - "]
pub type UsbpllSscGainW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `usbpll_ssc_en` reader - "]
pub type UsbpllSscEnR = crate::BitReader;
#[doc = "Field `usbpll_ssc_en` writer - "]
pub type UsbpllSscEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `usbpll_sdm_bypass` reader - "]
pub type UsbpllSdmBypassR = crate::BitReader;
#[doc = "Field `usbpll_sdm_bypass` writer - "]
pub type UsbpllSdmBypassW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `usbpll_sdm_order_sel` reader - "]
pub type UsbpllSdmOrderSelR = crate::BitReader;
#[doc = "Field `usbpll_sdm_order_sel` writer - "]
pub type UsbpllSdmOrderSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `usbpll_sdm_sig_dith_sel` reader - "]
pub type UsbpllSdmSigDithSelR = crate::FieldReader;
#[doc = "Field `usbpll_sdm_sig_dith_sel` writer - "]
pub type UsbpllSdmSigDithSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `usbpll_div2_en` reader - "]
pub type UsbpllDiv2EnR = crate::BitReader;
#[doc = "Field `usbpll_div2_en` writer - "]
pub type UsbpllDiv2EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `usbpll_clkout_en` reader - "]
pub type UsbpllClkoutEnR = crate::BitReader;
#[doc = "Field `usbpll_clkout_en` writer - "]
pub type UsbpllClkoutEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `usbpll_sel_sample_clk` reader - "]
pub type UsbpllSelSampleClkR = crate::FieldReader;
#[doc = "Field `usbpll_sel_sample_clk` writer - "]
pub type UsbpllSelSampleClkW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `usbpll_rstb` reader - "]
pub type UsbpllRstbR = crate::BitReader;
#[doc = "Field `usbpll_rstb` writer - "]
pub type UsbpllRstbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pu_usbpll_mmdiv` reader - "]
pub type PuUsbpllMmdivR = crate::BitReader;
#[doc = "Field `pu_usbpll_mmdiv` writer - "]
pub type PuUsbpllMmdivW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn usbpll_ssc_start(&self) -> UsbpllSscStartR {
        UsbpllSscStartR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn usbpll_ssc_start_gate_en(&self) -> UsbpllSscStartGateEnR {
        UsbpllSscStartGateEnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    pub fn usbpll_ssc_gain(&self) -> UsbpllSscGainR {
        UsbpllSscGainR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn usbpll_ssc_en(&self) -> UsbpllSscEnR {
        UsbpllSscEnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn usbpll_sdm_bypass(&self) -> UsbpllSdmBypassR {
        UsbpllSdmBypassR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn usbpll_sdm_order_sel(&self) -> UsbpllSdmOrderSelR {
        UsbpllSdmOrderSelR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn usbpll_sdm_sig_dith_sel(&self) -> UsbpllSdmSigDithSelR {
        UsbpllSdmSigDithSelR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn usbpll_div2_en(&self) -> UsbpllDiv2EnR {
        UsbpllDiv2EnR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn usbpll_clkout_en(&self) -> UsbpllClkoutEnR {
        UsbpllClkoutEnR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn usbpll_sel_sample_clk(&self) -> UsbpllSelSampleClkR {
        UsbpllSelSampleClkR::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn usbpll_rstb(&self) -> UsbpllRstbR {
        UsbpllRstbR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn pu_usbpll_mmdiv(&self) -> PuUsbpllMmdivR {
        PuUsbpllMmdivR::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn usbpll_ssc_start(&mut self) -> UsbpllSscStartW<'_, WifiPllCfg10Spec> {
        UsbpllSscStartW::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn usbpll_ssc_start_gate_en(&mut self) -> UsbpllSscStartGateEnW<'_, WifiPllCfg10Spec> {
        UsbpllSscStartGateEnW::new(self, 3)
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    pub fn usbpll_ssc_gain(&mut self) -> UsbpllSscGainW<'_, WifiPllCfg10Spec> {
        UsbpllSscGainW::new(self, 4)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn usbpll_ssc_en(&mut self) -> UsbpllSscEnW<'_, WifiPllCfg10Spec> {
        UsbpllSscEnW::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn usbpll_sdm_bypass(&mut self) -> UsbpllSdmBypassW<'_, WifiPllCfg10Spec> {
        UsbpllSdmBypassW::new(self, 9)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn usbpll_sdm_order_sel(&mut self) -> UsbpllSdmOrderSelW<'_, WifiPllCfg10Spec> {
        UsbpllSdmOrderSelW::new(self, 10)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn usbpll_sdm_sig_dith_sel(&mut self) -> UsbpllSdmSigDithSelW<'_, WifiPllCfg10Spec> {
        UsbpllSdmSigDithSelW::new(self, 16)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn usbpll_div2_en(&mut self) -> UsbpllDiv2EnW<'_, WifiPllCfg10Spec> {
        UsbpllDiv2EnW::new(self, 20)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn usbpll_clkout_en(&mut self) -> UsbpllClkoutEnW<'_, WifiPllCfg10Spec> {
        UsbpllClkoutEnW::new(self, 21)
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn usbpll_sel_sample_clk(&mut self) -> UsbpllSelSampleClkW<'_, WifiPllCfg10Spec> {
        UsbpllSelSampleClkW::new(self, 24)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn usbpll_rstb(&mut self) -> UsbpllRstbW<'_, WifiPllCfg10Spec> {
        UsbpllRstbW::new(self, 28)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn pu_usbpll_mmdiv(&mut self) -> PuUsbpllMmdivW<'_, WifiPllCfg10Spec> {
        PuUsbpllMmdivW::new(self, 29)
    }
}
#[doc = "WIFI PLL Configuration 10 - USB PLL SSC\n\nYou can [`read`](crate::Reg::read) this register and get [`wifi_pll_cfg10::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wifi_pll_cfg10::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WifiPllCfg10Spec;
impl crate::RegisterSpec for WifiPllCfg10Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wifi_pll_cfg10::R`](R) reader structure"]
impl crate::Readable for WifiPllCfg10Spec {}
#[doc = "`write(|w| ..)` method takes [`wifi_pll_cfg10::W`](W) writer structure"]
impl crate::Writable for WifiPllCfg10Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets WIFI_PLL_CFG10 to value 0"]
impl crate::Resettable for WifiPllCfg10Spec {}
