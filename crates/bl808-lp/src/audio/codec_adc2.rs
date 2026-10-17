#[doc = "Register `codec_adc2` reader"]
pub type R = crate::R<CodecAdc2Spec>;
#[doc = "Register `codec_adc2` writer"]
pub type W = crate::W<CodecAdc2Spec>;
#[doc = "Field `adc2_sdm_lp` reader - "]
pub type Adc2SdmLpR = crate::BitReader;
#[doc = "Field `adc2_sdm_lp` writer - "]
pub type Adc2SdmLpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `adc2_ckb_en` reader - "]
pub type Adc2CkbEnR = crate::BitReader;
#[doc = "Field `adc2_ckb_en` writer - "]
pub type Adc2CkbEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `adc2_pga_rcm_sel` reader - "]
pub type Adc2PgaRcmSelR = crate::FieldReader;
#[doc = "Field `adc2_pga_rcm_sel` writer - "]
pub type Adc2PgaRcmSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `adc2_pga_gain` reader - "]
pub type Adc2PgaGainR = crate::FieldReader;
#[doc = "Field `adc2_pga_gain` writer - "]
pub type Adc2PgaGainW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `adc2_ictrl_mic` reader - "]
pub type Adc2IctrlMicR = crate::FieldReader;
#[doc = "Field `adc2_ictrl_mic` writer - "]
pub type Adc2IctrlMicW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `adc2_ictrl_adc2` reader - "]
pub type Adc2IctrlAdc2R = crate::FieldReader;
#[doc = "Field `adc2_ictrl_adc2` writer - "]
pub type Adc2IctrlAdc2W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `adc2_ictrl_adc1` reader - "]
pub type Adc2IctrlAdc1R = crate::FieldReader;
#[doc = "Field `adc2_ictrl_adc1` writer - "]
pub type Adc2IctrlAdc1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `adc2_ictrl_aaf` reader - "]
pub type Adc2IctrlAafR = crate::FieldReader;
#[doc = "Field `adc2_ictrl_aaf` writer - "]
pub type Adc2IctrlAafW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `adc2_en_sdm` reader - "]
pub type Adc2EnSdmR = crate::BitReader;
#[doc = "Field `adc2_en_sdm` writer - "]
pub type Adc2EnSdmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `adc2_en_pga` reader - "]
pub type Adc2EnPgaR = crate::BitReader;
#[doc = "Field `adc2_en_pga` writer - "]
pub type Adc2EnPgaW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn adc2_sdm_lp(&self) -> Adc2SdmLpR {
        Adc2SdmLpR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn adc2_ckb_en(&self) -> Adc2CkbEnR {
        Adc2CkbEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn adc2_pga_rcm_sel(&self) -> Adc2PgaRcmSelR {
        Adc2PgaRcmSelR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn adc2_pga_gain(&self) -> Adc2PgaGainR {
        Adc2PgaGainR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn adc2_ictrl_mic(&self) -> Adc2IctrlMicR {
        Adc2IctrlMicR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn adc2_ictrl_adc2(&self) -> Adc2IctrlAdc2R {
        Adc2IctrlAdc2R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn adc2_ictrl_adc1(&self) -> Adc2IctrlAdc1R {
        Adc2IctrlAdc1R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    pub fn adc2_ictrl_aaf(&self) -> Adc2IctrlAafR {
        Adc2IctrlAafR::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn adc2_en_sdm(&self) -> Adc2EnSdmR {
        Adc2EnSdmR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn adc2_en_pga(&self) -> Adc2EnPgaR {
        Adc2EnPgaR::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn adc2_sdm_lp(&mut self) -> Adc2SdmLpW<'_, CodecAdc2Spec> {
        Adc2SdmLpW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn adc2_ckb_en(&mut self) -> Adc2CkbEnW<'_, CodecAdc2Spec> {
        Adc2CkbEnW::new(self, 1)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn adc2_pga_rcm_sel(&mut self) -> Adc2PgaRcmSelW<'_, CodecAdc2Spec> {
        Adc2PgaRcmSelW::new(self, 4)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn adc2_pga_gain(&mut self) -> Adc2PgaGainW<'_, CodecAdc2Spec> {
        Adc2PgaGainW::new(self, 8)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn adc2_ictrl_mic(&mut self) -> Adc2IctrlMicW<'_, CodecAdc2Spec> {
        Adc2IctrlMicW::new(self, 12)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn adc2_ictrl_adc2(&mut self) -> Adc2IctrlAdc2W<'_, CodecAdc2Spec> {
        Adc2IctrlAdc2W::new(self, 14)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn adc2_ictrl_adc1(&mut self) -> Adc2IctrlAdc1W<'_, CodecAdc2Spec> {
        Adc2IctrlAdc1W::new(self, 16)
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    pub fn adc2_ictrl_aaf(&mut self) -> Adc2IctrlAafW<'_, CodecAdc2Spec> {
        Adc2IctrlAafW::new(self, 18)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn adc2_en_sdm(&mut self) -> Adc2EnSdmW<'_, CodecAdc2Spec> {
        Adc2EnSdmW::new(self, 20)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn adc2_en_pga(&mut self) -> Adc2EnPgaW<'_, CodecAdc2Spec> {
        Adc2EnPgaW::new(self, 21)
    }
}
#[doc = "Analog ADC Channel 2 Control: PGA gain and analog front-end power\n\nYou can [`read`](crate::Reg::read) this register and get [`codec_adc2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`codec_adc2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CodecAdc2Spec;
impl crate::RegisterSpec for CodecAdc2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`codec_adc2::R`](R) reader structure"]
impl crate::Readable for CodecAdc2Spec {}
#[doc = "`write(|w| ..)` method takes [`codec_adc2::W`](W) writer structure"]
impl crate::Writable for CodecAdc2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets codec_adc2 to value 0"]
impl crate::Resettable for CodecAdc2Spec {}
