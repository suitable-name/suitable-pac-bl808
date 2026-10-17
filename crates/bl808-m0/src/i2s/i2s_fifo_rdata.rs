#[doc = "Register `I2S_FIFO_RDATA` reader"]
pub type R = crate::R<I2sFifoRdataSpec>;
#[doc = "Field `i2s_fifo_rdata` reader - "]
pub type I2sFifoRdataR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn i2s_fifo_rdata(&self) -> I2sFifoRdataR {
        I2sFifoRdataR::new(self.bits)
    }
}
#[doc = "I2S FIFO Read Data\n\nYou can [`read`](crate::Reg::read) this register and get [`i2s_fifo_rdata::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2sFifoRdataSpec;
impl crate::RegisterSpec for I2sFifoRdataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2s_fifo_rdata::R`](R) reader structure"]
impl crate::Readable for I2sFifoRdataSpec {}
#[doc = "`reset()` method sets I2S_FIFO_RDATA to value 0"]
impl crate::Resettable for I2sFifoRdataSpec {}
