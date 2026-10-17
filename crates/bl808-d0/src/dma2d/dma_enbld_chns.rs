#[doc = "Register `DMA_EnbldChns` reader"]
pub type R = crate::R<DmaEnbldChnsSpec>;
#[doc = "Field `EnabledChannels` reader - Enable status for channels 0-7."]
pub type EnabledChannelsR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Enable status for channels 0-7."]
    #[inline(always)]
    pub fn enabled_channels(&self) -> EnabledChannelsR {
        EnabledChannelsR::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Channel enable status\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_enbld_chns::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmaEnbldChnsSpec;
impl crate::RegisterSpec for DmaEnbldChnsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_enbld_chns::R`](R) reader structure"]
impl crate::Readable for DmaEnbldChnsSpec {}
#[doc = "`reset()` method sets DMA_EnbldChns to value 0"]
impl crate::Resettable for DmaEnbldChnsSpec {}
