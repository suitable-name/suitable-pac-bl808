#[doc = "Register `TZC_PSRAMB_TZSRG_R3` reader"]
pub type R = crate::R<TzcPsrambTzsrgR3Spec>;
#[doc = "Register `TZC_PSRAMB_TZSRG_R3` writer"]
pub type W = crate::W<TzcPsrambTzsrgR3Spec>;
impl W {}
#[doc = "PSRAM-B TZSRG Region 3 (Reserved)\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_psramb_tzsrg_r3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tzc_psramb_tzsrg_r3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TzcPsrambTzsrgR3Spec;
impl crate::RegisterSpec for TzcPsrambTzsrgR3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tzc_psramb_tzsrg_r3::R`](R) reader structure"]
impl crate::Readable for TzcPsrambTzsrgR3Spec {}
#[doc = "`write(|w| ..)` method takes [`tzc_psramb_tzsrg_r3::W`](W) writer structure"]
impl crate::Writable for TzcPsrambTzsrgR3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TZC_PSRAMB_TZSRG_R3 to value 0"]
impl crate::Resettable for TzcPsrambTzsrgR3Spec {}
