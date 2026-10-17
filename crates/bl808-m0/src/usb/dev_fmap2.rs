#[doc = "Register `DEV_FMAP2` reader"]
pub type R = crate::R<DevFmap2Spec>;
#[doc = "Register `DEV_FMAP2` writer"]
pub type W = crate::W<DevFmap2Spec>;
#[doc = "Field `EPNO_FIFO4` reader - "]
pub type EpnoFifo4R = crate::FieldReader;
#[doc = "Field `EPNO_FIFO4` writer - "]
pub type EpnoFifo4W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DIR_FIFO4` reader - "]
pub type DirFifo4R = crate::FieldReader;
#[doc = "Field `DIR_FIFO4` writer - "]
pub type DirFifo4W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `EPNO_FIFO5` reader - "]
pub type EpnoFifo5R = crate::FieldReader;
#[doc = "Field `EPNO_FIFO5` writer - "]
pub type EpnoFifo5W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DIR_FIFO5` reader - "]
pub type DirFifo5R = crate::FieldReader;
#[doc = "Field `DIR_FIFO5` writer - "]
pub type DirFifo5W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `EPNO_FIFO6` reader - "]
pub type EpnoFifo6R = crate::FieldReader;
#[doc = "Field `EPNO_FIFO6` writer - "]
pub type EpnoFifo6W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DIR_FIFO6` reader - "]
pub type DirFifo6R = crate::FieldReader;
#[doc = "Field `DIR_FIFO6` writer - "]
pub type DirFifo6W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `EPNO_FIFO7` reader - "]
pub type EpnoFifo7R = crate::FieldReader;
#[doc = "Field `EPNO_FIFO7` writer - "]
pub type EpnoFifo7W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DIR_FIFO7` reader - "]
pub type DirFifo7R = crate::FieldReader;
#[doc = "Field `DIR_FIFO7` writer - "]
pub type DirFifo7W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn epno_fifo4(&self) -> EpnoFifo4R {
        EpnoFifo4R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn dir_fifo4(&self) -> DirFifo4R {
        DirFifo4R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn epno_fifo5(&self) -> EpnoFifo5R {
        EpnoFifo5R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn dir_fifo5(&self) -> DirFifo5R {
        DirFifo5R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn epno_fifo6(&self) -> EpnoFifo6R {
        EpnoFifo6R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    pub fn dir_fifo6(&self) -> DirFifo6R {
        DirFifo6R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    pub fn epno_fifo7(&self) -> EpnoFifo7R {
        EpnoFifo7R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:29"]
    #[inline(always)]
    pub fn dir_fifo7(&self) -> DirFifo7R {
        DirFifo7R::new(((self.bits >> 28) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn epno_fifo4(&mut self) -> EpnoFifo4W<'_, DevFmap2Spec> {
        EpnoFifo4W::new(self, 0)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn dir_fifo4(&mut self) -> DirFifo4W<'_, DevFmap2Spec> {
        DirFifo4W::new(self, 4)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn epno_fifo5(&mut self) -> EpnoFifo5W<'_, DevFmap2Spec> {
        EpnoFifo5W::new(self, 8)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn dir_fifo5(&mut self) -> DirFifo5W<'_, DevFmap2Spec> {
        DirFifo5W::new(self, 12)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn epno_fifo6(&mut self) -> EpnoFifo6W<'_, DevFmap2Spec> {
        EpnoFifo6W::new(self, 16)
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    pub fn dir_fifo6(&mut self) -> DirFifo6W<'_, DevFmap2Spec> {
        DirFifo6W::new(self, 20)
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    pub fn epno_fifo7(&mut self) -> EpnoFifo7W<'_, DevFmap2Spec> {
        EpnoFifo7W::new(self, 24)
    }
    #[doc = "Bits 28:29"]
    #[inline(always)]
    pub fn dir_fifo7(&mut self) -> DirFifo7W<'_, DevFmap2Spec> {
        DirFifo7W::new(self, 28)
    }
}
#[doc = "Device FIFO Mapping 2 (FIFO 4-7)\n\nYou can [`read`](crate::Reg::read) this register and get [`dev_fmap2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dev_fmap2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DevFmap2Spec;
impl crate::RegisterSpec for DevFmap2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dev_fmap2::R`](R) reader structure"]
impl crate::Readable for DevFmap2Spec {}
#[doc = "`write(|w| ..)` method takes [`dev_fmap2::W`](W) writer structure"]
impl crate::Writable for DevFmap2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DEV_FMAP2 to value 0"]
impl crate::Resettable for DevFmap2Spec {}
