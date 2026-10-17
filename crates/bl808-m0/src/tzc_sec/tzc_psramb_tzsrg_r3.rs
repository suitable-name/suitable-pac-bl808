#[doc = "Register `TZC_PSRAMB_TZSRG_R3` reader"]
pub type R = crate::R<TzcPsrambTzsrgR3Spec>;
#[doc = "PSRAM-B Security Region 3 (Reserved/Dummy Register)\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_psramb_tzsrg_r3::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TzcPsrambTzsrgR3Spec;
impl crate::RegisterSpec for TzcPsrambTzsrgR3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tzc_psramb_tzsrg_r3::R`](R) reader structure"]
impl crate::Readable for TzcPsrambTzsrgR3Spec {}
#[doc = "`reset()` method sets TZC_PSRAMB_TZSRG_R3 to value 0"]
impl crate::Resettable for TzcPsrambTzsrgR3Spec {}
