#[doc = "Register `VDMA_FnPS1` reader"]
pub type R = crate::R<VdmaFnPs1Spec>;
#[doc = "Register `VDMA_FnPS1` writer"]
pub type W = crate::W<VdmaFnPs1Spec>;
#[doc = "Field `VDMA_START_FN` reader - "]
pub type VdmaStartFnR = crate::BitReader;
#[doc = "Field `VDMA_START_FN` writer - "]
pub type VdmaStartFnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VDMA_TYPE_FN` reader - "]
pub type VdmaTypeFnR = crate::BitReader;
#[doc = "Field `VDMA_TYPE_FN` writer - "]
pub type VdmaTypeFnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VDMA_IO_FN` reader - "]
pub type VdmaIoFnR = crate::BitReader;
#[doc = "Field `VDMA_IO_FN` writer - "]
pub type VdmaIoFnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VDMA_LEN_FN` reader - "]
pub type VdmaLenFnR = crate::FieldReader<u32>;
#[doc = "Field `VDMA_LEN_FN` writer - "]
pub type VdmaLenFnW<'a, REG> = crate::FieldWriter<'a, REG, 17, u32>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn vdma_start_fn(&self) -> VdmaStartFnR {
        VdmaStartFnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn vdma_type_fn(&self) -> VdmaTypeFnR {
        VdmaTypeFnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn vdma_io_fn(&self) -> VdmaIoFnR {
        VdmaIoFnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 8:24"]
    #[inline(always)]
    pub fn vdma_len_fn(&self) -> VdmaLenFnR {
        VdmaLenFnR::new((self.bits >> 8) & 0x0001_ffff)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn vdma_start_fn(&mut self) -> VdmaStartFnW<'_, VdmaFnPs1Spec> {
        VdmaStartFnW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn vdma_type_fn(&mut self) -> VdmaTypeFnW<'_, VdmaFnPs1Spec> {
        VdmaTypeFnW::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn vdma_io_fn(&mut self) -> VdmaIoFnW<'_, VdmaFnPs1Spec> {
        VdmaIoFnW::new(self, 2)
    }
    #[doc = "Bits 8:24"]
    #[inline(always)]
    pub fn vdma_len_fn(&mut self) -> VdmaLenFnW<'_, VdmaFnPs1Spec> {
        VdmaLenFnW::new(self, 8)
    }
}
#[doc = "VDMA FIFO n Parameter Set 1 (General structure for F4-F15)\n\nYou can [`read`](crate::Reg::read) this register and get [`vdma_fn_ps1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vdma_fn_ps1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VdmaFnPs1Spec;
impl crate::RegisterSpec for VdmaFnPs1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vdma_fn_ps1::R`](R) reader structure"]
impl crate::Readable for VdmaFnPs1Spec {}
#[doc = "`write(|w| ..)` method takes [`vdma_fn_ps1::W`](W) writer structure"]
impl crate::Writable for VdmaFnPs1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets VDMA_FnPS1 to value 0"]
impl crate::Resettable for VdmaFnPs1Spec {}
