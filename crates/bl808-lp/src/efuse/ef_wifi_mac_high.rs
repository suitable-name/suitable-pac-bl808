#[doc = "Register `EF_WIFI_MAC_HIGH` reader"]
pub type R = crate::R<EfWifiMacHighSpec>;
#[doc = "Register `EF_WIFI_MAC_HIGH` writer"]
pub type W = crate::W<EfWifiMacHighSpec>;
#[doc = "Field `ef_wifi_mac_high` reader - "]
pub type EfWifiMacHighR = crate::FieldReader<u32>;
#[doc = "Field `ef_wifi_mac_high` writer - "]
pub type EfWifiMacHighW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn ef_wifi_mac_high(&self) -> EfWifiMacHighR {
        EfWifiMacHighR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn ef_wifi_mac_high(&mut self) -> EfWifiMacHighW<'_, EfWifiMacHighSpec> {
        EfWifiMacHighW::new(self, 0)
    }
}
#[doc = "Wi-Fi MAC Address High Word (Bits 47:32)\n\nYou can [`read`](crate::Reg::read) this register and get [`ef_wifi_mac_high::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ef_wifi_mac_high::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EfWifiMacHighSpec;
impl crate::RegisterSpec for EfWifiMacHighSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ef_wifi_mac_high::R`](R) reader structure"]
impl crate::Readable for EfWifiMacHighSpec {}
#[doc = "`write(|w| ..)` method takes [`ef_wifi_mac_high::W`](W) writer structure"]
impl crate::Writable for EfWifiMacHighSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EF_WIFI_MAC_HIGH to value 0"]
impl crate::Resettable for EfWifiMacHighSpec {}
