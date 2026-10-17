#[doc = "Register `bt_rx_abort` reader"]
pub type R = crate::R<BtRxAbortSpec>;
#[doc = "Register `bt_rx_abort` writer"]
pub type W = crate::W<BtRxAbortSpec>;
#[doc = "Field `bt_rx_abort_en` reader - Enable BT RX abort on WLAN activity"]
pub type BtRxAbortEnR = crate::BitReader;
#[doc = "Field `bt_rx_abort_en` writer - Enable BT RX abort on WLAN activity"]
pub type BtRxAbortEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `bt_rx_abort_prio` reader - Priority threshold below which BT RX is aborted"]
pub type BtRxAbortPrioR = crate::FieldReader;
#[doc = "Field `bt_rx_abort_prio` writer - Priority threshold below which BT RX is aborted"]
pub type BtRxAbortPrioW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `bt_rx_abort_force` reader - Force abort BT RX regardless of priority"]
pub type BtRxAbortForceR = crate::BitReader;
#[doc = "Field `bt_rx_abort_force` writer - Force abort BT RX regardless of priority"]
pub type BtRxAbortForceW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable BT RX abort on WLAN activity"]
    #[inline(always)]
    pub fn bt_rx_abort_en(&self) -> BtRxAbortEnR {
        BtRxAbortEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Priority threshold below which BT RX is aborted"]
    #[inline(always)]
    pub fn bt_rx_abort_prio(&self) -> BtRxAbortPrioR {
        BtRxAbortPrioR::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 4 - Force abort BT RX regardless of priority"]
    #[inline(always)]
    pub fn bt_rx_abort_force(&self) -> BtRxAbortForceR {
        BtRxAbortForceR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable BT RX abort on WLAN activity"]
    #[inline(always)]
    pub fn bt_rx_abort_en(&mut self) -> BtRxAbortEnW<'_, BtRxAbortSpec> {
        BtRxAbortEnW::new(self, 0)
    }
    #[doc = "Bits 1:2 - Priority threshold below which BT RX is aborted"]
    #[inline(always)]
    pub fn bt_rx_abort_prio(&mut self) -> BtRxAbortPrioW<'_, BtRxAbortSpec> {
        BtRxAbortPrioW::new(self, 1)
    }
    #[doc = "Bit 4 - Force abort BT RX regardless of priority"]
    #[inline(always)]
    pub fn bt_rx_abort_force(&mut self) -> BtRxAbortForceW<'_, BtRxAbortSpec> {
        BtRxAbortForceW::new(self, 4)
    }
}
#[doc = "Bluetooth receive abort configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`bt_rx_abort::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bt_rx_abort::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BtRxAbortSpec;
impl crate::RegisterSpec for BtRxAbortSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bt_rx_abort::R`](R) reader structure"]
impl crate::Readable for BtRxAbortSpec {}
#[doc = "`write(|w| ..)` method takes [`bt_rx_abort::W`](W) writer structure"]
impl crate::Writable for BtRxAbortSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets bt_rx_abort to value 0"]
impl crate::Resettable for BtRxAbortSpec {}
