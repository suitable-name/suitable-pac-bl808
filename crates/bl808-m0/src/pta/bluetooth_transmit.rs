#[doc = "Register `bluetooth_transmit` reader"]
pub type R = crate::R<BluetoothTransmitSpec>;
#[doc = "Register `bluetooth_transmit` writer"]
pub type W = crate::W<BluetoothTransmitSpec>;
#[doc = "Field `bt_tx_en` reader - Enable BT transmit arbitration"]
pub type BtTxEnR = crate::BitReader;
#[doc = "Field `bt_tx_en` writer - Enable BT transmit arbitration"]
pub type BtTxEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `bt_tx_prio` reader - Bluetooth transmit priority level"]
pub type BtTxPrioR = crate::FieldReader;
#[doc = "Field `bt_tx_prio` writer - Bluetooth transmit priority level"]
pub type BtTxPrioW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `bt_tx_force_grant` reader - Force grant BT transmit regardless of WiFi state"]
pub type BtTxForceGrantR = crate::BitReader;
#[doc = "Field `bt_tx_force_grant` writer - Force grant BT transmit regardless of WiFi state"]
pub type BtTxForceGrantW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable BT transmit arbitration"]
    #[inline(always)]
    pub fn bt_tx_en(&self) -> BtTxEnR {
        BtTxEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Bluetooth transmit priority level"]
    #[inline(always)]
    pub fn bt_tx_prio(&self) -> BtTxPrioR {
        BtTxPrioR::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 4 - Force grant BT transmit regardless of WiFi state"]
    #[inline(always)]
    pub fn bt_tx_force_grant(&self) -> BtTxForceGrantR {
        BtTxForceGrantR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable BT transmit arbitration"]
    #[inline(always)]
    pub fn bt_tx_en(&mut self) -> BtTxEnW<'_, BluetoothTransmitSpec> {
        BtTxEnW::new(self, 0)
    }
    #[doc = "Bits 1:2 - Bluetooth transmit priority level"]
    #[inline(always)]
    pub fn bt_tx_prio(&mut self) -> BtTxPrioW<'_, BluetoothTransmitSpec> {
        BtTxPrioW::new(self, 1)
    }
    #[doc = "Bit 4 - Force grant BT transmit regardless of WiFi state"]
    #[inline(always)]
    pub fn bt_tx_force_grant(&mut self) -> BtTxForceGrantW<'_, BluetoothTransmitSpec> {
        BtTxForceGrantW::new(self, 4)
    }
}
#[doc = "Bluetooth transmit configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`bluetooth_transmit::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bluetooth_transmit::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BluetoothTransmitSpec;
impl crate::RegisterSpec for BluetoothTransmitSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bluetooth_transmit::R`](R) reader structure"]
impl crate::Readable for BluetoothTransmitSpec {}
#[doc = "`write(|w| ..)` method takes [`bluetooth_transmit::W`](W) writer structure"]
impl crate::Writable for BluetoothTransmitSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets bluetooth_transmit to value 0"]
impl crate::Resettable for BluetoothTransmitSpec {}
