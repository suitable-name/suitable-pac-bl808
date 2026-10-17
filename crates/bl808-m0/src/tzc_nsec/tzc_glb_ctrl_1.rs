#[doc = "Register `TZC_GLB_CTRL_1` reader"]
pub type R = crate::R<TzcGlbCtrl1Spec>;
#[doc = "Register `TZC_GLB_CTRL_1` writer"]
pub type W = crate::W<TzcGlbCtrl1Spec>;
impl W {}
#[doc = "Global Control 1 (Reserved)\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_glb_ctrl_1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tzc_glb_ctrl_1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TzcGlbCtrl1Spec;
impl crate::RegisterSpec for TzcGlbCtrl1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tzc_glb_ctrl_1::R`](R) reader structure"]
impl crate::Readable for TzcGlbCtrl1Spec {}
#[doc = "`write(|w| ..)` method takes [`tzc_glb_ctrl_1::W`](W) writer structure"]
impl crate::Writable for TzcGlbCtrl1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TZC_GLB_CTRL_1 to value 0"]
impl crate::Resettable for TzcGlbCtrl1Spec {}
