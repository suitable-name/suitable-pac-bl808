#[doc = "Register `IntStatus` reader"]
pub type R = crate::R<IntStatusSpec>;
#[doc = "Field `IntStatus` reader - Interrupt status for channels 0-7"]
pub type IntStatusR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Interrupt status for channels 0-7"]
    #[inline(always)]
    pub fn int_status(&self) -> IntStatusR {
        IntStatusR::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Status of the DMA interrupts after masking\n\nYou can [`read`](crate::Reg::read) this register and get [`int_status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntStatusSpec;
impl crate::RegisterSpec for IntStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_status::R`](R) reader structure"]
impl crate::Readable for IntStatusSpec {}
#[doc = "`reset()` method sets IntStatus to value 0"]
impl crate::Resettable for IntStatusSpec {}
