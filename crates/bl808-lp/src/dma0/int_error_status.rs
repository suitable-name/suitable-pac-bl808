#[doc = "Register `IntErrorStatus` reader"]
pub type R = crate::R<IntErrorStatusSpec>;
#[doc = "Field `IntErrorStatus` reader - Error interrupt status for channels 0-7"]
pub type IntErrorStatusR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Error interrupt status for channels 0-7"]
    #[inline(always)]
    pub fn int_error_status(&self) -> IntErrorStatusR {
        IntErrorStatusR::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Interrupt error status\n\nYou can [`read`](crate::Reg::read) this register and get [`int_error_status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntErrorStatusSpec;
impl crate::RegisterSpec for IntErrorStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_error_status::R`](R) reader structure"]
impl crate::Readable for IntErrorStatusSpec {}
#[doc = "`reset()` method sets IntErrorStatus to value 0"]
impl crate::Resettable for IntErrorStatusSpec {}
