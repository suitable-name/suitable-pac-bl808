#[doc = "Register `dbg_0` reader"]
pub type R = crate::R<Dbg0Spec>;
#[doc = "Register `dbg_0` writer"]
pub type W = crate::W<Dbg0Spec>;
#[doc = "Field `aud_sin_en` reader - "]
pub type AudSinEnR = crate::BitReader;
#[doc = "Field `aud_sin_en` writer - "]
pub type AudSinEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `aud_sin_step` reader - "]
pub type AudSinStepR = crate::FieldReader;
#[doc = "Field `aud_sin_step` writer - "]
pub type AudSinStepW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `audio_test_out_sel` reader - "]
pub type AudioTestOutSelR = crate::FieldReader;
#[doc = "Field `audio_test_out_sel` writer - "]
pub type AudioTestOutSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `dac_test_clkin_en` reader - "]
pub type DacTestClkinEnR = crate::BitReader;
#[doc = "Field `dac_test_clkin_en` writer - "]
pub type DacTestClkinEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `adc_test_clkin_en` reader - "]
pub type AdcTestClkinEnR = crate::BitReader;
#[doc = "Field `adc_test_clkin_en` writer - "]
pub type AdcTestClkinEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dac_test_din_en` reader - "]
pub type DacTestDinEnR = crate::BitReader;
#[doc = "Field `dac_test_din_en` writer - "]
pub type DacTestDinEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `adc_test_din_en` reader - "]
pub type AdcTestDinEnR = crate::BitReader;
#[doc = "Field `adc_test_din_en` writer - "]
pub type AdcTestDinEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `aud_test_read_sel` reader - "]
pub type AudTestReadSelR = crate::FieldReader;
#[doc = "Field `aud_test_read_sel` writer - "]
pub type AudTestReadSelW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn aud_sin_en(&self) -> AudSinEnR {
        AudSinEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3"]
    #[inline(always)]
    pub fn aud_sin_step(&self) -> AudSinStepR {
        AudSinStepR::new(((self.bits >> 1) & 7) as u8)
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    pub fn audio_test_out_sel(&self) -> AudioTestOutSelR {
        AudioTestOutSelR::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn dac_test_clkin_en(&self) -> DacTestClkinEnR {
        DacTestClkinEnR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn adc_test_clkin_en(&self) -> AdcTestClkinEnR {
        AdcTestClkinEnR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn dac_test_din_en(&self) -> DacTestDinEnR {
        DacTestDinEnR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn adc_test_din_en(&self) -> AdcTestDinEnR {
        AdcTestDinEnR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:29"]
    #[inline(always)]
    pub fn aud_test_read_sel(&self) -> AudTestReadSelR {
        AudTestReadSelR::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn aud_sin_en(&mut self) -> AudSinEnW<'_, Dbg0Spec> {
        AudSinEnW::new(self, 0)
    }
    #[doc = "Bits 1:3"]
    #[inline(always)]
    pub fn aud_sin_step(&mut self) -> AudSinStepW<'_, Dbg0Spec> {
        AudSinStepW::new(self, 1)
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    pub fn audio_test_out_sel(&mut self) -> AudioTestOutSelW<'_, Dbg0Spec> {
        AudioTestOutSelW::new(self, 18)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn dac_test_clkin_en(&mut self) -> DacTestClkinEnW<'_, Dbg0Spec> {
        DacTestClkinEnW::new(self, 20)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn adc_test_clkin_en(&mut self) -> AdcTestClkinEnW<'_, Dbg0Spec> {
        AdcTestClkinEnW::new(self, 21)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn dac_test_din_en(&mut self) -> DacTestDinEnW<'_, Dbg0Spec> {
        DacTestDinEnW::new(self, 22)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn adc_test_din_en(&mut self) -> AdcTestDinEnW<'_, Dbg0Spec> {
        AdcTestDinEnW::new(self, 23)
    }
    #[doc = "Bits 24:29"]
    #[inline(always)]
    pub fn aud_test_read_sel(&mut self) -> AudTestReadSelW<'_, Dbg0Spec> {
        AudTestReadSelW::new(self, 24)
    }
}
#[doc = "Audio System Debug Control: Sine generator and test point routing\n\nYou can [`read`](crate::Reg::read) this register and get [`dbg_0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbg_0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dbg0Spec;
impl crate::RegisterSpec for Dbg0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dbg_0::R`](R) reader structure"]
impl crate::Readable for Dbg0Spec {}
#[doc = "`write(|w| ..)` method takes [`dbg_0::W`](W) writer structure"]
impl crate::Writable for Dbg0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets dbg_0 to value 0"]
impl crate::Resettable for Dbg0Spec {}
