#[doc = "Register `DMA_SoftBReq` reader"]
pub type R = crate::R<DmaSoftBreqSpec>;
#[doc = "Register `DMA_SoftBReq` writer"]
pub type W = crate::W<DmaSoftBreqSpec>;
#[doc = "Field `SoftBReq` reader - "]
pub type SoftBreqR = crate::FieldReader<u32>;
#[doc = "Field `SoftBReq` writer - "]
pub type SoftBreqW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn soft_breq(&self) -> SoftBreqR {
        SoftBreqR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn soft_breq(&mut self) -> SoftBreqW<'_, DmaSoftBreqSpec> {
        SoftBreqW::new(self, 0)
    }
}
#[doc = "DMA Software Burst Request\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_soft_breq::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_soft_breq::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmaSoftBreqSpec;
impl crate::RegisterSpec for DmaSoftBreqSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_soft_breq::R`](R) reader structure"]
impl crate::Readable for DmaSoftBreqSpec {}
#[doc = "`write(|w| ..)` method takes [`dma_soft_breq::W`](W) writer structure"]
impl crate::Writable for DmaSoftBreqSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMA_SoftBReq to value 0"]
impl crate::Resettable for DmaSoftBreqSpec {}
