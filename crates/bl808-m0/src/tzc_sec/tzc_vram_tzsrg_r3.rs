#[doc = "Register `TZC_VRAM_TZSRG_R3` reader"]
pub type R = crate::R<TzcVramTzsrgR3Spec>;
#[doc = "VRAM Security Region 3 (Reserved/Dummy Register)\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_vram_tzsrg_r3::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TzcVramTzsrgR3Spec;
impl crate::RegisterSpec for TzcVramTzsrgR3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tzc_vram_tzsrg_r3::R`](R) reader structure"]
impl crate::Readable for TzcVramTzsrgR3Spec {}
#[doc = "`reset()` method sets TZC_VRAM_TZSRG_R3 to value 0"]
impl crate::Resettable for TzcVramTzsrgR3Spec {}
