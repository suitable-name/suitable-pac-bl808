#[doc = "Register `VDMA_FnPS2` reader"]
pub type R = crate::R<VdmaFnPs2Spec>;
#[doc = "Register `VDMA_FnPS2` writer"]
pub type W = crate::W<VdmaFnPs2Spec>;
#[doc = "Field `VDMA_MADDR_FN` reader - "]
pub type VdmaMaddrFnR = crate::FieldReader<u32>;
#[doc = "Field `VDMA_MADDR_FN` writer - "]
pub type VdmaMaddrFnW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn vdma_maddr_fn(&self) -> VdmaMaddrFnR {
        VdmaMaddrFnR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn vdma_maddr_fn(&mut self) -> VdmaMaddrFnW<'_, VdmaFnPs2Spec> {
        VdmaMaddrFnW::new(self, 0)
    }
}
#[doc = "VDMA FIFO n Parameter Set 2 (Memory Address) (General structure for F4-F15)\n\nYou can [`read`](crate::Reg::read) this register and get [`vdma_fn_ps2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vdma_fn_ps2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VdmaFnPs2Spec;
impl crate::RegisterSpec for VdmaFnPs2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vdma_fn_ps2::R`](R) reader structure"]
impl crate::Readable for VdmaFnPs2Spec {}
#[doc = "`write(|w| ..)` method takes [`vdma_fn_ps2::W`](W) writer structure"]
impl crate::Writable for VdmaFnPs2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets VDMA_FnPS2 to value 0"]
impl crate::Resettable for VdmaFnPs2Spec {}
