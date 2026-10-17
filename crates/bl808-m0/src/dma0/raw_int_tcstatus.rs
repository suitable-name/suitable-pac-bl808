#[doc = "Register `RawIntTCStatus` reader"]
pub type R = crate::R<RawIntTcstatusSpec>;
#[doc = "Field `RawIntTCStatus` reader - Raw terminal count interrupt status for channels 0-7"]
pub type RawIntTcstatusR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Raw terminal count interrupt status for channels 0-7"]
    #[inline(always)]
    pub fn raw_int_tcstatus(&self) -> RawIntTcstatusR {
        RawIntTcstatusR::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Status of the terminal count interrupt prior to masking\n\nYou can [`read`](crate::Reg::read) this register and get [`raw_int_tcstatus::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RawIntTcstatusSpec;
impl crate::RegisterSpec for RawIntTcstatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`raw_int_tcstatus::R`](R) reader structure"]
impl crate::Readable for RawIntTcstatusSpec {}
#[doc = "`reset()` method sets RawIntTCStatus to value 0"]
impl crate::Resettable for RawIntTcstatusSpec {}
