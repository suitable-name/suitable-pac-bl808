#[doc = "Register `INTERRUPT` reader"]
pub type R = crate::R<InterruptSpec>;
#[doc = "Field `receive_int` reader - "]
pub type ReceiveIntR = crate::BitReader;
#[doc = "Field `transmit_int` reader - "]
pub type TransmitIntR = crate::BitReader;
#[doc = "Field `error_int` reader - "]
pub type ErrorIntR = crate::BitReader;
#[doc = "Field `data_overrun_int` reader - "]
pub type DataOverrunIntR = crate::BitReader;
#[doc = "Field `error_passive_int` reader - "]
pub type ErrorPassiveIntR = crate::BitReader;
#[doc = "Field `arb_lost_int` reader - "]
pub type ArbLostIntR = crate::BitReader;
#[doc = "Field `bus_error_int` reader - "]
pub type BusErrorIntR = crate::BitReader;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn receive_int(&self) -> ReceiveIntR {
        ReceiveIntR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn transmit_int(&self) -> TransmitIntR {
        TransmitIntR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn error_int(&self) -> ErrorIntR {
        ErrorIntR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn data_overrun_int(&self) -> DataOverrunIntR {
        DataOverrunIntR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn error_passive_int(&self) -> ErrorPassiveIntR {
        ErrorPassiveIntR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn arb_lost_int(&self) -> ArbLostIntR {
        ArbLostIntR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn bus_error_int(&self) -> BusErrorIntR {
        BusErrorIntR::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "Interrupt Status Register (Reading clears CAN interrupt line)\n\nYou can [`read`](crate::Reg::read) this register and get [`interrupt::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct InterruptSpec;
impl crate::RegisterSpec for InterruptSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`interrupt::R`](R) reader structure"]
impl crate::Readable for InterruptSpec {}
#[doc = "`reset()` method sets INTERRUPT to value 0"]
impl crate::Resettable for InterruptSpec {}
