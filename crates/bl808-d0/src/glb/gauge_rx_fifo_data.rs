#[doc = "Register `GAUGE_RX_FIFO_DATA` reader"]
pub type R = crate::R<GaugeRxFifoDataSpec>;
#[doc = "Register `GAUGE_RX_FIFO_DATA` writer"]
pub type W = crate::W<GaugeRxFifoDataSpec>;
#[doc = "Field `gauge_rx_data` reader - "]
pub type GaugeRxDataR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn gauge_rx_data(&self) -> GaugeRxDataR {
        GaugeRxDataR::new(self.bits)
    }
}
impl W {}
#[doc = "Gauge RX FIFO Data\n\nYou can [`read`](crate::Reg::read) this register and get [`gauge_rx_fifo_data::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gauge_rx_fifo_data::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GaugeRxFifoDataSpec;
impl crate::RegisterSpec for GaugeRxFifoDataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gauge_rx_fifo_data::R`](R) reader structure"]
impl crate::Readable for GaugeRxFifoDataSpec {}
#[doc = "`write(|w| ..)` method takes [`gauge_rx_fifo_data::W`](W) writer structure"]
impl crate::Writable for GaugeRxFifoDataSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GAUGE_RX_FIFO_DATA to value 0"]
impl crate::Resettable for GaugeRxFifoDataSpec {}
