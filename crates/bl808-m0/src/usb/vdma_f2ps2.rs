#[doc = "Register `VDMA_F2PS2` reader"]
pub type R = crate::R<VdmaF2ps2Spec>;
#[doc = "Register `VDMA_F2PS2` writer"]
pub type W = crate::W<VdmaF2ps2Spec>;
#[doc = "Field `VDMA_MADDR_F2` reader - "]
pub type VdmaMaddrF2R = crate::FieldReader<u32>;
#[doc = "Field `VDMA_MADDR_F2` writer - "]
pub type VdmaMaddrF2W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn vdma_maddr_f2(&self) -> VdmaMaddrF2R {
        VdmaMaddrF2R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn vdma_maddr_f2(&mut self) -> VdmaMaddrF2W<'_, VdmaF2ps2Spec> {
        VdmaMaddrF2W::new(self, 0)
    }
}
#[doc = "VDMA FIFO 2 Parameter Set 2 (Memory Address)\n\nYou can [`read`](crate::Reg::read) this register and get [`vdma_f2ps2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vdma_f2ps2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VdmaF2ps2Spec;
impl crate::RegisterSpec for VdmaF2ps2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vdma_f2ps2::R`](R) reader structure"]
impl crate::Readable for VdmaF2ps2Spec {}
#[doc = "`write(|w| ..)` method takes [`vdma_f2ps2::W`](W) writer structure"]
impl crate::Writable for VdmaF2ps2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets VDMA_F2PS2 to value 0"]
impl crate::Resettable for VdmaF2ps2Spec {}
