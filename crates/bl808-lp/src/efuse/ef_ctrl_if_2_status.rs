#[doc = "Register `EF_CTRL_IF_2_STATUS` reader"]
pub type R = crate::R<EfCtrlIf2StatusSpec>;
#[doc = "Register `EF_CTRL_IF_2_STATUS` writer"]
pub type W = crate::W<EfCtrlIf2StatusSpec>;
impl W {}
#[doc = "eFuse Interface 2 Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ef_ctrl_if_2_status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ef_ctrl_if_2_status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EfCtrlIf2StatusSpec;
impl crate::RegisterSpec for EfCtrlIf2StatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ef_ctrl_if_2_status::R`](R) reader structure"]
impl crate::Readable for EfCtrlIf2StatusSpec {}
#[doc = "`write(|w| ..)` method takes [`ef_ctrl_if_2_status::W`](W) writer structure"]
impl crate::Writable for EfCtrlIf2StatusSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EF_CTRL_IF_2_STATUS to value 0"]
impl crate::Resettable for EfCtrlIf2StatusSpec {}
