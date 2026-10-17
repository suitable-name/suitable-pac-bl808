#[doc = "Register `TZC_L2SRAM_TZSRG_R3` reader"]
pub type R = crate::R<TzcL2sramTzsrgR3Spec>;
#[doc = "L2SRAM Security Region 3 (Reserved/Dummy Register)\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_l2sram_tzsrg_r3::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TzcL2sramTzsrgR3Spec;
impl crate::RegisterSpec for TzcL2sramTzsrgR3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tzc_l2sram_tzsrg_r3::R`](R) reader structure"]
impl crate::Readable for TzcL2sramTzsrgR3Spec {}
#[doc = "`reset()` method sets TZC_L2SRAM_TZSRG_R3 to value 0"]
impl crate::Resettable for TzcL2sramTzsrgR3Spec {}
