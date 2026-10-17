#[doc = "Register `TZC_MM_CTRL_1` reader"]
pub type R = crate::R<TzcMmCtrl1Spec>;
#[doc = "Register `TZC_MM_CTRL_1` writer"]
pub type W = crate::W<TzcMmCtrl1Spec>;
impl W {}
#[doc = "MM Control 1 (Reserved)\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_mm_ctrl_1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tzc_mm_ctrl_1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TzcMmCtrl1Spec;
impl crate::RegisterSpec for TzcMmCtrl1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tzc_mm_ctrl_1::R`](R) reader structure"]
impl crate::Readable for TzcMmCtrl1Spec {}
#[doc = "`write(|w| ..)` method takes [`tzc_mm_ctrl_1::W`](W) writer structure"]
impl crate::Writable for TzcMmCtrl1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TZC_MM_CTRL_1 to value 0"]
impl crate::Resettable for TzcMmCtrl1Spec {}
