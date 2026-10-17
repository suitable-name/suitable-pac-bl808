#[doc = "Register `TZC_MM_CTRL_1` reader"]
pub type R = crate::R<TzcMmCtrl1Spec>;
#[doc = "Multimedia TrustZone Control Register 1 (Reserved)\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_mm_ctrl_1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TzcMmCtrl1Spec;
impl crate::RegisterSpec for TzcMmCtrl1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tzc_mm_ctrl_1::R`](R) reader structure"]
impl crate::Readable for TzcMmCtrl1Spec {}
#[doc = "`reset()` method sets TZC_MM_CTRL_1 to value 0"]
impl crate::Resettable for TzcMmCtrl1Spec {}
