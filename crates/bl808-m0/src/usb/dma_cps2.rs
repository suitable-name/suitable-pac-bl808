#[doc = "Register `DMA_CPS2` reader"]
pub type R = crate::R<DmaCps2Spec>;
#[doc = "Register `DMA_CPS2` writer"]
pub type W = crate::W<DmaCps2Spec>;
#[doc = "Field `DMA_MADDR_HOV` reader - DMA Memory Address (Host Overwrite)"]
pub type DmaMaddrHovR = crate::FieldReader<u32>;
#[doc = "Field `DMA_MADDR_HOV` writer - DMA Memory Address (Host Overwrite)"]
pub type DmaMaddrHovW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - DMA Memory Address (Host Overwrite)"]
    #[inline(always)]
    pub fn dma_maddr_hov(&self) -> DmaMaddrHovR {
        DmaMaddrHovR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - DMA Memory Address (Host Overwrite)"]
    #[inline(always)]
    pub fn dma_maddr_hov(&mut self) -> DmaMaddrHovW<'_, DmaCps2Spec> {
        DmaMaddrHovW::new(self, 0)
    }
}
#[doc = "DMA Control Packet Set 2 (Memory Address)\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_cps2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_cps2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmaCps2Spec;
impl crate::RegisterSpec for DmaCps2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_cps2::R`](R) reader structure"]
impl crate::Readable for DmaCps2Spec {}
#[doc = "`write(|w| ..)` method takes [`dma_cps2::W`](W) writer structure"]
impl crate::Writable for DmaCps2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMA_CPS2 to value 0"]
impl crate::Resettable for DmaCps2Spec {}
