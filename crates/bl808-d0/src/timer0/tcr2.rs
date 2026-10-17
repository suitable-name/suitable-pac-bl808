#[doc = "Register `TCR2` reader"]
pub type R = crate::R<Tcr2Spec>;
#[doc = "Field `tcr2_cnt` reader - "]
pub type Tcr2CntR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn tcr2_cnt(&self) -> Tcr2CntR {
        Tcr2CntR::new(self.bits)
    }
}
#[doc = "Timer 2 Counter Value\n\nYou can [`read`](crate::Reg::read) this register and get [`tcr2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tcr2Spec;
impl crate::RegisterSpec for Tcr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tcr2::R`](R) reader structure"]
impl crate::Readable for Tcr2Spec {}
#[doc = "`reset()` method sets TCR2 to value 0"]
impl crate::Resettable for Tcr2Spec {}
