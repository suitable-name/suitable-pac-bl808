#[doc = "Register `DEV_RXZ` reader"]
pub type R = crate::R<DevRxzSpec>;
#[doc = "Field `RX0BYTE_EP1` reader - "]
pub type Rx0byteEp1R = crate::BitReader;
#[doc = "Field `RX0BYTE_EP2` reader - "]
pub type Rx0byteEp2R = crate::BitReader;
#[doc = "Field `RX0BYTE_EP3` reader - "]
pub type Rx0byteEp3R = crate::BitReader;
#[doc = "Field `RX0BYTE_EP4` reader - "]
pub type Rx0byteEp4R = crate::BitReader;
#[doc = "Field `RX0BYTE_EP5` reader - "]
pub type Rx0byteEp5R = crate::BitReader;
#[doc = "Field `RX0BYTE_EP6` reader - "]
pub type Rx0byteEp6R = crate::BitReader;
#[doc = "Field `RX0BYTE_EP7` reader - "]
pub type Rx0byteEp7R = crate::BitReader;
#[doc = "Field `RX0BYTE_EP8` reader - "]
pub type Rx0byteEp8R = crate::BitReader;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rx0byte_ep1(&self) -> Rx0byteEp1R {
        Rx0byteEp1R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn rx0byte_ep2(&self) -> Rx0byteEp2R {
        Rx0byteEp2R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn rx0byte_ep3(&self) -> Rx0byteEp3R {
        Rx0byteEp3R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn rx0byte_ep4(&self) -> Rx0byteEp4R {
        Rx0byteEp4R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn rx0byte_ep5(&self) -> Rx0byteEp5R {
        Rx0byteEp5R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn rx0byte_ep6(&self) -> Rx0byteEp6R {
        Rx0byteEp6R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn rx0byte_ep7(&self) -> Rx0byteEp7R {
        Rx0byteEp7R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn rx0byte_ep8(&self) -> Rx0byteEp8R {
        Rx0byteEp8R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "Device RX 0 Byte Status\n\nYou can [`read`](crate::Reg::read) this register and get [`dev_rxz::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DevRxzSpec;
impl crate::RegisterSpec for DevRxzSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dev_rxz::R`](R) reader structure"]
impl crate::Readable for DevRxzSpec {}
#[doc = "`reset()` method sets DEV_RXZ to value 0"]
impl crate::Resettable for DevRxzSpec {}
