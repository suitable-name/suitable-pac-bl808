#[doc = "Register `DEV_FMAP3` reader"]
pub type R = crate::R<DevFmap3Spec>;
#[doc = "Register `DEV_FMAP3` writer"]
pub type W = crate::W<DevFmap3Spec>;
#[doc = "Field `EPNO_FIFO8` reader - "]
pub type EpnoFifo8R = crate::FieldReader;
#[doc = "Field `EPNO_FIFO8` writer - "]
pub type EpnoFifo8W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DIR_FIFO8` reader - "]
pub type DirFifo8R = crate::FieldReader;
#[doc = "Field `DIR_FIFO8` writer - "]
pub type DirFifo8W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `EPNO_FIFO9` reader - "]
pub type EpnoFifo9R = crate::FieldReader;
#[doc = "Field `EPNO_FIFO9` writer - "]
pub type EpnoFifo9W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DIR_FIFO9` reader - "]
pub type DirFifo9R = crate::FieldReader;
#[doc = "Field `DIR_FIFO9` writer - "]
pub type DirFifo9W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `EPNO_FIFO10` reader - "]
pub type EpnoFifo10R = crate::FieldReader;
#[doc = "Field `EPNO_FIFO10` writer - "]
pub type EpnoFifo10W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DIR_FIFO10` reader - "]
pub type DirFifo10R = crate::FieldReader;
#[doc = "Field `DIR_FIFO10` writer - "]
pub type DirFifo10W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `EPNO_FIFO11` reader - "]
pub type EpnoFifo11R = crate::FieldReader;
#[doc = "Field `EPNO_FIFO11` writer - "]
pub type EpnoFifo11W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DIR_FIFO11` reader - "]
pub type DirFifo11R = crate::FieldReader;
#[doc = "Field `DIR_FIFO11` writer - "]
pub type DirFifo11W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn epno_fifo8(&self) -> EpnoFifo8R {
        EpnoFifo8R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn dir_fifo8(&self) -> DirFifo8R {
        DirFifo8R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn epno_fifo9(&self) -> EpnoFifo9R {
        EpnoFifo9R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn dir_fifo9(&self) -> DirFifo9R {
        DirFifo9R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn epno_fifo10(&self) -> EpnoFifo10R {
        EpnoFifo10R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    pub fn dir_fifo10(&self) -> DirFifo10R {
        DirFifo10R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    pub fn epno_fifo11(&self) -> EpnoFifo11R {
        EpnoFifo11R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:29"]
    #[inline(always)]
    pub fn dir_fifo11(&self) -> DirFifo11R {
        DirFifo11R::new(((self.bits >> 28) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn epno_fifo8(&mut self) -> EpnoFifo8W<'_, DevFmap3Spec> {
        EpnoFifo8W::new(self, 0)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn dir_fifo8(&mut self) -> DirFifo8W<'_, DevFmap3Spec> {
        DirFifo8W::new(self, 4)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn epno_fifo9(&mut self) -> EpnoFifo9W<'_, DevFmap3Spec> {
        EpnoFifo9W::new(self, 8)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn dir_fifo9(&mut self) -> DirFifo9W<'_, DevFmap3Spec> {
        DirFifo9W::new(self, 12)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn epno_fifo10(&mut self) -> EpnoFifo10W<'_, DevFmap3Spec> {
        EpnoFifo10W::new(self, 16)
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    pub fn dir_fifo10(&mut self) -> DirFifo10W<'_, DevFmap3Spec> {
        DirFifo10W::new(self, 20)
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    pub fn epno_fifo11(&mut self) -> EpnoFifo11W<'_, DevFmap3Spec> {
        EpnoFifo11W::new(self, 24)
    }
    #[doc = "Bits 28:29"]
    #[inline(always)]
    pub fn dir_fifo11(&mut self) -> DirFifo11W<'_, DevFmap3Spec> {
        DirFifo11W::new(self, 28)
    }
}
#[doc = "Device FIFO Mapping 3 (FIFO 8-11)\n\nYou can [`read`](crate::Reg::read) this register and get [`dev_fmap3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dev_fmap3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DevFmap3Spec;
impl crate::RegisterSpec for DevFmap3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dev_fmap3::R`](R) reader structure"]
impl crate::Readable for DevFmap3Spec {}
#[doc = "`write(|w| ..)` method takes [`dev_fmap3::W`](W) writer structure"]
impl crate::Writable for DevFmap3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DEV_FMAP3 to value 0"]
impl crate::Resettable for DevFmap3Spec {}
