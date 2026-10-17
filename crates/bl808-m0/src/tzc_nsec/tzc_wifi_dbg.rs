#[doc = "Register `TZC_WIFI_DBG` reader"]
pub type R = crate::R<TzcWifiDbgSpec>;
#[doc = "Field `tzc_mac_dbg_dis` reader - MAC Debug disable (1: disabled)"]
pub type TzcMacDbgDisR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - MAC Debug disable (1: disabled)"]
    #[inline(always)]
    pub fn tzc_mac_dbg_dis(&self) -> TzcMacDbgDisR {
        TzcMacDbgDisR::new((self.bits & 1) != 0)
    }
}
#[doc = "WIFI Debug Control\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_wifi_dbg::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TzcWifiDbgSpec;
impl crate::RegisterSpec for TzcWifiDbgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tzc_wifi_dbg::R`](R) reader structure"]
impl crate::Readable for TzcWifiDbgSpec {}
#[doc = "`reset()` method sets TZC_WIFI_DBG to value 0"]
impl crate::Resettable for TzcWifiDbgSpec {}
