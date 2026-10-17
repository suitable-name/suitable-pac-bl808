#[doc = "Register `padc_ana_cmd` reader"]
pub type R = crate::R<PadcAnaCmdSpec>;
#[doc = "Register `padc_ana_cmd` writer"]
pub type W = crate::W<PadcAnaCmdSpec>;
#[doc = "Field `padc_sdm_lp_en` reader - "]
pub type PadcSdmLpEnR = crate::BitReader;
#[doc = "Field `padc_sdm_lp_en` writer - "]
pub type PadcSdmLpEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `padc_pga_lp_en` reader - "]
pub type PadcPgaLpEnR = crate::BitReader;
#[doc = "Field `padc_pga_lp_en` writer - "]
pub type PadcPgaLpEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `padc_ictrl_diag` reader - "]
pub type PadcIctrlDiagR = crate::FieldReader;
#[doc = "Field `padc_ictrl_diag` writer - "]
pub type PadcIctrlDiagW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `padc_pga_gain` reader - "]
pub type PadcPgaGainR = crate::FieldReader;
#[doc = "Field `padc_pga_gain` writer - "]
pub type PadcPgaGainW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `padc_channel_sel` reader - "]
pub type PadcChannelSelR = crate::FieldReader;
#[doc = "Field `padc_channel_sel` writer - "]
pub type PadcChannelSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `padc_channel_en` reader - "]
pub type PadcChannelEnR = crate::BitReader;
#[doc = "Field `padc_channel_en` writer - "]
pub type PadcChannelEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `padc_conv` reader - Start ADC Conversion"]
pub type PadcConvR = crate::BitReader;
#[doc = "Field `padc_conv` writer - Start ADC Conversion"]
pub type PadcConvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `padc_sdm_pu` reader - SDM Power Up"]
pub type PadcSdmPuR = crate::BitReader;
#[doc = "Field `padc_sdm_pu` writer - SDM Power Up"]
pub type PadcSdmPuW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `padc_pga_pu` reader - PGA Power Up"]
pub type PadcPgaPuR = crate::BitReader;
#[doc = "Field `padc_pga_pu` writer - PGA Power Up"]
pub type PadcPgaPuW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn padc_sdm_lp_en(&self) -> PadcSdmLpEnR {
        PadcSdmLpEnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn padc_pga_lp_en(&self) -> PadcPgaLpEnR {
        PadcPgaLpEnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 9:10"]
    #[inline(always)]
    pub fn padc_ictrl_diag(&self) -> PadcIctrlDiagR {
        PadcIctrlDiagR::new(((self.bits >> 9) & 3) as u8)
    }
    #[doc = "Bits 11:13"]
    #[inline(always)]
    pub fn padc_pga_gain(&self) -> PadcPgaGainR {
        PadcPgaGainR::new(((self.bits >> 11) & 7) as u8)
    }
    #[doc = "Bits 21:22"]
    #[inline(always)]
    pub fn padc_channel_sel(&self) -> PadcChannelSelR {
        PadcChannelSelR::new(((self.bits >> 21) & 3) as u8)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn padc_channel_en(&self) -> PadcChannelEnR {
        PadcChannelEnR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 28 - Start ADC Conversion"]
    #[inline(always)]
    pub fn padc_conv(&self) -> PadcConvR {
        PadcConvR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 30 - SDM Power Up"]
    #[inline(always)]
    pub fn padc_sdm_pu(&self) -> PadcSdmPuR {
        PadcSdmPuR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - PGA Power Up"]
    #[inline(always)]
    pub fn padc_pga_pu(&self) -> PadcPgaPuR {
        PadcPgaPuR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn padc_sdm_lp_en(&mut self) -> PadcSdmLpEnW<'_, PadcAnaCmdSpec> {
        PadcSdmLpEnW::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn padc_pga_lp_en(&mut self) -> PadcPgaLpEnW<'_, PadcAnaCmdSpec> {
        PadcPgaLpEnW::new(self, 4)
    }
    #[doc = "Bits 9:10"]
    #[inline(always)]
    pub fn padc_ictrl_diag(&mut self) -> PadcIctrlDiagW<'_, PadcAnaCmdSpec> {
        PadcIctrlDiagW::new(self, 9)
    }
    #[doc = "Bits 11:13"]
    #[inline(always)]
    pub fn padc_pga_gain(&mut self) -> PadcPgaGainW<'_, PadcAnaCmdSpec> {
        PadcPgaGainW::new(self, 11)
    }
    #[doc = "Bits 21:22"]
    #[inline(always)]
    pub fn padc_channel_sel(&mut self) -> PadcChannelSelW<'_, PadcAnaCmdSpec> {
        PadcChannelSelW::new(self, 21)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn padc_channel_en(&mut self) -> PadcChannelEnW<'_, PadcAnaCmdSpec> {
        PadcChannelEnW::new(self, 23)
    }
    #[doc = "Bit 28 - Start ADC Conversion"]
    #[inline(always)]
    pub fn padc_conv(&mut self) -> PadcConvW<'_, PadcAnaCmdSpec> {
        PadcConvW::new(self, 28)
    }
    #[doc = "Bit 30 - SDM Power Up"]
    #[inline(always)]
    pub fn padc_sdm_pu(&mut self) -> PadcSdmPuW<'_, PadcAnaCmdSpec> {
        PadcSdmPuW::new(self, 30)
    }
    #[doc = "Bit 31 - PGA Power Up"]
    #[inline(always)]
    pub fn padc_pga_pu(&mut self) -> PadcPgaPuW<'_, PadcAnaCmdSpec> {
        PadcPgaPuW::new(self, 31)
    }
}
#[doc = "Passthrough ADC Analog Command: Start conversion and power triggers\n\nYou can [`read`](crate::Reg::read) this register and get [`padc_ana_cmd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`padc_ana_cmd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PadcAnaCmdSpec;
impl crate::RegisterSpec for PadcAnaCmdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`padc_ana_cmd::R`](R) reader structure"]
impl crate::Readable for PadcAnaCmdSpec {}
#[doc = "`write(|w| ..)` method takes [`padc_ana_cmd::W`](W) writer structure"]
impl crate::Writable for PadcAnaCmdSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets padc_ana_cmd to value 0"]
impl crate::Resettable for PadcAnaCmdSpec {}
