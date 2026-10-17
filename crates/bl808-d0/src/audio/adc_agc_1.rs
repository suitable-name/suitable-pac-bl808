#[doc = "Register `adc_agc_1` reader"]
pub type R = crate::R<AdcAgc1Spec>;
#[doc = "Register `adc_agc_1` writer"]
pub type W = crate::W<AdcAgc1Spec>;
#[doc = "Field `adc0_pga_hw_ctrl` reader - "]
pub type Adc0PgaHwCtrlR = crate::BitReader;
#[doc = "Field `adc0_pga_hw_ctrl` writer - "]
pub type Adc0PgaHwCtrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `adc1_pga_hw_ctrl` reader - "]
pub type Adc1PgaHwCtrlR = crate::BitReader;
#[doc = "Field `adc1_pga_hw_ctrl` writer - "]
pub type Adc1PgaHwCtrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `adc2_pga_hw_ctrl` reader - "]
pub type Adc2PgaHwCtrlR = crate::BitReader;
#[doc = "Field `adc2_pga_hw_ctrl` writer - "]
pub type Adc2PgaHwCtrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `agc_attack_time_ms` reader - "]
pub type AgcAttackTimeMsR = crate::FieldReader;
#[doc = "Field `agc_attack_time_ms` writer - "]
pub type AgcAttackTimeMsW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `agc_decay_time_ms` reader - "]
pub type AgcDecayTimeMsR = crate::FieldReader;
#[doc = "Field `agc_decay_time_ms` writer - "]
pub type AgcDecayTimeMsW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `agc_hold_time_ms` reader - "]
pub type AgcHoldTimeMsR = crate::FieldReader;
#[doc = "Field `agc_hold_time_ms` writer - "]
pub type AgcHoldTimeMsW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `agc_level_hyst_decay` reader - "]
pub type AgcLevelHystDecayR = crate::FieldReader;
#[doc = "Field `agc_level_hyst_decay` writer - "]
pub type AgcLevelHystDecayW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `agc_level_hyst_attack` reader - "]
pub type AgcLevelHystAttackR = crate::FieldReader;
#[doc = "Field `agc_level_hyst_attack` writer - "]
pub type AgcLevelHystAttackW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `agc_level_target` reader - "]
pub type AgcLevelTargetR = crate::FieldReader;
#[doc = "Field `agc_level_target` writer - "]
pub type AgcLevelTargetW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn adc0_pga_hw_ctrl(&self) -> Adc0PgaHwCtrlR {
        Adc0PgaHwCtrlR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn adc1_pga_hw_ctrl(&self) -> Adc1PgaHwCtrlR {
        Adc1PgaHwCtrlR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn adc2_pga_hw_ctrl(&self) -> Adc2PgaHwCtrlR {
        Adc2PgaHwCtrlR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn agc_attack_time_ms(&self) -> AgcAttackTimeMsR {
        AgcAttackTimeMsR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn agc_decay_time_ms(&self) -> AgcDecayTimeMsR {
        AgcDecayTimeMsR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn agc_hold_time_ms(&self) -> AgcHoldTimeMsR {
        AgcHoldTimeMsR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:20"]
    #[inline(always)]
    pub fn agc_level_hyst_decay(&self) -> AgcLevelHystDecayR {
        AgcLevelHystDecayR::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 21:25"]
    #[inline(always)]
    pub fn agc_level_hyst_attack(&self) -> AgcLevelHystAttackR {
        AgcLevelHystAttackR::new(((self.bits >> 21) & 0x1f) as u8)
    }
    #[doc = "Bits 26:31"]
    #[inline(always)]
    pub fn agc_level_target(&self) -> AgcLevelTargetR {
        AgcLevelTargetR::new(((self.bits >> 26) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn adc0_pga_hw_ctrl(&mut self) -> Adc0PgaHwCtrlW<'_, AdcAgc1Spec> {
        Adc0PgaHwCtrlW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn adc1_pga_hw_ctrl(&mut self) -> Adc1PgaHwCtrlW<'_, AdcAgc1Spec> {
        Adc1PgaHwCtrlW::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn adc2_pga_hw_ctrl(&mut self) -> Adc2PgaHwCtrlW<'_, AdcAgc1Spec> {
        Adc2PgaHwCtrlW::new(self, 2)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn agc_attack_time_ms(&mut self) -> AgcAttackTimeMsW<'_, AdcAgc1Spec> {
        AgcAttackTimeMsW::new(self, 4)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn agc_decay_time_ms(&mut self) -> AgcDecayTimeMsW<'_, AdcAgc1Spec> {
        AgcDecayTimeMsW::new(self, 8)
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn agc_hold_time_ms(&mut self) -> AgcHoldTimeMsW<'_, AdcAgc1Spec> {
        AgcHoldTimeMsW::new(self, 12)
    }
    #[doc = "Bits 16:20"]
    #[inline(always)]
    pub fn agc_level_hyst_decay(&mut self) -> AgcLevelHystDecayW<'_, AdcAgc1Spec> {
        AgcLevelHystDecayW::new(self, 16)
    }
    #[doc = "Bits 21:25"]
    #[inline(always)]
    pub fn agc_level_hyst_attack(&mut self) -> AgcLevelHystAttackW<'_, AdcAgc1Spec> {
        AgcLevelHystAttackW::new(self, 21)
    }
    #[doc = "Bits 26:31"]
    #[inline(always)]
    pub fn agc_level_target(&mut self) -> AgcLevelTargetW<'_, AdcAgc1Spec> {
        AgcLevelTargetW::new(self, 26)
    }
}
#[doc = "ADC Automatic Gain Control 1: Attack, decay, and hold timing\n\nYou can [`read`](crate::Reg::read) this register and get [`adc_agc_1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc_agc_1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AdcAgc1Spec;
impl crate::RegisterSpec for AdcAgc1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adc_agc_1::R`](R) reader structure"]
impl crate::Readable for AdcAgc1Spec {}
#[doc = "`write(|w| ..)` method takes [`adc_agc_1::W`](W) writer structure"]
impl crate::Writable for AdcAgc1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets adc_agc_1 to value 0"]
impl crate::Resettable for AdcAgc1Spec {}
