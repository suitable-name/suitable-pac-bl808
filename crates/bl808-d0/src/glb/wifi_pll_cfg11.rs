#[doc = "Register `WIFI_PLL_CFG11` reader"]
pub type R = crate::R<WifiPllCfg11Spec>;
#[doc = "Register `WIFI_PLL_CFG11` writer"]
pub type W = crate::W<WifiPllCfg11Spec>;
#[doc = "Field `usbpll_sdmin` reader - "]
pub type UsbpllSdminR = crate::FieldReader<u32>;
#[doc = "Field `usbpll_sdmin` writer - "]
pub type UsbpllSdminW<'a, REG> = crate::FieldWriter<'a, REG, 19, u32>;
impl R {
    #[doc = "Bits 0:18"]
    #[inline(always)]
    pub fn usbpll_sdmin(&self) -> UsbpllSdminR {
        UsbpllSdminR::new(self.bits & 0x0007_ffff)
    }
}
impl W {
    #[doc = "Bits 0:18"]
    #[inline(always)]
    pub fn usbpll_sdmin(&mut self) -> UsbpllSdminW<'_, WifiPllCfg11Spec> {
        UsbpllSdminW::new(self, 0)
    }
}
#[doc = "WIFI PLL Configuration 11 - USB PLL SDM\n\nYou can [`read`](crate::Reg::read) this register and get [`wifi_pll_cfg11::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wifi_pll_cfg11::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WifiPllCfg11Spec;
impl crate::RegisterSpec for WifiPllCfg11Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wifi_pll_cfg11::R`](R) reader structure"]
impl crate::Readable for WifiPllCfg11Spec {}
#[doc = "`write(|w| ..)` method takes [`wifi_pll_cfg11::W`](W) writer structure"]
impl crate::Writable for WifiPllCfg11Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets WIFI_PLL_CFG11 to value 0"]
impl crate::Resettable for WifiPllCfg11Spec {}
