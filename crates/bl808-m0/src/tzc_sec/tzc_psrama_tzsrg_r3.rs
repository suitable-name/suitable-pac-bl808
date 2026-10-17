#[doc = "Register `TZC_PSRAMA_TZSRG_R3` reader"]
pub type R = crate::R<TzcPsramaTzsrgR3Spec>;
#[doc = "PSRAM-A Security Region 3 (Reserved/Dummy Register)\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_psrama_tzsrg_r3::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TzcPsramaTzsrgR3Spec;
impl crate::RegisterSpec for TzcPsramaTzsrgR3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tzc_psrama_tzsrg_r3::R`](R) reader structure"]
impl crate::Readable for TzcPsramaTzsrgR3Spec {}
#[doc = "`reset()` method sets TZC_PSRAMA_TZSRG_R3 to value 0"]
impl crate::Resettable for TzcPsramaTzsrgR3Spec {}
