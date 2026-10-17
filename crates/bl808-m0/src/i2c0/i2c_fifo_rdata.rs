#[doc = "Register `I2C_FIFO_RDATA` reader"]
pub type R = crate::R<I2cFifoRdataSpec>;
#[doc = "Register `I2C_FIFO_RDATA` writer"]
pub type W = crate::W<I2cFifoRdataSpec>;
#[doc = "Field `i2c_fifo_rdata` reader - "]
pub type I2cFifoRdataR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn i2c_fifo_rdata(&self) -> I2cFifoRdataR {
        I2cFifoRdataR::new(self.bits)
    }
}
impl W {}
#[doc = "FIFO Read Data Port\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c_fifo_rdata::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c_fifo_rdata::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2cFifoRdataSpec;
impl crate::RegisterSpec for I2cFifoRdataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2c_fifo_rdata::R`](R) reader structure"]
impl crate::Readable for I2cFifoRdataSpec {}
#[doc = "`write(|w| ..)` method takes [`i2c_fifo_rdata::W`](W) writer structure"]
impl crate::Writable for I2cFifoRdataSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I2C_FIFO_RDATA to value 0"]
impl crate::Resettable for I2cFifoRdataSpec {}
