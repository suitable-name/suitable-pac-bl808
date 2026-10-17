#[doc = "Register `bluetooth_receive` reader"]
pub type R = crate::R<BluetoothReceiveSpec>;
#[doc = "Register `bluetooth_receive` writer"]
pub type W = crate::W<BluetoothReceiveSpec>;
#[doc = "Field `bt_rx_en` reader - Enable BT receive arbitration"]
pub type BtRxEnR = crate::BitReader;
#[doc = "Field `bt_rx_en` writer - Enable BT receive arbitration"]
pub type BtRxEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `bt_rx_prio` reader - Bluetooth receive priority level"]
pub type BtRxPrioR = crate::FieldReader;
#[doc = "Field `bt_rx_prio` writer - Bluetooth receive priority level"]
pub type BtRxPrioW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `bt_rx_force_grant` reader - Force grant BT receive regardless of WiFi state"]
pub type BtRxForceGrantR = crate::BitReader;
#[doc = "Field `bt_rx_force_grant` writer - Force grant BT receive regardless of WiFi state"]
pub type BtRxForceGrantW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable BT receive arbitration"]
    #[inline(always)]
    pub fn bt_rx_en(&self) -> BtRxEnR {
        BtRxEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Bluetooth receive priority level"]
    #[inline(always)]
    pub fn bt_rx_prio(&self) -> BtRxPrioR {
        BtRxPrioR::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 4 - Force grant BT receive regardless of WiFi state"]
    #[inline(always)]
    pub fn bt_rx_force_grant(&self) -> BtRxForceGrantR {
        BtRxForceGrantR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable BT receive arbitration"]
    #[inline(always)]
    pub fn bt_rx_en(&mut self) -> BtRxEnW<'_, BluetoothReceiveSpec> {
        BtRxEnW::new(self, 0)
    }
    #[doc = "Bits 1:2 - Bluetooth receive priority level"]
    #[inline(always)]
    pub fn bt_rx_prio(&mut self) -> BtRxPrioW<'_, BluetoothReceiveSpec> {
        BtRxPrioW::new(self, 1)
    }
    #[doc = "Bit 4 - Force grant BT receive regardless of WiFi state"]
    #[inline(always)]
    pub fn bt_rx_force_grant(&mut self) -> BtRxForceGrantW<'_, BluetoothReceiveSpec> {
        BtRxForceGrantW::new(self, 4)
    }
}
#[doc = "Bluetooth receive configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`bluetooth_receive::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bluetooth_receive::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BluetoothReceiveSpec;
impl crate::RegisterSpec for BluetoothReceiveSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bluetooth_receive::R`](R) reader structure"]
impl crate::Readable for BluetoothReceiveSpec {}
#[doc = "`write(|w| ..)` method takes [`bluetooth_receive::W`](W) writer structure"]
impl crate::Writable for BluetoothReceiveSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets bluetooth_receive to value 0"]
impl crate::Resettable for BluetoothReceiveSpec {}
