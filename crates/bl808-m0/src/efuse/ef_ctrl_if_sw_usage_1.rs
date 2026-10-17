#[doc = "Register `EF_CTRL_IF_SW_USAGE_1` reader"]
pub type R = crate::R<EfCtrlIfSwUsage1Spec>;
#[doc = "Register `EF_CTRL_IF_SW_USAGE_1` writer"]
pub type W = crate::W<EfCtrlIfSwUsage1Spec>;
#[doc = "Field `ef_if_sw_usage_1` reader - "]
pub type EfIfSwUsage1R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn ef_if_sw_usage_1(&self) -> EfIfSwUsage1R {
        EfIfSwUsage1R::new(self.bits)
    }
}
impl W {}
#[doc = "eFuse Software Usage Register 1 (Read from array)\n\nYou can [`read`](crate::Reg::read) this register and get [`ef_ctrl_if_sw_usage_1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ef_ctrl_if_sw_usage_1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EfCtrlIfSwUsage1Spec;
impl crate::RegisterSpec for EfCtrlIfSwUsage1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ef_ctrl_if_sw_usage_1::R`](R) reader structure"]
impl crate::Readable for EfCtrlIfSwUsage1Spec {}
#[doc = "`write(|w| ..)` method takes [`ef_ctrl_if_sw_usage_1::W`](W) writer structure"]
impl crate::Writable for EfCtrlIfSwUsage1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EF_CTRL_IF_SW_USAGE_1 to value 0"]
impl crate::Resettable for EfCtrlIfSwUsage1Spec {}
