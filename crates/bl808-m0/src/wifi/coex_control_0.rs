#[doc = "Register `coex_control_0` reader"]
pub type R = crate::R<CoexControl0Spec>;
#[doc = "Register `coex_control_0` writer"]
pub type W = crate::W<CoexControl0Spec>;
#[doc = "Field `coex_enable` reader - Master coexistence module enable"]
pub type CoexEnableR = crate::BitReader;
#[doc = "Field `coex_enable` writer - Master coexistence module enable"]
pub type CoexEnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `wifi_tx_prio` reader - Set WiFi TX priority level"]
pub type WifiTxPrioR = crate::FieldReader;
#[doc = "Field `wifi_tx_prio` writer - Set WiFi TX priority level"]
pub type WifiTxPrioW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `wifi_rx_prio` reader - Set WiFi RX priority level"]
pub type WifiRxPrioR = crate::FieldReader;
#[doc = "Field `wifi_rx_prio` writer - Set WiFi RX priority level"]
pub type WifiRxPrioW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ext_high_prio_req` reader - External high priority request flag"]
pub type ExtHighPrioReqR = crate::BitReader;
#[doc = "Field `ext_high_prio_req` writer - External high priority request flag"]
pub type ExtHighPrioReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `arb_mode` reader - Coexistence arbitration mode selection"]
pub type ArbModeR = crate::FieldReader;
#[doc = "Field `arb_mode` writer - Coexistence arbitration mode selection"]
pub type ArbModeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - Master coexistence module enable"]
    #[inline(always)]
    pub fn coex_enable(&self) -> CoexEnableR {
        CoexEnableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Set WiFi TX priority level"]
    #[inline(always)]
    pub fn wifi_tx_prio(&self) -> WifiTxPrioR {
        WifiTxPrioR::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bits 3:4 - Set WiFi RX priority level"]
    #[inline(always)]
    pub fn wifi_rx_prio(&self) -> WifiRxPrioR {
        WifiRxPrioR::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 8 - External high priority request flag"]
    #[inline(always)]
    pub fn ext_high_prio_req(&self) -> ExtHighPrioReqR {
        ExtHighPrioReqR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 16:17 - Coexistence arbitration mode selection"]
    #[inline(always)]
    pub fn arb_mode(&self) -> ArbModeR {
        ArbModeR::new(((self.bits >> 16) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Master coexistence module enable"]
    #[inline(always)]
    pub fn coex_enable(&mut self) -> CoexEnableW<'_, CoexControl0Spec> {
        CoexEnableW::new(self, 0)
    }
    #[doc = "Bits 1:2 - Set WiFi TX priority level"]
    #[inline(always)]
    pub fn wifi_tx_prio(&mut self) -> WifiTxPrioW<'_, CoexControl0Spec> {
        WifiTxPrioW::new(self, 1)
    }
    #[doc = "Bits 3:4 - Set WiFi RX priority level"]
    #[inline(always)]
    pub fn wifi_rx_prio(&mut self) -> WifiRxPrioW<'_, CoexControl0Spec> {
        WifiRxPrioW::new(self, 3)
    }
    #[doc = "Bit 8 - External high priority request flag"]
    #[inline(always)]
    pub fn ext_high_prio_req(&mut self) -> ExtHighPrioReqW<'_, CoexControl0Spec> {
        ExtHighPrioReqW::new(self, 8)
    }
    #[doc = "Bits 16:17 - Coexistence arbitration mode selection"]
    #[inline(always)]
    pub fn arb_mode(&mut self) -> ArbModeW<'_, CoexControl0Spec> {
        ArbModeW::new(self, 16)
    }
}
#[doc = "Coexistence Control Register 0 (Arbitration)\n\nYou can [`read`](crate::Reg::read) this register and get [`coex_control_0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`coex_control_0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CoexControl0Spec;
impl crate::RegisterSpec for CoexControl0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`coex_control_0::R`](R) reader structure"]
impl crate::Readable for CoexControl0Spec {}
#[doc = "`write(|w| ..)` method takes [`coex_control_0::W`](W) writer structure"]
impl crate::Writable for CoexControl0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets coex_control_0 to value 0"]
impl crate::Resettable for CoexControl0Spec {}
