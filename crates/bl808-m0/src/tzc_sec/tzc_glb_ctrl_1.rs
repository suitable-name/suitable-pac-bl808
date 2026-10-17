#[doc = "Register `TZC_GLB_CTRL_1` reader"]
pub type R = crate::R<TzcGlbCtrl1Spec>;
#[doc = "Global TrustZone Control Register 1 (Reserved)\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_glb_ctrl_1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TzcGlbCtrl1Spec;
impl crate::RegisterSpec for TzcGlbCtrl1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tzc_glb_ctrl_1::R`](R) reader structure"]
impl crate::Readable for TzcGlbCtrl1Spec {}
#[doc = "`reset()` method sets TZC_GLB_CTRL_1 to value 0"]
impl crate::Resettable for TzcGlbCtrl1Spec {}
