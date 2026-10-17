#[doc = "Register `VDMA_F3PS1` reader"]
pub type R = crate::R<VdmaF3ps1Spec>;
#[doc = "Register `VDMA_F3PS1` writer"]
pub type W = crate::W<VdmaF3ps1Spec>;
#[doc = "Field `VDMA_START_F3` reader - "]
pub type VdmaStartF3R = crate::BitReader;
#[doc = "Field `VDMA_START_F3` writer - "]
pub type VdmaStartF3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VDMA_TYPE_F3` reader - "]
pub type VdmaTypeF3R = crate::BitReader;
#[doc = "Field `VDMA_TYPE_F3` writer - "]
pub type VdmaTypeF3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VDMA_IO_F3` reader - "]
pub type VdmaIoF3R = crate::BitReader;
#[doc = "Field `VDMA_IO_F3` writer - "]
pub type VdmaIoF3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VDMA_LEN_F3` reader - "]
pub type VdmaLenF3R = crate::FieldReader<u32>;
#[doc = "Field `VDMA_LEN_F3` writer - "]
pub type VdmaLenF3W<'a, REG> = crate::FieldWriter<'a, REG, 17, u32>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn vdma_start_f3(&self) -> VdmaStartF3R {
        VdmaStartF3R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn vdma_type_f3(&self) -> VdmaTypeF3R {
        VdmaTypeF3R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn vdma_io_f3(&self) -> VdmaIoF3R {
        VdmaIoF3R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 8:24"]
    #[inline(always)]
    pub fn vdma_len_f3(&self) -> VdmaLenF3R {
        VdmaLenF3R::new((self.bits >> 8) & 0x0001_ffff)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn vdma_start_f3(&mut self) -> VdmaStartF3W<'_, VdmaF3ps1Spec> {
        VdmaStartF3W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn vdma_type_f3(&mut self) -> VdmaTypeF3W<'_, VdmaF3ps1Spec> {
        VdmaTypeF3W::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn vdma_io_f3(&mut self) -> VdmaIoF3W<'_, VdmaF3ps1Spec> {
        VdmaIoF3W::new(self, 2)
    }
    #[doc = "Bits 8:24"]
    #[inline(always)]
    pub fn vdma_len_f3(&mut self) -> VdmaLenF3W<'_, VdmaF3ps1Spec> {
        VdmaLenF3W::new(self, 8)
    }
}
#[doc = "VDMA FIFO 3 Parameter Set 1\n\nYou can [`read`](crate::Reg::read) this register and get [`vdma_f3ps1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vdma_f3ps1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VdmaF3ps1Spec;
impl crate::RegisterSpec for VdmaF3ps1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vdma_f3ps1::R`](R) reader structure"]
impl crate::Readable for VdmaF3ps1Spec {}
#[doc = "`write(|w| ..)` method takes [`vdma_f3ps1::W`](W) writer structure"]
impl crate::Writable for VdmaF3ps1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets VDMA_F3PS1 to value 0"]
impl crate::Resettable for VdmaF3ps1Spec {}
