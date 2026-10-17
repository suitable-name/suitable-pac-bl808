#[doc = "Register `TZC_ROM_TZSRG_R3` reader"]
pub type R = crate::R<TzcRomTzsrgR3Spec>;
#[doc = "ROM Security Region 3 (Reserved/Dummy Register)\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_rom_tzsrg_r3::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TzcRomTzsrgR3Spec;
impl crate::RegisterSpec for TzcRomTzsrgR3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tzc_rom_tzsrg_r3::R`](R) reader structure"]
impl crate::Readable for TzcRomTzsrgR3Spec {}
#[doc = "`reset()` method sets TZC_ROM_TZSRG_R3 to value 0"]
impl crate::Resettable for TzcRomTzsrgR3Spec {}
