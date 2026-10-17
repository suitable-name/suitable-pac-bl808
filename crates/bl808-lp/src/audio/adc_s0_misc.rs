#[doc = "Register `adc_s0_misc` reader"]
pub type R = crate::R<AdcS0MiscSpec>;
#[doc = "Register `adc_s0_misc` writer"]
pub type W = crate::W<AdcS0MiscSpec>;
#[doc = "Field `adc_s012_volume_update` reader - "]
pub type AdcS012VolumeUpdateR = crate::BitReader;
#[doc = "Field `adc_s012_volume_update` writer - "]
pub type AdcS012VolumeUpdateW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `adc_s012_mute` reader - "]
pub type AdcS012MuteR = crate::BitReader;
#[doc = "Field `adc_s012_mute` writer - "]
pub type AdcS012MuteW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `adc_s0_ctrl_zcd_timeout` reader - "]
pub type AdcS0CtrlZcdTimeoutR = crate::FieldReader;
#[doc = "Field `adc_s0_ctrl_zcd_timeout` writer - "]
pub type AdcS0CtrlZcdTimeoutW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn adc_s012_volume_update(&self) -> AdcS012VolumeUpdateR {
        AdcS012VolumeUpdateR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn adc_s012_mute(&self) -> AdcS012MuteR {
        AdcS012MuteR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn adc_s0_ctrl_zcd_timeout(&self) -> AdcS0CtrlZcdTimeoutR {
        AdcS0CtrlZcdTimeoutR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn adc_s012_volume_update(&mut self) -> AdcS012VolumeUpdateW<'_, AdcS0MiscSpec> {
        AdcS012VolumeUpdateW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn adc_s012_mute(&mut self) -> AdcS012MuteW<'_, AdcS0MiscSpec> {
        AdcS012MuteW::new(self, 1)
    }
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn adc_s0_ctrl_zcd_timeout(&mut self) -> AdcS0CtrlZcdTimeoutW<'_, AdcS0MiscSpec> {
        AdcS0CtrlZcdTimeoutW::new(self, 28)
    }
}
#[doc = "ADC Sideband 0 misc: Global mute and update synchronization\n\nYou can [`read`](crate::Reg::read) this register and get [`adc_s0_misc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc_s0_misc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AdcS0MiscSpec;
impl crate::RegisterSpec for AdcS0MiscSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adc_s0_misc::R`](R) reader structure"]
impl crate::Readable for AdcS0MiscSpec {}
#[doc = "`write(|w| ..)` method takes [`adc_s0_misc::W`](W) writer structure"]
impl crate::Writable for AdcS0MiscSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets adc_s0_misc to value 0"]
impl crate::Resettable for AdcS0MiscSpec {}
