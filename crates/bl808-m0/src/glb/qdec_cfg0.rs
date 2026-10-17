#[doc = "Register `QDEC_CFG0` reader"]
pub type R = crate::R<QdecCfg0Spec>;
#[doc = "Register `QDEC_CFG0` writer"]
pub type W = crate::W<QdecCfg0Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "QDEC Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`qdec_cfg0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qdec_cfg0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct QdecCfg0Spec;
impl crate::RegisterSpec for QdecCfg0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`qdec_cfg0::R`](R) reader structure"]
impl crate::Readable for QdecCfg0Spec {}
#[doc = "`write(|w| ..)` method takes [`qdec_cfg0::W`](W) writer structure"]
impl crate::Writable for QdecCfg0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets QDEC_CFG0 to value 0"]
impl crate::Resettable for QdecCfg0Spec {}
