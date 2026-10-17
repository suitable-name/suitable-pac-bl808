#[doc = "Register `DEV_INMPS4` reader"]
pub type R = crate::R<DevInmps4Spec>;
#[doc = "Register `DEV_INMPS4` writer"]
pub type W = crate::W<DevInmps4Spec>;
#[doc = "Field `MAXPS_IEP4` reader - "]
pub type MaxpsIep4R = crate::FieldReader<u16>;
#[doc = "Field `MAXPS_IEP4` writer - "]
pub type MaxpsIep4W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `STL_IEP4` reader - "]
pub type StlIep4R = crate::BitReader;
#[doc = "Field `STL_IEP4` writer - "]
pub type StlIep4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSTG_IEP4` reader - "]
pub type RstgIep4R = crate::BitReader;
#[doc = "Field `RSTG_IEP4` writer - "]
pub type RstgIep4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_NUM_HBW_IEP4` reader - "]
pub type TxNumHbwIep4R = crate::FieldReader;
#[doc = "Field `TX_NUM_HBW_IEP4` writer - "]
pub type TxNumHbwIep4W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TX0BYTE_IEP4` reader - "]
pub type Tx0byteIep4R = crate::BitReader;
#[doc = "Field `TX0BYTE_IEP4` writer - "]
pub type Tx0byteIep4W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:10"]
    #[inline(always)]
    pub fn maxps_iep4(&self) -> MaxpsIep4R {
        MaxpsIep4R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn stl_iep4(&self) -> StlIep4R {
        StlIep4R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn rstg_iep4(&self) -> RstgIep4R {
        RstgIep4R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:14"]
    #[inline(always)]
    pub fn tx_num_hbw_iep4(&self) -> TxNumHbwIep4R {
        TxNumHbwIep4R::new(((self.bits >> 13) & 3) as u8)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn tx0byte_iep4(&self) -> Tx0byteIep4R {
        Tx0byteIep4R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:10"]
    #[inline(always)]
    pub fn maxps_iep4(&mut self) -> MaxpsIep4W<'_, DevInmps4Spec> {
        MaxpsIep4W::new(self, 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn stl_iep4(&mut self) -> StlIep4W<'_, DevInmps4Spec> {
        StlIep4W::new(self, 11)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn rstg_iep4(&mut self) -> RstgIep4W<'_, DevInmps4Spec> {
        RstgIep4W::new(self, 12)
    }
    #[doc = "Bits 13:14"]
    #[inline(always)]
    pub fn tx_num_hbw_iep4(&mut self) -> TxNumHbwIep4W<'_, DevInmps4Spec> {
        TxNumHbwIep4W::new(self, 13)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn tx0byte_iep4(&mut self) -> Tx0byteIep4W<'_, DevInmps4Spec> {
        Tx0byteIep4W::new(self, 15)
    }
}
#[doc = "Device IN Endpoint 4 Max Packet Size\n\nYou can [`read`](crate::Reg::read) this register and get [`dev_inmps4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dev_inmps4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DevInmps4Spec;
impl crate::RegisterSpec for DevInmps4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dev_inmps4::R`](R) reader structure"]
impl crate::Readable for DevInmps4Spec {}
#[doc = "`write(|w| ..)` method takes [`dev_inmps4::W`](W) writer structure"]
impl crate::Writable for DevInmps4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DEV_INMPS4 to value 0"]
impl crate::Resettable for DevInmps4Spec {}
