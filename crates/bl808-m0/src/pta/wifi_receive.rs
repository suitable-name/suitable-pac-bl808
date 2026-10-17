#[doc = "Register `wifi_receive` reader"]
pub type R = crate::R<WifiReceiveSpec>;
#[doc = "Register `wifi_receive` writer"]
pub type W = crate::W<WifiReceiveSpec>;
#[doc = "Field `wifi_rx_en` reader - Enable WiFi receive arbitration"]
pub type WifiRxEnR = crate::BitReader;
#[doc = "Field `wifi_rx_en` writer - Enable WiFi receive arbitration"]
pub type WifiRxEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `wifi_rx_prio` reader - WiFi receive priority level"]
pub type WifiRxPrioR = crate::FieldReader;
#[doc = "Field `wifi_rx_prio` writer - WiFi receive priority level"]
pub type WifiRxPrioW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `wifi_rx_force_grant` reader - Force grant WiFi receive regardless of BT state"]
pub type WifiRxForceGrantR = crate::BitReader;
#[doc = "Field `wifi_rx_force_grant` writer - Force grant WiFi receive regardless of BT state"]
pub type WifiRxForceGrantW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable WiFi receive arbitration"]
    #[inline(always)]
    pub fn wifi_rx_en(&self) -> WifiRxEnR {
        WifiRxEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - WiFi receive priority level"]
    #[inline(always)]
    pub fn wifi_rx_prio(&self) -> WifiRxPrioR {
        WifiRxPrioR::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 4 - Force grant WiFi receive regardless of BT state"]
    #[inline(always)]
    pub fn wifi_rx_force_grant(&self) -> WifiRxForceGrantR {
        WifiRxForceGrantR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable WiFi receive arbitration"]
    #[inline(always)]
    pub fn wifi_rx_en(&mut self) -> WifiRxEnW<'_, WifiReceiveSpec> {
        WifiRxEnW::new(self, 0)
    }
    #[doc = "Bits 1:2 - WiFi receive priority level"]
    #[inline(always)]
    pub fn wifi_rx_prio(&mut self) -> WifiRxPrioW<'_, WifiReceiveSpec> {
        WifiRxPrioW::new(self, 1)
    }
    #[doc = "Bit 4 - Force grant WiFi receive regardless of BT state"]
    #[inline(always)]
    pub fn wifi_rx_force_grant(&mut self) -> WifiRxForceGrantW<'_, WifiReceiveSpec> {
        WifiRxForceGrantW::new(self, 4)
    }
}
#[doc = "WiFi receive configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`wifi_receive::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wifi_receive::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WifiReceiveSpec;
impl crate::RegisterSpec for WifiReceiveSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wifi_receive::R`](R) reader structure"]
impl crate::Readable for WifiReceiveSpec {}
#[doc = "`write(|w| ..)` method takes [`wifi_receive::W`](W) writer structure"]
impl crate::Writable for WifiReceiveSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets wifi_receive to value 0"]
impl crate::Resettable for WifiReceiveSpec {}
