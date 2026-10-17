#[doc = "Register `IRTX_FIFO_CONFIG_0` reader"]
pub type R = crate::R<IrtxFifoConfig0Spec>;
#[doc = "Register `IRTX_FIFO_CONFIG_0` writer"]
pub type W = crate::W<IrtxFifoConfig0Spec>;
#[doc = "Field `irtx_dma_en` reader - Enable IRTX DMA request"]
pub type IrtxDmaEnR = crate::BitReader;
#[doc = "Field `irtx_dma_en` writer - Enable IRTX DMA request"]
pub type IrtxDmaEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tx_fifo_clr` reader - Clear TX FIFO"]
pub type TxFifoClrR = crate::BitReader;
#[doc = "Field `tx_fifo_clr` writer - Clear TX FIFO"]
pub type TxFifoClrW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `rx_fifo_clr` reader - Clear RX FIFO"]
pub type RxFifoClrR = crate::BitReader;
#[doc = "Field `rx_fifo_clr` writer - Clear RX FIFO"]
pub type RxFifoClrW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `tx_fifo_overflow` reader - TX FIFO overflow flag"]
pub type TxFifoOverflowR = crate::BitReader;
#[doc = "Field `tx_fifo_underflow` reader - TX FIFO underflow flag"]
pub type TxFifoUnderflowR = crate::BitReader;
#[doc = "Field `rx_fifo_overflow` reader - RX FIFO overflow flag"]
pub type RxFifoOverflowR = crate::BitReader;
#[doc = "Field `rx_fifo_underflow` reader - RX FIFO underflow flag"]
pub type RxFifoUnderflowR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Enable IRTX DMA request"]
    #[inline(always)]
    pub fn irtx_dma_en(&self) -> IrtxDmaEnR {
        IrtxDmaEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Clear TX FIFO"]
    #[inline(always)]
    pub fn tx_fifo_clr(&self) -> TxFifoClrR {
        TxFifoClrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Clear RX FIFO"]
    #[inline(always)]
    pub fn rx_fifo_clr(&self) -> RxFifoClrR {
        RxFifoClrR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - TX FIFO overflow flag"]
    #[inline(always)]
    pub fn tx_fifo_overflow(&self) -> TxFifoOverflowR {
        TxFifoOverflowR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - TX FIFO underflow flag"]
    #[inline(always)]
    pub fn tx_fifo_underflow(&self) -> TxFifoUnderflowR {
        TxFifoUnderflowR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - RX FIFO overflow flag"]
    #[inline(always)]
    pub fn rx_fifo_overflow(&self) -> RxFifoOverflowR {
        RxFifoOverflowR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - RX FIFO underflow flag"]
    #[inline(always)]
    pub fn rx_fifo_underflow(&self) -> RxFifoUnderflowR {
        RxFifoUnderflowR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable IRTX DMA request"]
    #[inline(always)]
    pub fn irtx_dma_en(&mut self) -> IrtxDmaEnW<'_, IrtxFifoConfig0Spec> {
        IrtxDmaEnW::new(self, 0)
    }
    #[doc = "Bit 2 - Clear TX FIFO"]
    #[inline(always)]
    pub fn tx_fifo_clr(&mut self) -> TxFifoClrW<'_, IrtxFifoConfig0Spec> {
        TxFifoClrW::new(self, 2)
    }
    #[doc = "Bit 3 - Clear RX FIFO"]
    #[inline(always)]
    pub fn rx_fifo_clr(&mut self) -> RxFifoClrW<'_, IrtxFifoConfig0Spec> {
        RxFifoClrW::new(self, 3)
    }
}
#[doc = "IRTX FIFO Configuration and Status 0\n\nYou can [`read`](crate::Reg::read) this register and get [`irtx_fifo_config_0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irtx_fifo_config_0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IrtxFifoConfig0Spec;
impl crate::RegisterSpec for IrtxFifoConfig0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`irtx_fifo_config_0::R`](R) reader structure"]
impl crate::Readable for IrtxFifoConfig0Spec {}
#[doc = "`write(|w| ..)` method takes [`irtx_fifo_config_0::W`](W) writer structure"]
impl crate::Writable for IrtxFifoConfig0Spec {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x0c;
}
#[doc = "`reset()` method sets IRTX_FIFO_CONFIG_0 to value 0"]
impl crate::Resettable for IrtxFifoConfig0Spec {}
