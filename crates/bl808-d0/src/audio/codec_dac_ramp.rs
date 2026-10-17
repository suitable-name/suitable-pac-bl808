#[doc = "Register `codec_dac_ramp` reader"]
pub type R = crate::R<CodecDacRampSpec>;
#[doc = "Register `codec_dac_ramp` writer"]
pub type W = crate::W<CodecDacRampSpec>;
#[doc = "Field `dac_rg_lo_vref_ramp_dcl_en` reader - "]
pub type DacRgLoVrefRampDclEnR = crate::BitReader;
#[doc = "Field `dac_rg_lo_vref_ramp_dcl_en` writer - "]
pub type DacRgLoVrefRampDclEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dac_rg_lineout_vo_pulldown` reader - "]
pub type DacRgLineoutVoPulldownR = crate::BitReader;
#[doc = "Field `dac_rg_lineout_vo_pulldown` writer - "]
pub type DacRgLineoutVoPulldownW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dac_rg_pu_vref_ramp` reader - "]
pub type DacRgPuVrefRampR = crate::BitReader;
#[doc = "Field `dac_rg_pu_vref_ramp` writer - "]
pub type DacRgPuVrefRampW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dac_rg_pu_lineout` reader - "]
pub type DacRgPuLineoutR = crate::BitReader;
#[doc = "Field `dac_rg_pu_lineout` writer - "]
pub type DacRgPuLineoutW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dac_rg_vref_ramp_step_t` reader - "]
pub type DacRgVrefRampStepTR = crate::FieldReader;
#[doc = "Field `dac_rg_vref_ramp_step_t` writer - "]
pub type DacRgVrefRampStepTW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `dac_rg_sw_vref` reader - "]
pub type DacRgSwVrefR = crate::BitReader;
#[doc = "Field `dac_rg_sw_vref` writer - "]
pub type DacRgSwVrefW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dac_rg_vref_ramp_delay` reader - "]
pub type DacRgVrefRampDelayR = crate::FieldReader;
#[doc = "Field `dac_rg_vref_ramp_delay` writer - "]
pub type DacRgVrefRampDelayW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `dac_rg_lo_vref_ramp_dvld` reader - "]
pub type DacRgLoVrefRampDvldR = crate::BitReader;
#[doc = "Field `dac_automute_en` reader - "]
pub type DacAutomuteEnR = crate::BitReader;
#[doc = "Field `dac_automute_en` writer - "]
pub type DacAutomuteEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dg_dac_mute` reader - "]
pub type DgDacMuteR = crate::BitReader;
#[doc = "Field `dg_vo_pulldown` reader - "]
pub type DgVoPulldownR = crate::BitReader;
#[doc = "Field `dg_pu_vref_ramp` reader - "]
pub type DgPuVrefRampR = crate::BitReader;
#[doc = "Field `dg_vref_ramp_step` reader - "]
pub type DgVrefRampStepR = crate::FieldReader<u16>;
#[doc = "Field `dg_sw_vref` reader - "]
pub type DgSwVrefR = crate::BitReader;
#[doc = "Field `dg_dacr_en_lineout` reader - "]
pub type DgDacrEnLineoutR = crate::BitReader;
#[doc = "Field `dg_dacl_en_lineout` reader - "]
pub type DgDaclEnLineoutR = crate::BitReader;
#[doc = "Field `dac_rg_vref_ramp_dcl_mode` reader - "]
pub type DacRgVrefRampDclModeR = crate::FieldReader;
#[doc = "Field `dac_rg_vref_ramp_dcl_mode` writer - "]
pub type DacRgVrefRampDclModeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn dac_rg_lo_vref_ramp_dcl_en(&self) -> DacRgLoVrefRampDclEnR {
        DacRgLoVrefRampDclEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn dac_rg_lineout_vo_pulldown(&self) -> DacRgLineoutVoPulldownR {
        DacRgLineoutVoPulldownR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn dac_rg_pu_vref_ramp(&self) -> DacRgPuVrefRampR {
        DacRgPuVrefRampR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn dac_rg_pu_lineout(&self) -> DacRgPuLineoutR {
        DacRgPuLineoutR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    pub fn dac_rg_vref_ramp_step_t(&self) -> DacRgVrefRampStepTR {
        DacRgVrefRampStepTR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn dac_rg_sw_vref(&self) -> DacRgSwVrefR {
        DacRgSwVrefR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn dac_rg_vref_ramp_delay(&self) -> DacRgVrefRampDelayR {
        DacRgVrefRampDelayR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn dac_rg_lo_vref_ramp_dvld(&self) -> DacRgLoVrefRampDvldR {
        DacRgLoVrefRampDvldR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn dac_automute_en(&self) -> DacAutomuteEnR {
        DacAutomuteEnR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn dg_dac_mute(&self) -> DgDacMuteR {
        DgDacMuteR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn dg_vo_pulldown(&self) -> DgVoPulldownR {
        DgVoPulldownR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn dg_pu_vref_ramp(&self) -> DgPuVrefRampR {
        DgPuVrefRampR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 15:26"]
    #[inline(always)]
    pub fn dg_vref_ramp_step(&self) -> DgVrefRampStepR {
        DgVrefRampStepR::new(((self.bits >> 15) & 0x0fff) as u16)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn dg_sw_vref(&self) -> DgSwVrefR {
        DgSwVrefR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn dg_dacr_en_lineout(&self) -> DgDacrEnLineoutR {
        DgDacrEnLineoutR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn dg_dacl_en_lineout(&self) -> DgDaclEnLineoutR {
        DgDaclEnLineoutR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bits 30:31"]
    #[inline(always)]
    pub fn dac_rg_vref_ramp_dcl_mode(&self) -> DacRgVrefRampDclModeR {
        DacRgVrefRampDclModeR::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn dac_rg_lo_vref_ramp_dcl_en(&mut self) -> DacRgLoVrefRampDclEnW<'_, CodecDacRampSpec> {
        DacRgLoVrefRampDclEnW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn dac_rg_lineout_vo_pulldown(&mut self) -> DacRgLineoutVoPulldownW<'_, CodecDacRampSpec> {
        DacRgLineoutVoPulldownW::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn dac_rg_pu_vref_ramp(&mut self) -> DacRgPuVrefRampW<'_, CodecDacRampSpec> {
        DacRgPuVrefRampW::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn dac_rg_pu_lineout(&mut self) -> DacRgPuLineoutW<'_, CodecDacRampSpec> {
        DacRgPuLineoutW::new(self, 3)
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    pub fn dac_rg_vref_ramp_step_t(&mut self) -> DacRgVrefRampStepTW<'_, CodecDacRampSpec> {
        DacRgVrefRampStepTW::new(self, 4)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn dac_rg_sw_vref(&mut self) -> DacRgSwVrefW<'_, CodecDacRampSpec> {
        DacRgSwVrefW::new(self, 7)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn dac_rg_vref_ramp_delay(&mut self) -> DacRgVrefRampDelayW<'_, CodecDacRampSpec> {
        DacRgVrefRampDelayW::new(self, 8)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn dac_automute_en(&mut self) -> DacAutomuteEnW<'_, CodecDacRampSpec> {
        DacAutomuteEnW::new(self, 11)
    }
    #[doc = "Bits 30:31"]
    #[inline(always)]
    pub fn dac_rg_vref_ramp_dcl_mode(&mut self) -> DacRgVrefRampDclModeW<'_, CodecDacRampSpec> {
        DacRgVrefRampDclModeW::new(self, 30)
    }
}
#[doc = "Analog DAC Ramp Control: Anti-pop hardware ramping for line-out\n\nYou can [`read`](crate::Reg::read) this register and get [`codec_dac_ramp::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`codec_dac_ramp::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CodecDacRampSpec;
impl crate::RegisterSpec for CodecDacRampSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`codec_dac_ramp::R`](R) reader structure"]
impl crate::Readable for CodecDacRampSpec {}
#[doc = "`write(|w| ..)` method takes [`codec_dac_ramp::W`](W) writer structure"]
impl crate::Writable for CodecDacRampSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets codec_dac_ramp to value 0"]
impl crate::Resettable for CodecDacRampSpec {}
