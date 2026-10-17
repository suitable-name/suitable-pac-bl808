#[doc = "Register `DEV_INMPS3` reader"]
pub type R = crate::R<DevInmps3Spec>;
#[doc = "Register `DEV_INMPS3` writer"]
pub type W = crate::W<DevInmps3Spec>;
#[doc = "Field `MAXPS_IEP3` reader - "]
pub type MaxpsIep3R = crate::FieldReader<u16>;
#[doc = "Field `MAXPS_IEP3` writer - "]
pub type MaxpsIep3W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `STL_IEP3` reader - "]
pub type StlIep3R = crate::BitReader;
#[doc = "Field `STL_IEP3` writer - "]
pub type StlIep3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSTG_IEP3` reader - "]
pub type RstgIep3R = crate::BitReader;
#[doc = "Field `RSTG_IEP3` writer - "]
pub type RstgIep3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_NUM_HBW_IEP3` reader - "]
pub type TxNumHbwIep3R = crate::FieldReader;
#[doc = "Field `TX_NUM_HBW_IEP3` writer - "]
pub type TxNumHbwIep3W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TX0BYTE_IEP3` reader - "]
pub type Tx0byteIep3R = crate::BitReader;
#[doc = "Field `TX0BYTE_IEP3` writer - "]
pub type Tx0byteIep3W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:10"]
    #[inline(always)]
    pub fn maxps_iep3(&self) -> MaxpsIep3R {
        MaxpsIep3R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn stl_iep3(&self) -> StlIep3R {
        StlIep3R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn rstg_iep3(&self) -> RstgIep3R {
        RstgIep3R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:14"]
    #[inline(always)]
    pub fn tx_num_hbw_iep3(&self) -> TxNumHbwIep3R {
        TxNumHbwIep3R::new(((self.bits >> 13) & 3) as u8)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn tx0byte_iep3(&self) -> Tx0byteIep3R {
        Tx0byteIep3R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:10"]
    #[inline(always)]
    pub fn maxps_iep3(&mut self) -> MaxpsIep3W<'_, DevInmps3Spec> {
        MaxpsIep3W::new(self, 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn stl_iep3(&mut self) -> StlIep3W<'_, DevInmps3Spec> {
        StlIep3W::new(self, 11)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn rstg_iep3(&mut self) -> RstgIep3W<'_, DevInmps3Spec> {
        RstgIep3W::new(self, 12)
    }
    #[doc = "Bits 13:14"]
    #[inline(always)]
    pub fn tx_num_hbw_iep3(&mut self) -> TxNumHbwIep3W<'_, DevInmps3Spec> {
        TxNumHbwIep3W::new(self, 13)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn tx0byte_iep3(&mut self) -> Tx0byteIep3W<'_, DevInmps3Spec> {
        Tx0byteIep3W::new(self, 15)
    }
}
#[doc = "Device IN Endpoint 3 Max Packet Size\n\nYou can [`read`](crate::Reg::read) this register and get [`dev_inmps3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dev_inmps3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DevInmps3Spec;
impl crate::RegisterSpec for DevInmps3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dev_inmps3::R`](R) reader structure"]
impl crate::Readable for DevInmps3Spec {}
#[doc = "`write(|w| ..)` method takes [`dev_inmps3::W`](W) writer structure"]
impl crate::Writable for DevInmps3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DEV_INMPS3 to value 0"]
impl crate::Resettable for DevInmps3Spec {}
