#[doc = "Register `DSI_FIFO_CONFIG_0` reader"]
pub type R = crate::R<DsiFifoConfig0Spec>;
#[doc = "Register `DSI_FIFO_CONFIG_0` writer"]
pub type W = crate::W<DsiFifoConfig0Spec>;
#[doc = "Field `dsi_dma_tx_en` reader - "]
pub type DsiDmaTxEnR = crate::BitReader;
#[doc = "Field `dsi_dma_tx_en` writer - "]
pub type DsiDmaTxEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dsi_dma_rx_en` reader - "]
pub type DsiDmaRxEnR = crate::BitReader;
#[doc = "Field `dsi_dma_rx_en` writer - "]
pub type DsiDmaRxEnW<'a, REG> = crate::BitWriter<'a, REG>;
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
    pub fn dsi_dma_tx_en(&self) -> DsiDmaTxEnR {
        DsiDmaTxEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn dsi_dma_rx_en(&self) -> DsiDmaRxEnR {
        DsiDmaRxEnR::new(((self.bits >> 1) & 1) != 0)
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
    pub fn dsi_dma_tx_en(&mut self) -> DsiDmaTxEnW<'_, DsiFifoConfig0Spec> {
        DsiDmaTxEnW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn dsi_dma_rx_en(&mut self) -> DsiDmaRxEnW<'_, DsiFifoConfig0Spec> {
        DsiDmaRxEnW::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn tx_fifo_clr(&mut self) -> TxFifoClrW<'_, DsiFifoConfig0Spec> {
        TxFifoClrW::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn rx_fifo_clr(&mut self) -> RxFifoClrW<'_, DsiFifoConfig0Spec> {
        RxFifoClrW::new(self, 3)
    }
}
#[doc = "FIFO Configuration 0\n\nYou can [`read`](crate::Reg::read) this register and get [`dsi_fifo_config_0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsi_fifo_config_0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DsiFifoConfig0Spec;
impl crate::RegisterSpec for DsiFifoConfig0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsi_fifo_config_0::R`](R) reader structure"]
impl crate::Readable for DsiFifoConfig0Spec {}
#[doc = "`write(|w| ..)` method takes [`dsi_fifo_config_0::W`](W) writer structure"]
impl crate::Writable for DsiFifoConfig0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DSI_FIFO_CONFIG_0 to value 0"]
impl crate::Resettable for DsiFifoConfig0Spec {}
