#[doc = "Register `DEV_INMPS6` reader"]
pub type R = crate::R<DevInmps6Spec>;
#[doc = "Register `DEV_INMPS6` writer"]
pub type W = crate::W<DevInmps6Spec>;
#[doc = "Field `MAXPS_IEP6` reader - "]
pub type MaxpsIep6R = crate::FieldReader<u16>;
#[doc = "Field `MAXPS_IEP6` writer - "]
pub type MaxpsIep6W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `STL_IEP6` reader - "]
pub type StlIep6R = crate::BitReader;
#[doc = "Field `STL_IEP6` writer - "]
pub type StlIep6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSTG_IEP6` reader - "]
pub type RstgIep6R = crate::BitReader;
#[doc = "Field `RSTG_IEP6` writer - "]
pub type RstgIep6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_NUM_HBW_IEP6` reader - "]
pub type TxNumHbwIep6R = crate::FieldReader;
#[doc = "Field `TX_NUM_HBW_IEP6` writer - "]
pub type TxNumHbwIep6W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TX0BYTE_IEP6` reader - "]
pub type Tx0byteIep6R = crate::BitReader;
#[doc = "Field `TX0BYTE_IEP6` writer - "]
pub type Tx0byteIep6W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:10"]
    #[inline(always)]
    pub fn maxps_iep6(&self) -> MaxpsIep6R {
        MaxpsIep6R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn stl_iep6(&self) -> StlIep6R {
        StlIep6R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn rstg_iep6(&self) -> RstgIep6R {
        RstgIep6R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:14"]
    #[inline(always)]
    pub fn tx_num_hbw_iep6(&self) -> TxNumHbwIep6R {
        TxNumHbwIep6R::new(((self.bits >> 13) & 3) as u8)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn tx0byte_iep6(&self) -> Tx0byteIep6R {
        Tx0byteIep6R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:10"]
    #[inline(always)]
    pub fn maxps_iep6(&mut self) -> MaxpsIep6W<'_, DevInmps6Spec> {
        MaxpsIep6W::new(self, 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn stl_iep6(&mut self) -> StlIep6W<'_, DevInmps6Spec> {
        StlIep6W::new(self, 11)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn rstg_iep6(&mut self) -> RstgIep6W<'_, DevInmps6Spec> {
        RstgIep6W::new(self, 12)
    }
    #[doc = "Bits 13:14"]
    #[inline(always)]
    pub fn tx_num_hbw_iep6(&mut self) -> TxNumHbwIep6W<'_, DevInmps6Spec> {
        TxNumHbwIep6W::new(self, 13)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn tx0byte_iep6(&mut self) -> Tx0byteIep6W<'_, DevInmps6Spec> {
        Tx0byteIep6W::new(self, 15)
    }
}
#[doc = "Device IN Endpoint 6 Max Packet Size\n\nYou can [`read`](crate::Reg::read) this register and get [`dev_inmps6::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dev_inmps6::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DevInmps6Spec;
impl crate::RegisterSpec for DevInmps6Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dev_inmps6::R`](R) reader structure"]
impl crate::Readable for DevInmps6Spec {}
#[doc = "`write(|w| ..)` method takes [`dev_inmps6::W`](W) writer structure"]
impl crate::Writable for DevInmps6Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DEV_INMPS6 to value 0"]
impl crate::Resettable for DevInmps6Spec {}
