#[doc = "Register `IntTCStatus` reader"]
pub type R = crate::R<IntTcstatusSpec>;
#[doc = "Field `IntTCStatus` reader - Terminal count interrupt status for channels 0-7"]
pub type IntTcstatusR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Terminal count interrupt status for channels 0-7"]
    #[inline(always)]
    pub fn int_tcstatus(&self) -> IntTcstatusR {
        IntTcstatusR::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Interrupt terminal count request status\n\nYou can [`read`](crate::Reg::read) this register and get [`int_tcstatus::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntTcstatusSpec;
impl crate::RegisterSpec for IntTcstatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_tcstatus::R`](R) reader structure"]
impl crate::Readable for IntTcstatusSpec {}
#[doc = "`reset()` method sets IntTCStatus to value 0"]
impl crate::Resettable for IntTcstatusSpec {}
