#[doc = "Register `codec_dac` reader"]
pub type R = crate::R<CodecDacSpec>;
#[doc = "Register `codec_dac` writer"]
pub type W = crate::W<CodecDacSpec>;
#[doc = "Field `dacr_lineout_gain` reader - "]
pub type DacrLineoutGainR = crate::FieldReader;
#[doc = "Field `dacr_lineout_gain` writer - "]
pub type DacrLineoutGainW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `dacr_en_lineout` reader - "]
pub type DacrEnLineoutR = crate::BitReader;
#[doc = "Field `dacr_en_lineout` writer - "]
pub type DacrEnLineoutW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dacr_en_dac` reader - "]
pub type DacrEnDacR = crate::BitReader;
#[doc = "Field `dacr_en_dac` writer - "]
pub type DacrEnDacW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dacr_lineout_mode` reader - "]
pub type DacrLineoutModeR = crate::BitReader;
#[doc = "Field `dacr_lineout_mode` writer - "]
pub type DacrLineoutModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dacr_ictrl_lineout` reader - "]
pub type DacrIctrlLineoutR = crate::FieldReader;
#[doc = "Field `dacr_ictrl_lineout` writer - "]
pub type DacrIctrlLineoutW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `dacr_sdac_lineout` reader - "]
pub type DacrSdacLineoutR = crate::BitReader;
#[doc = "Field `dacr_sdac_lineout` writer - "]
pub type DacrSdacLineoutW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dacr_sdebug_lineout` reader - "]
pub type DacrSdebugLineoutR = crate::BitReader;
#[doc = "Field `dacr_sdebug_lineout` writer - "]
pub type DacrSdebugLineoutW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dacr_ictrl_dac` reader - "]
pub type DacrIctrlDacR = crate::FieldReader;
#[doc = "Field `dacr_ictrl_dac` writer - "]
pub type DacrIctrlDacW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `dacr_ckb_en` reader - "]
pub type DacrCkbEnR = crate::BitReader;
#[doc = "Field `dacr_ckb_en` writer - "]
pub type DacrCkbEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dacl_lineout_gain` reader - "]
pub type DaclLineoutGainR = crate::FieldReader;
#[doc = "Field `dacl_lineout_gain` writer - "]
pub type DaclLineoutGainW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `dacl_en_lineout` reader - "]
pub type DaclEnLineoutR = crate::BitReader;
#[doc = "Field `dacl_en_lineout` writer - "]
pub type DaclEnLineoutW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dacl_en_dac` reader - "]
pub type DaclEnDacR = crate::BitReader;
#[doc = "Field `dacl_en_dac` writer - "]
pub type DaclEnDacW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dacl_lineout_mode` reader - "]
pub type DaclLineoutModeR = crate::BitReader;
#[doc = "Field `dacl_lineout_mode` writer - "]
pub type DaclLineoutModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dacl_ictrl_lineout` reader - "]
pub type DaclIctrlLineoutR = crate::FieldReader;
#[doc = "Field `dacl_ictrl_lineout` writer - "]
pub type DaclIctrlLineoutW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `dacl_sdac_lineout` reader - "]
pub type DaclSdacLineoutR = crate::BitReader;
#[doc = "Field `dacl_sdac_lineout` writer - "]
pub type DaclSdacLineoutW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dacl_sdebug_lineout` reader - "]
pub type DaclSdebugLineoutR = crate::BitReader;
#[doc = "Field `dacl_sdebug_lineout` writer - "]
pub type DaclSdebugLineoutW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dacl_ictrl_dac` reader - "]
pub type DaclIctrlDacR = crate::FieldReader;
#[doc = "Field `dacl_ictrl_dac` writer - "]
pub type DaclIctrlDacW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `dacl_ckb_en` reader - "]
pub type DaclCkbEnR = crate::BitReader;
#[doc = "Field `dacl_ckb_en` writer - "]
pub type DaclCkbEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dac_rg_lo_vref_ramp_dcl_busy` reader - "]
pub type DacRgLoVrefRampDclBusyR = crate::BitReader;
impl R {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn dacr_lineout_gain(&self) -> DacrLineoutGainR {
        DacrLineoutGainR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn dacr_en_lineout(&self) -> DacrEnLineoutR {
        DacrEnLineoutR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn dacr_en_dac(&self) -> DacrEnDacR {
        DacrEnDacR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn dacr_lineout_mode(&self) -> DacrLineoutModeR {
        DacrLineoutModeR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn dacr_ictrl_lineout(&self) -> DacrIctrlLineoutR {
        DacrIctrlLineoutR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn dacr_sdac_lineout(&self) -> DacrSdacLineoutR {
        DacrSdacLineoutR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn dacr_sdebug_lineout(&self) -> DacrSdebugLineoutR {
        DacrSdebugLineoutR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn dacr_ictrl_dac(&self) -> DacrIctrlDacR {
        DacrIctrlDacR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn dacr_ckb_en(&self) -> DacrCkbEnR {
        DacrCkbEnR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 16:18"]
    #[inline(always)]
    pub fn dacl_lineout_gain(&self) -> DaclLineoutGainR {
        DaclLineoutGainR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn dacl_en_lineout(&self) -> DaclEnLineoutR {
        DaclEnLineoutR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn dacl_en_dac(&self) -> DaclEnDacR {
        DaclEnDacR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn dacl_lineout_mode(&self) -> DaclLineoutModeR {
        DaclLineoutModeR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn dacl_ictrl_lineout(&self) -> DaclIctrlLineoutR {
        DaclIctrlLineoutR::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn dacl_sdac_lineout(&self) -> DaclSdacLineoutR {
        DaclSdacLineoutR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn dacl_sdebug_lineout(&self) -> DaclSdebugLineoutR {
        DaclSdebugLineoutR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:29"]
    #[inline(always)]
    pub fn dacl_ictrl_dac(&self) -> DaclIctrlDacR {
        DaclIctrlDacR::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn dacl_ckb_en(&self) -> DaclCkbEnR {
        DaclCkbEnR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn dac_rg_lo_vref_ramp_dcl_busy(&self) -> DacRgLoVrefRampDclBusyR {
        DacRgLoVrefRampDclBusyR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn dacr_lineout_gain(&mut self) -> DacrLineoutGainW<'_, CodecDacSpec> {
        DacrLineoutGainW::new(self, 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn dacr_en_lineout(&mut self) -> DacrEnLineoutW<'_, CodecDacSpec> {
        DacrEnLineoutW::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn dacr_en_dac(&mut self) -> DacrEnDacW<'_, CodecDacSpec> {
        DacrEnDacW::new(self, 4)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn dacr_lineout_mode(&mut self) -> DacrLineoutModeW<'_, CodecDacSpec> {
        DacrLineoutModeW::new(self, 6)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn dacr_ictrl_lineout(&mut self) -> DacrIctrlLineoutW<'_, CodecDacSpec> {
        DacrIctrlLineoutW::new(self, 8)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn dacr_sdac_lineout(&mut self) -> DacrSdacLineoutW<'_, CodecDacSpec> {
        DacrSdacLineoutW::new(self, 10)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn dacr_sdebug_lineout(&mut self) -> DacrSdebugLineoutW<'_, CodecDacSpec> {
        DacrSdebugLineoutW::new(self, 11)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn dacr_ictrl_dac(&mut self) -> DacrIctrlDacW<'_, CodecDacSpec> {
        DacrIctrlDacW::new(self, 12)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn dacr_ckb_en(&mut self) -> DacrCkbEnW<'_, CodecDacSpec> {
        DacrCkbEnW::new(self, 14)
    }
    #[doc = "Bits 16:18"]
    #[inline(always)]
    pub fn dacl_lineout_gain(&mut self) -> DaclLineoutGainW<'_, CodecDacSpec> {
        DaclLineoutGainW::new(self, 16)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn dacl_en_lineout(&mut self) -> DaclEnLineoutW<'_, CodecDacSpec> {
        DaclEnLineoutW::new(self, 19)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn dacl_en_dac(&mut self) -> DaclEnDacW<'_, CodecDacSpec> {
        DaclEnDacW::new(self, 20)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn dacl_lineout_mode(&mut self) -> DaclLineoutModeW<'_, CodecDacSpec> {
        DaclLineoutModeW::new(self, 22)
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn dacl_ictrl_lineout(&mut self) -> DaclIctrlLineoutW<'_, CodecDacSpec> {
        DaclIctrlLineoutW::new(self, 24)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn dacl_sdac_lineout(&mut self) -> DaclSdacLineoutW<'_, CodecDacSpec> {
        DaclSdacLineoutW::new(self, 26)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn dacl_sdebug_lineout(&mut self) -> DaclSdebugLineoutW<'_, CodecDacSpec> {
        DaclSdebugLineoutW::new(self, 27)
    }
    #[doc = "Bits 28:29"]
    #[inline(always)]
    pub fn dacl_ictrl_dac(&mut self) -> DaclIctrlDacW<'_, CodecDacSpec> {
        DaclIctrlDacW::new(self, 28)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn dacl_ckb_en(&mut self) -> DaclCkbEnW<'_, CodecDacSpec> {
        DaclCkbEnW::new(self, 30)
    }
}
#[doc = "Analog DAC Control: Line-out gain and source selection\n\nYou can [`read`](crate::Reg::read) this register and get [`codec_dac::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`codec_dac::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CodecDacSpec;
impl crate::RegisterSpec for CodecDacSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`codec_dac::R`](R) reader structure"]
impl crate::Readable for CodecDacSpec {}
#[doc = "`write(|w| ..)` method takes [`codec_dac::W`](W) writer structure"]
impl crate::Writable for CodecDacSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets codec_dac to value 0"]
impl crate::Resettable for CodecDacSpec {}
