#[doc = "Register `DEV_EPMAP0` reader"]
pub type R = crate::R<DevEpmap0Spec>;
#[doc = "Register `DEV_EPMAP0` writer"]
pub type W = crate::W<DevEpmap0Spec>;
#[doc = "Field `FNO_IEP1` reader - FIFO Number for IN Endpoint 1"]
pub type FnoIep1R = crate::FieldReader;
#[doc = "Field `FNO_IEP1` writer - FIFO Number for IN Endpoint 1"]
pub type FnoIep1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `FNO_OEP1` reader - FIFO Number for OUT Endpoint 1"]
pub type FnoOep1R = crate::FieldReader;
#[doc = "Field `FNO_OEP1` writer - FIFO Number for OUT Endpoint 1"]
pub type FnoOep1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `FNO_IEP2` reader - "]
pub type FnoIep2R = crate::FieldReader;
#[doc = "Field `FNO_IEP2` writer - "]
pub type FnoIep2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `FNO_OEP2` reader - "]
pub type FnoOep2R = crate::FieldReader;
#[doc = "Field `FNO_OEP2` writer - "]
pub type FnoOep2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `FNO_IEP3` reader - "]
pub type FnoIep3R = crate::FieldReader;
#[doc = "Field `FNO_IEP3` writer - "]
pub type FnoIep3W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `FNO_OEP3` reader - "]
pub type FnoOep3R = crate::FieldReader;
#[doc = "Field `FNO_OEP3` writer - "]
pub type FnoOep3W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `FNO_IEP4` reader - "]
pub type FnoIep4R = crate::FieldReader;
#[doc = "Field `FNO_IEP4` writer - "]
pub type FnoIep4W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `FNO_OEP4` reader - "]
pub type FnoOep4R = crate::FieldReader;
#[doc = "Field `FNO_OEP4` writer - "]
pub type FnoOep4W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - FIFO Number for IN Endpoint 1"]
    #[inline(always)]
    pub fn fno_iep1(&self) -> FnoIep1R {
        FnoIep1R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - FIFO Number for OUT Endpoint 1"]
    #[inline(always)]
    pub fn fno_oep1(&self) -> FnoOep1R {
        FnoOep1R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn fno_iep2(&self) -> FnoIep2R {
        FnoIep2R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn fno_oep2(&self) -> FnoOep2R {
        FnoOep2R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn fno_iep3(&self) -> FnoIep3R {
        FnoIep3R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23"]
    #[inline(always)]
    pub fn fno_oep3(&self) -> FnoOep3R {
        FnoOep3R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    pub fn fno_iep4(&self) -> FnoIep4R {
        FnoIep4R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn fno_oep4(&self) -> FnoOep4R {
        FnoOep4R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - FIFO Number for IN Endpoint 1"]
    #[inline(always)]
    pub fn fno_iep1(&mut self) -> FnoIep1W<'_, DevEpmap0Spec> {
        FnoIep1W::new(self, 0)
    }
    #[doc = "Bits 4:7 - FIFO Number for OUT Endpoint 1"]
    #[inline(always)]
    pub fn fno_oep1(&mut self) -> FnoOep1W<'_, DevEpmap0Spec> {
        FnoOep1W::new(self, 4)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn fno_iep2(&mut self) -> FnoIep2W<'_, DevEpmap0Spec> {
        FnoIep2W::new(self, 8)
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn fno_oep2(&mut self) -> FnoOep2W<'_, DevEpmap0Spec> {
        FnoOep2W::new(self, 12)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn fno_iep3(&mut self) -> FnoIep3W<'_, DevEpmap0Spec> {
        FnoIep3W::new(self, 16)
    }
    #[doc = "Bits 20:23"]
    #[inline(always)]
    pub fn fno_oep3(&mut self) -> FnoOep3W<'_, DevEpmap0Spec> {
        FnoOep3W::new(self, 20)
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    pub fn fno_iep4(&mut self) -> FnoIep4W<'_, DevEpmap0Spec> {
        FnoIep4W::new(self, 24)
    }
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn fno_oep4(&mut self) -> FnoOep4W<'_, DevEpmap0Spec> {
        FnoOep4W::new(self, 28)
    }
}
#[doc = "Device Endpoint/FIFO Mapping 0 (EP1-EP4)\n\nYou can [`read`](crate::Reg::read) this register and get [`dev_epmap0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dev_epmap0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DevEpmap0Spec;
impl crate::RegisterSpec for DevEpmap0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dev_epmap0::R`](R) reader structure"]
impl crate::Readable for DevEpmap0Spec {}
#[doc = "`write(|w| ..)` method takes [`dev_epmap0::W`](W) writer structure"]
impl crate::Writable for DevEpmap0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DEV_EPMAP0 to value 0"]
impl crate::Resettable for DevEpmap0Spec {}
