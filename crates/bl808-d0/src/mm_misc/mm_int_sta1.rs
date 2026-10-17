#[doc = "Register `MM_INT_STA1` reader"]
pub type R = crate::R<MmIntSta1Spec>;
#[doc = "Field `mm_int_sta1` reader - Active status of MM interrupts 32-63 before masking."]
pub type MmIntSta1R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Active status of MM interrupts 32-63 before masking."]
    #[inline(always)]
    pub fn mm_int_sta1(&self) -> MmIntSta1R {
        MmIntSta1R::new(self.bits)
    }
}
#[doc = "Multimedia Interrupt Status Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`mm_int_sta1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MmIntSta1Spec;
impl crate::RegisterSpec for MmIntSta1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mm_int_sta1::R`](R) reader structure"]
impl crate::Readable for MmIntSta1Spec {}
#[doc = "`reset()` method sets MM_INT_STA1 to value 0"]
impl crate::Resettable for MmIntSta1Spec {}
