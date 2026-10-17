#[doc = "Register `VDMA_CXFPS2` reader"]
pub type R = crate::R<VdmaCxfps2Spec>;
#[doc = "Register `VDMA_CXFPS2` writer"]
pub type W = crate::W<VdmaCxfps2Spec>;
#[doc = "Field `VDMA_MADDR_CXF` reader - "]
pub type VdmaMaddrCxfR = crate::FieldReader<u32>;
#[doc = "Field `VDMA_MADDR_CXF` writer - "]
pub type VdmaMaddrCxfW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn vdma_maddr_cxf(&self) -> VdmaMaddrCxfR {
        VdmaMaddrCxfR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn vdma_maddr_cxf(&mut self) -> VdmaMaddrCxfW<'_, VdmaCxfps2Spec> {
        VdmaMaddrCxfW::new(self, 0)
    }
}
#[doc = "VDMA Control Endpoint FIFO Parameter Set 2 (Memory Address)\n\nYou can [`read`](crate::Reg::read) this register and get [`vdma_cxfps2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vdma_cxfps2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VdmaCxfps2Spec;
impl crate::RegisterSpec for VdmaCxfps2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vdma_cxfps2::R`](R) reader structure"]
impl crate::Readable for VdmaCxfps2Spec {}
#[doc = "`write(|w| ..)` method takes [`vdma_cxfps2::W`](W) writer structure"]
impl crate::Writable for VdmaCxfps2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets VDMA_CXFPS2 to value 0"]
impl crate::Resettable for VdmaCxfps2Spec {}
