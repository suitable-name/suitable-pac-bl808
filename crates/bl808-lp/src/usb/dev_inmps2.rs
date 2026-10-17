#[doc = "Register `DEV_INMPS2` reader"]
pub type R = crate::R<DevInmps2Spec>;
#[doc = "Register `DEV_INMPS2` writer"]
pub type W = crate::W<DevInmps2Spec>;
#[doc = "Field `MAXPS_IEP2` reader - "]
pub type MaxpsIep2R = crate::FieldReader<u16>;
#[doc = "Field `MAXPS_IEP2` writer - "]
pub type MaxpsIep2W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `STL_IEP2` reader - Stall Endpoint 2"]
pub type StlIep2R = crate::BitReader;
#[doc = "Field `STL_IEP2` writer - Stall Endpoint 2"]
pub type StlIep2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSTG_IEP2` reader - Reset Toggle for Endpoint 2"]
pub type RstgIep2R = crate::BitReader;
#[doc = "Field `RSTG_IEP2` writer - Reset Toggle for Endpoint 2"]
pub type RstgIep2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_NUM_HBW_IEP2` reader - "]
pub type TxNumHbwIep2R = crate::FieldReader;
#[doc = "Field `TX_NUM_HBW_IEP2` writer - "]
pub type TxNumHbwIep2W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TX0BYTE_IEP2` reader - "]
pub type Tx0byteIep2R = crate::BitReader;
#[doc = "Field `TX0BYTE_IEP2` writer - "]
pub type Tx0byteIep2W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:10"]
    #[inline(always)]
    pub fn maxps_iep2(&self) -> MaxpsIep2R {
        MaxpsIep2R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bit 11 - Stall Endpoint 2"]
    #[inline(always)]
    pub fn stl_iep2(&self) -> StlIep2R {
        StlIep2R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Reset Toggle for Endpoint 2"]
    #[inline(always)]
    pub fn rstg_iep2(&self) -> RstgIep2R {
        RstgIep2R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:14"]
    #[inline(always)]
    pub fn tx_num_hbw_iep2(&self) -> TxNumHbwIep2R {
        TxNumHbwIep2R::new(((self.bits >> 13) & 3) as u8)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn tx0byte_iep2(&self) -> Tx0byteIep2R {
        Tx0byteIep2R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:10"]
    #[inline(always)]
    pub fn maxps_iep2(&mut self) -> MaxpsIep2W<'_, DevInmps2Spec> {
        MaxpsIep2W::new(self, 0)
    }
    #[doc = "Bit 11 - Stall Endpoint 2"]
    #[inline(always)]
    pub fn stl_iep2(&mut self) -> StlIep2W<'_, DevInmps2Spec> {
        StlIep2W::new(self, 11)
    }
    #[doc = "Bit 12 - Reset Toggle for Endpoint 2"]
    #[inline(always)]
    pub fn rstg_iep2(&mut self) -> RstgIep2W<'_, DevInmps2Spec> {
        RstgIep2W::new(self, 12)
    }
    #[doc = "Bits 13:14"]
    #[inline(always)]
    pub fn tx_num_hbw_iep2(&mut self) -> TxNumHbwIep2W<'_, DevInmps2Spec> {
        TxNumHbwIep2W::new(self, 13)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn tx0byte_iep2(&mut self) -> Tx0byteIep2W<'_, DevInmps2Spec> {
        Tx0byteIep2W::new(self, 15)
    }
}
#[doc = "Device IN Endpoint 2 Max Packet Size\n\nYou can [`read`](crate::Reg::read) this register and get [`dev_inmps2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dev_inmps2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DevInmps2Spec;
impl crate::RegisterSpec for DevInmps2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dev_inmps2::R`](R) reader structure"]
impl crate::Readable for DevInmps2Spec {}
#[doc = "`write(|w| ..)` method takes [`dev_inmps2::W`](W) writer structure"]
impl crate::Writable for DevInmps2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DEV_INMPS2 to value 0"]
impl crate::Resettable for DevInmps2Spec {}
