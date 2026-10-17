#[doc = "Register `b0` reader"]
pub type R = crate::R<B0Spec>;
#[doc = "Register `b0` writer"]
pub type W = crate::W<B0Spec>;
#[doc = "Field `val` reader - "]
pub type ValR = crate::FieldReader<u32>;
#[doc = "Field `val` writer - "]
pub type ValW<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
#[doc = "Field `byp` reader - Bypass this EQ stage"]
pub type BypR = crate::BitReader;
#[doc = "Field `byp` writer - Bypass this EQ stage"]
pub type BypW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `qfmt` reader - Quantization format for coefficients"]
pub type QfmtR = crate::BitReader;
#[doc = "Field `qfmt` writer - Quantization format for coefficients"]
pub type QfmtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `adc_peq_in_sel` reader - ADC PEQ input select (only applicable to bank 20)"]
pub type AdcPeqInSelR = crate::FieldReader;
#[doc = "Field `adc_peq_in_sel` writer - ADC PEQ input select (only applicable to bank 20)"]
pub type AdcPeqInSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:19"]
    #[inline(always)]
    pub fn val(&self) -> ValR {
        ValR::new(self.bits & 0x000f_ffff)
    }
    #[doc = "Bit 20 - Bypass this EQ stage"]
    #[inline(always)]
    pub fn byp(&self) -> BypR {
        BypR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Quantization format for coefficients"]
    #[inline(always)]
    pub fn qfmt(&self) -> QfmtR {
        QfmtR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 30:31 - ADC PEQ input select (only applicable to bank 20)"]
    #[inline(always)]
    pub fn adc_peq_in_sel(&self) -> AdcPeqInSelR {
        AdcPeqInSelR::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:19"]
    #[inline(always)]
    pub fn val(&mut self) -> ValW<'_, B0Spec> {
        ValW::new(self, 0)
    }
    #[doc = "Bit 20 - Bypass this EQ stage"]
    #[inline(always)]
    pub fn byp(&mut self) -> BypW<'_, B0Spec> {
        BypW::new(self, 20)
    }
    #[doc = "Bit 21 - Quantization format for coefficients"]
    #[inline(always)]
    pub fn qfmt(&mut self) -> QfmtW<'_, B0Spec> {
        QfmtW::new(self, 21)
    }
    #[doc = "Bits 30:31 - ADC PEQ input select (only applicable to bank 20)"]
    #[inline(always)]
    pub fn adc_peq_in_sel(&mut self) -> AdcPeqInSelW<'_, B0Spec> {
        AdcPeqInSelW::new(self, 30)
    }
}
#[doc = "PEQ Coefficient B0 (Numerator z0)\n\nYou can [`read`](crate::Reg::read) this register and get [`b0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`b0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct B0Spec;
impl crate::RegisterSpec for B0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`b0::R`](R) reader structure"]
impl crate::Readable for B0Spec {}
#[doc = "`write(|w| ..)` method takes [`b0::W`](W) writer structure"]
impl crate::Writable for B0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets b0 to value 0"]
impl crate::Resettable for B0Spec {}
