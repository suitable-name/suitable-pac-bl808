#[doc = "Register `TZC_OCRAM_TZSRG_R3` reader"]
pub type R = crate::R<TzcOcramTzsrgR3Spec>;
#[doc = "OCRAM Security Region 3 (Reserved/Dummy Register)\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_ocram_tzsrg_r3::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TzcOcramTzsrgR3Spec;
impl crate::RegisterSpec for TzcOcramTzsrgR3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tzc_ocram_tzsrg_r3::R`](R) reader structure"]
impl crate::Readable for TzcOcramTzsrgR3Spec {}
#[doc = "`reset()` method sets TZC_OCRAM_TZSRG_R3 to value 0"]
impl crate::Resettable for TzcOcramTzsrgR3Spec {}
