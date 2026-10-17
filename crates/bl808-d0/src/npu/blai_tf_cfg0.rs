#[doc = "Register `BLAI_TF_CFG0` reader"]
pub type R = crate::R<BlaiTfCfg0Spec>;
#[doc = "Register `BLAI_TF_CFG0` writer"]
pub type W = crate::W<BlaiTfCfg0Spec>;
#[doc = "Field `reg_tenflow_en` reader - TensorFlow compatibility enable"]
pub type RegTenflowEnR = crate::BitReader;
#[doc = "Field `reg_tenflow_en` writer - TensorFlow compatibility enable"]
pub type RegTenflowEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 31 - TensorFlow compatibility enable"]
    #[inline(always)]
    pub fn reg_tenflow_en(&self) -> RegTenflowEnR {
        RegTenflowEnR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - TensorFlow compatibility enable"]
    #[inline(always)]
    pub fn reg_tenflow_en(&mut self) -> RegTenflowEnW<'_, BlaiTfCfg0Spec> {
        RegTenflowEnW::new(self, 31)
    }
}
#[doc = "BLAI TensorFlow Compatibility Configuration 0\n\nYou can [`read`](crate::Reg::read) this register and get [`blai_tf_cfg0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blai_tf_cfg0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BlaiTfCfg0Spec;
impl crate::RegisterSpec for BlaiTfCfg0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`blai_tf_cfg0::R`](R) reader structure"]
impl crate::Readable for BlaiTfCfg0Spec {}
#[doc = "`write(|w| ..)` method takes [`blai_tf_cfg0::W`](W) writer structure"]
impl crate::Writable for BlaiTfCfg0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BLAI_TF_CFG0 to value 0"]
impl crate::Resettable for BlaiTfCfg0Spec {}
