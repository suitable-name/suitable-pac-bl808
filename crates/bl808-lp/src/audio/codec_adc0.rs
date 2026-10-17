#[doc = "Register `codec_adc0` reader"]
pub type R = crate::R<CodecAdc0Spec>;
#[doc = "Register `codec_adc0` writer"]
pub type W = crate::W<CodecAdc0Spec>;
#[doc = "Field `adc0_sdm_lp` reader - "]
pub type Adc0SdmLpR = crate::BitReader;
#[doc = "Field `adc0_sdm_lp` writer - "]
pub type Adc0SdmLpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `adc0_ckb_en` reader - "]
pub type Adc0CkbEnR = crate::BitReader;
#[doc = "Field `adc0_ckb_en` writer - "]
pub type Adc0CkbEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `adc0_pga_rcm_sel` reader - "]
pub type Adc0PgaRcmSelR = crate::FieldReader;
#[doc = "Field `adc0_pga_rcm_sel` writer - "]
pub type Adc0PgaRcmSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `adc0_pga_gain` reader - "]
pub type Adc0PgaGainR = crate::FieldReader;
#[doc = "Field `adc0_pga_gain` writer - "]
pub type Adc0PgaGainW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `adc0_ictrl_mic` reader - "]
pub type Adc0IctrlMicR = crate::FieldReader;
#[doc = "Field `adc0_ictrl_mic` writer - "]
pub type Adc0IctrlMicW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `adc0_ictrl_adc2` reader - "]
pub type Adc0IctrlAdc2R = crate::FieldReader;
#[doc = "Field `adc0_ictrl_adc2` writer - "]
pub type Adc0IctrlAdc2W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `adc0_ictrl_adc1` reader - "]
pub type Adc0IctrlAdc1R = crate::FieldReader;
#[doc = "Field `adc0_ictrl_adc1` writer - "]
pub type Adc0IctrlAdc1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `adc0_ictrl_aaf` reader - "]
pub type Adc0IctrlAafR = crate::FieldReader;
#[doc = "Field `adc0_ictrl_aaf` writer - "]
pub type Adc0IctrlAafW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `adc0_en_sdm` reader - "]
pub type Adc0EnSdmR = crate::BitReader;
#[doc = "Field `adc0_en_sdm` writer - "]
pub type Adc0EnSdmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `adc0_en_pga` reader - "]
pub type Adc0EnPgaR = crate::BitReader;
#[doc = "Field `adc0_en_pga` writer - "]
pub type Adc0EnPgaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `adc_dither_sel` reader - "]
pub type AdcDitherSelR = crate::FieldReader;
#[doc = "Field `adc_dither_sel` writer - "]
pub type AdcDitherSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn adc0_sdm_lp(&self) -> Adc0SdmLpR {
        Adc0SdmLpR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn adc0_ckb_en(&self) -> Adc0CkbEnR {
        Adc0CkbEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn adc0_pga_rcm_sel(&self) -> Adc0PgaRcmSelR {
        Adc0PgaRcmSelR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn adc0_pga_gain(&self) -> Adc0PgaGainR {
        Adc0PgaGainR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn adc0_ictrl_mic(&self) -> Adc0IctrlMicR {
        Adc0IctrlMicR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn adc0_ictrl_adc2(&self) -> Adc0IctrlAdc2R {
        Adc0IctrlAdc2R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn adc0_ictrl_adc1(&self) -> Adc0IctrlAdc1R {
        Adc0IctrlAdc1R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    pub fn adc0_ictrl_aaf(&self) -> Adc0IctrlAafR {
        Adc0IctrlAafR::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn adc0_en_sdm(&self) -> Adc0EnSdmR {
        Adc0EnSdmR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn adc0_en_pga(&self) -> Adc0EnPgaR {
        Adc0EnPgaR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 22:23"]
    #[inline(always)]
    pub fn adc_dither_sel(&self) -> AdcDitherSelR {
        AdcDitherSelR::new(((self.bits >> 22) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn adc0_sdm_lp(&mut self) -> Adc0SdmLpW<'_, CodecAdc0Spec> {
        Adc0SdmLpW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn adc0_ckb_en(&mut self) -> Adc0CkbEnW<'_, CodecAdc0Spec> {
        Adc0CkbEnW::new(self, 1)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn adc0_pga_rcm_sel(&mut self) -> Adc0PgaRcmSelW<'_, CodecAdc0Spec> {
        Adc0PgaRcmSelW::new(self, 4)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn adc0_pga_gain(&mut self) -> Adc0PgaGainW<'_, CodecAdc0Spec> {
        Adc0PgaGainW::new(self, 8)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn adc0_ictrl_mic(&mut self) -> Adc0IctrlMicW<'_, CodecAdc0Spec> {
        Adc0IctrlMicW::new(self, 12)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn adc0_ictrl_adc2(&mut self) -> Adc0IctrlAdc2W<'_, CodecAdc0Spec> {
        Adc0IctrlAdc2W::new(self, 14)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn adc0_ictrl_adc1(&mut self) -> Adc0IctrlAdc1W<'_, CodecAdc0Spec> {
        Adc0IctrlAdc1W::new(self, 16)
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    pub fn adc0_ictrl_aaf(&mut self) -> Adc0IctrlAafW<'_, CodecAdc0Spec> {
        Adc0IctrlAafW::new(self, 18)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn adc0_en_sdm(&mut self) -> Adc0EnSdmW<'_, CodecAdc0Spec> {
        Adc0EnSdmW::new(self, 20)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn adc0_en_pga(&mut self) -> Adc0EnPgaW<'_, CodecAdc0Spec> {
        Adc0EnPgaW::new(self, 21)
    }
    #[doc = "Bits 22:23"]
    #[inline(always)]
    pub fn adc_dither_sel(&mut self) -> AdcDitherSelW<'_, CodecAdc0Spec> {
        AdcDitherSelW::new(self, 22)
    }
}
#[doc = "Analog ADC Channel 0 Control: PGA gain and analog front-end power\n\nYou can [`read`](crate::Reg::read) this register and get [`codec_adc0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`codec_adc0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CodecAdc0Spec;
impl crate::RegisterSpec for CodecAdc0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`codec_adc0::R`](R) reader structure"]
impl crate::Readable for CodecAdc0Spec {}
#[doc = "`write(|w| ..)` method takes [`codec_adc0::W`](W) writer structure"]
impl crate::Writable for CodecAdc0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets codec_adc0 to value 0"]
impl crate::Resettable for CodecAdc0Spec {}
