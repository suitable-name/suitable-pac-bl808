#[doc = "Register `I2S_FIFO_CONFIG_0` reader"]
pub type R = crate::R<I2sFifoConfig0Spec>;
#[doc = "Register `I2S_FIFO_CONFIG_0` writer"]
pub type W = crate::W<I2sFifoConfig0Spec>;
#[doc = "Field `i2s_dma_tx_en` reader - TX DMA enable"]
pub type I2sDmaTxEnR = crate::BitReader;
#[doc = "Field `i2s_dma_tx_en` writer - TX DMA enable"]
pub type I2sDmaTxEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `i2s_dma_rx_en` reader - RX DMA enable"]
pub type I2sDmaRxEnR = crate::BitReader;
#[doc = "Field `i2s_dma_rx_en` writer - RX DMA enable"]
pub type I2sDmaRxEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tx_fifo_clr` writer - Clear TX FIFO"]
pub type TxFifoClrW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `rx_fifo_clr` writer - Clear RX FIFO"]
pub type RxFifoClrW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `tx_fifo_overflow` reader - TX FIFO overflow status"]
pub type TxFifoOverflowR = crate::BitReader;
#[doc = "Field `tx_fifo_underflow` reader - TX FIFO underflow status"]
pub type TxFifoUnderflowR = crate::BitReader;
#[doc = "Field `rx_fifo_overflow` reader - RX FIFO overflow status"]
pub type RxFifoOverflowR = crate::BitReader;
#[doc = "Field `rx_fifo_underflow` reader - RX FIFO underflow status"]
pub type RxFifoUnderflowR = crate::BitReader;
#[doc = "Field `cr_fifo_lr_merge` reader - FIFO L/R data merge enable"]
pub type CrFifoLrMergeR = crate::BitReader;
#[doc = "Field `cr_fifo_lr_merge` writer - FIFO L/R data merge enable"]
pub type CrFifoLrMergeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_fifo_lr_exchg` reader - FIFO L/R data exchange enable"]
pub type CrFifoLrExchgR = crate::BitReader;
#[doc = "Field `cr_fifo_lr_exchg` writer - FIFO L/R data exchange enable"]
pub type CrFifoLrExchgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_fifo_24b_lj` reader - FIFO 24-bit Left-Justified mode"]
pub type CrFifo24bLjR = crate::BitReader;
#[doc = "Field `cr_fifo_24b_lj` writer - FIFO 24-bit Left-Justified mode"]
pub type CrFifo24bLjW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - TX DMA enable"]
    #[inline(always)]
    pub fn i2s_dma_tx_en(&self) -> I2sDmaTxEnR {
        I2sDmaTxEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RX DMA enable"]
    #[inline(always)]
    pub fn i2s_dma_rx_en(&self) -> I2sDmaRxEnR {
        I2sDmaRxEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - TX FIFO overflow status"]
    #[inline(always)]
    pub fn tx_fifo_overflow(&self) -> TxFifoOverflowR {
        TxFifoOverflowR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - TX FIFO underflow status"]
    #[inline(always)]
    pub fn tx_fifo_underflow(&self) -> TxFifoUnderflowR {
        TxFifoUnderflowR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - RX FIFO overflow status"]
    #[inline(always)]
    pub fn rx_fifo_overflow(&self) -> RxFifoOverflowR {
        RxFifoOverflowR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - RX FIFO underflow status"]
    #[inline(always)]
    pub fn rx_fifo_underflow(&self) -> RxFifoUnderflowR {
        RxFifoUnderflowR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - FIFO L/R data merge enable"]
    #[inline(always)]
    pub fn cr_fifo_lr_merge(&self) -> CrFifoLrMergeR {
        CrFifoLrMergeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - FIFO L/R data exchange enable"]
    #[inline(always)]
    pub fn cr_fifo_lr_exchg(&self) -> CrFifoLrExchgR {
        CrFifoLrExchgR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - FIFO 24-bit Left-Justified mode"]
    #[inline(always)]
    pub fn cr_fifo_24b_lj(&self) -> CrFifo24bLjR {
        CrFifo24bLjR::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TX DMA enable"]
    #[inline(always)]
    pub fn i2s_dma_tx_en(&mut self) -> I2sDmaTxEnW<'_, I2sFifoConfig0Spec> {
        I2sDmaTxEnW::new(self, 0)
    }
    #[doc = "Bit 1 - RX DMA enable"]
    #[inline(always)]
    pub fn i2s_dma_rx_en(&mut self) -> I2sDmaRxEnW<'_, I2sFifoConfig0Spec> {
        I2sDmaRxEnW::new(self, 1)
    }
    #[doc = "Bit 2 - Clear TX FIFO"]
    #[inline(always)]
    pub fn tx_fifo_clr(&mut self) -> TxFifoClrW<'_, I2sFifoConfig0Spec> {
        TxFifoClrW::new(self, 2)
    }
    #[doc = "Bit 3 - Clear RX FIFO"]
    #[inline(always)]
    pub fn rx_fifo_clr(&mut self) -> RxFifoClrW<'_, I2sFifoConfig0Spec> {
        RxFifoClrW::new(self, 3)
    }
    #[doc = "Bit 8 - FIFO L/R data merge enable"]
    #[inline(always)]
    pub fn cr_fifo_lr_merge(&mut self) -> CrFifoLrMergeW<'_, I2sFifoConfig0Spec> {
        CrFifoLrMergeW::new(self, 8)
    }
    #[doc = "Bit 9 - FIFO L/R data exchange enable"]
    #[inline(always)]
    pub fn cr_fifo_lr_exchg(&mut self) -> CrFifoLrExchgW<'_, I2sFifoConfig0Spec> {
        CrFifoLrExchgW::new(self, 9)
    }
    #[doc = "Bit 10 - FIFO 24-bit Left-Justified mode"]
    #[inline(always)]
    pub fn cr_fifo_24b_lj(&mut self) -> CrFifo24bLjW<'_, I2sFifoConfig0Spec> {
        CrFifo24bLjW::new(self, 10)
    }
}
#[doc = "I2S FIFO Configuration Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`i2s_fifo_config_0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2s_fifo_config_0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2sFifoConfig0Spec;
impl crate::RegisterSpec for I2sFifoConfig0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2s_fifo_config_0::R`](R) reader structure"]
impl crate::Readable for I2sFifoConfig0Spec {}
#[doc = "`write(|w| ..)` method takes [`i2s_fifo_config_0::W`](W) writer structure"]
impl crate::Writable for I2sFifoConfig0Spec {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x0c;
}
#[doc = "`reset()` method sets I2S_FIFO_CONFIG_0 to value 0"]
impl crate::Resettable for I2sFifoConfig0Spec {}
