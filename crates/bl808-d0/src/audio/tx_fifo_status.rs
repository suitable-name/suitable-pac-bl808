#[doc = "Register `tx_fifo_status` reader"]
pub type R = crate::R<TxFifoStatusSpec>;
#[doc = "Field `txo_int` reader - "]
pub type TxoIntR = crate::BitReader;
#[doc = "Field `txu_int` reader - "]
pub type TxuIntR = crate::BitReader;
#[doc = "Field `txa_int` reader - "]
pub type TxaIntR = crate::BitReader;
#[doc = "Field `txa_cnt` reader - "]
pub type TxaCntR = crate::FieldReader;
#[doc = "Field `txa` reader - "]
pub type TxaR = crate::BitReader;
impl R {
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn txo_int(&self) -> TxoIntR {
        TxoIntR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn txu_int(&self) -> TxuIntR {
        TxuIntR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn txa_int(&self) -> TxaIntR {
        TxaIntR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 16:21"]
    #[inline(always)]
    pub fn txa_cnt(&self) -> TxaCntR {
        TxaCntR::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn txa(&self) -> TxaR {
        TxaR::new(((self.bits >> 24) & 1) != 0)
    }
}
#[doc = "TX FIFO Status: Request status and available space count\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_fifo_status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxFifoStatusSpec;
impl crate::RegisterSpec for TxFifoStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tx_fifo_status::R`](R) reader structure"]
impl crate::Readable for TxFifoStatusSpec {}
#[doc = "`reset()` method sets tx_fifo_status to value 0"]
impl crate::Resettable for TxFifoStatusSpec {}
