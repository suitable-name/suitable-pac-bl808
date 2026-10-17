#[doc = "Register `GAUGE_RX_FIFO_CTRL` reader"]
pub type R = crate::R<GaugeRxFifoCtrlSpec>;
#[doc = "Register `GAUGE_RX_FIFO_CTRL` writer"]
pub type W = crate::W<GaugeRxFifoCtrlSpec>;
#[doc = "Field `gauge_rx_fifo_flush` reader - "]
pub type GaugeRxFifoFlushR = crate::BitReader;
#[doc = "Field `gauge_rx_fifo_flush` writer - "]
pub type GaugeRxFifoFlushW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `gauge_rxo_int_en` reader - "]
pub type GaugeRxoIntEnR = crate::BitReader;
#[doc = "Field `gauge_rxo_int_en` writer - "]
pub type GaugeRxoIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `gauge_rxu_int_en` reader - "]
pub type GaugeRxuIntEnR = crate::BitReader;
#[doc = "Field `gauge_rxu_int_en` writer - "]
pub type GaugeRxuIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `gauge_rxa_int_en` reader - "]
pub type GaugeRxaIntEnR = crate::BitReader;
#[doc = "Field `gauge_rxa_int_en` writer - "]
pub type GaugeRxaIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `gauge_rx_drq_en` reader - "]
pub type GaugeRxDrqEnR = crate::BitReader;
#[doc = "Field `gauge_rx_drq_en` writer - "]
pub type GaugeRxDrqEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `gauge_rx_data_res` reader - "]
pub type GaugeRxDataResR = crate::BitReader;
#[doc = "Field `gauge_rx_data_res` writer - "]
pub type GaugeRxDataResW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `gauge_rx_ch_en` reader - "]
pub type GaugeRxChEnR = crate::BitReader;
#[doc = "Field `gauge_rx_ch_en` writer - "]
pub type GaugeRxChEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `gauge_rx_drq_cnt` reader - "]
pub type GaugeRxDrqCntR = crate::FieldReader;
#[doc = "Field `gauge_rx_drq_cnt` writer - "]
pub type GaugeRxDrqCntW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `gauge_rx_trg_level` reader - "]
pub type GaugeRxTrgLevelR = crate::FieldReader;
#[doc = "Field `gauge_rx_trg_level` writer - "]
pub type GaugeRxTrgLevelW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `gauge_rx_data_mode` reader - "]
pub type GaugeRxDataModeR = crate::FieldReader;
#[doc = "Field `gauge_rx_data_mode` writer - "]
pub type GaugeRxDataModeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn gauge_rx_fifo_flush(&self) -> GaugeRxFifoFlushR {
        GaugeRxFifoFlushR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn gauge_rxo_int_en(&self) -> GaugeRxoIntEnR {
        GaugeRxoIntEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn gauge_rxu_int_en(&self) -> GaugeRxuIntEnR {
        GaugeRxuIntEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn gauge_rxa_int_en(&self) -> GaugeRxaIntEnR {
        GaugeRxaIntEnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn gauge_rx_drq_en(&self) -> GaugeRxDrqEnR {
        GaugeRxDrqEnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn gauge_rx_data_res(&self) -> GaugeRxDataResR {
        GaugeRxDataResR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn gauge_rx_ch_en(&self) -> GaugeRxChEnR {
        GaugeRxChEnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn gauge_rx_drq_cnt(&self) -> GaugeRxDrqCntR {
        GaugeRxDrqCntR::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:18"]
    #[inline(always)]
    pub fn gauge_rx_trg_level(&self) -> GaugeRxTrgLevelR {
        GaugeRxTrgLevelR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn gauge_rx_data_mode(&self) -> GaugeRxDataModeR {
        GaugeRxDataModeR::new(((self.bits >> 24) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn gauge_rx_fifo_flush(&mut self) -> GaugeRxFifoFlushW<'_, GaugeRxFifoCtrlSpec> {
        GaugeRxFifoFlushW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn gauge_rxo_int_en(&mut self) -> GaugeRxoIntEnW<'_, GaugeRxFifoCtrlSpec> {
        GaugeRxoIntEnW::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn gauge_rxu_int_en(&mut self) -> GaugeRxuIntEnW<'_, GaugeRxFifoCtrlSpec> {
        GaugeRxuIntEnW::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn gauge_rxa_int_en(&mut self) -> GaugeRxaIntEnW<'_, GaugeRxFifoCtrlSpec> {
        GaugeRxaIntEnW::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn gauge_rx_drq_en(&mut self) -> GaugeRxDrqEnW<'_, GaugeRxFifoCtrlSpec> {
        GaugeRxDrqEnW::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn gauge_rx_data_res(&mut self) -> GaugeRxDataResW<'_, GaugeRxFifoCtrlSpec> {
        GaugeRxDataResW::new(self, 5)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn gauge_rx_ch_en(&mut self) -> GaugeRxChEnW<'_, GaugeRxFifoCtrlSpec> {
        GaugeRxChEnW::new(self, 8)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn gauge_rx_drq_cnt(&mut self) -> GaugeRxDrqCntW<'_, GaugeRxFifoCtrlSpec> {
        GaugeRxDrqCntW::new(self, 14)
    }
    #[doc = "Bits 16:18"]
    #[inline(always)]
    pub fn gauge_rx_trg_level(&mut self) -> GaugeRxTrgLevelW<'_, GaugeRxFifoCtrlSpec> {
        GaugeRxTrgLevelW::new(self, 16)
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn gauge_rx_data_mode(&mut self) -> GaugeRxDataModeW<'_, GaugeRxFifoCtrlSpec> {
        GaugeRxDataModeW::new(self, 24)
    }
}
#[doc = "Gauge RX FIFO Control\n\nYou can [`read`](crate::Reg::read) this register and get [`gauge_rx_fifo_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gauge_rx_fifo_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GaugeRxFifoCtrlSpec;
impl crate::RegisterSpec for GaugeRxFifoCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gauge_rx_fifo_ctrl::R`](R) reader structure"]
impl crate::Readable for GaugeRxFifoCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`gauge_rx_fifo_ctrl::W`](W) writer structure"]
impl crate::Writable for GaugeRxFifoCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GAUGE_RX_FIFO_CTRL to value 0"]
impl crate::Resettable for GaugeRxFifoCtrlSpec {}
