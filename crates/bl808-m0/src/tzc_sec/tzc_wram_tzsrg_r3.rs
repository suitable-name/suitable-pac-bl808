#[doc = "Register `TZC_WRAM_TZSRG_R3` reader"]
pub type R = crate::R<TzcWramTzsrgR3Spec>;
#[doc = "WRAM Security Region 3 (Reserved/Dummy Register)\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_wram_tzsrg_r3::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TzcWramTzsrgR3Spec;
impl crate::RegisterSpec for TzcWramTzsrgR3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tzc_wram_tzsrg_r3::R`](R) reader structure"]
impl crate::Readable for TzcWramTzsrgR3Spec {}
#[doc = "`reset()` method sets TZC_WRAM_TZSRG_R3 to value 0"]
impl crate::Resettable for TzcWramTzsrgR3Spec {}
