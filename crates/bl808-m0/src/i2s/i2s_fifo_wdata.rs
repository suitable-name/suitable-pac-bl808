#[doc = "Register `I2S_FIFO_WDATA` writer"]
pub type W = crate::W<I2sFifoWdataSpec>;
#[doc = "Field `i2s_fifo_wdata` writer - "]
pub type I2sFifoWdataW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn i2s_fifo_wdata(&mut self) -> I2sFifoWdataW<'_, I2sFifoWdataSpec> {
        I2sFifoWdataW::new(self, 0)
    }
}
#[doc = "I2S FIFO Write Data\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2s_fifo_wdata::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2sFifoWdataSpec;
impl crate::RegisterSpec for I2sFifoWdataSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`i2s_fifo_wdata::W`](W) writer structure"]
impl crate::Writable for I2sFifoWdataSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I2S_FIFO_WDATA to value 0"]
impl crate::Resettable for I2sFifoWdataSpec {}
