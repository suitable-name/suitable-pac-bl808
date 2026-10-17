#[doc = "Register `TCVSYN2` reader"]
pub type R = crate::R<Tcvsyn2Spec>;
#[doc = "Field `tcr2_cnt_sync` reader - "]
pub type Tcr2CntSyncR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn tcr2_cnt_sync(&self) -> Tcr2CntSyncR {
        Tcr2CntSyncR::new(self.bits)
    }
}
#[doc = "Timer 2 Counter Sync Value\n\nYou can [`read`](crate::Reg::read) this register and get [`tcvsyn2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tcvsyn2Spec;
impl crate::RegisterSpec for Tcvsyn2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tcvsyn2::R`](R) reader structure"]
impl crate::Readable for Tcvsyn2Spec {}
#[doc = "`reset()` method sets TCVSYN2 to value 0"]
impl crate::Resettable for Tcvsyn2Spec {}
