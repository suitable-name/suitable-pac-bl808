#[doc = "Register `TCVWR3` reader"]
pub type R = crate::R<Tcvwr3Spec>;
#[doc = "Field `tcr3_cnt_lat` reader - "]
pub type Tcr3CntLatR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn tcr3_cnt_lat(&self) -> Tcr3CntLatR {
        Tcr3CntLatR::new(self.bits)
    }
}
#[doc = "Timer 3 Counter Latch Value\n\nYou can [`read`](crate::Reg::read) this register and get [`tcvwr3::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tcvwr3Spec;
impl crate::RegisterSpec for Tcvwr3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tcvwr3::R`](R) reader structure"]
impl crate::Readable for Tcvwr3Spec {}
#[doc = "`reset()` method sets TCVWR3 to value 0"]
impl crate::Resettable for Tcvwr3Spec {}
