#[doc = "Register `DEV_INMPS8` reader"]
pub type R = crate::R<DevInmps8Spec>;
#[doc = "Register `DEV_INMPS8` writer"]
pub type W = crate::W<DevInmps8Spec>;
#[doc = "Field `MAXPS_IEP8` reader - "]
pub type MaxpsIep8R = crate::FieldReader<u16>;
#[doc = "Field `MAXPS_IEP8` writer - "]
pub type MaxpsIep8W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `STL_IEP8` reader - "]
pub type StlIep8R = crate::BitReader;
#[doc = "Field `STL_IEP8` writer - "]
pub type StlIep8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSTG_IEP8` reader - "]
pub type RstgIep8R = crate::BitReader;
#[doc = "Field `RSTG_IEP8` writer - "]
pub type RstgIep8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_NUM_HBW_IEP8` reader - "]
pub type TxNumHbwIep8R = crate::FieldReader;
#[doc = "Field `TX_NUM_HBW_IEP8` writer - "]
pub type TxNumHbwIep8W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TX0BYTE_IEP8` reader - "]
pub type Tx0byteIep8R = crate::BitReader;
#[doc = "Field `TX0BYTE_IEP8` writer - "]
pub type Tx0byteIep8W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:10"]
    #[inline(always)]
    pub fn maxps_iep8(&self) -> MaxpsIep8R {
        MaxpsIep8R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn stl_iep8(&self) -> StlIep8R {
        StlIep8R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn rstg_iep8(&self) -> RstgIep8R {
        RstgIep8R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:14"]
    #[inline(always)]
    pub fn tx_num_hbw_iep8(&self) -> TxNumHbwIep8R {
        TxNumHbwIep8R::new(((self.bits >> 13) & 3) as u8)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn tx0byte_iep8(&self) -> Tx0byteIep8R {
        Tx0byteIep8R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:10"]
    #[inline(always)]
    pub fn maxps_iep8(&mut self) -> MaxpsIep8W<'_, DevInmps8Spec> {
        MaxpsIep8W::new(self, 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn stl_iep8(&mut self) -> StlIep8W<'_, DevInmps8Spec> {
        StlIep8W::new(self, 11)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn rstg_iep8(&mut self) -> RstgIep8W<'_, DevInmps8Spec> {
        RstgIep8W::new(self, 12)
    }
    #[doc = "Bits 13:14"]
    #[inline(always)]
    pub fn tx_num_hbw_iep8(&mut self) -> TxNumHbwIep8W<'_, DevInmps8Spec> {
        TxNumHbwIep8W::new(self, 13)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn tx0byte_iep8(&mut self) -> Tx0byteIep8W<'_, DevInmps8Spec> {
        Tx0byteIep8W::new(self, 15)
    }
}
#[doc = "Device IN Endpoint 8 Max Packet Size\n\nYou can [`read`](crate::Reg::read) this register and get [`dev_inmps8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dev_inmps8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DevInmps8Spec;
impl crate::RegisterSpec for DevInmps8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dev_inmps8::R`](R) reader structure"]
impl crate::Readable for DevInmps8Spec {}
#[doc = "`write(|w| ..)` method takes [`dev_inmps8::W`](W) writer structure"]
impl crate::Writable for DevInmps8Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DEV_INMPS8 to value 0"]
impl crate::Resettable for DevInmps8Spec {}
