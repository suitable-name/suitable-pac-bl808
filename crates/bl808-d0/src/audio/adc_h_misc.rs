#[doc = "Register `adc_h_misc` reader"]
pub type R = crate::R<AdcHMiscSpec>;
#[doc = "Register `adc_h_misc` writer"]
pub type W = crate::W<AdcHMiscSpec>;
#[doc = "Field `adc_h_ctrl_zcd_timeout` reader - "]
pub type AdcHCtrlZcdTimeoutR = crate::FieldReader;
#[doc = "Field `adc_h_ctrl_zcd_timeout` writer - "]
pub type AdcHCtrlZcdTimeoutW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn adc_h_ctrl_zcd_timeout(&self) -> AdcHCtrlZcdTimeoutR {
        AdcHCtrlZcdTimeoutR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn adc_h_ctrl_zcd_timeout(&mut self) -> AdcHCtrlZcdTimeoutW<'_, AdcHMiscSpec> {
        AdcHCtrlZcdTimeoutW::new(self, 28)
    }
}
#[doc = "ADC High-rate miscellaneous: Zero-crossing timeouts\n\nYou can [`read`](crate::Reg::read) this register and get [`adc_h_misc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc_h_misc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AdcHMiscSpec;
impl crate::RegisterSpec for AdcHMiscSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adc_h_misc::R`](R) reader structure"]
impl crate::Readable for AdcHMiscSpec {}
#[doc = "`write(|w| ..)` method takes [`adc_h_misc::W`](W) writer structure"]
impl crate::Writable for AdcHMiscSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets adc_h_misc to value 0"]
impl crate::Resettable for AdcHMiscSpec {}
