#[doc = "Register `DMA_Sync` reader"]
pub type R = crate::R<DmaSyncSpec>;
#[doc = "Register `DMA_Sync` writer"]
pub type W = crate::W<DmaSyncSpec>;
#[doc = "Field `DMA_Sync` reader - Synchronization for DMA request signals"]
pub type DmaSyncR = crate::FieldReader<u32>;
#[doc = "Field `DMA_Sync` writer - Synchronization for DMA request signals"]
pub type DmaSyncW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Synchronization for DMA request signals"]
    #[inline(always)]
    pub fn dma_sync(&self) -> DmaSyncR {
        DmaSyncR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Synchronization for DMA request signals"]
    #[inline(always)]
    pub fn dma_sync(&mut self) -> DmaSyncW<'_, DmaSyncSpec> {
        DmaSyncW::new(self, 0)
    }
}
#[doc = "DMA Synchronization Logic\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_sync::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_sync::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmaSyncSpec;
impl crate::RegisterSpec for DmaSyncSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_sync::R`](R) reader structure"]
impl crate::Readable for DmaSyncSpec {}
#[doc = "`write(|w| ..)` method takes [`dma_sync::W`](W) writer structure"]
impl crate::Writable for DmaSyncSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMA_Sync to value 0"]
impl crate::Resettable for DmaSyncSpec {}
