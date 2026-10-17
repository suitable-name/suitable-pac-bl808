#[doc = "Register `DEV_INMPS5` reader"]
pub type R = crate::R<DevInmps5Spec>;
#[doc = "Register `DEV_INMPS5` writer"]
pub type W = crate::W<DevInmps5Spec>;
#[doc = "Field `MAXPS_IEP5` reader - "]
pub type MaxpsIep5R = crate::FieldReader<u16>;
#[doc = "Field `MAXPS_IEP5` writer - "]
pub type MaxpsIep5W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `STL_IEP5` reader - "]
pub type StlIep5R = crate::BitReader;
#[doc = "Field `STL_IEP5` writer - "]
pub type StlIep5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSTG_IEP5` reader - "]
pub type RstgIep5R = crate::BitReader;
#[doc = "Field `RSTG_IEP5` writer - "]
pub type RstgIep5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_NUM_HBW_IEP5` reader - "]
pub type TxNumHbwIep5R = crate::FieldReader;
#[doc = "Field `TX_NUM_HBW_IEP5` writer - "]
pub type TxNumHbwIep5W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TX0BYTE_IEP5` reader - "]
pub type Tx0byteIep5R = crate::BitReader;
#[doc = "Field `TX0BYTE_IEP5` writer - "]
pub type Tx0byteIep5W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:10"]
    #[inline(always)]
    pub fn maxps_iep5(&self) -> MaxpsIep5R {
        MaxpsIep5R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn stl_iep5(&self) -> StlIep5R {
        StlIep5R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn rstg_iep5(&self) -> RstgIep5R {
        RstgIep5R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:14"]
    #[inline(always)]
    pub fn tx_num_hbw_iep5(&self) -> TxNumHbwIep5R {
        TxNumHbwIep5R::new(((self.bits >> 13) & 3) as u8)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn tx0byte_iep5(&self) -> Tx0byteIep5R {
        Tx0byteIep5R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:10"]
    #[inline(always)]
    pub fn maxps_iep5(&mut self) -> MaxpsIep5W<'_, DevInmps5Spec> {
        MaxpsIep5W::new(self, 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn stl_iep5(&mut self) -> StlIep5W<'_, DevInmps5Spec> {
        StlIep5W::new(self, 11)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn rstg_iep5(&mut self) -> RstgIep5W<'_, DevInmps5Spec> {
        RstgIep5W::new(self, 12)
    }
    #[doc = "Bits 13:14"]
    #[inline(always)]
    pub fn tx_num_hbw_iep5(&mut self) -> TxNumHbwIep5W<'_, DevInmps5Spec> {
        TxNumHbwIep5W::new(self, 13)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn tx0byte_iep5(&mut self) -> Tx0byteIep5W<'_, DevInmps5Spec> {
        Tx0byteIep5W::new(self, 15)
    }
}
#[doc = "Device IN Endpoint 5 Max Packet Size\n\nYou can [`read`](crate::Reg::read) this register and get [`dev_inmps5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dev_inmps5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DevInmps5Spec;
impl crate::RegisterSpec for DevInmps5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dev_inmps5::R`](R) reader structure"]
impl crate::Readable for DevInmps5Spec {}
#[doc = "`write(|w| ..)` method takes [`dev_inmps5::W`](W) writer structure"]
impl crate::Writable for DevInmps5Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DEV_INMPS5 to value 0"]
impl crate::Resettable for DevInmps5Spec {}
