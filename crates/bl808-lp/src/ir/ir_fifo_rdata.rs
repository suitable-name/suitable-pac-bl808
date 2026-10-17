#[doc = "Register `IR_FIFO_RDATA` reader"]
pub type R = crate::R<IrFifoRdataSpec>;
#[doc = "Field `rx_fifo_rdata` reader - Pop data from RX FIFO"]
pub type RxFifoRdataR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Pop data from RX FIFO"]
    #[inline(always)]
    pub fn rx_fifo_rdata(&self) -> RxFifoRdataR {
        RxFifoRdataR::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "IR RX FIFO Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ir_fifo_rdata::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IrFifoRdataSpec;
impl crate::RegisterSpec for IrFifoRdataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ir_fifo_rdata::R`](R) reader structure"]
impl crate::Readable for IrFifoRdataSpec {}
#[doc = "`reset()` method sets IR_FIFO_RDATA to value 0"]
impl crate::Resettable for IrFifoRdataSpec {}
