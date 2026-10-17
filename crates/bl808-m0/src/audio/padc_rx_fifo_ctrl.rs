#[doc = "Register `padc_rx_fifo_ctrl` reader"]
pub type R = crate::R<PadcRxFifoCtrlSpec>;
#[doc = "Register `padc_rx_fifo_ctrl` writer"]
pub type W = crate::W<PadcRxFifoCtrlSpec>;
#[doc = "Field `padc_rx_fifo_flush` writer - "]
pub type PadcRxFifoFlushW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `padc_rxo_int_en` reader - "]
pub type PadcRxoIntEnR = crate::BitReader;
#[doc = "Field `padc_rxo_int_en` writer - "]
pub type PadcRxoIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `padc_rxu_int_en` reader - "]
pub type PadcRxuIntEnR = crate::BitReader;
#[doc = "Field `padc_rxu_int_en` writer - "]
pub type PadcRxuIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `padc_rx_drq_en` reader - "]
pub type PadcRxDrqEnR = crate::BitReader;
#[doc = "Field `padc_rx_drq_en` writer - "]
pub type PadcRxDrqEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `padc_rx_ch_en` reader - "]
pub type PadcRxChEnR = crate::BitReader;
#[doc = "Field `padc_rx_ch_en` writer - "]
pub type PadcRxChEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `padc_rx_trg_level` reader - "]
pub type PadcRxTrgLevelR = crate::FieldReader;
#[doc = "Field `padc_rx_trg_level` writer - "]
pub type PadcRxTrgLevelW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn padc_rxo_int_en(&self) -> PadcRxoIntEnR {
        PadcRxoIntEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn padc_rxu_int_en(&self) -> PadcRxuIntEnR {
        PadcRxuIntEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn padc_rx_drq_en(&self) -> PadcRxDrqEnR {
        PadcRxDrqEnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn padc_rx_ch_en(&self) -> PadcRxChEnR {
        PadcRxChEnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn padc_rx_trg_level(&self) -> PadcRxTrgLevelR {
        PadcRxTrgLevelR::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn padc_rx_fifo_flush(&mut self) -> PadcRxFifoFlushW<'_, PadcRxFifoCtrlSpec> {
        PadcRxFifoFlushW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn padc_rxo_int_en(&mut self) -> PadcRxoIntEnW<'_, PadcRxFifoCtrlSpec> {
        PadcRxoIntEnW::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn padc_rxu_int_en(&mut self) -> PadcRxuIntEnW<'_, PadcRxFifoCtrlSpec> {
        PadcRxuIntEnW::new(self, 2)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn padc_rx_drq_en(&mut self) -> PadcRxDrqEnW<'_, PadcRxFifoCtrlSpec> {
        PadcRxDrqEnW::new(self, 4)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn padc_rx_ch_en(&mut self) -> PadcRxChEnW<'_, PadcRxFifoCtrlSpec> {
        PadcRxChEnW::new(self, 8)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn padc_rx_trg_level(&mut self) -> PadcRxTrgLevelW<'_, PadcRxFifoCtrlSpec> {
        PadcRxTrgLevelW::new(self, 16)
    }
}
#[doc = "Passthrough ADC RX FIFO Control: DMA and interrupt triggers\n\nYou can [`read`](crate::Reg::read) this register and get [`padc_rx_fifo_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`padc_rx_fifo_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PadcRxFifoCtrlSpec;
impl crate::RegisterSpec for PadcRxFifoCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`padc_rx_fifo_ctrl::R`](R) reader structure"]
impl crate::Readable for PadcRxFifoCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`padc_rx_fifo_ctrl::W`](W) writer structure"]
impl crate::Writable for PadcRxFifoCtrlSpec {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x01;
}
#[doc = "`reset()` method sets padc_rx_fifo_ctrl to value 0"]
impl crate::Resettable for PadcRxFifoCtrlSpec {}
