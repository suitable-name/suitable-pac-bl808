#[doc = "Register `GAUGE_RX_FIFO_STATUS` reader"]
pub type R = crate::R<GaugeRxFifoStatusSpec>;
#[doc = "Register `GAUGE_RX_FIFO_STATUS` writer"]
pub type W = crate::W<GaugeRxFifoStatusSpec>;
#[doc = "Field `gauge_rxo_int` reader - "]
pub type GaugeRxoIntR = crate::BitReader;
#[doc = "Field `gauge_rxu_int` reader - "]
pub type GaugeRxuIntR = crate::BitReader;
#[doc = "Field `gauge_rxa_int` reader - "]
pub type GaugeRxaIntR = crate::BitReader;
#[doc = "Field `gauge_rxa_cnt` reader - "]
pub type GaugeRxaCntR = crate::FieldReader;
#[doc = "Field `gauge_rxa` reader - "]
pub type GaugeRxaR = crate::BitReader;
impl R {
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn gauge_rxo_int(&self) -> GaugeRxoIntR {
        GaugeRxoIntR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn gauge_rxu_int(&self) -> GaugeRxuIntR {
        GaugeRxuIntR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn gauge_rxa_int(&self) -> GaugeRxaIntR {
        GaugeRxaIntR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 16:18"]
    #[inline(always)]
    pub fn gauge_rxa_cnt(&self) -> GaugeRxaCntR {
        GaugeRxaCntR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn gauge_rxa(&self) -> GaugeRxaR {
        GaugeRxaR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {}
#[doc = "Gauge RX FIFO Status\n\nYou can [`read`](crate::Reg::read) this register and get [`gauge_rx_fifo_status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gauge_rx_fifo_status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GaugeRxFifoStatusSpec;
impl crate::RegisterSpec for GaugeRxFifoStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gauge_rx_fifo_status::R`](R) reader structure"]
impl crate::Readable for GaugeRxFifoStatusSpec {}
#[doc = "`write(|w| ..)` method takes [`gauge_rx_fifo_status::W`](W) writer structure"]
impl crate::Writable for GaugeRxFifoStatusSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GAUGE_RX_FIFO_STATUS to value 0"]
impl crate::Resettable for GaugeRxFifoStatusSpec {}
