#[doc = "Register `tx_fifo_data` writer"]
pub type W = crate::W<TxFifoDataSpec>;
#[doc = "Field `tx_data` writer - "]
pub type TxDataW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn tx_data(&mut self) -> TxDataW<'_, TxFifoDataSpec> {
        TxDataW::new(self, 0)
    }
}
#[doc = "TX FIFO Data: Write portal for outgoing audio streams\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tx_fifo_data::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxFifoDataSpec;
impl crate::RegisterSpec for TxFifoDataSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`tx_fifo_data::W`](W) writer structure"]
impl crate::Writable for TxFifoDataSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets tx_fifo_data to value 0"]
impl crate::Resettable for TxFifoDataSpec {}
