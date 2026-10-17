#[doc = "Register `VDMA_F1PS1` reader"]
pub type R = crate::R<VdmaF1ps1Spec>;
#[doc = "Register `VDMA_F1PS1` writer"]
pub type W = crate::W<VdmaF1ps1Spec>;
#[doc = "Field `VDMA_START_F1` reader - "]
pub type VdmaStartF1R = crate::BitReader;
#[doc = "Field `VDMA_START_F1` writer - "]
pub type VdmaStartF1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VDMA_TYPE_F1` reader - "]
pub type VdmaTypeF1R = crate::BitReader;
#[doc = "Field `VDMA_TYPE_F1` writer - "]
pub type VdmaTypeF1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VDMA_IO_F1` reader - "]
pub type VdmaIoF1R = crate::BitReader;
#[doc = "Field `VDMA_IO_F1` writer - "]
pub type VdmaIoF1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VDMA_LEN_F1` reader - "]
pub type VdmaLenF1R = crate::FieldReader<u32>;
#[doc = "Field `VDMA_LEN_F1` writer - "]
pub type VdmaLenF1W<'a, REG> = crate::FieldWriter<'a, REG, 17, u32>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn vdma_start_f1(&self) -> VdmaStartF1R {
        VdmaStartF1R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn vdma_type_f1(&self) -> VdmaTypeF1R {
        VdmaTypeF1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn vdma_io_f1(&self) -> VdmaIoF1R {
        VdmaIoF1R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 8:24"]
    #[inline(always)]
    pub fn vdma_len_f1(&self) -> VdmaLenF1R {
        VdmaLenF1R::new((self.bits >> 8) & 0x0001_ffff)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn vdma_start_f1(&mut self) -> VdmaStartF1W<'_, VdmaF1ps1Spec> {
        VdmaStartF1W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn vdma_type_f1(&mut self) -> VdmaTypeF1W<'_, VdmaF1ps1Spec> {
        VdmaTypeF1W::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn vdma_io_f1(&mut self) -> VdmaIoF1W<'_, VdmaF1ps1Spec> {
        VdmaIoF1W::new(self, 2)
    }
    #[doc = "Bits 8:24"]
    #[inline(always)]
    pub fn vdma_len_f1(&mut self) -> VdmaLenF1W<'_, VdmaF1ps1Spec> {
        VdmaLenF1W::new(self, 8)
    }
}
#[doc = "VDMA FIFO 1 Parameter Set 1\n\nYou can [`read`](crate::Reg::read) this register and get [`vdma_f1ps1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vdma_f1ps1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VdmaF1ps1Spec;
impl crate::RegisterSpec for VdmaF1ps1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vdma_f1ps1::R`](R) reader structure"]
impl crate::Readable for VdmaF1ps1Spec {}
#[doc = "`write(|w| ..)` method takes [`vdma_f1ps1::W`](W) writer structure"]
impl crate::Writable for VdmaF1ps1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets VDMA_F1PS1 to value 0"]
impl crate::Resettable for VdmaF1ps1Spec {}
