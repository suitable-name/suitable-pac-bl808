#[doc = "Register `wlan_rx_abort` reader"]
pub type R = crate::R<WlanRxAbortSpec>;
#[doc = "Register `wlan_rx_abort` writer"]
pub type W = crate::W<WlanRxAbortSpec>;
#[doc = "Field `wlan_rx_abort_en` reader - Enable WLAN RX abort on BT activity"]
pub type WlanRxAbortEnR = crate::BitReader;
#[doc = "Field `wlan_rx_abort_en` writer - Enable WLAN RX abort on BT activity"]
pub type WlanRxAbortEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `wlan_rx_abort_prio` reader - Priority threshold below which WLAN RX is aborted"]
pub type WlanRxAbortPrioR = crate::FieldReader;
#[doc = "Field `wlan_rx_abort_prio` writer - Priority threshold below which WLAN RX is aborted"]
pub type WlanRxAbortPrioW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `wlan_rx_abort_force` reader - Force abort WLAN RX regardless of priority"]
pub type WlanRxAbortForceR = crate::BitReader;
#[doc = "Field `wlan_rx_abort_force` writer - Force abort WLAN RX regardless of priority"]
pub type WlanRxAbortForceW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable WLAN RX abort on BT activity"]
    #[inline(always)]
    pub fn wlan_rx_abort_en(&self) -> WlanRxAbortEnR {
        WlanRxAbortEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Priority threshold below which WLAN RX is aborted"]
    #[inline(always)]
    pub fn wlan_rx_abort_prio(&self) -> WlanRxAbortPrioR {
        WlanRxAbortPrioR::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 4 - Force abort WLAN RX regardless of priority"]
    #[inline(always)]
    pub fn wlan_rx_abort_force(&self) -> WlanRxAbortForceR {
        WlanRxAbortForceR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable WLAN RX abort on BT activity"]
    #[inline(always)]
    pub fn wlan_rx_abort_en(&mut self) -> WlanRxAbortEnW<'_, WlanRxAbortSpec> {
        WlanRxAbortEnW::new(self, 0)
    }
    #[doc = "Bits 1:2 - Priority threshold below which WLAN RX is aborted"]
    #[inline(always)]
    pub fn wlan_rx_abort_prio(&mut self) -> WlanRxAbortPrioW<'_, WlanRxAbortSpec> {
        WlanRxAbortPrioW::new(self, 1)
    }
    #[doc = "Bit 4 - Force abort WLAN RX regardless of priority"]
    #[inline(always)]
    pub fn wlan_rx_abort_force(&mut self) -> WlanRxAbortForceW<'_, WlanRxAbortSpec> {
        WlanRxAbortForceW::new(self, 4)
    }
}
#[doc = "WLAN receive abort configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`wlan_rx_abort::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wlan_rx_abort::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WlanRxAbortSpec;
impl crate::RegisterSpec for WlanRxAbortSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wlan_rx_abort::R`](R) reader structure"]
impl crate::Readable for WlanRxAbortSpec {}
#[doc = "`write(|w| ..)` method takes [`wlan_rx_abort::W`](W) writer structure"]
impl crate::Writable for WlanRxAbortSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets wlan_rx_abort to value 0"]
impl crate::Resettable for WlanRxAbortSpec {}
