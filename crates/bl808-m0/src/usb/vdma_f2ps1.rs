#[doc = "Register `VDMA_F2PS1` reader"]
pub type R = crate::R<VdmaF2ps1Spec>;
#[doc = "Register `VDMA_F2PS1` writer"]
pub type W = crate::W<VdmaF2ps1Spec>;
#[doc = "Field `VDMA_START_F2` reader - "]
pub type VdmaStartF2R = crate::BitReader;
#[doc = "Field `VDMA_START_F2` writer - "]
pub type VdmaStartF2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VDMA_TYPE_F2` reader - "]
pub type VdmaTypeF2R = crate::BitReader;
#[doc = "Field `VDMA_TYPE_F2` writer - "]
pub type VdmaTypeF2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VDMA_IO_F2` reader - "]
pub type VdmaIoF2R = crate::BitReader;
#[doc = "Field `VDMA_IO_F2` writer - "]
pub type VdmaIoF2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VDMA_LEN_F2` reader - "]
pub type VdmaLenF2R = crate::FieldReader<u32>;
#[doc = "Field `VDMA_LEN_F2` writer - "]
pub type VdmaLenF2W<'a, REG> = crate::FieldWriter<'a, REG, 17, u32>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn vdma_start_f2(&self) -> VdmaStartF2R {
        VdmaStartF2R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn vdma_type_f2(&self) -> VdmaTypeF2R {
        VdmaTypeF2R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn vdma_io_f2(&self) -> VdmaIoF2R {
        VdmaIoF2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 8:24"]
    #[inline(always)]
    pub fn vdma_len_f2(&self) -> VdmaLenF2R {
        VdmaLenF2R::new((self.bits >> 8) & 0x0001_ffff)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn vdma_start_f2(&mut self) -> VdmaStartF2W<'_, VdmaF2ps1Spec> {
        VdmaStartF2W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn vdma_type_f2(&mut self) -> VdmaTypeF2W<'_, VdmaF2ps1Spec> {
        VdmaTypeF2W::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn vdma_io_f2(&mut self) -> VdmaIoF2W<'_, VdmaF2ps1Spec> {
        VdmaIoF2W::new(self, 2)
    }
    #[doc = "Bits 8:24"]
    #[inline(always)]
    pub fn vdma_len_f2(&mut self) -> VdmaLenF2W<'_, VdmaF2ps1Spec> {
        VdmaLenF2W::new(self, 8)
    }
}
#[doc = "VDMA FIFO 2 Parameter Set 1\n\nYou can [`read`](crate::Reg::read) this register and get [`vdma_f2ps1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vdma_f2ps1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VdmaF2ps1Spec;
impl crate::RegisterSpec for VdmaF2ps1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vdma_f2ps1::R`](R) reader structure"]
impl crate::Readable for VdmaF2ps1Spec {}
#[doc = "`write(|w| ..)` method takes [`vdma_f2ps1::W`](W) writer structure"]
impl crate::Writable for VdmaF2ps1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets VDMA_F2PS1 to value 0"]
impl crate::Resettable for VdmaF2ps1Spec {}
