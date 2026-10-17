#[doc = "Register `DMA_IntStatus` reader"]
pub type R = crate::R<DmaIntStatusSpec>;
#[doc = "Field `IntStatus` reader - Interrupt status for channels 0-7. Bit n corresponds to channel n."]
pub type IntStatusR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Interrupt status for channels 0-7. Bit n corresponds to channel n."]
    #[inline(always)]
    pub fn int_status(&self) -> IntStatusR {
        IntStatusR::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Status of the DMA interrupts after masking\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_int_status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmaIntStatusSpec;
impl crate::RegisterSpec for DmaIntStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_int_status::R`](R) reader structure"]
impl crate::Readable for DmaIntStatusSpec {}
#[doc = "`reset()` method sets DMA_IntStatus to value 0"]
impl crate::Resettable for DmaIntStatusSpec {}
