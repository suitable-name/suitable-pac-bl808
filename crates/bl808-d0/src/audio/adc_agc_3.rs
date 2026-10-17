#[doc = "Register `adc_agc_3` reader"]
pub type R = crate::R<AdcAgc3Spec>;
#[doc = "Register `adc_agc_3` writer"]
pub type W = crate::W<AdcAgc3Spec>;
#[doc = "Field `agc_pga_ctrl_en` reader - "]
pub type AgcPgaCtrlEnR = crate::BitReader;
#[doc = "Field `agc_pga_ctrl_en` writer - "]
pub type AgcPgaCtrlEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `agc_average_time` reader - "]
pub type AgcAverageTimeR = crate::FieldReader;
#[doc = "Field `agc_average_time` writer - "]
pub type AgcAverageTimeW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `agc_release_time` reader - "]
pub type AgcReleaseTimeR = crate::FieldReader;
#[doc = "Field `agc_release_time` writer - "]
pub type AgcReleaseTimeW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `agc_attack_time` reader - "]
pub type AgcAttackTimeR = crate::FieldReader;
#[doc = "Field `agc_attack_time` writer - "]
pub type AgcAttackTimeW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `agc_detect_mode` reader - "]
pub type AgcDetectModeR = crate::BitReader;
#[doc = "Field `agc_detect_mode` writer - "]
pub type AgcDetectModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `agc_src_select` reader - "]
pub type AgcSrcSelectR = crate::FieldReader;
#[doc = "Field `agc_src_select` writer - "]
pub type AgcSrcSelectW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `agc_dbv_settle_time` reader - "]
pub type AgcDbvSettleTimeR = crate::FieldReader;
#[doc = "Field `agc_dbv_settle_time` writer - "]
pub type AgcDbvSettleTimeW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `agc_noise_exit_time_ms` reader - "]
pub type AgcNoiseExitTimeMsR = crate::FieldReader;
#[doc = "Field `agc_noise_exit_time_ms` writer - "]
pub type AgcNoiseExitTimeMsW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn agc_pga_ctrl_en(&self) -> AgcPgaCtrlEnR {
        AgcPgaCtrlEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:7"]
    #[inline(always)]
    pub fn agc_average_time(&self) -> AgcAverageTimeR {
        AgcAverageTimeR::new(((self.bits >> 3) & 0x1f) as u8)
    }
    #[doc = "Bits 8:12"]
    #[inline(always)]
    pub fn agc_release_time(&self) -> AgcReleaseTimeR {
        AgcReleaseTimeR::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 13:17"]
    #[inline(always)]
    pub fn agc_attack_time(&self) -> AgcAttackTimeR {
        AgcAttackTimeR::new(((self.bits >> 13) & 0x1f) as u8)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn agc_detect_mode(&self) -> AgcDetectModeR {
        AgcDetectModeR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 19:20"]
    #[inline(always)]
    pub fn agc_src_select(&self) -> AgcSrcSelectR {
        AgcSrcSelectR::new(((self.bits >> 19) & 3) as u8)
    }
    #[doc = "Bits 21:24"]
    #[inline(always)]
    pub fn agc_dbv_settle_time(&self) -> AgcDbvSettleTimeR {
        AgcDbvSettleTimeR::new(((self.bits >> 21) & 0x0f) as u8)
    }
    #[doc = "Bits 25:31"]
    #[inline(always)]
    pub fn agc_noise_exit_time_ms(&self) -> AgcNoiseExitTimeMsR {
        AgcNoiseExitTimeMsR::new(((self.bits >> 25) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn agc_pga_ctrl_en(&mut self) -> AgcPgaCtrlEnW<'_, AdcAgc3Spec> {
        AgcPgaCtrlEnW::new(self, 2)
    }
    #[doc = "Bits 3:7"]
    #[inline(always)]
    pub fn agc_average_time(&mut self) -> AgcAverageTimeW<'_, AdcAgc3Spec> {
        AgcAverageTimeW::new(self, 3)
    }
    #[doc = "Bits 8:12"]
    #[inline(always)]
    pub fn agc_release_time(&mut self) -> AgcReleaseTimeW<'_, AdcAgc3Spec> {
        AgcReleaseTimeW::new(self, 8)
    }
    #[doc = "Bits 13:17"]
    #[inline(always)]
    pub fn agc_attack_time(&mut self) -> AgcAttackTimeW<'_, AdcAgc3Spec> {
        AgcAttackTimeW::new(self, 13)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn agc_detect_mode(&mut self) -> AgcDetectModeW<'_, AdcAgc3Spec> {
        AgcDetectModeW::new(self, 18)
    }
    #[doc = "Bits 19:20"]
    #[inline(always)]
    pub fn agc_src_select(&mut self) -> AgcSrcSelectW<'_, AdcAgc3Spec> {
        AgcSrcSelectW::new(self, 19)
    }
    #[doc = "Bits 21:24"]
    #[inline(always)]
    pub fn agc_dbv_settle_time(&mut self) -> AgcDbvSettleTimeW<'_, AdcAgc3Spec> {
        AgcDbvSettleTimeW::new(self, 21)
    }
    #[doc = "Bits 25:31"]
    #[inline(always)]
    pub fn agc_noise_exit_time_ms(&mut self) -> AgcNoiseExitTimeMsW<'_, AdcAgc3Spec> {
        AgcNoiseExitTimeMsW::new(self, 25)
    }
}
#[doc = "ADC Automatic Gain Control 3: Detection mode and noise exit timing\n\nYou can [`read`](crate::Reg::read) this register and get [`adc_agc_3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc_agc_3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AdcAgc3Spec;
impl crate::RegisterSpec for AdcAgc3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adc_agc_3::R`](R) reader structure"]
impl crate::Readable for AdcAgc3Spec {}
#[doc = "`write(|w| ..)` method takes [`adc_agc_3::W`](W) writer structure"]
impl crate::Writable for AdcAgc3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets adc_agc_3 to value 0"]
impl crate::Resettable for AdcAgc3Spec {}
