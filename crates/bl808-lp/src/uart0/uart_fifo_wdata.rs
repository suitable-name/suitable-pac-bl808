#[doc = "Register `UART_FIFO_WDATA` writer"]
pub type W = crate::W<UartFifoWdataSpec>;
#[doc = "Field `uart_fifo_wdata` writer - "]
pub type UartFifoWdataW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn uart_fifo_wdata(&mut self) -> UartFifoWdataW<'_, UartFifoWdataSpec> {
        UartFifoWdataW::new(self, 0)
    }
}
#[doc = "UART TX FIFO Data Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart_fifo_wdata::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UartFifoWdataSpec;
impl crate::RegisterSpec for UartFifoWdataSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`uart_fifo_wdata::W`](W) writer structure"]
impl crate::Writable for UartFifoWdataSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UART_FIFO_WDATA to value 0"]
impl crate::Resettable for UartFifoWdataSpec {}
