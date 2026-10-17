#[doc = "Register `coex_state` reader"]
pub type R = crate::R<CoexStateSpec>;
#[doc = "Field `sts_wifi_tx_active` reader - Status: WiFi TX is active"]
pub type StsWifiTxActiveR = crate::BitReader;
#[doc = "Field `sts_wifi_rx_active` reader - Status: WiFi RX is active"]
pub type StsWifiRxActiveR = crate::BitReader;
#[doc = "Field `sts_ext_busy` reader - Status: External radio (BT/Zigbee) is asserting busy"]
pub type StsExtBusyR = crate::BitReader;
#[doc = "Field `sts_current_grant` reader - Status: Which radio currently holds the grant (0=WiFi, 1=External)"]
pub type StsCurrentGrantR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Status: WiFi TX is active"]
    #[inline(always)]
    pub fn sts_wifi_tx_active(&self) -> StsWifiTxActiveR {
        StsWifiTxActiveR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Status: WiFi RX is active"]
    #[inline(always)]
    pub fn sts_wifi_rx_active(&self) -> StsWifiRxActiveR {
        StsWifiRxActiveR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Status: External radio (BT/Zigbee) is asserting busy"]
    #[inline(always)]
    pub fn sts_ext_busy(&self) -> StsExtBusyR {
        StsExtBusyR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Status: Which radio currently holds the grant (0=WiFi, 1=External)"]
    #[inline(always)]
    pub fn sts_current_grant(&self) -> StsCurrentGrantR {
        StsCurrentGrantR::new(((self.bits >> 4) & 1) != 0)
    }
}
#[doc = "Coexistence State Register (Read-Only)\n\nYou can [`read`](crate::Reg::read) this register and get [`coex_state::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CoexStateSpec;
impl crate::RegisterSpec for CoexStateSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`coex_state::R`](R) reader structure"]
impl crate::Readable for CoexStateSpec {}
#[doc = "`reset()` method sets coex_state to value 0"]
impl crate::Resettable for CoexStateSpec {}
