#[doc = "Register `vad_ctrl_1` reader"]
pub type R = crate::R<VadCtrl1Spec>;
#[doc = "Register `vad_ctrl_1` writer"]
pub type W = crate::W<VadCtrl1Spec>;
#[doc = "Field `vad_nth_scale` reader - "]
pub type VadNthScaleR = crate::FieldReader;
#[doc = "Field `vad_nth_scale` writer - "]
pub type VadNthScaleW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `vad_nth_bias` reader - "]
pub type VadNthBiasR = crate::FieldReader<u16>;
#[doc = "Field `vad_nth_bias` writer - "]
pub type VadNthBiasW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `vad_nbuf_discard` reader - "]
pub type VadNbufDiscardR = crate::FieldReader;
#[doc = "Field `vad_nbuf_discard` writer - "]
pub type VadNbufDiscardW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn vad_nth_scale(&self) -> VadNthScaleR {
        VadNthScaleR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:23"]
    #[inline(always)]
    pub fn vad_nth_bias(&self) -> VadNthBiasR {
        VadNthBiasR::new(((self.bits >> 8) & 0xffff) as u16)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn vad_nbuf_discard(&self) -> VadNbufDiscardR {
        VadNbufDiscardR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn vad_nth_scale(&mut self) -> VadNthScaleW<'_, VadCtrl1Spec> {
        VadNthScaleW::new(self, 0)
    }
    #[doc = "Bits 8:23"]
    #[inline(always)]
    pub fn vad_nth_bias(&mut self) -> VadNthBiasW<'_, VadCtrl1Spec> {
        VadNthBiasW::new(self, 8)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn vad_nbuf_discard(&mut self) -> VadNbufDiscardW<'_, VadCtrl1Spec> {
        VadNbufDiscardW::new(self, 24)
    }
}
#[doc = "Voice Activity Detection Control 1: Scaling and bias\n\nYou can [`read`](crate::Reg::read) this register and get [`vad_ctrl_1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vad_ctrl_1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VadCtrl1Spec;
impl crate::RegisterSpec for VadCtrl1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vad_ctrl_1::R`](R) reader structure"]
impl crate::Readable for VadCtrl1Spec {}
#[doc = "`write(|w| ..)` method takes [`vad_ctrl_1::W`](W) writer structure"]
impl crate::Writable for VadCtrl1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets vad_ctrl_1 to value 0"]
impl crate::Resettable for VadCtrl1Spec {}
