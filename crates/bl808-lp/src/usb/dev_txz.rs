#[doc = "Register `DEV_TXZ` reader"]
pub type R = crate::R<DevTxzSpec>;
#[doc = "Field `TX0BYTE_EP1` reader - "]
pub type Tx0byteEp1R = crate::BitReader;
#[doc = "Field `TX0BYTE_EP2` reader - "]
pub type Tx0byteEp2R = crate::BitReader;
#[doc = "Field `TX0BYTE_EP3` reader - "]
pub type Tx0byteEp3R = crate::BitReader;
#[doc = "Field `TX0BYTE_EP4` reader - "]
pub type Tx0byteEp4R = crate::BitReader;
#[doc = "Field `TX0BYTE_EP5` reader - "]
pub type Tx0byteEp5R = crate::BitReader;
#[doc = "Field `TX0BYTE_EP6` reader - "]
pub type Tx0byteEp6R = crate::BitReader;
#[doc = "Field `TX0BYTE_EP7` reader - "]
pub type Tx0byteEp7R = crate::BitReader;
#[doc = "Field `TX0BYTE_EP8` reader - "]
pub type Tx0byteEp8R = crate::BitReader;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn tx0byte_ep1(&self) -> Tx0byteEp1R {
        Tx0byteEp1R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn tx0byte_ep2(&self) -> Tx0byteEp2R {
        Tx0byteEp2R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn tx0byte_ep3(&self) -> Tx0byteEp3R {
        Tx0byteEp3R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn tx0byte_ep4(&self) -> Tx0byteEp4R {
        Tx0byteEp4R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn tx0byte_ep5(&self) -> Tx0byteEp5R {
        Tx0byteEp5R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn tx0byte_ep6(&self) -> Tx0byteEp6R {
        Tx0byteEp6R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn tx0byte_ep7(&self) -> Tx0byteEp7R {
        Tx0byteEp7R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn tx0byte_ep8(&self) -> Tx0byteEp8R {
        Tx0byteEp8R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "Device TX 0 Byte Status\n\nYou can [`read`](crate::Reg::read) this register and get [`dev_txz::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DevTxzSpec;
impl crate::RegisterSpec for DevTxzSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dev_txz::R`](R) reader structure"]
impl crate::Readable for DevTxzSpec {}
#[doc = "`reset()` method sets DEV_TXZ to value 0"]
impl crate::Resettable for DevTxzSpec {}
