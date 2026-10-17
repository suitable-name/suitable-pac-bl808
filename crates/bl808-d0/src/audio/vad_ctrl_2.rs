#[doc = "Register `vad_ctrl_2` reader"]
pub type R = crate::R<VadCtrl2Spec>;
#[doc = "Register `vad_ctrl_2` writer"]
pub type W = crate::W<VadCtrl2Spec>;
#[doc = "Field `vad_nth_minlmt` reader - "]
pub type VadNthMinlmtR = crate::FieldReader<u16>;
#[doc = "Field `vad_nth_minlmt` writer - "]
pub type VadNthMinlmtW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `vad_nth_maxlmt` reader - "]
pub type VadNthMaxlmtR = crate::FieldReader<u16>;
#[doc = "Field `vad_nth_maxlmt` writer - "]
pub type VadNthMaxlmtW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn vad_nth_minlmt(&self) -> VadNthMinlmtR {
        VadNthMinlmtR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn vad_nth_maxlmt(&self) -> VadNthMaxlmtR {
        VadNthMaxlmtR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn vad_nth_minlmt(&mut self) -> VadNthMinlmtW<'_, VadCtrl2Spec> {
        VadNthMinlmtW::new(self, 0)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn vad_nth_maxlmt(&mut self) -> VadNthMaxlmtW<'_, VadCtrl2Spec> {
        VadNthMaxlmtW::new(self, 16)
    }
}
#[doc = "Voice Activity Detection Control 2: Threshold limits\n\nYou can [`read`](crate::Reg::read) this register and get [`vad_ctrl_2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vad_ctrl_2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VadCtrl2Spec;
impl crate::RegisterSpec for VadCtrl2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vad_ctrl_2::R`](R) reader structure"]
impl crate::Readable for VadCtrl2Spec {}
#[doc = "`write(|w| ..)` method takes [`vad_ctrl_2::W`](W) writer structure"]
impl crate::Writable for VadCtrl2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets vad_ctrl_2 to value 0"]
impl crate::Resettable for VadCtrl2Spec {}
