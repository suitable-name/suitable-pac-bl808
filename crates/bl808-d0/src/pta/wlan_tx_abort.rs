#[doc = "Register `wlan_tx_abort` reader"]
pub type R = crate::R<WlanTxAbortSpec>;
#[doc = "Register `wlan_tx_abort` writer"]
pub type W = crate::W<WlanTxAbortSpec>;
#[doc = "Field `wlan_tx_abort_en` reader - Enable WLAN TX abort on BT activity"]
pub type WlanTxAbortEnR = crate::BitReader;
#[doc = "Field `wlan_tx_abort_en` writer - Enable WLAN TX abort on BT activity"]
pub type WlanTxAbortEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `wlan_tx_abort_prio` reader - Priority threshold below which WLAN TX is aborted"]
pub type WlanTxAbortPrioR = crate::FieldReader;
#[doc = "Field `wlan_tx_abort_prio` writer - Priority threshold below which WLAN TX is aborted"]
pub type WlanTxAbortPrioW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `wlan_tx_abort_force` reader - Force abort WLAN TX regardless of priority"]
pub type WlanTxAbortForceR = crate::BitReader;
#[doc = "Field `wlan_tx_abort_force` writer - Force abort WLAN TX regardless of priority"]
pub type WlanTxAbortForceW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable WLAN TX abort on BT activity"]
    #[inline(always)]
    pub fn wlan_tx_abort_en(&self) -> WlanTxAbortEnR {
        WlanTxAbortEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Priority threshold below which WLAN TX is aborted"]
    #[inline(always)]
    pub fn wlan_tx_abort_prio(&self) -> WlanTxAbortPrioR {
        WlanTxAbortPrioR::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 4 - Force abort WLAN TX regardless of priority"]
    #[inline(always)]
    pub fn wlan_tx_abort_force(&self) -> WlanTxAbortForceR {
        WlanTxAbortForceR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable WLAN TX abort on BT activity"]
    #[inline(always)]
    pub fn wlan_tx_abort_en(&mut self) -> WlanTxAbortEnW<'_, WlanTxAbortSpec> {
        WlanTxAbortEnW::new(self, 0)
    }
    #[doc = "Bits 1:2 - Priority threshold below which WLAN TX is aborted"]
    #[inline(always)]
    pub fn wlan_tx_abort_prio(&mut self) -> WlanTxAbortPrioW<'_, WlanTxAbortSpec> {
        WlanTxAbortPrioW::new(self, 1)
    }
    #[doc = "Bit 4 - Force abort WLAN TX regardless of priority"]
    #[inline(always)]
    pub fn wlan_tx_abort_force(&mut self) -> WlanTxAbortForceW<'_, WlanTxAbortSpec> {
        WlanTxAbortForceW::new(self, 4)
    }
}
#[doc = "WLAN transmit abort configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`wlan_tx_abort::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wlan_tx_abort::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WlanTxAbortSpec;
impl crate::RegisterSpec for WlanTxAbortSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wlan_tx_abort::R`](R) reader structure"]
impl crate::Readable for WlanTxAbortSpec {}
#[doc = "`write(|w| ..)` method takes [`wlan_tx_abort::W`](W) writer structure"]
impl crate::Writable for WlanTxAbortSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets wlan_tx_abort to value 0"]
impl crate::Resettable for WlanTxAbortSpec {}
