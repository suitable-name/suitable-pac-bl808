#[doc = "Register `adc_s1` reader"]
pub type R = crate::R<AdcS1Spec>;
#[doc = "Register `adc_s1` writer"]
pub type W = crate::W<AdcS1Spec>;
#[doc = "Field `adc_s1_ctrl_rmp_rate` reader - "]
pub type AdcS1CtrlRmpRateR = crate::FieldReader;
#[doc = "Field `adc_s1_ctrl_rmp_rate` writer - "]
pub type AdcS1CtrlRmpRateW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `adc_s1_ctrl_zcd_rate` reader - "]
pub type AdcS1CtrlZcdRateR = crate::FieldReader;
#[doc = "Field `adc_s1_ctrl_zcd_rate` writer - "]
pub type AdcS1CtrlZcdRateW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `adc_s1_ctrl_mode` reader - "]
pub type AdcS1CtrlModeR = crate::FieldReader;
#[doc = "Field `adc_s1_ctrl_mode` writer - "]
pub type AdcS1CtrlModeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `adc_s1_volume_update` reader - "]
pub type AdcS1VolumeUpdateR = crate::BitReader;
#[doc = "Field `adc_s1_volume_update` writer - "]
pub type AdcS1VolumeUpdateW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `adc_s1_volume` reader - "]
pub type AdcS1VolumeR = crate::FieldReader<u16>;
#[doc = "Field `adc_s1_volume` writer - "]
pub type AdcS1VolumeW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `adc_s1_mute_rmpup_rate` reader - "]
pub type AdcS1MuteRmpupRateR = crate::FieldReader;
#[doc = "Field `adc_s1_mute_rmpup_rate` writer - "]
pub type AdcS1MuteRmpupRateW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `adc_s1_mute_rmpdn_rate` reader - "]
pub type AdcS1MuteRmpdnRateR = crate::FieldReader;
#[doc = "Field `adc_s1_mute_rmpdn_rate` writer - "]
pub type AdcS1MuteRmpdnRateW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `adc_s1_mute_softmode` reader - "]
pub type AdcS1MuteSoftmodeR = crate::BitReader;
#[doc = "Field `adc_s1_mute_softmode` writer - "]
pub type AdcS1MuteSoftmodeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `adc_s1_mute` reader - "]
pub type AdcS1MuteR = crate::BitReader;
#[doc = "Field `adc_s1_mute` writer - "]
pub type AdcS1MuteW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 2:5"]
    #[inline(always)]
    pub fn adc_s1_ctrl_rmp_rate(&self) -> AdcS1CtrlRmpRateR {
        AdcS1CtrlRmpRateR::new(((self.bits >> 2) & 0x0f) as u8)
    }
    #[doc = "Bits 6:9"]
    #[inline(always)]
    pub fn adc_s1_ctrl_zcd_rate(&self) -> AdcS1CtrlZcdRateR {
        AdcS1CtrlZcdRateR::new(((self.bits >> 6) & 0x0f) as u8)
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn adc_s1_ctrl_mode(&self) -> AdcS1CtrlModeR {
        AdcS1CtrlModeR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn adc_s1_volume_update(&self) -> AdcS1VolumeUpdateR {
        AdcS1VolumeUpdateR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:21"]
    #[inline(always)]
    pub fn adc_s1_volume(&self) -> AdcS1VolumeR {
        AdcS1VolumeR::new(((self.bits >> 13) & 0x01ff) as u16)
    }
    #[doc = "Bits 22:25"]
    #[inline(always)]
    pub fn adc_s1_mute_rmpup_rate(&self) -> AdcS1MuteRmpupRateR {
        AdcS1MuteRmpupRateR::new(((self.bits >> 22) & 0x0f) as u8)
    }
    #[doc = "Bits 26:29"]
    #[inline(always)]
    pub fn adc_s1_mute_rmpdn_rate(&self) -> AdcS1MuteRmpdnRateR {
        AdcS1MuteRmpdnRateR::new(((self.bits >> 26) & 0x0f) as u8)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn adc_s1_mute_softmode(&self) -> AdcS1MuteSoftmodeR {
        AdcS1MuteSoftmodeR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn adc_s1_mute(&self) -> AdcS1MuteR {
        AdcS1MuteR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 2:5"]
    #[inline(always)]
    pub fn adc_s1_ctrl_rmp_rate(&mut self) -> AdcS1CtrlRmpRateW<'_, AdcS1Spec> {
        AdcS1CtrlRmpRateW::new(self, 2)
    }
    #[doc = "Bits 6:9"]
    #[inline(always)]
    pub fn adc_s1_ctrl_zcd_rate(&mut self) -> AdcS1CtrlZcdRateW<'_, AdcS1Spec> {
        AdcS1CtrlZcdRateW::new(self, 6)
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn adc_s1_ctrl_mode(&mut self) -> AdcS1CtrlModeW<'_, AdcS1Spec> {
        AdcS1CtrlModeW::new(self, 10)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn adc_s1_volume_update(&mut self) -> AdcS1VolumeUpdateW<'_, AdcS1Spec> {
        AdcS1VolumeUpdateW::new(self, 12)
    }
    #[doc = "Bits 13:21"]
    #[inline(always)]
    pub fn adc_s1_volume(&mut self) -> AdcS1VolumeW<'_, AdcS1Spec> {
        AdcS1VolumeW::new(self, 13)
    }
    #[doc = "Bits 22:25"]
    #[inline(always)]
    pub fn adc_s1_mute_rmpup_rate(&mut self) -> AdcS1MuteRmpupRateW<'_, AdcS1Spec> {
        AdcS1MuteRmpupRateW::new(self, 22)
    }
    #[doc = "Bits 26:29"]
    #[inline(always)]
    pub fn adc_s1_mute_rmpdn_rate(&mut self) -> AdcS1MuteRmpdnRateW<'_, AdcS1Spec> {
        AdcS1MuteRmpdnRateW::new(self, 26)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn adc_s1_mute_softmode(&mut self) -> AdcS1MuteSoftmodeW<'_, AdcS1Spec> {
        AdcS1MuteSoftmodeW::new(self, 30)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn adc_s1_mute(&mut self) -> AdcS1MuteW<'_, AdcS1Spec> {
        AdcS1MuteW::new(self, 31)
    }
}
#[doc = "ADC Sideband 1: Volume and mute control for secondary audio path\n\nYou can [`read`](crate::Reg::read) this register and get [`adc_s1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc_s1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AdcS1Spec;
impl crate::RegisterSpec for AdcS1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adc_s1::R`](R) reader structure"]
impl crate::Readable for AdcS1Spec {}
#[doc = "`write(|w| ..)` method takes [`adc_s1::W`](W) writer structure"]
impl crate::Writable for AdcS1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets adc_s1 to value 0"]
impl crate::Resettable for AdcS1Spec {}
