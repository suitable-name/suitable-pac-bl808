#[doc = "Register `EF_WIFI_MAC_LOW` reader"]
pub type R = crate::R<EfWifiMacLowSpec>;
#[doc = "Register `EF_WIFI_MAC_LOW` writer"]
pub type W = crate::W<EfWifiMacLowSpec>;
#[doc = "Field `ef_wifi_mac_low` reader - "]
pub type EfWifiMacLowR = crate::FieldReader<u32>;
#[doc = "Field `ef_wifi_mac_low` writer - "]
pub type EfWifiMacLowW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn ef_wifi_mac_low(&self) -> EfWifiMacLowR {
        EfWifiMacLowR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn ef_wifi_mac_low(&mut self) -> EfWifiMacLowW<'_, EfWifiMacLowSpec> {
        EfWifiMacLowW::new(self, 0)
    }
}
#[doc = "Wi-Fi MAC Address Low Word (Bits 31:0)\n\nYou can [`read`](crate::Reg::read) this register and get [`ef_wifi_mac_low::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ef_wifi_mac_low::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EfWifiMacLowSpec;
impl crate::RegisterSpec for EfWifiMacLowSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ef_wifi_mac_low::R`](R) reader structure"]
impl crate::Readable for EfWifiMacLowSpec {}
#[doc = "`write(|w| ..)` method takes [`ef_wifi_mac_low::W`](W) writer structure"]
impl crate::Writable for EfWifiMacLowSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EF_WIFI_MAC_LOW to value 0"]
impl crate::Resettable for EfWifiMacLowSpec {}
