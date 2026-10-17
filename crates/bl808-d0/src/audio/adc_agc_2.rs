#[doc = "Register `adc_agc_2` reader"]
pub type R = crate::R<AdcAgc2Spec>;
#[doc = "Register `adc_agc_2` writer"]
pub type W = crate::W<AdcAgc2Spec>;
#[doc = "Field `agc_noise_enter_time_ms` reader - "]
pub type AgcNoiseEnterTimeMsR = crate::FieldReader;
#[doc = "Field `agc_noise_enter_time_ms` writer - "]
pub type AgcNoiseEnterTimeMsW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `agc_silence_gain_wait_ms` reader - "]
pub type AgcSilenceGainWaitMsR = crate::FieldReader;
#[doc = "Field `agc_silence_gain_wait_ms` writer - "]
pub type AgcSilenceGainWaitMsW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `agc_silence_gain_qdb` reader - "]
pub type AgcSilenceGainQdbR = crate::FieldReader<u16>;
#[doc = "Field `agc_silence_gain_qdb` writer - "]
pub type AgcSilenceGainQdbW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `agc_noise_gate_hyst` reader - "]
pub type AgcNoiseGateHystR = crate::FieldReader;
#[doc = "Field `agc_noise_gate_hyst` writer - "]
pub type AgcNoiseGateHystW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `agc_noise_gate_th` reader - "]
pub type AgcNoiseGateThR = crate::FieldReader;
#[doc = "Field `agc_noise_gate_th` writer - "]
pub type AgcNoiseGateThW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:6"]
    #[inline(always)]
    pub fn agc_noise_enter_time_ms(&self) -> AgcNoiseEnterTimeMsR {
        AgcNoiseEnterTimeMsR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 7:10"]
    #[inline(always)]
    pub fn agc_silence_gain_wait_ms(&self) -> AgcSilenceGainWaitMsR {
        AgcSilenceGainWaitMsR::new(((self.bits >> 7) & 0x0f) as u8)
    }
    #[doc = "Bits 11:20"]
    #[inline(always)]
    pub fn agc_silence_gain_qdb(&self) -> AgcSilenceGainQdbR {
        AgcSilenceGainQdbR::new(((self.bits >> 11) & 0x03ff) as u16)
    }
    #[doc = "Bits 21:23"]
    #[inline(always)]
    pub fn agc_noise_gate_hyst(&self) -> AgcNoiseGateHystR {
        AgcNoiseGateHystR::new(((self.bits >> 21) & 7) as u8)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn agc_noise_gate_th(&self) -> AgcNoiseGateThR {
        AgcNoiseGateThR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6"]
    #[inline(always)]
    pub fn agc_noise_enter_time_ms(&mut self) -> AgcNoiseEnterTimeMsW<'_, AdcAgc2Spec> {
        AgcNoiseEnterTimeMsW::new(self, 0)
    }
    #[doc = "Bits 7:10"]
    #[inline(always)]
    pub fn agc_silence_gain_wait_ms(&mut self) -> AgcSilenceGainWaitMsW<'_, AdcAgc2Spec> {
        AgcSilenceGainWaitMsW::new(self, 7)
    }
    #[doc = "Bits 11:20"]
    #[inline(always)]
    pub fn agc_silence_gain_qdb(&mut self) -> AgcSilenceGainQdbW<'_, AdcAgc2Spec> {
        AgcSilenceGainQdbW::new(self, 11)
    }
    #[doc = "Bits 21:23"]
    #[inline(always)]
    pub fn agc_noise_gate_hyst(&mut self) -> AgcNoiseGateHystW<'_, AdcAgc2Spec> {
        AgcNoiseGateHystW::new(self, 21)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn agc_noise_gate_th(&mut self) -> AgcNoiseGateThW<'_, AdcAgc2Spec> {
        AgcNoiseGateThW::new(self, 24)
    }
}
#[doc = "ADC Automatic Gain Control 2: Noise gate and silence gain\n\nYou can [`read`](crate::Reg::read) this register and get [`adc_agc_2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc_agc_2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AdcAgc2Spec;
impl crate::RegisterSpec for AdcAgc2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adc_agc_2::R`](R) reader structure"]
impl crate::Readable for AdcAgc2Spec {}
#[doc = "`write(|w| ..)` method takes [`adc_agc_2::W`](W) writer structure"]
impl crate::Writable for AdcAgc2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets adc_agc_2 to value 0"]
impl crate::Resettable for AdcAgc2Spec {}
