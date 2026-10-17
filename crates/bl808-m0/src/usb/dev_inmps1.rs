#[doc = "Register `DEV_INMPS1` reader"]
pub type R = crate::R<DevInmps1Spec>;
#[doc = "Register `DEV_INMPS1` writer"]
pub type W = crate::W<DevInmps1Spec>;
#[doc = "Field `MAXPS_IEP1` reader - "]
pub type MaxpsIep1R = crate::FieldReader<u16>;
#[doc = "Field `MAXPS_IEP1` writer - "]
pub type MaxpsIep1W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `STL_IEP1` reader - Stall Endpoint 1"]
pub type StlIep1R = crate::BitReader;
#[doc = "Field `STL_IEP1` writer - Stall Endpoint 1"]
pub type StlIep1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSTG_IEP1` reader - Reset Toggle for Endpoint 1"]
pub type RstgIep1R = crate::BitReader;
#[doc = "Field `RSTG_IEP1` writer - Reset Toggle for Endpoint 1"]
pub type RstgIep1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_NUM_HBW_IEP1` reader - TX number of high bandwidth transactions"]
pub type TxNumHbwIep1R = crate::FieldReader;
#[doc = "Field `TX_NUM_HBW_IEP1` writer - TX number of high bandwidth transactions"]
pub type TxNumHbwIep1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TX0BYTE_IEP1` reader - TX 0 byte control"]
pub type Tx0byteIep1R = crate::BitReader;
#[doc = "Field `TX0BYTE_IEP1` writer - TX 0 byte control"]
pub type Tx0byteIep1W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:10"]
    #[inline(always)]
    pub fn maxps_iep1(&self) -> MaxpsIep1R {
        MaxpsIep1R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bit 11 - Stall Endpoint 1"]
    #[inline(always)]
    pub fn stl_iep1(&self) -> StlIep1R {
        StlIep1R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Reset Toggle for Endpoint 1"]
    #[inline(always)]
    pub fn rstg_iep1(&self) -> RstgIep1R {
        RstgIep1R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:14 - TX number of high bandwidth transactions"]
    #[inline(always)]
    pub fn tx_num_hbw_iep1(&self) -> TxNumHbwIep1R {
        TxNumHbwIep1R::new(((self.bits >> 13) & 3) as u8)
    }
    #[doc = "Bit 15 - TX 0 byte control"]
    #[inline(always)]
    pub fn tx0byte_iep1(&self) -> Tx0byteIep1R {
        Tx0byteIep1R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:10"]
    #[inline(always)]
    pub fn maxps_iep1(&mut self) -> MaxpsIep1W<'_, DevInmps1Spec> {
        MaxpsIep1W::new(self, 0)
    }
    #[doc = "Bit 11 - Stall Endpoint 1"]
    #[inline(always)]
    pub fn stl_iep1(&mut self) -> StlIep1W<'_, DevInmps1Spec> {
        StlIep1W::new(self, 11)
    }
    #[doc = "Bit 12 - Reset Toggle for Endpoint 1"]
    #[inline(always)]
    pub fn rstg_iep1(&mut self) -> RstgIep1W<'_, DevInmps1Spec> {
        RstgIep1W::new(self, 12)
    }
    #[doc = "Bits 13:14 - TX number of high bandwidth transactions"]
    #[inline(always)]
    pub fn tx_num_hbw_iep1(&mut self) -> TxNumHbwIep1W<'_, DevInmps1Spec> {
        TxNumHbwIep1W::new(self, 13)
    }
    #[doc = "Bit 15 - TX 0 byte control"]
    #[inline(always)]
    pub fn tx0byte_iep1(&mut self) -> Tx0byteIep1W<'_, DevInmps1Spec> {
        Tx0byteIep1W::new(self, 15)
    }
}
#[doc = "Device IN Endpoint 1 Max Packet Size\n\nYou can [`read`](crate::Reg::read) this register and get [`dev_inmps1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dev_inmps1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DevInmps1Spec;
impl crate::RegisterSpec for DevInmps1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dev_inmps1::R`](R) reader structure"]
impl crate::Readable for DevInmps1Spec {}
#[doc = "`write(|w| ..)` method takes [`dev_inmps1::W`](W) writer structure"]
impl crate::Writable for DevInmps1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DEV_INMPS1 to value 0"]
impl crate::Resettable for DevInmps1Spec {}
