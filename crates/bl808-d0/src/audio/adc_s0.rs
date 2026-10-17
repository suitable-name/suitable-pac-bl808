#[doc = "Register `adc_s0` reader"]
pub type R = crate::R<AdcS0Spec>;
#[doc = "Register `adc_s0` writer"]
pub type W = crate::W<AdcS0Spec>;
#[doc = "Field `adc_s0_ctrl_rmp_rate` reader - "]
pub type AdcS0CtrlRmpRateR = crate::FieldReader;
#[doc = "Field `adc_s0_ctrl_rmp_rate` writer - "]
pub type AdcS0CtrlRmpRateW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `adc_s0_ctrl_zcd_rate` reader - "]
pub type AdcS0CtrlZcdRateR = crate::FieldReader;
#[doc = "Field `adc_s0_ctrl_zcd_rate` writer - "]
pub type AdcS0CtrlZcdRateW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `adc_s0_ctrl_mode` reader - "]
pub type AdcS0CtrlModeR = crate::FieldReader;
#[doc = "Field `adc_s0_ctrl_mode` writer - "]
pub type AdcS0CtrlModeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `adc_s0_volume_update` reader - "]
pub type AdcS0VolumeUpdateR = crate::BitReader;
#[doc = "Field `adc_s0_volume_update` writer - "]
pub type AdcS0VolumeUpdateW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `adc_s0_volume` reader - "]
pub type AdcS0VolumeR = crate::FieldReader<u16>;
#[doc = "Field `adc_s0_volume` writer - "]
pub type AdcS0VolumeW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `adc_s0_mute_rmpup_rate` reader - "]
pub type AdcS0MuteRmpupRateR = crate::FieldReader;
#[doc = "Field `adc_s0_mute_rmpup_rate` writer - "]
pub type AdcS0MuteRmpupRateW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `adc_s0_mute_rmpdn_rate` reader - "]
pub type AdcS0MuteRmpdnRateR = crate::FieldReader;
#[doc = "Field `adc_s0_mute_rmpdn_rate` writer - "]
pub type AdcS0MuteRmpdnRateW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `adc_s0_mute_softmode` reader - "]
pub type AdcS0MuteSoftmodeR = crate::BitReader;
#[doc = "Field `adc_s0_mute_softmode` writer - "]
pub type AdcS0MuteSoftmodeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `adc_s0_mute` reader - "]
pub type AdcS0MuteR = crate::BitReader;
#[doc = "Field `adc_s0_mute` writer - "]
pub type AdcS0MuteW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 2:5"]
    #[inline(always)]
    pub fn adc_s0_ctrl_rmp_rate(&self) -> AdcS0CtrlRmpRateR {
        AdcS0CtrlRmpRateR::new(((self.bits >> 2) & 0x0f) as u8)
    }
    #[doc = "Bits 6:9"]
    #[inline(always)]
    pub fn adc_s0_ctrl_zcd_rate(&self) -> AdcS0CtrlZcdRateR {
        AdcS0CtrlZcdRateR::new(((self.bits >> 6) & 0x0f) as u8)
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn adc_s0_ctrl_mode(&self) -> AdcS0CtrlModeR {
        AdcS0CtrlModeR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn adc_s0_volume_update(&self) -> AdcS0VolumeUpdateR {
        AdcS0VolumeUpdateR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:21"]
    #[inline(always)]
    pub fn adc_s0_volume(&self) -> AdcS0VolumeR {
        AdcS0VolumeR::new(((self.bits >> 13) & 0x01ff) as u16)
    }
    #[doc = "Bits 22:25"]
    #[inline(always)]
    pub fn adc_s0_mute_rmpup_rate(&self) -> AdcS0MuteRmpupRateR {
        AdcS0MuteRmpupRateR::new(((self.bits >> 22) & 0x0f) as u8)
    }
    #[doc = "Bits 26:29"]
    #[inline(always)]
    pub fn adc_s0_mute_rmpdn_rate(&self) -> AdcS0MuteRmpdnRateR {
        AdcS0MuteRmpdnRateR::new(((self.bits >> 26) & 0x0f) as u8)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn adc_s0_mute_softmode(&self) -> AdcS0MuteSoftmodeR {
        AdcS0MuteSoftmodeR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn adc_s0_mute(&self) -> AdcS0MuteR {
        AdcS0MuteR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 2:5"]
    #[inline(always)]
    pub fn adc_s0_ctrl_rmp_rate(&mut self) -> AdcS0CtrlRmpRateW<'_, AdcS0Spec> {
        AdcS0CtrlRmpRateW::new(self, 2)
    }
    #[doc = "Bits 6:9"]
    #[inline(always)]
    pub fn adc_s0_ctrl_zcd_rate(&mut self) -> AdcS0CtrlZcdRateW<'_, AdcS0Spec> {
        AdcS0CtrlZcdRateW::new(self, 6)
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn adc_s0_ctrl_mode(&mut self) -> AdcS0CtrlModeW<'_, AdcS0Spec> {
        AdcS0CtrlModeW::new(self, 10)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn adc_s0_volume_update(&mut self) -> AdcS0VolumeUpdateW<'_, AdcS0Spec> {
        AdcS0VolumeUpdateW::new(self, 12)
    }
    #[doc = "Bits 13:21"]
    #[inline(always)]
    pub fn adc_s0_volume(&mut self) -> AdcS0VolumeW<'_, AdcS0Spec> {
        AdcS0VolumeW::new(self, 13)
    }
    #[doc = "Bits 22:25"]
    #[inline(always)]
    pub fn adc_s0_mute_rmpup_rate(&mut self) -> AdcS0MuteRmpupRateW<'_, AdcS0Spec> {
        AdcS0MuteRmpupRateW::new(self, 22)
    }
    #[doc = "Bits 26:29"]
    #[inline(always)]
    pub fn adc_s0_mute_rmpdn_rate(&mut self) -> AdcS0MuteRmpdnRateW<'_, AdcS0Spec> {
        AdcS0MuteRmpdnRateW::new(self, 26)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn adc_s0_mute_softmode(&mut self) -> AdcS0MuteSoftmodeW<'_, AdcS0Spec> {
        AdcS0MuteSoftmodeW::new(self, 30)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn adc_s0_mute(&mut self) -> AdcS0MuteW<'_, AdcS0Spec> {
        AdcS0MuteW::new(self, 31)
    }
}
#[doc = "ADC Sideband 0: Volume and mute control for secondary audio path\n\nYou can [`read`](crate::Reg::read) this register and get [`adc_s0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc_s0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AdcS0Spec;
impl crate::RegisterSpec for AdcS0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adc_s0::R`](R) reader structure"]
impl crate::Readable for AdcS0Spec {}
#[doc = "`write(|w| ..)` method takes [`adc_s0::W`](W) writer structure"]
impl crate::Writable for AdcS0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets adc_s0 to value 0"]
impl crate::Resettable for AdcS0Spec {}
