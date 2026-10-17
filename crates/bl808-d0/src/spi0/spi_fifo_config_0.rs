#[doc = "Register `SPI_FIFO_CONFIG_0` reader"]
pub type R = crate::R<SpiFifoConfig0Spec>;
#[doc = "Register `SPI_FIFO_CONFIG_0` writer"]
pub type W = crate::W<SpiFifoConfig0Spec>;
#[doc = "Field `spi_dma_tx_en` reader - Enable TX DMA request"]
pub type SpiDmaTxEnR = crate::BitReader;
#[doc = "Field `spi_dma_tx_en` writer - Enable TX DMA request"]
pub type SpiDmaTxEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `spi_dma_rx_en` reader - Enable RX DMA request"]
pub type SpiDmaRxEnR = crate::BitReader;
#[doc = "Field `spi_dma_rx_en` writer - Enable RX DMA request"]
pub type SpiDmaRxEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tx_fifo_clr` writer - Clear TX FIFO"]
pub type TxFifoClrW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `rx_fifo_clr` writer - Clear RX FIFO"]
pub type RxFifoClrW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `tx_fifo_overflow` reader - "]
pub type TxFifoOverflowR = crate::BitReader;
#[doc = "Field `tx_fifo_underflow` reader - "]
pub type TxFifoUnderflowR = crate::BitReader;
#[doc = "Field `rx_fifo_overflow` reader - "]
pub type RxFifoOverflowR = crate::BitReader;
#[doc = "Field `rx_fifo_underflow` reader - "]
pub type RxFifoUnderflowR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Enable TX DMA request"]
    #[inline(always)]
    pub fn spi_dma_tx_en(&self) -> SpiDmaTxEnR {
        SpiDmaTxEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable RX DMA request"]
    #[inline(always)]
    pub fn spi_dma_rx_en(&self) -> SpiDmaRxEnR {
        SpiDmaRxEnR::new(((self.bits >> 1) & 1) != 0)
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
    #[doc = "Bit 0 - Enable TX DMA request"]
    #[inline(always)]
    pub fn spi_dma_tx_en(&mut self) -> SpiDmaTxEnW<'_, SpiFifoConfig0Spec> {
        SpiDmaTxEnW::new(self, 0)
    }
    #[doc = "Bit 1 - Enable RX DMA request"]
    #[inline(always)]
    pub fn spi_dma_rx_en(&mut self) -> SpiDmaRxEnW<'_, SpiFifoConfig0Spec> {
        SpiDmaRxEnW::new(self, 1)
    }
    #[doc = "Bit 2 - Clear TX FIFO"]
    #[inline(always)]
    pub fn tx_fifo_clr(&mut self) -> TxFifoClrW<'_, SpiFifoConfig0Spec> {
        TxFifoClrW::new(self, 2)
    }
    #[doc = "Bit 3 - Clear RX FIFO"]
    #[inline(always)]
    pub fn rx_fifo_clr(&mut self) -> RxFifoClrW<'_, SpiFifoConfig0Spec> {
        RxFifoClrW::new(self, 3)
    }
}
#[doc = "SPI FIFO Configuration 0\n\nYou can [`read`](crate::Reg::read) this register and get [`spi_fifo_config_0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_fifo_config_0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SpiFifoConfig0Spec;
impl crate::RegisterSpec for SpiFifoConfig0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi_fifo_config_0::R`](R) reader structure"]
impl crate::Readable for SpiFifoConfig0Spec {}
#[doc = "`write(|w| ..)` method takes [`spi_fifo_config_0::W`](W) writer structure"]
impl crate::Writable for SpiFifoConfig0Spec {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x0c;
}
#[doc = "`reset()` method sets SPI_FIFO_CONFIG_0 to value 0"]
impl crate::Resettable for SpiFifoConfig0Spec {}
