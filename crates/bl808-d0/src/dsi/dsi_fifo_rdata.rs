#[doc = "Register `DSI_FIFO_RDATA` reader"]
pub type R = crate::R<DsiFifoRdataSpec>;
#[doc = "Field `dsi_fifo_rdata` reader - "]
pub type DsiFifoRdataR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn dsi_fifo_rdata(&self) -> DsiFifoRdataR {
        DsiFifoRdataR::new(self.bits)
    }
}
#[doc = "FIFO Read Data\n\nYou can [`read`](crate::Reg::read) this register and get [`dsi_fifo_rdata::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DsiFifoRdataSpec;
impl crate::RegisterSpec for DsiFifoRdataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsi_fifo_rdata::R`](R) reader structure"]
impl crate::Readable for DsiFifoRdataSpec {}
#[doc = "`reset()` method sets DSI_FIFO_RDATA to value 0"]
impl crate::Resettable for DsiFifoRdataSpec {}
