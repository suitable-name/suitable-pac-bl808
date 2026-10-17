#[doc = "Register `adc_1` reader"]
pub type R = crate::R<Adc1Spec>;
#[doc = "Register `adc_1` writer"]
pub type W = crate::W<Adc1Spec>;
#[doc = "Field `adc_0_k1` reader - "]
pub type Adc0K1R = crate::FieldReader;
#[doc = "Field `adc_0_k1` writer - "]
pub type Adc0K1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `adc_0_k1_en` reader - "]
pub type Adc0K1EnR = crate::BitReader;
#[doc = "Field `adc_0_k1_en` writer - "]
pub type Adc0K1EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `adc_0_k2` reader - "]
pub type Adc0K2R = crate::FieldReader;
#[doc = "Field `adc_0_k2` writer - "]
pub type Adc0K2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `adc_0_k2_en` reader - "]
pub type Adc0K2EnR = crate::BitReader;
#[doc = "Field `adc_0_k2_en` writer - "]
pub type Adc0K2EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `adc_1_k1` reader - "]
pub type Adc1K1R = crate::FieldReader;
#[doc = "Field `adc_1_k1` writer - "]
pub type Adc1K1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `adc_1_k1_en` reader - "]
pub type Adc1K1EnR = crate::BitReader;
#[doc = "Field `adc_1_k1_en` writer - "]
pub type Adc1K1EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `adc_1_k2` reader - "]
pub type Adc1K2R = crate::FieldReader;
#[doc = "Field `adc_1_k2` writer - "]
pub type Adc1K2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `adc_1_k2_en` reader - "]
pub type Adc1K2EnR = crate::BitReader;
#[doc = "Field `adc_1_k2_en` writer - "]
pub type Adc1K2EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `adc_2_k1` reader - "]
pub type Adc2K1R = crate::FieldReader;
#[doc = "Field `adc_2_k1` writer - "]
pub type Adc2K1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `adc_2_k1_en` reader - "]
pub type Adc2K1EnR = crate::BitReader;
#[doc = "Field `adc_2_k1_en` writer - "]
pub type Adc2K1EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `adc_2_k2` reader - "]
pub type Adc2K2R = crate::FieldReader;
#[doc = "Field `adc_2_k2` writer - "]
pub type Adc2K2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `adc_2_k2_en` reader - "]
pub type Adc2K2EnR = crate::BitReader;
#[doc = "Field `adc_2_k2_en` writer - "]
pub type Adc2K2EnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn adc_0_k1(&self) -> Adc0K1R {
        Adc0K1R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn adc_0_k1_en(&self) -> Adc0K1EnR {
        Adc0K1EnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:8"]
    #[inline(always)]
    pub fn adc_0_k2(&self) -> Adc0K2R {
        Adc0K2R::new(((self.bits >> 5) & 0x0f) as u8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn adc_0_k2_en(&self) -> Adc0K2EnR {
        Adc0K2EnR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:13"]
    #[inline(always)]
    pub fn adc_1_k1(&self) -> Adc1K1R {
        Adc1K1R::new(((self.bits >> 10) & 0x0f) as u8)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn adc_1_k1_en(&self) -> Adc1K1EnR {
        Adc1K1EnR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 15:18"]
    #[inline(always)]
    pub fn adc_1_k2(&self) -> Adc1K2R {
        Adc1K2R::new(((self.bits >> 15) & 0x0f) as u8)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn adc_1_k2_en(&self) -> Adc1K2EnR {
        Adc1K2EnR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:23"]
    #[inline(always)]
    pub fn adc_2_k1(&self) -> Adc2K1R {
        Adc2K1R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn adc_2_k1_en(&self) -> Adc2K1EnR {
        Adc2K1EnR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:28"]
    #[inline(always)]
    pub fn adc_2_k2(&self) -> Adc2K2R {
        Adc2K2R::new(((self.bits >> 25) & 0x0f) as u8)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn adc_2_k2_en(&self) -> Adc2K2EnR {
        Adc2K2EnR::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn adc_0_k1(&mut self) -> Adc0K1W<'_, Adc1Spec> {
        Adc0K1W::new(self, 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn adc_0_k1_en(&mut self) -> Adc0K1EnW<'_, Adc1Spec> {
        Adc0K1EnW::new(self, 4)
    }
    #[doc = "Bits 5:8"]
    #[inline(always)]
    pub fn adc_0_k2(&mut self) -> Adc0K2W<'_, Adc1Spec> {
        Adc0K2W::new(self, 5)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn adc_0_k2_en(&mut self) -> Adc0K2EnW<'_, Adc1Spec> {
        Adc0K2EnW::new(self, 9)
    }
    #[doc = "Bits 10:13"]
    #[inline(always)]
    pub fn adc_1_k1(&mut self) -> Adc1K1W<'_, Adc1Spec> {
        Adc1K1W::new(self, 10)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn adc_1_k1_en(&mut self) -> Adc1K1EnW<'_, Adc1Spec> {
        Adc1K1EnW::new(self, 14)
    }
    #[doc = "Bits 15:18"]
    #[inline(always)]
    pub fn adc_1_k2(&mut self) -> Adc1K2W<'_, Adc1Spec> {
        Adc1K2W::new(self, 15)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn adc_1_k2_en(&mut self) -> Adc1K2EnW<'_, Adc1Spec> {
        Adc1K2EnW::new(self, 19)
    }
    #[doc = "Bits 20:23"]
    #[inline(always)]
    pub fn adc_2_k1(&mut self) -> Adc2K1W<'_, Adc1Spec> {
        Adc2K1W::new(self, 20)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn adc_2_k1_en(&mut self) -> Adc2K1EnW<'_, Adc1Spec> {
        Adc2K1EnW::new(self, 24)
    }
    #[doc = "Bits 25:28"]
    #[inline(always)]
    pub fn adc_2_k2(&mut self) -> Adc2K2W<'_, Adc1Spec> {
        Adc2K2W::new(self, 25)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn adc_2_k2_en(&mut self) -> Adc2K2EnW<'_, Adc1Spec> {
        Adc2K2EnW::new(self, 29)
    }
}
#[doc = "ADC Analog Gain and Control Loop Config 1\n\nYou can [`read`](crate::Reg::read) this register and get [`adc_1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc_1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Adc1Spec;
impl crate::RegisterSpec for Adc1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adc_1::R`](R) reader structure"]
impl crate::Readable for Adc1Spec {}
#[doc = "`write(|w| ..)` method takes [`adc_1::W`](W) writer structure"]
impl crate::Writable for Adc1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets adc_1 to value 0"]
impl crate::Resettable for Adc1Spec {}
