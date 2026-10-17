#[doc = "Register `padc_rx_fifo_status` reader"]
pub type R = crate::R<PadcRxFifoStatusSpec>;
#[doc = "Field `padc_rxo_int` reader - "]
pub type PadcRxoIntR = crate::BitReader;
#[doc = "Field `padc_rxu_int` reader - "]
pub type PadcRxuIntR = crate::BitReader;
#[doc = "Field `padc_rxa_cnt` reader - "]
pub type PadcRxaCntR = crate::FieldReader;
#[doc = "Field `padc_rxa` reader - "]
pub type PadcRxaR = crate::BitReader;
impl R {
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn padc_rxo_int(&self) -> PadcRxoIntR {
        PadcRxoIntR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn padc_rxu_int(&self) -> PadcRxuIntR {
        PadcRxuIntR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn padc_rxa_cnt(&self) -> PadcRxaCntR {
        PadcRxaCntR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn padc_rxa(&self) -> PadcRxaR {
        PadcRxaR::new(((self.bits >> 24) & 1) != 0)
    }
}
#[doc = "Passthrough ADC RX FIFO Status: Occupancy and error status\n\nYou can [`read`](crate::Reg::read) this register and get [`padc_rx_fifo_status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PadcRxFifoStatusSpec;
impl crate::RegisterSpec for PadcRxFifoStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`padc_rx_fifo_status::R`](R) reader structure"]
impl crate::Readable for PadcRxFifoStatusSpec {}
#[doc = "`reset()` method sets padc_rx_fifo_status to value 0"]
impl crate::Resettable for PadcRxFifoStatusSpec {}
