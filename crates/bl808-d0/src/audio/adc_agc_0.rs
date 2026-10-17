#[doc = "Register `adc_agc_0` reader"]
pub type R = crate::R<AdcAgc0Spec>;
#[doc = "Register `adc_agc_0` writer"]
pub type W = crate::W<AdcAgc0Spec>;
#[doc = "Field `adc_agc_en` reader - "]
pub type AdcAgcEnR = crate::BitReader;
#[doc = "Field `adc_agc_en` writer - "]
pub type AdcAgcEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `agc_min_gain_qdb` reader - "]
pub type AgcMinGainQdbR = crate::FieldReader<u16>;
#[doc = "Field `agc_min_gain_qdb` writer - "]
pub type AgcMinGainQdbW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `agc_max_gain_qdb` reader - "]
pub type AgcMaxGainQdbR = crate::FieldReader<u16>;
#[doc = "Field `agc_max_gain_qdb` writer - "]
pub type AgcMaxGainQdbW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `agc_force_gain_ind` reader - "]
pub type AgcForceGainIndR = crate::FieldReader<u16>;
#[doc = "Field `agc_force_gain_ind` writer - "]
pub type AgcForceGainIndW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `agc_force_gain_en` reader - "]
pub type AgcForceGainEnR = crate::BitReader;
#[doc = "Field `agc_force_gain_en` writer - "]
pub type AgcForceGainEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn adc_agc_en(&self) -> AdcAgcEnR {
        AdcAgcEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 3:11"]
    #[inline(always)]
    pub fn agc_min_gain_qdb(&self) -> AgcMinGainQdbR {
        AgcMinGainQdbR::new(((self.bits >> 3) & 0x01ff) as u16)
    }
    #[doc = "Bits 12:20"]
    #[inline(always)]
    pub fn agc_max_gain_qdb(&self) -> AgcMaxGainQdbR {
        AgcMaxGainQdbR::new(((self.bits >> 12) & 0x01ff) as u16)
    }
    #[doc = "Bits 21:30"]
    #[inline(always)]
    pub fn agc_force_gain_ind(&self) -> AgcForceGainIndR {
        AgcForceGainIndR::new(((self.bits >> 21) & 0x03ff) as u16)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn agc_force_gain_en(&self) -> AgcForceGainEnR {
        AgcForceGainEnR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn adc_agc_en(&mut self) -> AdcAgcEnW<'_, AdcAgc0Spec> {
        AdcAgcEnW::new(self, 0)
    }
    #[doc = "Bits 3:11"]
    #[inline(always)]
    pub fn agc_min_gain_qdb(&mut self) -> AgcMinGainQdbW<'_, AdcAgc0Spec> {
        AgcMinGainQdbW::new(self, 3)
    }
    #[doc = "Bits 12:20"]
    #[inline(always)]
    pub fn agc_max_gain_qdb(&mut self) -> AgcMaxGainQdbW<'_, AdcAgc0Spec> {
        AgcMaxGainQdbW::new(self, 12)
    }
    #[doc = "Bits 21:30"]
    #[inline(always)]
    pub fn agc_force_gain_ind(&mut self) -> AgcForceGainIndW<'_, AdcAgc0Spec> {
        AgcForceGainIndW::new(self, 21)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn agc_force_gain_en(&mut self) -> AgcForceGainEnW<'_, AdcAgc0Spec> {
        AgcForceGainEnW::new(self, 31)
    }
}
#[doc = "ADC Automatic Gain Control 0: Thresholds and forced gain management\n\nYou can [`read`](crate::Reg::read) this register and get [`adc_agc_0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc_agc_0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AdcAgc0Spec;
impl crate::RegisterSpec for AdcAgc0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adc_agc_0::R`](R) reader structure"]
impl crate::Readable for AdcAgc0Spec {}
#[doc = "`write(|w| ..)` method takes [`adc_agc_0::W`](W) writer structure"]
impl crate::Writable for AdcAgc0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets adc_agc_0 to value 0"]
impl crate::Resettable for AdcAgc0Spec {}
