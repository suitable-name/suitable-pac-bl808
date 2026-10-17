#[doc = "Register `UART_STATUS` reader"]
pub type R = crate::R<UartStatusSpec>;
#[doc = "Field `sts_utx_bus_busy` reader - "]
pub type StsUtxBusBusyR = crate::BitReader;
#[doc = "Field `sts_urx_bus_busy` reader - "]
pub type StsUrxBusBusyR = crate::BitReader;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn sts_utx_bus_busy(&self) -> StsUtxBusBusyR {
        StsUtxBusBusyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn sts_urx_bus_busy(&self) -> StsUrxBusBusyR {
        StsUrxBusBusyR::new(((self.bits >> 1) & 1) != 0)
    }
}
#[doc = "UART Busy Status\n\nYou can [`read`](crate::Reg::read) this register and get [`uart_status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UartStatusSpec;
impl crate::RegisterSpec for UartStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uart_status::R`](R) reader structure"]
impl crate::Readable for UartStatusSpec {}
#[doc = "`reset()` method sets UART_STATUS to value 0"]
impl crate::Resettable for UartStatusSpec {}
