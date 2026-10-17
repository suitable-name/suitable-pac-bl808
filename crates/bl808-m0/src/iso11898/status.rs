#[doc = "Register `STATUS` reader"]
pub type R = crate::R<StatusSpec>;
#[doc = "Field `receive_buffer_sts` reader - "]
pub type ReceiveBufferStsR = crate::BitReader;
#[doc = "Field `data_overrun_sts` reader - "]
pub type DataOverrunStsR = crate::BitReader;
#[doc = "Field `transmit_buffer_sts` reader - "]
pub type TransmitBufferStsR = crate::BitReader;
#[doc = "Field `transmit_complete_sts` reader - "]
pub type TransmitCompleteStsR = crate::BitReader;
#[doc = "Field `receive_sts` reader - "]
pub type ReceiveStsR = crate::BitReader;
#[doc = "Field `transmit_sts` reader - "]
pub type TransmitStsR = crate::BitReader;
#[doc = "Field `error_sts` reader - "]
pub type ErrorStsR = crate::BitReader;
#[doc = "Field `bus_sts` reader - "]
pub type BusStsR = crate::BitReader;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn receive_buffer_sts(&self) -> ReceiveBufferStsR {
        ReceiveBufferStsR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn data_overrun_sts(&self) -> DataOverrunStsR {
        DataOverrunStsR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn transmit_buffer_sts(&self) -> TransmitBufferStsR {
        TransmitBufferStsR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn transmit_complete_sts(&self) -> TransmitCompleteStsR {
        TransmitCompleteStsR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn receive_sts(&self) -> ReceiveStsR {
        ReceiveStsR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn transmit_sts(&self) -> TransmitStsR {
        TransmitStsR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn error_sts(&self) -> ErrorStsR {
        ErrorStsR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn bus_sts(&self) -> BusStsR {
        BusStsR::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatusSpec;
impl crate::RegisterSpec for StatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for StatusSpec {}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for StatusSpec {}
