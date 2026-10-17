#[doc = "Register `vad_ctrl_0` reader"]
pub type R = crate::R<VadCtrl0Spec>;
#[doc = "Register `vad_ctrl_0` writer"]
pub type W = crate::W<VadCtrl0Spec>;
#[doc = "Field `vad_nth_minlmt_en` reader - Enable VAD minimum threshold limit"]
pub type VadNthMinlmtEnR = crate::BitReader;
#[doc = "Field `vad_nth_minlmt_en` writer - Enable VAD minimum threshold limit"]
pub type VadNthMinlmtEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `vad_nth_maxlmt_en` reader - Enable VAD maximum threshold limit"]
pub type VadNthMaxlmtEnR = crate::BitReader;
#[doc = "Field `vad_nth_maxlmt_en` writer - Enable VAD maximum threshold limit"]
pub type VadNthMaxlmtEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `vad_nbuf_monit_en` reader - Enable VAD noise buffer monitoring"]
pub type VadNbufMonitEnR = crate::BitReader;
#[doc = "Field `vad_nbuf_monit_en` writer - Enable VAD noise buffer monitoring"]
pub type VadNbufMonitEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `vad_nbuf_monit_ctrl` reader - "]
pub type VadNbufMonitCtrlR = crate::FieldReader;
#[doc = "Field `vad_nbuf_monit_ctrl` writer - "]
pub type VadNbufMonitCtrlW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `vad_nbuf_len_ctrl` reader - "]
pub type VadNbufLenCtrlR = crate::FieldReader;
#[doc = "Field `vad_nbuf_len_ctrl` writer - "]
pub type VadNbufLenCtrlW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `vad_out_sel` reader - "]
pub type VadOutSelR = crate::FieldReader;
#[doc = "Field `vad_out_sel` writer - "]
pub type VadOutSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `vad_frametime_ctrl` reader - "]
pub type VadFrametimeCtrlR = crate::FieldReader;
#[doc = "Field `vad_frametime_ctrl` writer - "]
pub type VadFrametimeCtrlW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `vad_frameshift_ctrl` reader - "]
pub type VadFrameshiftCtrlR = crate::FieldReader;
#[doc = "Field `vad_frameshift_ctrl` writer - "]
pub type VadFrameshiftCtrlW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `vad_out` reader - VAD Decision Output"]
pub type VadOutR = crate::BitReader;
#[doc = "Field `vad_in_sel` reader - VAD Input Source Select"]
pub type VadInSelR = crate::FieldReader;
#[doc = "Field `vad_in_sel` writer - VAD Input Source Select"]
pub type VadInSelW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `vad_en` reader - Enable VAD Module"]
pub type VadEnR = crate::BitReader;
#[doc = "Field `vad_en` writer - Enable VAD Module"]
pub type VadEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable VAD minimum threshold limit"]
    #[inline(always)]
    pub fn vad_nth_minlmt_en(&self) -> VadNthMinlmtEnR {
        VadNthMinlmtEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable VAD maximum threshold limit"]
    #[inline(always)]
    pub fn vad_nth_maxlmt_en(&self) -> VadNthMaxlmtEnR {
        VadNthMaxlmtEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable VAD noise buffer monitoring"]
    #[inline(always)]
    pub fn vad_nbuf_monit_en(&self) -> VadNbufMonitEnR {
        VadNbufMonitEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn vad_nbuf_monit_ctrl(&self) -> VadNbufMonitCtrlR {
        VadNbufMonitCtrlR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn vad_nbuf_len_ctrl(&self) -> VadNbufLenCtrlR {
        VadNbufLenCtrlR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn vad_out_sel(&self) -> VadOutSelR {
        VadOutSelR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn vad_frametime_ctrl(&self) -> VadFrametimeCtrlR {
        VadFrametimeCtrlR::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:18"]
    #[inline(always)]
    pub fn vad_frameshift_ctrl(&self) -> VadFrameshiftCtrlR {
        VadFrameshiftCtrlR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 19 - VAD Decision Output"]
    #[inline(always)]
    pub fn vad_out(&self) -> VadOutR {
        VadOutR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:22 - VAD Input Source Select"]
    #[inline(always)]
    pub fn vad_in_sel(&self) -> VadInSelR {
        VadInSelR::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bit 23 - Enable VAD Module"]
    #[inline(always)]
    pub fn vad_en(&self) -> VadEnR {
        VadEnR::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable VAD minimum threshold limit"]
    #[inline(always)]
    pub fn vad_nth_minlmt_en(&mut self) -> VadNthMinlmtEnW<'_, VadCtrl0Spec> {
        VadNthMinlmtEnW::new(self, 0)
    }
    #[doc = "Bit 1 - Enable VAD maximum threshold limit"]
    #[inline(always)]
    pub fn vad_nth_maxlmt_en(&mut self) -> VadNthMaxlmtEnW<'_, VadCtrl0Spec> {
        VadNthMaxlmtEnW::new(self, 1)
    }
    #[doc = "Bit 2 - Enable VAD noise buffer monitoring"]
    #[inline(always)]
    pub fn vad_nbuf_monit_en(&mut self) -> VadNbufMonitEnW<'_, VadCtrl0Spec> {
        VadNbufMonitEnW::new(self, 2)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn vad_nbuf_monit_ctrl(&mut self) -> VadNbufMonitCtrlW<'_, VadCtrl0Spec> {
        VadNbufMonitCtrlW::new(self, 4)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn vad_nbuf_len_ctrl(&mut self) -> VadNbufLenCtrlW<'_, VadCtrl0Spec> {
        VadNbufLenCtrlW::new(self, 8)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn vad_out_sel(&mut self) -> VadOutSelW<'_, VadCtrl0Spec> {
        VadOutSelW::new(self, 12)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn vad_frametime_ctrl(&mut self) -> VadFrametimeCtrlW<'_, VadCtrl0Spec> {
        VadFrametimeCtrlW::new(self, 14)
    }
    #[doc = "Bits 16:18"]
    #[inline(always)]
    pub fn vad_frameshift_ctrl(&mut self) -> VadFrameshiftCtrlW<'_, VadCtrl0Spec> {
        VadFrameshiftCtrlW::new(self, 16)
    }
    #[doc = "Bits 20:22 - VAD Input Source Select"]
    #[inline(always)]
    pub fn vad_in_sel(&mut self) -> VadInSelW<'_, VadCtrl0Spec> {
        VadInSelW::new(self, 20)
    }
    #[doc = "Bit 23 - Enable VAD Module"]
    #[inline(always)]
    pub fn vad_en(&mut self) -> VadEnW<'_, VadCtrl0Spec> {
        VadEnW::new(self, 23)
    }
}
#[doc = "Voice Activity Detection Control 0\n\nYou can [`read`](crate::Reg::read) this register and get [`vad_ctrl_0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vad_ctrl_0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VadCtrl0Spec;
impl crate::RegisterSpec for VadCtrl0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vad_ctrl_0::R`](R) reader structure"]
impl crate::Readable for VadCtrl0Spec {}
#[doc = "`write(|w| ..)` method takes [`vad_ctrl_0::W`](W) writer structure"]
impl crate::Writable for VadCtrl0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets vad_ctrl_0 to value 0"]
impl crate::Resettable for VadCtrl0Spec {}
