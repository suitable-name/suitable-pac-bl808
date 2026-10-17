#[doc = "Register `CONFIG0` reader"]
pub type R = crate::R<Config0Spec>;
#[doc = "Register `CONFIG0` writer"]
pub type W = crate::W<Config0Spec>;
#[doc = "Field `pwm_clk_div` reader - Clock divider for the PWM group"]
pub type PwmClkDivR = crate::FieldReader<u16>;
#[doc = "Field `pwm_clk_div` writer - Clock divider for the PWM group"]
pub type PwmClkDivW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `pwm_stop_on_rept` reader - Stop PWM output after repeat count reached"]
pub type PwmStopOnReptR = crate::BitReader;
#[doc = "Field `pwm_stop_on_rept` writer - Stop PWM output after repeat count reached"]
pub type PwmStopOnReptW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pwm_adc_trg_src` reader - ADC trigger source selection"]
pub type PwmAdcTrgSrcR = crate::FieldReader;
#[doc = "Field `pwm_adc_trg_src` writer - ADC trigger source selection"]
pub type PwmAdcTrgSrcW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `pwm_sw_break_en` reader - Enable software break"]
pub type PwmSwBreakEnR = crate::BitReader;
#[doc = "Field `pwm_sw_break_en` writer - Enable software break"]
pub type PwmSwBreakEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pwm_ext_break_en` reader - Enable external break"]
pub type PwmExtBreakEnR = crate::BitReader;
#[doc = "Field `pwm_ext_break_en` writer - Enable external break"]
pub type PwmExtBreakEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pwm_ext_break_pl` reader - External break polarity"]
pub type PwmExtBreakPlR = crate::BitReader;
#[doc = "Field `pwm_ext_break_pl` writer - External break polarity"]
pub type PwmExtBreakPlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pwm_stop_en` reader - Enable PWM stop"]
pub type PwmStopEnR = crate::BitReader;
#[doc = "Field `pwm_stop_en` writer - Enable PWM stop"]
pub type PwmStopEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pwm_stop_mode` reader - PWM stop mode selection"]
pub type PwmStopModeR = crate::BitReader;
#[doc = "Field `pwm_stop_mode` writer - PWM stop mode selection"]
pub type PwmStopModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pwm_sts_stop` reader - PWM stop status"]
pub type PwmStsStopR = crate::BitReader;
#[doc = "Field `reg_clk_sel` reader - PWM clock source selection"]
pub type RegClkSelR = crate::FieldReader;
#[doc = "Field `reg_clk_sel` writer - PWM clock source selection"]
pub type RegClkSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:15 - Clock divider for the PWM group"]
    #[inline(always)]
    pub fn pwm_clk_div(&self) -> PwmClkDivR {
        PwmClkDivR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 19 - Stop PWM output after repeat count reached"]
    #[inline(always)]
    pub fn pwm_stop_on_rept(&self) -> PwmStopOnReptR {
        PwmStopOnReptR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:23 - ADC trigger source selection"]
    #[inline(always)]
    pub fn pwm_adc_trg_src(&self) -> PwmAdcTrgSrcR {
        PwmAdcTrgSrcR::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - Enable software break"]
    #[inline(always)]
    pub fn pwm_sw_break_en(&self) -> PwmSwBreakEnR {
        PwmSwBreakEnR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Enable external break"]
    #[inline(always)]
    pub fn pwm_ext_break_en(&self) -> PwmExtBreakEnR {
        PwmExtBreakEnR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - External break polarity"]
    #[inline(always)]
    pub fn pwm_ext_break_pl(&self) -> PwmExtBreakPlR {
        PwmExtBreakPlR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Enable PWM stop"]
    #[inline(always)]
    pub fn pwm_stop_en(&self) -> PwmStopEnR {
        PwmStopEnR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - PWM stop mode selection"]
    #[inline(always)]
    pub fn pwm_stop_mode(&self) -> PwmStopModeR {
        PwmStopModeR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - PWM stop status"]
    #[inline(always)]
    pub fn pwm_sts_stop(&self) -> PwmStsStopR {
        PwmStsStopR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bits 30:31 - PWM clock source selection"]
    #[inline(always)]
    pub fn reg_clk_sel(&self) -> RegClkSelR {
        RegClkSelR::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15 - Clock divider for the PWM group"]
    #[inline(always)]
    pub fn pwm_clk_div(&mut self) -> PwmClkDivW<'_, Config0Spec> {
        PwmClkDivW::new(self, 0)
    }
    #[doc = "Bit 19 - Stop PWM output after repeat count reached"]
    #[inline(always)]
    pub fn pwm_stop_on_rept(&mut self) -> PwmStopOnReptW<'_, Config0Spec> {
        PwmStopOnReptW::new(self, 19)
    }
    #[doc = "Bits 20:23 - ADC trigger source selection"]
    #[inline(always)]
    pub fn pwm_adc_trg_src(&mut self) -> PwmAdcTrgSrcW<'_, Config0Spec> {
        PwmAdcTrgSrcW::new(self, 20)
    }
    #[doc = "Bit 24 - Enable software break"]
    #[inline(always)]
    pub fn pwm_sw_break_en(&mut self) -> PwmSwBreakEnW<'_, Config0Spec> {
        PwmSwBreakEnW::new(self, 24)
    }
    #[doc = "Bit 25 - Enable external break"]
    #[inline(always)]
    pub fn pwm_ext_break_en(&mut self) -> PwmExtBreakEnW<'_, Config0Spec> {
        PwmExtBreakEnW::new(self, 25)
    }
    #[doc = "Bit 26 - External break polarity"]
    #[inline(always)]
    pub fn pwm_ext_break_pl(&mut self) -> PwmExtBreakPlW<'_, Config0Spec> {
        PwmExtBreakPlW::new(self, 26)
    }
    #[doc = "Bit 27 - Enable PWM stop"]
    #[inline(always)]
    pub fn pwm_stop_en(&mut self) -> PwmStopEnW<'_, Config0Spec> {
        PwmStopEnW::new(self, 27)
    }
    #[doc = "Bit 28 - PWM stop mode selection"]
    #[inline(always)]
    pub fn pwm_stop_mode(&mut self) -> PwmStopModeW<'_, Config0Spec> {
        PwmStopModeW::new(self, 28)
    }
    #[doc = "Bits 30:31 - PWM clock source selection"]
    #[inline(always)]
    pub fn reg_clk_sel(&mut self) -> RegClkSelW<'_, Config0Spec> {
        RegClkSelW::new(self, 30)
    }
}
#[doc = "PWM Configuration Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`config0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`config0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Config0Spec;
impl crate::RegisterSpec for Config0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`config0::R`](R) reader structure"]
impl crate::Readable for Config0Spec {}
#[doc = "`write(|w| ..)` method takes [`config0::W`](W) writer structure"]
impl crate::Writable for Config0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CONFIG0 to value 0x10f0_0000"]
impl crate::Resettable for Config0Spec {
    const RESET_VALUE: u32 = 0x10f0_0000;
}
