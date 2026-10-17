#[doc = "Register `RawIntErrorStatus` reader"]
pub type R = crate::R<RawIntErrorStatusSpec>;
#[doc = "Field `RawIntErrorStatus` reader - Raw error interrupt status for channels 0-7"]
pub type RawIntErrorStatusR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Raw error interrupt status for channels 0-7"]
    #[inline(always)]
    pub fn raw_int_error_status(&self) -> RawIntErrorStatusR {
        RawIntErrorStatusR::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Status of the error interrupt prior to masking\n\nYou can [`read`](crate::Reg::read) this register and get [`raw_int_error_status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RawIntErrorStatusSpec;
impl crate::RegisterSpec for RawIntErrorStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`raw_int_error_status::R`](R) reader structure"]
impl crate::Readable for RawIntErrorStatusSpec {}
#[doc = "`reset()` method sets RawIntErrorStatus to value 0"]
impl crate::Resettable for RawIntErrorStatusSpec {}
