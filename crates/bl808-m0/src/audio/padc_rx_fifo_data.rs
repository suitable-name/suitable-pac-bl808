#[doc = "Register `padc_rx_fifo_data` reader"]
pub type R = crate::R<PadcRxFifoDataSpec>;
#[doc = "Field `padc_rx_data` reader - "]
pub type PadcRxDataR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn padc_rx_data(&self) -> PadcRxDataR {
        PadcRxDataR::new(self.bits)
    }
}
#[doc = "Passthrough ADC RX FIFO Data Port: Read portal for conversion streams\n\nYou can [`read`](crate::Reg::read) this register and get [`padc_rx_fifo_data::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PadcRxFifoDataSpec;
impl crate::RegisterSpec for PadcRxFifoDataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`padc_rx_fifo_data::R`](R) reader structure"]
impl crate::Readable for PadcRxFifoDataSpec {}
#[doc = "`reset()` method sets padc_rx_fifo_data to value 0"]
impl crate::Resettable for PadcRxFifoDataSpec {}
