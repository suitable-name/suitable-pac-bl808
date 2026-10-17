#[doc = "Register `TZC_WIFI_DBG` reader"]
pub type R = crate::R<TzcWifiDbgSpec>;
#[doc = "Register `TZC_WIFI_DBG` writer"]
pub type W = crate::W<TzcWifiDbgSpec>;
#[doc = "Field `tzc_mac_dbg_dis` reader - Disable MAC debug access"]
pub type TzcMacDbgDisR = crate::BitReader;
#[doc = "Field `tzc_mac_dbg_dis` writer - Disable MAC debug access"]
pub type TzcMacDbgDisW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Disable MAC debug access"]
    #[inline(always)]
    pub fn tzc_mac_dbg_dis(&self) -> TzcMacDbgDisR {
        TzcMacDbgDisR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Disable MAC debug access"]
    #[inline(always)]
    pub fn tzc_mac_dbg_dis(&mut self) -> TzcMacDbgDisW<'_, TzcWifiDbgSpec> {
        TzcMacDbgDisW::new(self, 0)
    }
}
#[doc = "WiFi Debug Disable Control\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_wifi_dbg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tzc_wifi_dbg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TzcWifiDbgSpec;
impl crate::RegisterSpec for TzcWifiDbgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tzc_wifi_dbg::R`](R) reader structure"]
impl crate::Readable for TzcWifiDbgSpec {}
#[doc = "`write(|w| ..)` method takes [`tzc_wifi_dbg::W`](W) writer structure"]
impl crate::Writable for TzcWifiDbgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TZC_WIFI_DBG to value 0x01"]
impl crate::Resettable for TzcWifiDbgSpec {
    const RESET_VALUE: u32 = 0x01;
}
