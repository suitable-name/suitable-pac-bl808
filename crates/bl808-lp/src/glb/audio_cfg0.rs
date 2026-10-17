#[doc = "Register `AUDIO_CFG0` reader"]
pub type R = crate::R<AudioCfg0Spec>;
#[doc = "Register `AUDIO_CFG0` writer"]
pub type W = crate::W<AudioCfg0Spec>;
#[doc = "Field `reg_audio_pdm_clk_div` reader - "]
pub type RegAudioPdmClkDivR = crate::FieldReader;
#[doc = "Field `reg_audio_pdm_clk_div` writer - "]
pub type RegAudioPdmClkDivW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `reg_audio_pdm_clk_en` reader - "]
pub type RegAudioPdmClkEnR = crate::BitReader;
#[doc = "Field `reg_audio_pdm_clk_en` writer - "]
pub type RegAudioPdmClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_audio_adc_clk_div` reader - "]
pub type RegAudioAdcClkDivR = crate::FieldReader;
#[doc = "Field `reg_audio_adc_clk_div` writer - "]
pub type RegAudioAdcClkDivW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `reg_audio_adc_clk_en` reader - "]
pub type RegAudioAdcClkEnR = crate::BitReader;
#[doc = "Field `reg_audio_adc_clk_en` writer - "]
pub type RegAudioAdcClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_audio_dac_clk_div` reader - "]
pub type RegAudioDacClkDivR = crate::FieldReader;
#[doc = "Field `reg_audio_dac_clk_div` writer - "]
pub type RegAudioDacClkDivW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `reg_audio_dac_clk_en` reader - "]
pub type RegAudioDacClkEnR = crate::BitReader;
#[doc = "Field `reg_audio_dac_clk_en` writer - "]
pub type RegAudioDacClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_audio_auto_div_en` reader - "]
pub type RegAudioAutoDivEnR = crate::BitReader;
#[doc = "Field `reg_audio_auto_div_en` writer - "]
pub type RegAudioAutoDivEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn reg_audio_pdm_clk_div(&self) -> RegAudioPdmClkDivR {
        RegAudioPdmClkDivR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn reg_audio_pdm_clk_en(&self) -> RegAudioPdmClkEnR {
        RegAudioPdmClkEnR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:13"]
    #[inline(always)]
    pub fn reg_audio_adc_clk_div(&self) -> RegAudioAdcClkDivR {
        RegAudioAdcClkDivR::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn reg_audio_adc_clk_en(&self) -> RegAudioAdcClkEnR {
        RegAudioAdcClkEnR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:21"]
    #[inline(always)]
    pub fn reg_audio_dac_clk_div(&self) -> RegAudioDacClkDivR {
        RegAudioDacClkDivR::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn reg_audio_dac_clk_en(&self) -> RegAudioDacClkEnR {
        RegAudioDacClkEnR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn reg_audio_auto_div_en(&self) -> RegAudioAutoDivEnR {
        RegAudioAutoDivEnR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn reg_audio_pdm_clk_div(&mut self) -> RegAudioPdmClkDivW<'_, AudioCfg0Spec> {
        RegAudioPdmClkDivW::new(self, 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn reg_audio_pdm_clk_en(&mut self) -> RegAudioPdmClkEnW<'_, AudioCfg0Spec> {
        RegAudioPdmClkEnW::new(self, 7)
    }
    #[doc = "Bits 8:13"]
    #[inline(always)]
    pub fn reg_audio_adc_clk_div(&mut self) -> RegAudioAdcClkDivW<'_, AudioCfg0Spec> {
        RegAudioAdcClkDivW::new(self, 8)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn reg_audio_adc_clk_en(&mut self) -> RegAudioAdcClkEnW<'_, AudioCfg0Spec> {
        RegAudioAdcClkEnW::new(self, 15)
    }
    #[doc = "Bits 16:21"]
    #[inline(always)]
    pub fn reg_audio_dac_clk_div(&mut self) -> RegAudioDacClkDivW<'_, AudioCfg0Spec> {
        RegAudioDacClkDivW::new(self, 16)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn reg_audio_dac_clk_en(&mut self) -> RegAudioDacClkEnW<'_, AudioCfg0Spec> {
        RegAudioDacClkEnW::new(self, 23)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn reg_audio_auto_div_en(&mut self) -> RegAudioAutoDivEnW<'_, AudioCfg0Spec> {
        RegAudioAutoDivEnW::new(self, 31)
    }
}
#[doc = "Audio Clock Configuration 0 (PDM/ADC/DAC)\n\nYou can [`read`](crate::Reg::read) this register and get [`audio_cfg0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`audio_cfg0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AudioCfg0Spec;
impl crate::RegisterSpec for AudioCfg0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`audio_cfg0::R`](R) reader structure"]
impl crate::Readable for AudioCfg0Spec {}
#[doc = "`write(|w| ..)` method takes [`audio_cfg0::W`](W) writer structure"]
impl crate::Writable for AudioCfg0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AUDIO_CFG0 to value 0"]
impl crate::Resettable for AudioCfg0Spec {}
