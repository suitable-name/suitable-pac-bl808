#[doc = "Register `codec_adc1` reader"]
pub type R = crate::R<CodecAdc1Spec>;
#[doc = "Register `codec_adc1` writer"]
pub type W = crate::W<CodecAdc1Spec>;
#[doc = "Field `adc1_sdm_lp` reader - "]
pub type Adc1SdmLpR = crate::BitReader;
#[doc = "Field `adc1_sdm_lp` writer - "]
pub type Adc1SdmLpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `adc1_ckb_en` reader - "]
pub type Adc1CkbEnR = crate::BitReader;
#[doc = "Field `adc1_ckb_en` writer - "]
pub type Adc1CkbEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `adc1_pga_rcm_sel` reader - "]
pub type Adc1PgaRcmSelR = crate::FieldReader;
#[doc = "Field `adc1_pga_rcm_sel` writer - "]
pub type Adc1PgaRcmSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `adc1_pga_gain` reader - "]
pub type Adc1PgaGainR = crate::FieldReader;
#[doc = "Field `adc1_pga_gain` writer - "]
pub type Adc1PgaGainW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `adc1_ictrl_mic` reader - "]
pub type Adc1IctrlMicR = crate::FieldReader;
#[doc = "Field `adc1_ictrl_mic` writer - "]
pub type Adc1IctrlMicW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `adc1_ictrl_adc2` reader - "]
pub type Adc1IctrlAdc2R = crate::FieldReader;
#[doc = "Field `adc1_ictrl_adc2` writer - "]
pub type Adc1IctrlAdc2W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `adc1_ictrl_adc1` reader - "]
pub type Adc1IctrlAdc1R = crate::FieldReader;
#[doc = "Field `adc1_ictrl_adc1` writer - "]
pub type Adc1IctrlAdc1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `adc1_ictrl_aaf` reader - "]
pub type Adc1IctrlAafR = crate::FieldReader;
#[doc = "Field `adc1_ictrl_aaf` writer - "]
pub type Adc1IctrlAafW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `adc1_en_sdm` reader - "]
pub type Adc1EnSdmR = crate::BitReader;
#[doc = "Field `adc1_en_sdm` writer - "]
pub type Adc1EnSdmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `adc1_en_pga` reader - "]
pub type Adc1EnPgaR = crate::BitReader;
#[doc = "Field `adc1_en_pga` writer - "]
pub type Adc1EnPgaW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn adc1_sdm_lp(&self) -> Adc1SdmLpR {
        Adc1SdmLpR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn adc1_ckb_en(&self) -> Adc1CkbEnR {
        Adc1CkbEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn adc1_pga_rcm_sel(&self) -> Adc1PgaRcmSelR {
        Adc1PgaRcmSelR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn adc1_pga_gain(&self) -> Adc1PgaGainR {
        Adc1PgaGainR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn adc1_ictrl_mic(&self) -> Adc1IctrlMicR {
        Adc1IctrlMicR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn adc1_ictrl_adc2(&self) -> Adc1IctrlAdc2R {
        Adc1IctrlAdc2R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn adc1_ictrl_adc1(&self) -> Adc1IctrlAdc1R {
        Adc1IctrlAdc1R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    pub fn adc1_ictrl_aaf(&self) -> Adc1IctrlAafR {
        Adc1IctrlAafR::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn adc1_en_sdm(&self) -> Adc1EnSdmR {
        Adc1EnSdmR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn adc1_en_pga(&self) -> Adc1EnPgaR {
        Adc1EnPgaR::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn adc1_sdm_lp(&mut self) -> Adc1SdmLpW<'_, CodecAdc1Spec> {
        Adc1SdmLpW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn adc1_ckb_en(&mut self) -> Adc1CkbEnW<'_, CodecAdc1Spec> {
        Adc1CkbEnW::new(self, 1)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn adc1_pga_rcm_sel(&mut self) -> Adc1PgaRcmSelW<'_, CodecAdc1Spec> {
        Adc1PgaRcmSelW::new(self, 4)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn adc1_pga_gain(&mut self) -> Adc1PgaGainW<'_, CodecAdc1Spec> {
        Adc1PgaGainW::new(self, 8)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn adc1_ictrl_mic(&mut self) -> Adc1IctrlMicW<'_, CodecAdc1Spec> {
        Adc1IctrlMicW::new(self, 12)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn adc1_ictrl_adc2(&mut self) -> Adc1IctrlAdc2W<'_, CodecAdc1Spec> {
        Adc1IctrlAdc2W::new(self, 14)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn adc1_ictrl_adc1(&mut self) -> Adc1IctrlAdc1W<'_, CodecAdc1Spec> {
        Adc1IctrlAdc1W::new(self, 16)
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    pub fn adc1_ictrl_aaf(&mut self) -> Adc1IctrlAafW<'_, CodecAdc1Spec> {
        Adc1IctrlAafW::new(self, 18)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn adc1_en_sdm(&mut self) -> Adc1EnSdmW<'_, CodecAdc1Spec> {
        Adc1EnSdmW::new(self, 20)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn adc1_en_pga(&mut self) -> Adc1EnPgaW<'_, CodecAdc1Spec> {
        Adc1EnPgaW::new(self, 21)
    }
}
#[doc = "Analog ADC Channel 1 Control: PGA gain and analog front-end power\n\nYou can [`read`](crate::Reg::read) this register and get [`codec_adc1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`codec_adc1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CodecAdc1Spec;
impl crate::RegisterSpec for CodecAdc1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`codec_adc1::R`](R) reader structure"]
impl crate::Readable for CodecAdc1Spec {}
#[doc = "`write(|w| ..)` method takes [`codec_adc1::W`](W) writer structure"]
impl crate::Writable for CodecAdc1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets codec_adc1 to value 0"]
impl crate::Resettable for CodecAdc1Spec {}
