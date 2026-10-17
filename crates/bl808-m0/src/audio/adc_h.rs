#[doc = "Register `adc_h` reader"]
pub type R = crate::R<AdcHSpec>;
#[doc = "Register `adc_h` writer"]
pub type W = crate::W<AdcHSpec>;
#[doc = "Field `adc_h_ctrl_rmp_rate` reader - "]
pub type AdcHCtrlRmpRateR = crate::FieldReader;
#[doc = "Field `adc_h_ctrl_rmp_rate` writer - "]
pub type AdcHCtrlRmpRateW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `adc_h_ctrl_zcd_rate` reader - "]
pub type AdcHCtrlZcdRateR = crate::FieldReader;
#[doc = "Field `adc_h_ctrl_zcd_rate` writer - "]
pub type AdcHCtrlZcdRateW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `adc_h_ctrl_mode` reader - "]
pub type AdcHCtrlModeR = crate::FieldReader;
#[doc = "Field `adc_h_ctrl_mode` writer - "]
pub type AdcHCtrlModeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `adc_h_volume_update` reader - "]
pub type AdcHVolumeUpdateR = crate::BitReader;
#[doc = "Field `adc_h_volume_update` writer - "]
pub type AdcHVolumeUpdateW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `adc_h_volume` reader - "]
pub type AdcHVolumeR = crate::FieldReader<u16>;
#[doc = "Field `adc_h_mute_rmpup_rate` reader - "]
pub type AdcHMuteRmpupRateR = crate::FieldReader;
#[doc = "Field `adc_h_mute_rmpup_rate` writer - "]
pub type AdcHMuteRmpupRateW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `adc_h_mute_rmpdn_rate` reader - "]
pub type AdcHMuteRmpdnRateR = crate::FieldReader;
#[doc = "Field `adc_h_mute_rmpdn_rate` writer - "]
pub type AdcHMuteRmpdnRateW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `adc_h_mute_softmode` reader - "]
pub type AdcHMuteSoftmodeR = crate::BitReader;
#[doc = "Field `adc_h_mute_softmode` writer - "]
pub type AdcHMuteSoftmodeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `adc_h_mute` reader - "]
pub type AdcHMuteR = crate::BitReader;
#[doc = "Field `adc_h_mute` writer - "]
pub type AdcHMuteW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 2:5"]
    #[inline(always)]
    pub fn adc_h_ctrl_rmp_rate(&self) -> AdcHCtrlRmpRateR {
        AdcHCtrlRmpRateR::new(((self.bits >> 2) & 0x0f) as u8)
    }
    #[doc = "Bits 6:9"]
    #[inline(always)]
    pub fn adc_h_ctrl_zcd_rate(&self) -> AdcHCtrlZcdRateR {
        AdcHCtrlZcdRateR::new(((self.bits >> 6) & 0x0f) as u8)
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn adc_h_ctrl_mode(&self) -> AdcHCtrlModeR {
        AdcHCtrlModeR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn adc_h_volume_update(&self) -> AdcHVolumeUpdateR {
        AdcHVolumeUpdateR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:21"]
    #[inline(always)]
    pub fn adc_h_volume(&self) -> AdcHVolumeR {
        AdcHVolumeR::new(((self.bits >> 13) & 0x01ff) as u16)
    }
    #[doc = "Bits 22:25"]
    #[inline(always)]
    pub fn adc_h_mute_rmpup_rate(&self) -> AdcHMuteRmpupRateR {
        AdcHMuteRmpupRateR::new(((self.bits >> 22) & 0x0f) as u8)
    }
    #[doc = "Bits 26:29"]
    #[inline(always)]
    pub fn adc_h_mute_rmpdn_rate(&self) -> AdcHMuteRmpdnRateR {
        AdcHMuteRmpdnRateR::new(((self.bits >> 26) & 0x0f) as u8)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn adc_h_mute_softmode(&self) -> AdcHMuteSoftmodeR {
        AdcHMuteSoftmodeR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn adc_h_mute(&self) -> AdcHMuteR {
        AdcHMuteR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 2:5"]
    #[inline(always)]
    pub fn adc_h_ctrl_rmp_rate(&mut self) -> AdcHCtrlRmpRateW<'_, AdcHSpec> {
        AdcHCtrlRmpRateW::new(self, 2)
    }
    #[doc = "Bits 6:9"]
    #[inline(always)]
    pub fn adc_h_ctrl_zcd_rate(&mut self) -> AdcHCtrlZcdRateW<'_, AdcHSpec> {
        AdcHCtrlZcdRateW::new(self, 6)
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn adc_h_ctrl_mode(&mut self) -> AdcHCtrlModeW<'_, AdcHSpec> {
        AdcHCtrlModeW::new(self, 10)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn adc_h_volume_update(&mut self) -> AdcHVolumeUpdateW<'_, AdcHSpec> {
        AdcHVolumeUpdateW::new(self, 12)
    }
    #[doc = "Bits 22:25"]
    #[inline(always)]
    pub fn adc_h_mute_rmpup_rate(&mut self) -> AdcHMuteRmpupRateW<'_, AdcHSpec> {
        AdcHMuteRmpupRateW::new(self, 22)
    }
    #[doc = "Bits 26:29"]
    #[inline(always)]
    pub fn adc_h_mute_rmpdn_rate(&mut self) -> AdcHMuteRmpdnRateW<'_, AdcHSpec> {
        AdcHMuteRmpdnRateW::new(self, 26)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn adc_h_mute_softmode(&mut self) -> AdcHMuteSoftmodeW<'_, AdcHSpec> {
        AdcHMuteSoftmodeW::new(self, 30)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn adc_h_mute(&mut self) -> AdcHMuteW<'_, AdcHSpec> {
        AdcHMuteW::new(self, 31)
    }
}
#[doc = "ADC High-rate path: Main audio volume and mute control\n\nYou can [`read`](crate::Reg::read) this register and get [`adc_h::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc_h::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AdcHSpec;
impl crate::RegisterSpec for AdcHSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adc_h::R`](R) reader structure"]
impl crate::Readable for AdcHSpec {}
#[doc = "`write(|w| ..)` method takes [`adc_h::W`](W) writer structure"]
impl crate::Writable for AdcHSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets adc_h to value 0"]
impl crate::Resettable for AdcHSpec {}
