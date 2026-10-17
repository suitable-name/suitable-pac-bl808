#[doc = "Register `vad_ctrl_5` reader"]
pub type R = crate::R<VadCtrl5Spec>;
#[doc = "Field `vad_noise_th` reader - VAD Noise Threshold Status"]
pub type VadNoiseThR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:23 - VAD Noise Threshold Status"]
    #[inline(always)]
    pub fn vad_noise_th(&self) -> VadNoiseThR {
        VadNoiseThR::new(self.bits & 0x00ff_ffff)
    }
}
#[doc = "Voice Activity Detection Control 5: Noise threshold status\n\nYou can [`read`](crate::Reg::read) this register and get [`vad_ctrl_5::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VadCtrl5Spec;
impl crate::RegisterSpec for VadCtrl5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vad_ctrl_5::R`](R) reader structure"]
impl crate::Readable for VadCtrl5Spec {}
#[doc = "`reset()` method sets vad_ctrl_5 to value 0"]
impl crate::Resettable for VadCtrl5Spec {}
