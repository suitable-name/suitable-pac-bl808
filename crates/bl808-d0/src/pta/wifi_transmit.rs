#[doc = "Register `wifi_transmit` reader"]
pub type R = crate::R<WifiTransmitSpec>;
#[doc = "Register `wifi_transmit` writer"]
pub type W = crate::W<WifiTransmitSpec>;
#[doc = "Field `wifi_tx_en` reader - Enable WiFi transmit arbitration"]
pub type WifiTxEnR = crate::BitReader;
#[doc = "Field `wifi_tx_en` writer - Enable WiFi transmit arbitration"]
pub type WifiTxEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `wifi_tx_prio` reader - WiFi transmit priority level"]
pub type WifiTxPrioR = crate::FieldReader;
#[doc = "Field `wifi_tx_prio` writer - WiFi transmit priority level"]
pub type WifiTxPrioW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `wifi_tx_force_grant` reader - Force grant WiFi transmit regardless of BT state"]
pub type WifiTxForceGrantR = crate::BitReader;
#[doc = "Field `wifi_tx_force_grant` writer - Force grant WiFi transmit regardless of BT state"]
pub type WifiTxForceGrantW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable WiFi transmit arbitration"]
    #[inline(always)]
    pub fn wifi_tx_en(&self) -> WifiTxEnR {
        WifiTxEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - WiFi transmit priority level"]
    #[inline(always)]
    pub fn wifi_tx_prio(&self) -> WifiTxPrioR {
        WifiTxPrioR::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 4 - Force grant WiFi transmit regardless of BT state"]
    #[inline(always)]
    pub fn wifi_tx_force_grant(&self) -> WifiTxForceGrantR {
        WifiTxForceGrantR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable WiFi transmit arbitration"]
    #[inline(always)]
    pub fn wifi_tx_en(&mut self) -> WifiTxEnW<'_, WifiTransmitSpec> {
        WifiTxEnW::new(self, 0)
    }
    #[doc = "Bits 1:2 - WiFi transmit priority level"]
    #[inline(always)]
    pub fn wifi_tx_prio(&mut self) -> WifiTxPrioW<'_, WifiTransmitSpec> {
        WifiTxPrioW::new(self, 1)
    }
    #[doc = "Bit 4 - Force grant WiFi transmit regardless of BT state"]
    #[inline(always)]
    pub fn wifi_tx_force_grant(&mut self) -> WifiTxForceGrantW<'_, WifiTransmitSpec> {
        WifiTxForceGrantW::new(self, 4)
    }
}
#[doc = "WiFi transmit configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`wifi_transmit::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wifi_transmit::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WifiTransmitSpec;
impl crate::RegisterSpec for WifiTransmitSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wifi_transmit::R`](R) reader structure"]
impl crate::Readable for WifiTransmitSpec {}
#[doc = "`write(|w| ..)` method takes [`wifi_transmit::W`](W) writer structure"]
impl crate::Writable for WifiTransmitSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets wifi_transmit to value 0"]
impl crate::Resettable for WifiTransmitSpec {}
