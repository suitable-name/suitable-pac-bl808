#[doc = "Register `bt_tx_abort` reader"]
pub type R = crate::R<BtTxAbortSpec>;
#[doc = "Register `bt_tx_abort` writer"]
pub type W = crate::W<BtTxAbortSpec>;
#[doc = "Field `bt_tx_abort_en` reader - Enable BT TX abort on WLAN activity"]
pub type BtTxAbortEnR = crate::BitReader;
#[doc = "Field `bt_tx_abort_en` writer - Enable BT TX abort on WLAN activity"]
pub type BtTxAbortEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `bt_tx_abort_prio` reader - Priority threshold below which BT TX is aborted"]
pub type BtTxAbortPrioR = crate::FieldReader;
#[doc = "Field `bt_tx_abort_prio` writer - Priority threshold below which BT TX is aborted"]
pub type BtTxAbortPrioW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `bt_tx_abort_force` reader - Force abort BT TX regardless of priority"]
pub type BtTxAbortForceR = crate::BitReader;
#[doc = "Field `bt_tx_abort_force` writer - Force abort BT TX regardless of priority"]
pub type BtTxAbortForceW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable BT TX abort on WLAN activity"]
    #[inline(always)]
    pub fn bt_tx_abort_en(&self) -> BtTxAbortEnR {
        BtTxAbortEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Priority threshold below which BT TX is aborted"]
    #[inline(always)]
    pub fn bt_tx_abort_prio(&self) -> BtTxAbortPrioR {
        BtTxAbortPrioR::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 4 - Force abort BT TX regardless of priority"]
    #[inline(always)]
    pub fn bt_tx_abort_force(&self) -> BtTxAbortForceR {
        BtTxAbortForceR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable BT TX abort on WLAN activity"]
    #[inline(always)]
    pub fn bt_tx_abort_en(&mut self) -> BtTxAbortEnW<'_, BtTxAbortSpec> {
        BtTxAbortEnW::new(self, 0)
    }
    #[doc = "Bits 1:2 - Priority threshold below which BT TX is aborted"]
    #[inline(always)]
    pub fn bt_tx_abort_prio(&mut self) -> BtTxAbortPrioW<'_, BtTxAbortSpec> {
        BtTxAbortPrioW::new(self, 1)
    }
    #[doc = "Bit 4 - Force abort BT TX regardless of priority"]
    #[inline(always)]
    pub fn bt_tx_abort_force(&mut self) -> BtTxAbortForceW<'_, BtTxAbortSpec> {
        BtTxAbortForceW::new(self, 4)
    }
}
#[doc = "Bluetooth transmit abort configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`bt_tx_abort::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bt_tx_abort::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BtTxAbortSpec;
impl crate::RegisterSpec for BtTxAbortSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bt_tx_abort::R`](R) reader structure"]
impl crate::Readable for BtTxAbortSpec {}
#[doc = "`write(|w| ..)` method takes [`bt_tx_abort::W`](W) writer structure"]
impl crate::Writable for BtTxAbortSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets bt_tx_abort to value 0"]
impl crate::Resettable for BtTxAbortSpec {}
