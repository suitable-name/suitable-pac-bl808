#[doc = "Register `DEV_INMPS7` reader"]
pub type R = crate::R<DevInmps7Spec>;
#[doc = "Register `DEV_INMPS7` writer"]
pub type W = crate::W<DevInmps7Spec>;
#[doc = "Field `MAXPS_IEP7` reader - "]
pub type MaxpsIep7R = crate::FieldReader<u16>;
#[doc = "Field `MAXPS_IEP7` writer - "]
pub type MaxpsIep7W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `STL_IEP7` reader - "]
pub type StlIep7R = crate::BitReader;
#[doc = "Field `STL_IEP7` writer - "]
pub type StlIep7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSTG_IEP7` reader - "]
pub type RstgIep7R = crate::BitReader;
#[doc = "Field `RSTG_IEP7` writer - "]
pub type RstgIep7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_NUM_HBW_IEP7` reader - "]
pub type TxNumHbwIep7R = crate::FieldReader;
#[doc = "Field `TX_NUM_HBW_IEP7` writer - "]
pub type TxNumHbwIep7W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TX0BYTE_IEP7` reader - "]
pub type Tx0byteIep7R = crate::BitReader;
#[doc = "Field `TX0BYTE_IEP7` writer - "]
pub type Tx0byteIep7W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:10"]
    #[inline(always)]
    pub fn maxps_iep7(&self) -> MaxpsIep7R {
        MaxpsIep7R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn stl_iep7(&self) -> StlIep7R {
        StlIep7R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn rstg_iep7(&self) -> RstgIep7R {
        RstgIep7R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:14"]
    #[inline(always)]
    pub fn tx_num_hbw_iep7(&self) -> TxNumHbwIep7R {
        TxNumHbwIep7R::new(((self.bits >> 13) & 3) as u8)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn tx0byte_iep7(&self) -> Tx0byteIep7R {
        Tx0byteIep7R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:10"]
    #[inline(always)]
    pub fn maxps_iep7(&mut self) -> MaxpsIep7W<'_, DevInmps7Spec> {
        MaxpsIep7W::new(self, 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn stl_iep7(&mut self) -> StlIep7W<'_, DevInmps7Spec> {
        StlIep7W::new(self, 11)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn rstg_iep7(&mut self) -> RstgIep7W<'_, DevInmps7Spec> {
        RstgIep7W::new(self, 12)
    }
    #[doc = "Bits 13:14"]
    #[inline(always)]
    pub fn tx_num_hbw_iep7(&mut self) -> TxNumHbwIep7W<'_, DevInmps7Spec> {
        TxNumHbwIep7W::new(self, 13)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn tx0byte_iep7(&mut self) -> Tx0byteIep7W<'_, DevInmps7Spec> {
        Tx0byteIep7W::new(self, 15)
    }
}
#[doc = "Device IN Endpoint 7 Max Packet Size\n\nYou can [`read`](crate::Reg::read) this register and get [`dev_inmps7::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dev_inmps7::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DevInmps7Spec;
impl crate::RegisterSpec for DevInmps7Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dev_inmps7::R`](R) reader structure"]
impl crate::Readable for DevInmps7Spec {}
#[doc = "`write(|w| ..)` method takes [`dev_inmps7::W`](W) writer structure"]
impl crate::Writable for DevInmps7Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DEV_INMPS7 to value 0"]
impl crate::Resettable for DevInmps7Spec {}
