#[doc = "Register `UART_FIFO_CONFIG_0` reader"]
pub type R = crate::R<UartFifoConfig0Spec>;
#[doc = "Register `UART_FIFO_CONFIG_0` writer"]
pub type W = crate::W<UartFifoConfig0Spec>;
#[doc = "Field `uart_dma_tx_en` reader - "]
pub type UartDmaTxEnR = crate::BitReader;
#[doc = "Field `uart_dma_tx_en` writer - "]
pub type UartDmaTxEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `uart_dma_rx_en` reader - "]
pub type UartDmaRxEnR = crate::BitReader;
#[doc = "Field `uart_dma_rx_en` writer - "]
pub type UartDmaRxEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tx_fifo_clr` writer - "]
pub type TxFifoClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rx_fifo_clr` writer - "]
pub type RxFifoClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tx_fifo_overflow` reader - "]
pub type TxFifoOverflowR = crate::BitReader;
#[doc = "Field `tx_fifo_underflow` reader - "]
pub type TxFifoUnderflowR = crate::BitReader;
#[doc = "Field `rx_fifo_overflow` reader - "]
pub type RxFifoOverflowR = crate::BitReader;
#[doc = "Field `rx_fifo_underflow` reader - "]
pub type RxFifoUnderflowR = crate::BitReader;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn uart_dma_tx_en(&self) -> UartDmaTxEnR {
        UartDmaTxEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn uart_dma_rx_en(&self) -> UartDmaRxEnR {
        UartDmaRxEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn tx_fifo_overflow(&self) -> TxFifoOverflowR {
        TxFifoOverflowR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn tx_fifo_underflow(&self) -> TxFifoUnderflowR {
        TxFifoUnderflowR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn rx_fifo_overflow(&self) -> RxFifoOverflowR {
        RxFifoOverflowR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn rx_fifo_underflow(&self) -> RxFifoUnderflowR {
        RxFifoUnderflowR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn uart_dma_tx_en(&mut self) -> UartDmaTxEnW<'_, UartFifoConfig0Spec> {
        UartDmaTxEnW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn uart_dma_rx_en(&mut self) -> UartDmaRxEnW<'_, UartFifoConfig0Spec> {
        UartDmaRxEnW::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn tx_fifo_clr(&mut self) -> TxFifoClrW<'_, UartFifoConfig0Spec> {
        TxFifoClrW::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn rx_fifo_clr(&mut self) -> RxFifoClrW<'_, UartFifoConfig0Spec> {
        RxFifoClrW::new(self, 3)
    }
}
#[doc = "UART FIFO Configuration 0\n\nYou can [`read`](crate::Reg::read) this register and get [`uart_fifo_config_0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart_fifo_config_0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UartFifoConfig0Spec;
impl crate::RegisterSpec for UartFifoConfig0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uart_fifo_config_0::R`](R) reader structure"]
impl crate::Readable for UartFifoConfig0Spec {}
#[doc = "`write(|w| ..)` method takes [`uart_fifo_config_0::W`](W) writer structure"]
impl crate::Writable for UartFifoConfig0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UART_FIFO_CONFIG_0 to value 0"]
impl crate::Resettable for UartFifoConfig0Spec {}
