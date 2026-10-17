#[doc = "Register `WIFI_PLL_CFG12` reader"]
pub type R = crate::R<WifiPllCfg12Spec>;
#[doc = "Register `WIFI_PLL_CFG12` writer"]
pub type W = crate::W<WifiPllCfg12Spec>;
#[doc = "Field `usbpll_ssc_cnt` reader - "]
pub type UsbpllSscCntR = crate::FieldReader<u16>;
#[doc = "Field `usbpll_ssc_cnt` writer - "]
pub type UsbpllSscCntW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    #[doc = "Bits 0:8"]
    #[inline(always)]
    pub fn usbpll_ssc_cnt(&self) -> UsbpllSscCntR {
        UsbpllSscCntR::new((self.bits & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8"]
    #[inline(always)]
    pub fn usbpll_ssc_cnt(&mut self) -> UsbpllSscCntW<'_, WifiPllCfg12Spec> {
        UsbpllSscCntW::new(self, 0)
    }
}
#[doc = "WIFI PLL Configuration 12 - USB PLL SSC Count\n\nYou can [`read`](crate::Reg::read) this register and get [`wifi_pll_cfg12::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wifi_pll_cfg12::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WifiPllCfg12Spec;
impl crate::RegisterSpec for WifiPllCfg12Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wifi_pll_cfg12::R`](R) reader structure"]
impl crate::Readable for WifiPllCfg12Spec {}
#[doc = "`write(|w| ..)` method takes [`wifi_pll_cfg12::W`](W) writer structure"]
impl crate::Writable for WifiPllCfg12Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets WIFI_PLL_CFG12 to value 0"]
impl crate::Resettable for WifiPllCfg12Spec {}
