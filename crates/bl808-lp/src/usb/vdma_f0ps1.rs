#[doc = "Register `VDMA_F0PS1` reader"]
pub type R = crate::R<VdmaF0ps1Spec>;
#[doc = "Register `VDMA_F0PS1` writer"]
pub type W = crate::W<VdmaF0ps1Spec>;
#[doc = "Field `VDMA_START_F0` reader - "]
pub type VdmaStartF0R = crate::BitReader;
#[doc = "Field `VDMA_START_F0` writer - "]
pub type VdmaStartF0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VDMA_TYPE_F0` reader - "]
pub type VdmaTypeF0R = crate::BitReader;
#[doc = "Field `VDMA_TYPE_F0` writer - "]
pub type VdmaTypeF0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VDMA_IO_F0` reader - "]
pub type VdmaIoF0R = crate::BitReader;
#[doc = "Field `VDMA_IO_F0` writer - "]
pub type VdmaIoF0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VDMA_LEN_F0` reader - "]
pub type VdmaLenF0R = crate::FieldReader<u32>;
#[doc = "Field `VDMA_LEN_F0` writer - "]
pub type VdmaLenF0W<'a, REG> = crate::FieldWriter<'a, REG, 17, u32>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn vdma_start_f0(&self) -> VdmaStartF0R {
        VdmaStartF0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn vdma_type_f0(&self) -> VdmaTypeF0R {
        VdmaTypeF0R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn vdma_io_f0(&self) -> VdmaIoF0R {
        VdmaIoF0R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 8:24"]
    #[inline(always)]
    pub fn vdma_len_f0(&self) -> VdmaLenF0R {
        VdmaLenF0R::new((self.bits >> 8) & 0x0001_ffff)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn vdma_start_f0(&mut self) -> VdmaStartF0W<'_, VdmaF0ps1Spec> {
        VdmaStartF0W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn vdma_type_f0(&mut self) -> VdmaTypeF0W<'_, VdmaF0ps1Spec> {
        VdmaTypeF0W::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn vdma_io_f0(&mut self) -> VdmaIoF0W<'_, VdmaF0ps1Spec> {
        VdmaIoF0W::new(self, 2)
    }
    #[doc = "Bits 8:24"]
    #[inline(always)]
    pub fn vdma_len_f0(&mut self) -> VdmaLenF0W<'_, VdmaF0ps1Spec> {
        VdmaLenF0W::new(self, 8)
    }
}
#[doc = "VDMA FIFO 0 Parameter Set 1\n\nYou can [`read`](crate::Reg::read) this register and get [`vdma_f0ps1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vdma_f0ps1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VdmaF0ps1Spec;
impl crate::RegisterSpec for VdmaF0ps1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vdma_f0ps1::R`](R) reader structure"]
impl crate::Readable for VdmaF0ps1Spec {}
#[doc = "`write(|w| ..)` method takes [`vdma_f0ps1::W`](W) writer structure"]
impl crate::Writable for VdmaF0ps1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets VDMA_F0PS1 to value 0"]
impl crate::Resettable for VdmaF0ps1Spec {}
