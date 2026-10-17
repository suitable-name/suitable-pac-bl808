#[doc = "Register `TZC_XRAM_TZSRG_R3` reader"]
pub type R = crate::R<TzcXramTzsrgR3Spec>;
#[doc = "XRAM Security Region 3 (Reserved/Dummy Register)\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_xram_tzsrg_r3::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TzcXramTzsrgR3Spec;
impl crate::RegisterSpec for TzcXramTzsrgR3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tzc_xram_tzsrg_r3::R`](R) reader structure"]
impl crate::Readable for TzcXramTzsrgR3Spec {}
#[doc = "`reset()` method sets TZC_XRAM_TZSRG_R3 to value 0"]
impl crate::Resettable for TzcXramTzsrgR3Spec {}
