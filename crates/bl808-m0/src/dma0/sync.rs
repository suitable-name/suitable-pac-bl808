#[doc = "Register `Sync` reader"]
pub type R = crate::R<SyncSpec>;
#[doc = "Register `Sync` writer"]
pub type W = crate::W<SyncSpec>;
#[doc = "Field `DMA_Sync` reader - "]
pub type DmaSyncR = crate::FieldReader<u32>;
#[doc = "Field `DMA_Sync` writer - "]
pub type DmaSyncW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn dma_sync(&self) -> DmaSyncR {
        DmaSyncR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn dma_sync(&mut self) -> DmaSyncW<'_, SyncSpec> {
        DmaSyncW::new(self, 0)
    }
}
#[doc = "DMA Synchronization Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sync::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sync::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SyncSpec;
impl crate::RegisterSpec for SyncSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sync::R`](R) reader structure"]
impl crate::Readable for SyncSpec {}
#[doc = "`write(|w| ..)` method takes [`sync::W`](W) writer structure"]
impl crate::Writable for SyncSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets Sync to value 0"]
impl crate::Resettable for SyncSpec {}
