#[doc = "Register `EF_CTRL_IF_2_MANUAL` reader"]
pub type R = crate::R<EfCtrlIf2ManualSpec>;
#[doc = "Register `EF_CTRL_IF_2_MANUAL` writer"]
pub type W = crate::W<EfCtrlIf2ManualSpec>;
impl W {}
#[doc = "eFuse Interface 2 Manual Operation Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ef_ctrl_if_2_manual::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ef_ctrl_if_2_manual::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EfCtrlIf2ManualSpec;
impl crate::RegisterSpec for EfCtrlIf2ManualSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ef_ctrl_if_2_manual::R`](R) reader structure"]
impl crate::Readable for EfCtrlIf2ManualSpec {}
#[doc = "`write(|w| ..)` method takes [`ef_ctrl_if_2_manual::W`](W) writer structure"]
impl crate::Writable for EfCtrlIf2ManualSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EF_CTRL_IF_2_MANUAL to value 0"]
impl crate::Resettable for EfCtrlIf2ManualSpec {}
