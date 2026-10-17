#[doc = "Register `DEBUG_CFG0` reader"]
pub type R = crate::R<DebugCfg0Spec>;
#[doc = "Register `DEBUG_CFG0` writer"]
pub type W = crate::W<DebugCfg0Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Debug Configuration 0\n\nYou can [`read`](crate::Reg::read) this register and get [`debug_cfg0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`debug_cfg0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DebugCfg0Spec;
impl crate::RegisterSpec for DebugCfg0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`debug_cfg0::R`](R) reader structure"]
impl crate::Readable for DebugCfg0Spec {}
#[doc = "`write(|w| ..)` method takes [`debug_cfg0::W`](W) writer structure"]
impl crate::Writable for DebugCfg0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DEBUG_CFG0 to value 0"]
impl crate::Resettable for DebugCfg0Spec {}
