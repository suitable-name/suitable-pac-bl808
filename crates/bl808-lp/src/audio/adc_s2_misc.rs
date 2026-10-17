#[doc = "Register `adc_s2_misc` reader"]
pub type R = crate::R<AdcS2MiscSpec>;
#[doc = "Register `adc_s2_misc` writer"]
pub type W = crate::W<AdcS2MiscSpec>;
#[doc = "Field `adc_s2_ctrl_zcd_timeout` reader - "]
pub type AdcS2CtrlZcdTimeoutR = crate::FieldReader;
#[doc = "Field `adc_s2_ctrl_zcd_timeout` writer - "]
pub type AdcS2CtrlZcdTimeoutW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn adc_s2_ctrl_zcd_timeout(&self) -> AdcS2CtrlZcdTimeoutR {
        AdcS2CtrlZcdTimeoutR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn adc_s2_ctrl_zcd_timeout(&mut self) -> AdcS2CtrlZcdTimeoutW<'_, AdcS2MiscSpec> {
        AdcS2CtrlZcdTimeoutW::new(self, 28)
    }
}
#[doc = "ADC Sideband 2 misc: Zero-crossing timeout settings\n\nYou can [`read`](crate::Reg::read) this register and get [`adc_s2_misc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc_s2_misc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AdcS2MiscSpec;
impl crate::RegisterSpec for AdcS2MiscSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adc_s2_misc::R`](R) reader structure"]
impl crate::Readable for AdcS2MiscSpec {}
#[doc = "`write(|w| ..)` method takes [`adc_s2_misc::W`](W) writer structure"]
impl crate::Writable for AdcS2MiscSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets adc_s2_misc to value 0"]
impl crate::Resettable for AdcS2MiscSpec {}
