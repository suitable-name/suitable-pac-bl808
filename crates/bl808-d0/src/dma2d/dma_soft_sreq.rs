#[doc = "Register `DMA_SoftSReq` reader"]
pub type R = crate::R<DmaSoftSreqSpec>;
#[doc = "Register `DMA_SoftSReq` writer"]
pub type W = crate::W<DmaSoftSreqSpec>;
#[doc = "Field `SoftSReq` reader - "]
pub type SoftSreqR = crate::FieldReader<u32>;
#[doc = "Field `SoftSReq` writer - "]
pub type SoftSreqW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn soft_sreq(&self) -> SoftSreqR {
        SoftSreqR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn soft_sreq(&mut self) -> SoftSreqW<'_, DmaSoftSreqSpec> {
        SoftSreqW::new(self, 0)
    }
}
#[doc = "DMA Software Single Request\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_soft_sreq::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_soft_sreq::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmaSoftSreqSpec;
impl crate::RegisterSpec for DmaSoftSreqSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_soft_sreq::R`](R) reader structure"]
impl crate::Readable for DmaSoftSreqSpec {}
#[doc = "`write(|w| ..)` method takes [`dma_soft_sreq::W`](W) writer structure"]
impl crate::Writable for DmaSoftSreqSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMA_SoftSReq to value 0"]
impl crate::Resettable for DmaSoftSreqSpec {}
