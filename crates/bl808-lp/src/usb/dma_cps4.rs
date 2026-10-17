#[doc = "Register `DMA_CPS4` reader"]
pub type R = crate::R<DmaCps4Spec>;
#[doc = "DMA Control Packet Set 4 (Reserved)\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_cps4::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmaCps4Spec;
impl crate::RegisterSpec for DmaCps4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_cps4::R`](R) reader structure"]
impl crate::Readable for DmaCps4Spec {}
#[doc = "`reset()` method sets DMA_CPS4 to value 0"]
impl crate::Resettable for DmaCps4Spec {}
