#[doc = "Register `rx_fifo_status` reader"]
pub type R = crate::R<RxFifoStatusSpec>;
#[doc = "Field `rxo_int` reader - "]
pub type RxoIntR = crate::BitReader;
#[doc = "Field `rxu_int` reader - "]
pub type RxuIntR = crate::BitReader;
#[doc = "Field `rxa_int` reader - "]
pub type RxaIntR = crate::BitReader;
#[doc = "Field `rxa_cnt` reader - "]
pub type RxaCntR = crate::FieldReader;
#[doc = "Field `rxa` reader - "]
pub type RxaR = crate::BitReader;
impl R {
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn rxo_int(&self) -> RxoIntR {
        RxoIntR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn rxu_int(&self) -> RxuIntR {
        RxuIntR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn rxa_int(&self) -> RxaIntR {
        RxaIntR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 16:22"]
    #[inline(always)]
    pub fn rxa_cnt(&self) -> RxaCntR {
        RxaCntR::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn rxa(&self) -> RxaR {
        RxaR::new(((self.bits >> 24) & 1) != 0)
    }
}
#[doc = "RX FIFO Status: Overflow, underflow, and item count\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_fifo_status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxFifoStatusSpec;
impl crate::RegisterSpec for RxFifoStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rx_fifo_status::R`](R) reader structure"]
impl crate::Readable for RxFifoStatusSpec {}
#[doc = "`reset()` method sets rx_fifo_status to value 0"]
impl crate::Resettable for RxFifoStatusSpec {}
