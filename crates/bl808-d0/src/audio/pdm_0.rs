#[doc = "Register `pdm_0` reader"]
pub type R = crate::R<Pdm0Spec>;
#[doc = "Register `pdm_0` writer"]
pub type W = crate::W<Pdm0Spec>;
#[doc = "Field `pdm_0_en` reader - "]
pub type Pdm0EnR = crate::BitReader;
#[doc = "Field `pdm_0_en` writer - "]
pub type Pdm0EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pdm_1_en` reader - "]
pub type Pdm1EnR = crate::BitReader;
#[doc = "Field `pdm_1_en` writer - "]
pub type Pdm1EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pdm_2_en` reader - "]
pub type Pdm2EnR = crate::BitReader;
#[doc = "Field `pdm_2_en` writer - "]
pub type Pdm2EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `adc_0_pdm_sel` reader - "]
pub type Adc0PdmSelR = crate::FieldReader;
#[doc = "Field `adc_0_pdm_sel` writer - "]
pub type Adc0PdmSelW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `adc_1_pdm_sel` reader - "]
pub type Adc1PdmSelR = crate::FieldReader;
#[doc = "Field `adc_1_pdm_sel` writer - "]
pub type Adc1PdmSelW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `adc_2_pdm_sel` reader - "]
pub type Adc2PdmSelR = crate::FieldReader;
#[doc = "Field `adc_2_pdm_sel` writer - "]
pub type Adc2PdmSelW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn pdm_0_en(&self) -> Pdm0EnR {
        Pdm0EnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn pdm_1_en(&self) -> Pdm1EnR {
        Pdm1EnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn pdm_2_en(&self) -> Pdm2EnR {
        Pdm2EnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:5"]
    #[inline(always)]
    pub fn adc_0_pdm_sel(&self) -> Adc0PdmSelR {
        Adc0PdmSelR::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 6:8"]
    #[inline(always)]
    pub fn adc_1_pdm_sel(&self) -> Adc1PdmSelR {
        Adc1PdmSelR::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bits 9:11"]
    #[inline(always)]
    pub fn adc_2_pdm_sel(&self) -> Adc2PdmSelR {
        Adc2PdmSelR::new(((self.bits >> 9) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn pdm_0_en(&mut self) -> Pdm0EnW<'_, Pdm0Spec> {
        Pdm0EnW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn pdm_1_en(&mut self) -> Pdm1EnW<'_, Pdm0Spec> {
        Pdm1EnW::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn pdm_2_en(&mut self) -> Pdm2EnW<'_, Pdm0Spec> {
        Pdm2EnW::new(self, 2)
    }
    #[doc = "Bits 3:5"]
    #[inline(always)]
    pub fn adc_0_pdm_sel(&mut self) -> Adc0PdmSelW<'_, Pdm0Spec> {
        Adc0PdmSelW::new(self, 3)
    }
    #[doc = "Bits 6:8"]
    #[inline(always)]
    pub fn adc_1_pdm_sel(&mut self) -> Adc1PdmSelW<'_, Pdm0Spec> {
        Adc1PdmSelW::new(self, 6)
    }
    #[doc = "Bits 9:11"]
    #[inline(always)]
    pub fn adc_2_pdm_sel(&mut self) -> Adc2PdmSelW<'_, Pdm0Spec> {
        Adc2PdmSelW::new(self, 9)
    }
}
#[doc = "PDM Configuration 0: Pulse Density Modulation interface setup\n\nYou can [`read`](crate::Reg::read) this register and get [`pdm_0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdm_0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pdm0Spec;
impl crate::RegisterSpec for Pdm0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pdm_0::R`](R) reader structure"]
impl crate::Readable for Pdm0Spec {}
#[doc = "`write(|w| ..)` method takes [`pdm_0::W`](W) writer structure"]
impl crate::Writable for Pdm0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets pdm_0 to value 0"]
impl crate::Resettable for Pdm0Spec {}
