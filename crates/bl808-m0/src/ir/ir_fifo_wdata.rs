#[doc = "Register `IR_FIFO_WDATA` writer"]
pub type W = crate::W<IrFifoWdataSpec>;
#[doc = "Field `tx_fifo_wdata` writer - Push data into TX FIFO"]
pub type TxFifoWdataW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - Push data into TX FIFO"]
    #[inline(always)]
    pub fn tx_fifo_wdata(&mut self) -> TxFifoWdataW<'_, IrFifoWdataSpec> {
        TxFifoWdataW::new(self, 0)
    }
}
#[doc = "IR TX FIFO Data Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ir_fifo_wdata::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IrFifoWdataSpec;
impl crate::RegisterSpec for IrFifoWdataSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`ir_fifo_wdata::W`](W) writer structure"]
impl crate::Writable for IrFifoWdataSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IR_FIFO_WDATA to value 0"]
impl crate::Resettable for IrFifoWdataSpec {}
