#[doc = "Register `EF_CTRL_IF_CTRL_2` reader"]
pub type R = crate::R<EfCtrlIfCtrl2Spec>;
#[doc = "Register `EF_CTRL_IF_CTRL_2` writer"]
pub type W = crate::W<EfCtrlIfCtrl2Spec>;
impl W {}
#[doc = "eFuse Interface 2 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ef_ctrl_if_ctrl_2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ef_ctrl_if_ctrl_2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EfCtrlIfCtrl2Spec;
impl crate::RegisterSpec for EfCtrlIfCtrl2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ef_ctrl_if_ctrl_2::R`](R) reader structure"]
impl crate::Readable for EfCtrlIfCtrl2Spec {}
#[doc = "`write(|w| ..)` method takes [`ef_ctrl_if_ctrl_2::W`](W) writer structure"]
impl crate::Writable for EfCtrlIfCtrl2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EF_CTRL_IF_CTRL_2 to value 0"]
impl crate::Resettable for EfCtrlIfCtrl2Spec {}
