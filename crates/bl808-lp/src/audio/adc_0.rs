#[doc = "Register `adc_0` reader"]
pub type R = crate::R<Adc0Spec>;
#[doc = "Register `adc_0` writer"]
pub type W = crate::W<Adc0Spec>;
#[doc = "Field `adc_0_fir_mode` reader - "]
pub type Adc0FirModeR = crate::BitReader;
#[doc = "Field `adc_0_fir_mode` writer - "]
pub type Adc0FirModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `adc_1_fir_mode` reader - "]
pub type Adc1FirModeR = crate::BitReader;
#[doc = "Field `adc_1_fir_mode` writer - "]
pub type Adc1FirModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `adc_2_fir_mode` reader - "]
pub type Adc2FirModeR = crate::BitReader;
#[doc = "Field `adc_2_fir_mode` writer - "]
pub type Adc2FirModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `adc_0_scal` reader - "]
pub type Adc0ScalR = crate::FieldReader;
#[doc = "Field `adc_0_scal` writer - "]
pub type Adc0ScalW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `adc_1_scal` reader - "]
pub type Adc1ScalR = crate::FieldReader;
#[doc = "Field `adc_1_scal` writer - "]
pub type Adc1ScalW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `adc_2_scal` reader - "]
pub type Adc2ScalR = crate::FieldReader;
#[doc = "Field `adc_2_scal` writer - "]
pub type Adc2ScalW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `adc_dither_data` reader - "]
pub type AdcDitherDataR = crate::BitReader;
#[doc = "Field `adc_lfsr_mode` reader - "]
pub type AdcLfsrModeR = crate::FieldReader;
#[doc = "Field `adc_lfsr_mode` writer - "]
pub type AdcLfsrModeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn adc_0_fir_mode(&self) -> Adc0FirModeR {
        Adc0FirModeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn adc_1_fir_mode(&self) -> Adc1FirModeR {
        Adc1FirModeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn adc_2_fir_mode(&self) -> Adc2FirModeR {
        Adc2FirModeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:8"]
    #[inline(always)]
    pub fn adc_0_scal(&self) -> Adc0ScalR {
        Adc0ScalR::new(((self.bits >> 3) & 0x3f) as u8)
    }
    #[doc = "Bits 9:14"]
    #[inline(always)]
    pub fn adc_1_scal(&self) -> Adc1ScalR {
        Adc1ScalR::new(((self.bits >> 9) & 0x3f) as u8)
    }
    #[doc = "Bits 15:20"]
    #[inline(always)]
    pub fn adc_2_scal(&self) -> Adc2ScalR {
        Adc2ScalR::new(((self.bits >> 15) & 0x3f) as u8)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn adc_dither_data(&self) -> AdcDitherDataR {
        AdcDitherDataR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bits 30:31"]
    #[inline(always)]
    pub fn adc_lfsr_mode(&self) -> AdcLfsrModeR {
        AdcLfsrModeR::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn adc_0_fir_mode(&mut self) -> Adc0FirModeW<'_, Adc0Spec> {
        Adc0FirModeW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn adc_1_fir_mode(&mut self) -> Adc1FirModeW<'_, Adc0Spec> {
        Adc1FirModeW::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn adc_2_fir_mode(&mut self) -> Adc2FirModeW<'_, Adc0Spec> {
        Adc2FirModeW::new(self, 2)
    }
    #[doc = "Bits 3:8"]
    #[inline(always)]
    pub fn adc_0_scal(&mut self) -> Adc0ScalW<'_, Adc0Spec> {
        Adc0ScalW::new(self, 3)
    }
    #[doc = "Bits 9:14"]
    #[inline(always)]
    pub fn adc_1_scal(&mut self) -> Adc1ScalW<'_, Adc0Spec> {
        Adc1ScalW::new(self, 9)
    }
    #[doc = "Bits 15:20"]
    #[inline(always)]
    pub fn adc_2_scal(&mut self) -> Adc2ScalW<'_, Adc0Spec> {
        Adc2ScalW::new(self, 15)
    }
    #[doc = "Bits 30:31"]
    #[inline(always)]
    pub fn adc_lfsr_mode(&mut self) -> AdcLfsrModeW<'_, Adc0Spec> {
        AdcLfsrModeW::new(self, 30)
    }
}
#[doc = "ADC Filter and Scaling Configuration 0\n\nYou can [`read`](crate::Reg::read) this register and get [`adc_0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc_0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Adc0Spec;
impl crate::RegisterSpec for Adc0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adc_0::R`](R) reader structure"]
impl crate::Readable for Adc0Spec {}
#[doc = "`write(|w| ..)` method takes [`adc_0::W`](W) writer structure"]
impl crate::Writable for Adc0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets adc_0 to value 0"]
impl crate::Resettable for Adc0Spec {}
