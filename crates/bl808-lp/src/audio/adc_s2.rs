#[doc = "Register `adc_s2` reader"]
pub type R = crate::R<AdcS2Spec>;
#[doc = "Register `adc_s2` writer"]
pub type W = crate::W<AdcS2Spec>;
#[doc = "Field `adc_s2_ctrl_rmp_rate` reader - "]
pub type AdcS2CtrlRmpRateR = crate::FieldReader;
#[doc = "Field `adc_s2_ctrl_rmp_rate` writer - "]
pub type AdcS2CtrlRmpRateW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `adc_s2_ctrl_zcd_rate` reader - "]
pub type AdcS2CtrlZcdRateR = crate::FieldReader;
#[doc = "Field `adc_s2_ctrl_zcd_rate` writer - "]
pub type AdcS2CtrlZcdRateW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `adc_s2_ctrl_mode` reader - "]
pub type AdcS2CtrlModeR = crate::FieldReader;
#[doc = "Field `adc_s2_ctrl_mode` writer - "]
pub type AdcS2CtrlModeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `adc_s2_volume_update` reader - "]
pub type AdcS2VolumeUpdateR = crate::BitReader;
#[doc = "Field `adc_s2_volume_update` writer - "]
pub type AdcS2VolumeUpdateW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `adc_s2_volume` reader - "]
pub type AdcS2VolumeR = crate::FieldReader<u16>;
#[doc = "Field `adc_s2_volume` writer - "]
pub type AdcS2VolumeW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `adc_s2_mute_rmpup_rate` reader - "]
pub type AdcS2MuteRmpupRateR = crate::FieldReader;
#[doc = "Field `adc_s2_mute_rmpup_rate` writer - "]
pub type AdcS2MuteRmpupRateW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `adc_s2_mute_rmpdn_rate` reader - "]
pub type AdcS2MuteRmpdnRateR = crate::FieldReader;
#[doc = "Field `adc_s2_mute_rmpdn_rate` writer - "]
pub type AdcS2MuteRmpdnRateW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `adc_s2_mute_softmode` reader - "]
pub type AdcS2MuteSoftmodeR = crate::BitReader;
#[doc = "Field `adc_s2_mute_softmode` writer - "]
pub type AdcS2MuteSoftmodeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `adc_s2_mute` reader - "]
pub type AdcS2MuteR = crate::BitReader;
#[doc = "Field `adc_s2_mute` writer - "]
pub type AdcS2MuteW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 2:5"]
    #[inline(always)]
    pub fn adc_s2_ctrl_rmp_rate(&self) -> AdcS2CtrlRmpRateR {
        AdcS2CtrlRmpRateR::new(((self.bits >> 2) & 0x0f) as u8)
    }
    #[doc = "Bits 6:9"]
    #[inline(always)]
    pub fn adc_s2_ctrl_zcd_rate(&self) -> AdcS2CtrlZcdRateR {
        AdcS2CtrlZcdRateR::new(((self.bits >> 6) & 0x0f) as u8)
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn adc_s2_ctrl_mode(&self) -> AdcS2CtrlModeR {
        AdcS2CtrlModeR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn adc_s2_volume_update(&self) -> AdcS2VolumeUpdateR {
        AdcS2VolumeUpdateR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:21"]
    #[inline(always)]
    pub fn adc_s2_volume(&self) -> AdcS2VolumeR {
        AdcS2VolumeR::new(((self.bits >> 13) & 0x01ff) as u16)
    }
    #[doc = "Bits 22:25"]
    #[inline(always)]
    pub fn adc_s2_mute_rmpup_rate(&self) -> AdcS2MuteRmpupRateR {
        AdcS2MuteRmpupRateR::new(((self.bits >> 22) & 0x0f) as u8)
    }
    #[doc = "Bits 26:29"]
    #[inline(always)]
    pub fn adc_s2_mute_rmpdn_rate(&self) -> AdcS2MuteRmpdnRateR {
        AdcS2MuteRmpdnRateR::new(((self.bits >> 26) & 0x0f) as u8)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn adc_s2_mute_softmode(&self) -> AdcS2MuteSoftmodeR {
        AdcS2MuteSoftmodeR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn adc_s2_mute(&self) -> AdcS2MuteR {
        AdcS2MuteR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 2:5"]
    #[inline(always)]
    pub fn adc_s2_ctrl_rmp_rate(&mut self) -> AdcS2CtrlRmpRateW<'_, AdcS2Spec> {
        AdcS2CtrlRmpRateW::new(self, 2)
    }
    #[doc = "Bits 6:9"]
    #[inline(always)]
    pub fn adc_s2_ctrl_zcd_rate(&mut self) -> AdcS2CtrlZcdRateW<'_, AdcS2Spec> {
        AdcS2CtrlZcdRateW::new(self, 6)
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn adc_s2_ctrl_mode(&mut self) -> AdcS2CtrlModeW<'_, AdcS2Spec> {
        AdcS2CtrlModeW::new(self, 10)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn adc_s2_volume_update(&mut self) -> AdcS2VolumeUpdateW<'_, AdcS2Spec> {
        AdcS2VolumeUpdateW::new(self, 12)
    }
    #[doc = "Bits 13:21"]
    #[inline(always)]
    pub fn adc_s2_volume(&mut self) -> AdcS2VolumeW<'_, AdcS2Spec> {
        AdcS2VolumeW::new(self, 13)
    }
    #[doc = "Bits 22:25"]
    #[inline(always)]
    pub fn adc_s2_mute_rmpup_rate(&mut self) -> AdcS2MuteRmpupRateW<'_, AdcS2Spec> {
        AdcS2MuteRmpupRateW::new(self, 22)
    }
    #[doc = "Bits 26:29"]
    #[inline(always)]
    pub fn adc_s2_mute_rmpdn_rate(&mut self) -> AdcS2MuteRmpdnRateW<'_, AdcS2Spec> {
        AdcS2MuteRmpdnRateW::new(self, 26)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn adc_s2_mute_softmode(&mut self) -> AdcS2MuteSoftmodeW<'_, AdcS2Spec> {
        AdcS2MuteSoftmodeW::new(self, 30)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn adc_s2_mute(&mut self) -> AdcS2MuteW<'_, AdcS2Spec> {
        AdcS2MuteW::new(self, 31)
    }
}
#[doc = "ADC Sideband 2: Volume and mute control for secondary audio path\n\nYou can [`read`](crate::Reg::read) this register and get [`adc_s2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc_s2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AdcS2Spec;
impl crate::RegisterSpec for AdcS2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adc_s2::R`](R) reader structure"]
impl crate::Readable for AdcS2Spec {}
#[doc = "`write(|w| ..)` method takes [`adc_s2::W`](W) writer structure"]
impl crate::Writable for AdcS2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets adc_s2 to value 0"]
impl crate::Resettable for AdcS2Spec {}
