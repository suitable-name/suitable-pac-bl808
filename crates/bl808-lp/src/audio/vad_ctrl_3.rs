#[doc = "Register `vad_ctrl_3` reader"]
pub type R = crate::R<VadCtrl3Spec>;
#[doc = "Register `vad_ctrl_3` writer"]
pub type W = crate::W<VadCtrl3Spec>;
#[doc = "Field `vad_smooth_fall` reader - "]
pub type VadSmoothFallR = crate::FieldReader;
#[doc = "Field `vad_smooth_fall` writer - "]
pub type VadSmoothFallW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `vad_smooth_rise` reader - "]
pub type VadSmoothRiseR = crate::FieldReader;
#[doc = "Field `vad_smooth_rise` writer - "]
pub type VadSmoothRiseW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn vad_smooth_fall(&self) -> VadSmoothFallR {
        VadSmoothFallR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn vad_smooth_rise(&self) -> VadSmoothRiseR {
        VadSmoothRiseR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn vad_smooth_fall(&mut self) -> VadSmoothFallW<'_, VadCtrl3Spec> {
        VadSmoothFallW::new(self, 0)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn vad_smooth_rise(&mut self) -> VadSmoothRiseW<'_, VadCtrl3Spec> {
        VadSmoothRiseW::new(self, 8)
    }
}
#[doc = "Voice Activity Detection Control 3: Smoothing factors\n\nYou can [`read`](crate::Reg::read) this register and get [`vad_ctrl_3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vad_ctrl_3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VadCtrl3Spec;
impl crate::RegisterSpec for VadCtrl3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vad_ctrl_3::R`](R) reader structure"]
impl crate::Readable for VadCtrl3Spec {}
#[doc = "`write(|w| ..)` method takes [`vad_ctrl_3::W`](W) writer structure"]
impl crate::Writable for VadCtrl3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets vad_ctrl_3 to value 0"]
impl crate::Resettable for VadCtrl3Spec {}
