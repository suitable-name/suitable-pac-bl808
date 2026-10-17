#[doc = "Register `DEBUG_CFG1` reader"]
pub type R = crate::R<DebugCfg1Spec>;
#[doc = "Register `DEBUG_CFG1` writer"]
pub type W = crate::W<DebugCfg1Spec>;
#[doc = "Field `debug_ndreset_gate` reader - "]
pub type DebugNdresetGateR = crate::BitReader;
#[doc = "Field `debug_ndreset_gate` writer - "]
pub type DebugNdresetGateW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn debug_ndreset_gate(&self) -> DebugNdresetGateR {
        DebugNdresetGateR::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn debug_ndreset_gate(&mut self) -> DebugNdresetGateW<'_, DebugCfg1Spec> {
        DebugNdresetGateW::new(self, 20)
    }
}
#[doc = "Debug Configuration 1 (NDReset Gate)\n\nYou can [`read`](crate::Reg::read) this register and get [`debug_cfg1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`debug_cfg1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DebugCfg1Spec;
impl crate::RegisterSpec for DebugCfg1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`debug_cfg1::R`](R) reader structure"]
impl crate::Readable for DebugCfg1Spec {}
#[doc = "`write(|w| ..)` method takes [`debug_cfg1::W`](W) writer structure"]
impl crate::Writable for DebugCfg1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DEBUG_CFG1 to value 0"]
impl crate::Resettable for DebugCfg1Spec {}
