#[doc = "Register `TCR3` reader"]
pub type R = crate::R<Tcr3Spec>;
#[doc = "Field `tcr3_cnt` reader - "]
pub type Tcr3CntR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn tcr3_cnt(&self) -> Tcr3CntR {
        Tcr3CntR::new(self.bits)
    }
}
#[doc = "Timer 3 Counter Value\n\nYou can [`read`](crate::Reg::read) this register and get [`tcr3::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tcr3Spec;
impl crate::RegisterSpec for Tcr3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tcr3::R`](R) reader structure"]
impl crate::Readable for Tcr3Spec {}
#[doc = "`reset()` method sets TCR3 to value 0"]
impl crate::Resettable for Tcr3Spec {}
