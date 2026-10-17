#[doc = "Register `rx_fifo_data` reader"]
pub type R = crate::R<RxFifoDataSpec>;
#[doc = "Field `rx_data` reader - "]
pub type RxDataR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn rx_data(&self) -> RxDataR {
        RxDataR::new(self.bits)
    }
}
#[doc = "RX FIFO Data: Read portal for incoming audio streams\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_fifo_data::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxFifoDataSpec;
impl crate::RegisterSpec for RxFifoDataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rx_fifo_data::R`](R) reader structure"]
impl crate::Readable for RxFifoDataSpec {}
#[doc = "`reset()` method sets rx_fifo_data to value 0"]
impl crate::Resettable for RxFifoDataSpec {}
