#[doc = "Register `GPDAC_TX_FIFO_STATUS` reader"]
pub type R = crate::R<GpdacTxFifoStatusSpec>;
#[doc = "Field `tx_fifo_empty` reader - "]
pub type TxFifoEmptyR = crate::BitReader;
#[doc = "Field `tx_fifo_full` reader - "]
pub type TxFifoFullR = crate::BitReader;
#[doc = "Field `tx_cs` reader - TX Control Status"]
pub type TxCsR = crate::FieldReader;
#[doc = "Field `TxFifoRdPtr` reader - "]
pub type TxFifoRdPtrR = crate::FieldReader;
#[doc = "Field `TxFifoWrPtr` reader - "]
pub type TxFifoWrPtrR = crate::FieldReader;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn tx_fifo_empty(&self) -> TxFifoEmptyR {
        TxFifoEmptyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn tx_fifo_full(&self) -> TxFifoFullR {
        TxFifoFullR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - TX Control Status"]
    #[inline(always)]
    pub fn tx_cs(&self) -> TxCsR {
        TxCsR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn tx_fifo_rd_ptr(&self) -> TxFifoRdPtrR {
        TxFifoRdPtrR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn tx_fifo_wr_ptr(&self) -> TxFifoWrPtrR {
        TxFifoWrPtrR::new(((self.bits >> 8) & 3) as u8)
    }
}
#[doc = "GPDAC TX FIFO Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpdac_tx_fifo_status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpdacTxFifoStatusSpec;
impl crate::RegisterSpec for GpdacTxFifoStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpdac_tx_fifo_status::R`](R) reader structure"]
impl crate::Readable for GpdacTxFifoStatusSpec {}
#[doc = "`reset()` method sets GPDAC_TX_FIFO_STATUS to value 0x80"]
impl crate::Resettable for GpdacTxFifoStatusSpec {
    const RESET_VALUE: u32 = 0x80;
}
