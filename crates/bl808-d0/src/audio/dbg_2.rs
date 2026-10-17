#[doc = "Register `dbg_2` reader"]
pub type R = crate::R<Dbg2Spec>;
#[doc = "Register `dbg_2` writer"]
pub type W = crate::W<Dbg2Spec>;
#[doc = "Field `adc_fir_4s_val` reader - ADC FIR 4S Value"]
pub type AdcFir4sValR = crate::FieldReader<u32>;
#[doc = "Field `adc_fir_4s_val` writer - ADC FIR 4S Value"]
pub type AdcFir4sValW<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
#[doc = "Field `adc_0_fir_4s_en` reader - "]
pub type Adc0Fir4sEnR = crate::BitReader;
#[doc = "Field `adc_0_fir_4s_en` writer - "]
pub type Adc0Fir4sEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `adc_1_fir_4s_en` reader - "]
pub type Adc1Fir4sEnR = crate::BitReader;
#[doc = "Field `adc_1_fir_4s_en` writer - "]
pub type Adc1Fir4sEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `adc_2_fir_4s_en` reader - "]
pub type Adc2Fir4sEnR = crate::BitReader;
#[doc = "Field `adc_2_fir_4s_en` writer - "]
pub type Adc2Fir4sEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `adc_in_0_test_sel` reader - "]
pub type AdcIn0TestSelR = crate::BitReader;
#[doc = "Field `adc_in_0_test_sel` writer - "]
pub type AdcIn0TestSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `adc_in_1_test_sel` reader - "]
pub type AdcIn1TestSelR = crate::BitReader;
#[doc = "Field `adc_in_1_test_sel` writer - "]
pub type AdcIn1TestSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `adc_in_2_test_sel` reader - "]
pub type AdcIn2TestSelR = crate::BitReader;
#[doc = "Field `adc_in_2_test_sel` writer - "]
pub type AdcIn2TestSelW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:19 - ADC FIR 4S Value"]
    #[inline(always)]
    pub fn adc_fir_4s_val(&self) -> AdcFir4sValR {
        AdcFir4sValR::new(self.bits & 0x000f_ffff)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn adc_0_fir_4s_en(&self) -> Adc0Fir4sEnR {
        Adc0Fir4sEnR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn adc_1_fir_4s_en(&self) -> Adc1Fir4sEnR {
        Adc1Fir4sEnR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn adc_2_fir_4s_en(&self) -> Adc2Fir4sEnR {
        Adc2Fir4sEnR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn adc_in_0_test_sel(&self) -> AdcIn0TestSelR {
        AdcIn0TestSelR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn adc_in_1_test_sel(&self) -> AdcIn1TestSelR {
        AdcIn1TestSelR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn adc_in_2_test_sel(&self) -> AdcIn2TestSelR {
        AdcIn2TestSelR::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:19 - ADC FIR 4S Value"]
    #[inline(always)]
    pub fn adc_fir_4s_val(&mut self) -> AdcFir4sValW<'_, Dbg2Spec> {
        AdcFir4sValW::new(self, 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn adc_0_fir_4s_en(&mut self) -> Adc0Fir4sEnW<'_, Dbg2Spec> {
        Adc0Fir4sEnW::new(self, 20)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn adc_1_fir_4s_en(&mut self) -> Adc1Fir4sEnW<'_, Dbg2Spec> {
        Adc1Fir4sEnW::new(self, 21)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn adc_2_fir_4s_en(&mut self) -> Adc2Fir4sEnW<'_, Dbg2Spec> {
        Adc2Fir4sEnW::new(self, 22)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn adc_in_0_test_sel(&mut self) -> AdcIn0TestSelW<'_, Dbg2Spec> {
        AdcIn0TestSelW::new(self, 23)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn adc_in_1_test_sel(&mut self) -> AdcIn1TestSelW<'_, Dbg2Spec> {
        AdcIn1TestSelW::new(self, 24)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn adc_in_2_test_sel(&mut self) -> AdcIn2TestSelW<'_, Dbg2Spec> {
        AdcIn2TestSelW::new(self, 25)
    }
}
#[doc = "Debug Register 2: FIR filter and ADC test selection\n\nYou can [`read`](crate::Reg::read) this register and get [`dbg_2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbg_2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dbg2Spec;
impl crate::RegisterSpec for Dbg2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dbg_2::R`](R) reader structure"]
impl crate::Readable for Dbg2Spec {}
#[doc = "`write(|w| ..)` method takes [`dbg_2::W`](W) writer structure"]
impl crate::Writable for Dbg2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets dbg_2 to value 0"]
impl crate::Resettable for Dbg2Spec {}
