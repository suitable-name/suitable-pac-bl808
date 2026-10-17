#[doc = "Register `codec_power1` reader"]
pub type R = crate::R<CodecPower1Spec>;
#[doc = "Register `codec_power1` writer"]
pub type W = crate::W<CodecPower1Spec>;
#[doc = "Field `micbias_pu` reader - MIC Bias Power Up"]
pub type MicbiasPuR = crate::BitReader;
#[doc = "Field `micbias_pu` writer - MIC Bias Power Up"]
pub type MicbiasPuW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `micbias_chop_en` reader - "]
pub type MicbiasChopEnR = crate::BitReader;
#[doc = "Field `micbias_chop_en` writer - "]
pub type MicbiasChopEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `micbias_clk_chop_sel` reader - "]
pub type MicbiasClkChopSelR = crate::FieldReader;
#[doc = "Field `micbias_clk_chop_sel` writer - "]
pub type MicbiasClkChopSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `micbias_vsel` reader - "]
pub type MicbiasVselR = crate::FieldReader;
#[doc = "Field `micbias_vsel` writer - "]
pub type MicbiasVselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `padc_ibias_source` reader - "]
pub type PadcIbiasSourceR = crate::BitReader;
#[doc = "Field `padc_ibias_source` writer - "]
pub type PadcIbiasSourceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `padc_power_ten` reader - "]
pub type PadcPowerTenR = crate::BitReader;
#[doc = "Field `padc_power_ten` writer - "]
pub type PadcPowerTenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `padc_power_tmux` reader - "]
pub type PadcPowerTmuxR = crate::FieldReader;
#[doc = "Field `padc_power_tmux` writer - "]
pub type PadcPowerTmuxW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `codec_vrda_en` reader - "]
pub type CodecVrdaEnR = crate::BitReader;
#[doc = "Field `codec_vrda_en` writer - "]
pub type CodecVrdaEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `codec_ictrl_vcm` reader - "]
pub type CodecIctrlVcmR = crate::FieldReader;
#[doc = "Field `codec_ictrl_vcm` writer - "]
pub type CodecIctrlVcmW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `codec_ibias_ctrl` reader - "]
pub type CodecIbiasCtrlR = crate::FieldReader;
#[doc = "Field `codec_ibias_ctrl` writer - "]
pub type CodecIbiasCtrlW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bit 0 - MIC Bias Power Up"]
    #[inline(always)]
    pub fn micbias_pu(&self) -> MicbiasPuR {
        MicbiasPuR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn micbias_chop_en(&self) -> MicbiasChopEnR {
        MicbiasChopEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn micbias_clk_chop_sel(&self) -> MicbiasClkChopSelR {
        MicbiasClkChopSelR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn micbias_vsel(&self) -> MicbiasVselR {
        MicbiasVselR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn padc_ibias_source(&self) -> PadcIbiasSourceR {
        PadcIbiasSourceR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn padc_power_ten(&self) -> PadcPowerTenR {
        PadcPowerTenR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn padc_power_tmux(&self) -> PadcPowerTmuxR {
        PadcPowerTmuxR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn codec_vrda_en(&self) -> CodecVrdaEnR {
        CodecVrdaEnR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:14"]
    #[inline(always)]
    pub fn codec_ictrl_vcm(&self) -> CodecIctrlVcmR {
        CodecIctrlVcmR::new(((self.bits >> 13) & 3) as u8)
    }
    #[doc = "Bits 16:18"]
    #[inline(always)]
    pub fn codec_ibias_ctrl(&self) -> CodecIbiasCtrlR {
        CodecIbiasCtrlR::new(((self.bits >> 16) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - MIC Bias Power Up"]
    #[inline(always)]
    pub fn micbias_pu(&mut self) -> MicbiasPuW<'_, CodecPower1Spec> {
        MicbiasPuW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn micbias_chop_en(&mut self) -> MicbiasChopEnW<'_, CodecPower1Spec> {
        MicbiasChopEnW::new(self, 1)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn micbias_clk_chop_sel(&mut self) -> MicbiasClkChopSelW<'_, CodecPower1Spec> {
        MicbiasClkChopSelW::new(self, 2)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn micbias_vsel(&mut self) -> MicbiasVselW<'_, CodecPower1Spec> {
        MicbiasVselW::new(self, 4)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn padc_ibias_source(&mut self) -> PadcIbiasSourceW<'_, CodecPower1Spec> {
        PadcIbiasSourceW::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn padc_power_ten(&mut self) -> PadcPowerTenW<'_, CodecPower1Spec> {
        PadcPowerTenW::new(self, 7)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn padc_power_tmux(&mut self) -> PadcPowerTmuxW<'_, CodecPower1Spec> {
        PadcPowerTmuxW::new(self, 8)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn codec_vrda_en(&mut self) -> CodecVrdaEnW<'_, CodecPower1Spec> {
        CodecVrdaEnW::new(self, 12)
    }
    #[doc = "Bits 13:14"]
    #[inline(always)]
    pub fn codec_ictrl_vcm(&mut self) -> CodecIctrlVcmW<'_, CodecPower1Spec> {
        CodecIctrlVcmW::new(self, 13)
    }
    #[doc = "Bits 16:18"]
    #[inline(always)]
    pub fn codec_ibias_ctrl(&mut self) -> CodecIbiasCtrlW<'_, CodecPower1Spec> {
        CodecIbiasCtrlW::new(self, 16)
    }
}
#[doc = "Analog Codec Power Control 1: MicBias and PGA power\n\nYou can [`read`](crate::Reg::read) this register and get [`codec_power1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`codec_power1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CodecPower1Spec;
impl crate::RegisterSpec for CodecPower1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`codec_power1::R`](R) reader structure"]
impl crate::Readable for CodecPower1Spec {}
#[doc = "`write(|w| ..)` method takes [`codec_power1::W`](W) writer structure"]
impl crate::Writable for CodecPower1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets codec_power1 to value 0"]
impl crate::Resettable for CodecPower1Spec {}
