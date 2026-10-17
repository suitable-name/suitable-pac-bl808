#[doc = "Register `EF_CTRL_IF_SW_USAGE_0` reader"]
pub type R = crate::R<EfCtrlIfSwUsage0Spec>;
#[doc = "Register `EF_CTRL_IF_SW_USAGE_0` writer"]
pub type W = crate::W<EfCtrlIfSwUsage0Spec>;
#[doc = "Field `ef_if_sw_usage_0` reader - "]
pub type EfIfSwUsage0R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn ef_if_sw_usage_0(&self) -> EfIfSwUsage0R {
        EfIfSwUsage0R::new(self.bits)
    }
}
impl W {}
#[doc = "eFuse Software Usage Register 0 (Read from array)\n\nYou can [`read`](crate::Reg::read) this register and get [`ef_ctrl_if_sw_usage_0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ef_ctrl_if_sw_usage_0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EfCtrlIfSwUsage0Spec;
impl crate::RegisterSpec for EfCtrlIfSwUsage0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ef_ctrl_if_sw_usage_0::R`](R) reader structure"]
impl crate::Readable for EfCtrlIfSwUsage0Spec {}
#[doc = "`write(|w| ..)` method takes [`ef_ctrl_if_sw_usage_0::W`](W) writer structure"]
impl crate::Writable for EfCtrlIfSwUsage0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EF_CTRL_IF_SW_USAGE_0 to value 0"]
impl crate::Resettable for EfCtrlIfSwUsage0Spec {}
