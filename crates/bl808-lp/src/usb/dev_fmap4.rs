#[doc = "Register `DEV_FMAP4` reader"]
pub type R = crate::R<DevFmap4Spec>;
#[doc = "Register `DEV_FMAP4` writer"]
pub type W = crate::W<DevFmap4Spec>;
#[doc = "Field `EPNO_FIFO12` reader - "]
pub type EpnoFifo12R = crate::FieldReader;
#[doc = "Field `EPNO_FIFO12` writer - "]
pub type EpnoFifo12W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DIR_FIFO12` reader - "]
pub type DirFifo12R = crate::FieldReader;
#[doc = "Field `DIR_FIFO12` writer - "]
pub type DirFifo12W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `EPNO_FIFO13` reader - "]
pub type EpnoFifo13R = crate::FieldReader;
#[doc = "Field `EPNO_FIFO13` writer - "]
pub type EpnoFifo13W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DIR_FIFO13` reader - "]
pub type DirFifo13R = crate::FieldReader;
#[doc = "Field `DIR_FIFO13` writer - "]
pub type DirFifo13W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `EPNO_FIFO14` reader - "]
pub type EpnoFifo14R = crate::FieldReader;
#[doc = "Field `EPNO_FIFO14` writer - "]
pub type EpnoFifo14W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DIR_FIFO14` reader - "]
pub type DirFifo14R = crate::FieldReader;
#[doc = "Field `DIR_FIFO14` writer - "]
pub type DirFifo14W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `EPNO_FIFO15` reader - "]
pub type EpnoFifo15R = crate::FieldReader;
#[doc = "Field `EPNO_FIFO15` writer - "]
pub type EpnoFifo15W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DIR_FIFO15` reader - "]
pub type DirFifo15R = crate::FieldReader;
#[doc = "Field `DIR_FIFO15` writer - "]
pub type DirFifo15W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn epno_fifo12(&self) -> EpnoFifo12R {
        EpnoFifo12R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn dir_fifo12(&self) -> DirFifo12R {
        DirFifo12R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn epno_fifo13(&self) -> EpnoFifo13R {
        EpnoFifo13R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn dir_fifo13(&self) -> DirFifo13R {
        DirFifo13R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn epno_fifo14(&self) -> EpnoFifo14R {
        EpnoFifo14R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    pub fn dir_fifo14(&self) -> DirFifo14R {
        DirFifo14R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    pub fn epno_fifo15(&self) -> EpnoFifo15R {
        EpnoFifo15R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:29"]
    #[inline(always)]
    pub fn dir_fifo15(&self) -> DirFifo15R {
        DirFifo15R::new(((self.bits >> 28) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn epno_fifo12(&mut self) -> EpnoFifo12W<'_, DevFmap4Spec> {
        EpnoFifo12W::new(self, 0)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn dir_fifo12(&mut self) -> DirFifo12W<'_, DevFmap4Spec> {
        DirFifo12W::new(self, 4)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn epno_fifo13(&mut self) -> EpnoFifo13W<'_, DevFmap4Spec> {
        EpnoFifo13W::new(self, 8)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn dir_fifo13(&mut self) -> DirFifo13W<'_, DevFmap4Spec> {
        DirFifo13W::new(self, 12)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn epno_fifo14(&mut self) -> EpnoFifo14W<'_, DevFmap4Spec> {
        EpnoFifo14W::new(self, 16)
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    pub fn dir_fifo14(&mut self) -> DirFifo14W<'_, DevFmap4Spec> {
        DirFifo14W::new(self, 20)
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    pub fn epno_fifo15(&mut self) -> EpnoFifo15W<'_, DevFmap4Spec> {
        EpnoFifo15W::new(self, 24)
    }
    #[doc = "Bits 28:29"]
    #[inline(always)]
    pub fn dir_fifo15(&mut self) -> DirFifo15W<'_, DevFmap4Spec> {
        DirFifo15W::new(self, 28)
    }
}
#[doc = "Device FIFO Mapping 4 (FIFO 12-15)\n\nYou can [`read`](crate::Reg::read) this register and get [`dev_fmap4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dev_fmap4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DevFmap4Spec;
impl crate::RegisterSpec for DevFmap4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dev_fmap4::R`](R) reader structure"]
impl crate::Readable for DevFmap4Spec {}
#[doc = "`write(|w| ..)` method takes [`dev_fmap4::W`](W) writer structure"]
impl crate::Writable for DevFmap4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DEV_FMAP4 to value 0"]
impl crate::Resettable for DevFmap4Spec {}
