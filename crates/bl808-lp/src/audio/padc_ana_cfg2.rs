#[doc = "Register `padc_ana_cfg2` reader"]
pub type R = crate::R<PadcAnaCfg2Spec>;
#[doc = "Register `padc_ana_cfg2` writer"]
pub type W = crate::W<PadcAnaCfg2Spec>;
#[doc = "Field `padc_sdm_chop_phas` reader - "]
pub type PadcSdmChopPhasR = crate::BitReader;
#[doc = "Field `padc_sdm_chop_phas` writer - "]
pub type PadcSdmChopPhasW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `padc_sdm_chop_freq` reader - "]
pub type PadcSdmChopFreqR = crate::FieldReader;
#[doc = "Field `padc_sdm_chop_freq` writer - "]
pub type PadcSdmChopFreqW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `padc_sdm_chop_en` reader - "]
pub type PadcSdmChopEnR = crate::BitReader;
#[doc = "Field `padc_sdm_chop_en` writer - "]
pub type PadcSdmChopEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `padc_dither_order` reader - "]
pub type PadcDitherOrderR = crate::BitReader;
#[doc = "Field `padc_dither_order` writer - "]
pub type PadcDitherOrderW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `padc_dither_sel` reader - "]
pub type PadcDitherSelR = crate::FieldReader;
#[doc = "Field `padc_dither_sel` writer - "]
pub type PadcDitherSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `padc_quan_gain` reader - "]
pub type PadcQuanGainR = crate::FieldReader;
#[doc = "Field `padc_quan_gain` writer - "]
pub type PadcQuanGainW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `padc_dem_en` reader - "]
pub type PadcDemEnR = crate::BitReader;
#[doc = "Field `padc_dem_en` writer - "]
pub type PadcDemEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `padc_ictrl_adc` reader - "]
pub type PadcIctrlAdcR = crate::FieldReader;
#[doc = "Field `padc_ictrl_adc` writer - "]
pub type PadcIctrlAdcW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn padc_sdm_chop_phas(&self) -> PadcSdmChopPhasR {
        PadcSdmChopPhasR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3"]
    #[inline(always)]
    pub fn padc_sdm_chop_freq(&self) -> PadcSdmChopFreqR {
        PadcSdmChopFreqR::new(((self.bits >> 1) & 7) as u8)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn padc_sdm_chop_en(&self) -> PadcSdmChopEnR {
        PadcSdmChopEnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn padc_dither_order(&self) -> PadcDitherOrderR {
        PadcDitherOrderR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn padc_dither_sel(&self) -> PadcDitherSelR {
        PadcDitherSelR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn padc_quan_gain(&self) -> PadcQuanGainR {
        PadcQuanGainR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn padc_dem_en(&self) -> PadcDemEnR {
        PadcDemEnR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 15:16"]
    #[inline(always)]
    pub fn padc_ictrl_adc(&self) -> PadcIctrlAdcR {
        PadcIctrlAdcR::new(((self.bits >> 15) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn padc_sdm_chop_phas(&mut self) -> PadcSdmChopPhasW<'_, PadcAnaCfg2Spec> {
        PadcSdmChopPhasW::new(self, 0)
    }
    #[doc = "Bits 1:3"]
    #[inline(always)]
    pub fn padc_sdm_chop_freq(&mut self) -> PadcSdmChopFreqW<'_, PadcAnaCfg2Spec> {
        PadcSdmChopFreqW::new(self, 1)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn padc_sdm_chop_en(&mut self) -> PadcSdmChopEnW<'_, PadcAnaCfg2Spec> {
        PadcSdmChopEnW::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn padc_dither_order(&mut self) -> PadcDitherOrderW<'_, PadcAnaCfg2Spec> {
        PadcDitherOrderW::new(self, 5)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn padc_dither_sel(&mut self) -> PadcDitherSelW<'_, PadcAnaCfg2Spec> {
        PadcDitherSelW::new(self, 6)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn padc_quan_gain(&mut self) -> PadcQuanGainW<'_, PadcAnaCfg2Spec> {
        PadcQuanGainW::new(self, 8)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn padc_dem_en(&mut self) -> PadcDemEnW<'_, PadcAnaCfg2Spec> {
        PadcDemEnW::new(self, 10)
    }
    #[doc = "Bits 15:16"]
    #[inline(always)]
    pub fn padc_ictrl_adc(&mut self) -> PadcIctrlAdcW<'_, PadcAnaCfg2Spec> {
        PadcIctrlAdcW::new(self, 15)
    }
}
#[doc = "Passthrough ADC Analog Config 2: Dither and modulation settings\n\nYou can [`read`](crate::Reg::read) this register and get [`padc_ana_cfg2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`padc_ana_cfg2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PadcAnaCfg2Spec;
impl crate::RegisterSpec for PadcAnaCfg2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`padc_ana_cfg2::R`](R) reader structure"]
impl crate::Readable for PadcAnaCfg2Spec {}
#[doc = "`write(|w| ..)` method takes [`padc_ana_cfg2::W`](W) writer structure"]
impl crate::Writable for PadcAnaCfg2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets padc_ana_cfg2 to value 0"]
impl crate::Resettable for PadcAnaCfg2Spec {}
