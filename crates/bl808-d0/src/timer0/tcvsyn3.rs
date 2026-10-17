#[doc = "Register `TCVSYN3` reader"]
pub type R = crate::R<Tcvsyn3Spec>;
#[doc = "Field `tcr3_cnt_sync` reader - "]
pub type Tcr3CntSyncR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn tcr3_cnt_sync(&self) -> Tcr3CntSyncR {
        Tcr3CntSyncR::new(self.bits)
    }
}
#[doc = "Timer 3 Counter Sync Value\n\nYou can [`read`](crate::Reg::read) this register and get [`tcvsyn3::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tcvsyn3Spec;
impl crate::RegisterSpec for Tcvsyn3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tcvsyn3::R`](R) reader structure"]
impl crate::Readable for Tcvsyn3Spec {}
#[doc = "`reset()` method sets TCVSYN3 to value 0"]
impl crate::Resettable for Tcvsyn3Spec {}
