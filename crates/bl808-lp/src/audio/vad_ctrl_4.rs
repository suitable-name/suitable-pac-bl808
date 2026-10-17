#[doc = "Register `vad_ctrl_4` reader"]
pub type R = crate::R<VadCtrl4Spec>;
#[doc = "Field `vad_ek` reader - VAD EK Estimation"]
pub type VadEkR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:23 - VAD EK Estimation"]
    #[inline(always)]
    pub fn vad_ek(&self) -> VadEkR {
        VadEkR::new(self.bits & 0x00ff_ffff)
    }
}
#[doc = "Voice Activity Detection Control 4: EK Estimation status\n\nYou can [`read`](crate::Reg::read) this register and get [`vad_ctrl_4::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VadCtrl4Spec;
impl crate::RegisterSpec for VadCtrl4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vad_ctrl_4::R`](R) reader structure"]
impl crate::Readable for VadCtrl4Spec {}
#[doc = "`reset()` method sets vad_ctrl_4 to value 0"]
impl crate::Resettable for VadCtrl4Spec {}
