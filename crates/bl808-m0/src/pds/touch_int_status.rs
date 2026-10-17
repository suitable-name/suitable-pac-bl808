#[doc = "Register `TOUCH_INT_STATUS` reader"]
pub type R = crate::R<TouchIntStatusSpec>;
#[doc = "Field `touch_int_status` reader - "]
pub type TouchIntStatusR = crate::FieldReader<u16>;
#[doc = "Field `touch_end_flag` reader - "]
pub type TouchEndFlagR = crate::BitReader;
impl R {
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn touch_int_status(&self) -> TouchIntStatusR {
        TouchIntStatusR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn touch_end_flag(&self) -> TouchEndFlagR {
        TouchEndFlagR::new(((self.bits >> 12) & 1) != 0)
    }
}
#[doc = "Touch sensor interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`touch_int_status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TouchIntStatusSpec;
impl crate::RegisterSpec for TouchIntStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`touch_int_status::R`](R) reader structure"]
impl crate::Readable for TouchIntStatusSpec {}
#[doc = "`reset()` method sets TOUCH_INT_STATUS to value 0"]
impl crate::Resettable for TouchIntStatusSpec {}
