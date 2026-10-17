#[doc = "Register `UART_FIFO_RDATA` reader"]
pub type R = crate::R<UartFifoRdataSpec>;
#[doc = "Field `uart_fifo_rdata` reader - "]
pub type UartFifoRdataR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn uart_fifo_rdata(&self) -> UartFifoRdataR {
        UartFifoRdataR::new((self.bits & 0xff) as u8)
    }
}
#[doc = "UART RX FIFO Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`uart_fifo_rdata::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UartFifoRdataSpec;
impl crate::RegisterSpec for UartFifoRdataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uart_fifo_rdata::R`](R) reader structure"]
impl crate::Readable for UartFifoRdataSpec {}
#[doc = "`reset()` method sets UART_FIFO_RDATA to value 0"]
impl crate::Resettable for UartFifoRdataSpec {}
