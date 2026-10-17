#[doc = "Register `GPADC_DMA_RDATA` reader"]
pub type R = crate::R<GpadcDmaRdataSpec>;
#[doc = "Field `gpadc_dma_rdata` reader - FIFO result (26 bits: Channel ID + Result)"]
pub type GpadcDmaRdataR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:25 - FIFO result (26 bits: Channel ID + Result)"]
    #[inline(always)]
    pub fn gpadc_dma_rdata(&self) -> GpadcDmaRdataR {
        GpadcDmaRdataR::new(self.bits & 0x03ff_ffff)
    }
}
#[doc = "GPADC DMA Read Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpadc_dma_rdata::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpadcDmaRdataSpec;
impl crate::RegisterSpec for GpadcDmaRdataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpadc_dma_rdata::R`](R) reader structure"]
impl crate::Readable for GpadcDmaRdataSpec {}
#[doc = "`reset()` method sets GPADC_DMA_RDATA to value 0"]
impl crate::Resettable for GpadcDmaRdataSpec {}
