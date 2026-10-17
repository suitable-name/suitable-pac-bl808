#[doc = "Register `DEV_EPMAP1` reader"]
pub type R = crate::R<DevEpmap1Spec>;
#[doc = "Register `DEV_EPMAP1` writer"]
pub type W = crate::W<DevEpmap1Spec>;
#[doc = "Field `FNO_IEP5` reader - "]
pub type FnoIep5R = crate::FieldReader;
#[doc = "Field `FNO_IEP5` writer - "]
pub type FnoIep5W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `FNO_OEP5` reader - "]
pub type FnoOep5R = crate::FieldReader;
#[doc = "Field `FNO_OEP5` writer - "]
pub type FnoOep5W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `FNO_IEP6` reader - "]
pub type FnoIep6R = crate::FieldReader;
#[doc = "Field `FNO_IEP6` writer - "]
pub type FnoIep6W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `FNO_OEP6` reader - "]
pub type FnoOep6R = crate::FieldReader;
#[doc = "Field `FNO_OEP6` writer - "]
pub type FnoOep6W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `FNO_IEP7` reader - "]
pub type FnoIep7R = crate::FieldReader;
#[doc = "Field `FNO_IEP7` writer - "]
pub type FnoIep7W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `FNO_OEP7` reader - "]
pub type FnoOep7R = crate::FieldReader;
#[doc = "Field `FNO_OEP7` writer - "]
pub type FnoOep7W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `FNO_IEP8` reader - "]
pub type FnoIep8R = crate::FieldReader;
#[doc = "Field `FNO_IEP8` writer - "]
pub type FnoIep8W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `FNO_OEP8` reader - "]
pub type FnoOep8R = crate::FieldReader;
#[doc = "Field `FNO_OEP8` writer - "]
pub type FnoOep8W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn fno_iep5(&self) -> FnoIep5R {
        FnoIep5R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn fno_oep5(&self) -> FnoOep5R {
        FnoOep5R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn fno_iep6(&self) -> FnoIep6R {
        FnoIep6R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn fno_oep6(&self) -> FnoOep6R {
        FnoOep6R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn fno_iep7(&self) -> FnoIep7R {
        FnoIep7R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23"]
    #[inline(always)]
    pub fn fno_oep7(&self) -> FnoOep7R {
        FnoOep7R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    pub fn fno_iep8(&self) -> FnoIep8R {
        FnoIep8R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn fno_oep8(&self) -> FnoOep8R {
        FnoOep8R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn fno_iep5(&mut self) -> FnoIep5W<'_, DevEpmap1Spec> {
        FnoIep5W::new(self, 0)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn fno_oep5(&mut self) -> FnoOep5W<'_, DevEpmap1Spec> {
        FnoOep5W::new(self, 4)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn fno_iep6(&mut self) -> FnoIep6W<'_, DevEpmap1Spec> {
        FnoIep6W::new(self, 8)
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn fno_oep6(&mut self) -> FnoOep6W<'_, DevEpmap1Spec> {
        FnoOep6W::new(self, 12)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn fno_iep7(&mut self) -> FnoIep7W<'_, DevEpmap1Spec> {
        FnoIep7W::new(self, 16)
    }
    #[doc = "Bits 20:23"]
    #[inline(always)]
    pub fn fno_oep7(&mut self) -> FnoOep7W<'_, DevEpmap1Spec> {
        FnoOep7W::new(self, 20)
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    pub fn fno_iep8(&mut self) -> FnoIep8W<'_, DevEpmap1Spec> {
        FnoIep8W::new(self, 24)
    }
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn fno_oep8(&mut self) -> FnoOep8W<'_, DevEpmap1Spec> {
        FnoOep8W::new(self, 28)
    }
}
#[doc = "Device Endpoint/FIFO Mapping 1 (EP5-EP8)\n\nYou can [`read`](crate::Reg::read) this register and get [`dev_epmap1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dev_epmap1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DevEpmap1Spec;
impl crate::RegisterSpec for DevEpmap1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dev_epmap1::R`](R) reader structure"]
impl crate::Readable for DevEpmap1Spec {}
#[doc = "`write(|w| ..)` method takes [`dev_epmap1::W`](W) writer structure"]
impl crate::Writable for DevEpmap1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DEV_EPMAP1 to value 0"]
impl crate::Resettable for DevEpmap1Spec {}
