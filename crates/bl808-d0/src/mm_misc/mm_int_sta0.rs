#[doc = "Register `MM_INT_STA0` reader"]
pub type R = crate::R<MmIntSta0Spec>;
#[doc = "Field `mm_int_sta0` reader - Active status of MM interrupts 0-31 before masking."]
pub type MmIntSta0R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Active status of MM interrupts 0-31 before masking."]
    #[inline(always)]
    pub fn mm_int_sta0(&self) -> MmIntSta0R {
        MmIntSta0R::new(self.bits)
    }
}
#[doc = "Multimedia Interrupt Status Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`mm_int_sta0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MmIntSta0Spec;
impl crate::RegisterSpec for MmIntSta0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mm_int_sta0::R`](R) reader structure"]
impl crate::Readable for MmIntSta0Spec {}
#[doc = "`reset()` method sets MM_INT_STA0 to value 0"]
impl crate::Resettable for MmIntSta0Spec {}
