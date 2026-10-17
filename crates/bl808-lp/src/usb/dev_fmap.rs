#[doc = "Register `DEV_FMAP` reader"]
pub type R = crate::R<DevFmapSpec>;
#[doc = "Register `DEV_FMAP` writer"]
pub type W = crate::W<DevFmapSpec>;
#[doc = "Field `EPNO_FIFO0` reader - Endpoint Number for FIFO 0"]
pub type EpnoFifo0R = crate::FieldReader;
#[doc = "Field `EPNO_FIFO0` writer - Endpoint Number for FIFO 0"]
pub type EpnoFifo0W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DIR_FIFO0` reader - Direction for FIFO 0 (0:OUT, 1:IN, 2:CX)"]
pub type DirFifo0R = crate::FieldReader;
#[doc = "Field `DIR_FIFO0` writer - Direction for FIFO 0 (0:OUT, 1:IN, 2:CX)"]
pub type DirFifo0W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `EPNO_FIFO1` reader - "]
pub type EpnoFifo1R = crate::FieldReader;
#[doc = "Field `EPNO_FIFO1` writer - "]
pub type EpnoFifo1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DIR_FIFO1` reader - "]
pub type DirFifo1R = crate::FieldReader;
#[doc = "Field `DIR_FIFO1` writer - "]
pub type DirFifo1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `EPNO_FIFO2` reader - "]
pub type EpnoFifo2R = crate::FieldReader;
#[doc = "Field `EPNO_FIFO2` writer - "]
pub type EpnoFifo2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DIR_FIFO2` reader - "]
pub type DirFifo2R = crate::FieldReader;
#[doc = "Field `DIR_FIFO2` writer - "]
pub type DirFifo2W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `EPNO_FIFO3` reader - "]
pub type EpnoFifo3R = crate::FieldReader;
#[doc = "Field `EPNO_FIFO3` writer - "]
pub type EpnoFifo3W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DIR_FIFO3` reader - "]
pub type DirFifo3R = crate::FieldReader;
#[doc = "Field `DIR_FIFO3` writer - "]
pub type DirFifo3W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:3 - Endpoint Number for FIFO 0"]
    #[inline(always)]
    pub fn epno_fifo0(&self) -> EpnoFifo0R {
        EpnoFifo0R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:5 - Direction for FIFO 0 (0:OUT, 1:IN, 2:CX)"]
    #[inline(always)]
    pub fn dir_fifo0(&self) -> DirFifo0R {
        DirFifo0R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn epno_fifo1(&self) -> EpnoFifo1R {
        EpnoFifo1R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn dir_fifo1(&self) -> DirFifo1R {
        DirFifo1R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn epno_fifo2(&self) -> EpnoFifo2R {
        EpnoFifo2R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    pub fn dir_fifo2(&self) -> DirFifo2R {
        DirFifo2R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    pub fn epno_fifo3(&self) -> EpnoFifo3R {
        EpnoFifo3R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:29"]
    #[inline(always)]
    pub fn dir_fifo3(&self) -> DirFifo3R {
        DirFifo3R::new(((self.bits >> 28) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Endpoint Number for FIFO 0"]
    #[inline(always)]
    pub fn epno_fifo0(&mut self) -> EpnoFifo0W<'_, DevFmapSpec> {
        EpnoFifo0W::new(self, 0)
    }
    #[doc = "Bits 4:5 - Direction for FIFO 0 (0:OUT, 1:IN, 2:CX)"]
    #[inline(always)]
    pub fn dir_fifo0(&mut self) -> DirFifo0W<'_, DevFmapSpec> {
        DirFifo0W::new(self, 4)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn epno_fifo1(&mut self) -> EpnoFifo1W<'_, DevFmapSpec> {
        EpnoFifo1W::new(self, 8)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn dir_fifo1(&mut self) -> DirFifo1W<'_, DevFmapSpec> {
        DirFifo1W::new(self, 12)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn epno_fifo2(&mut self) -> EpnoFifo2W<'_, DevFmapSpec> {
        EpnoFifo2W::new(self, 16)
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    pub fn dir_fifo2(&mut self) -> DirFifo2W<'_, DevFmapSpec> {
        DirFifo2W::new(self, 20)
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    pub fn epno_fifo3(&mut self) -> EpnoFifo3W<'_, DevFmapSpec> {
        EpnoFifo3W::new(self, 24)
    }
    #[doc = "Bits 28:29"]
    #[inline(always)]
    pub fn dir_fifo3(&mut self) -> DirFifo3W<'_, DevFmapSpec> {
        DirFifo3W::new(self, 28)
    }
}
#[doc = "Device FIFO Mapping (FIFO 0-3)\n\nYou can [`read`](crate::Reg::read) this register and get [`dev_fmap::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dev_fmap::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DevFmapSpec;
impl crate::RegisterSpec for DevFmapSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dev_fmap::R`](R) reader structure"]
impl crate::Readable for DevFmapSpec {}
#[doc = "`write(|w| ..)` method takes [`dev_fmap::W`](W) writer structure"]
impl crate::Writable for DevFmapSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DEV_FMAP to value 0"]
impl crate::Resettable for DevFmapSpec {}
