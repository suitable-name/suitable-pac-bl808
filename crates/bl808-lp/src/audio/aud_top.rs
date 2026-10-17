#[doc = "Register `aud_top` reader"]
pub type R = crate::R<AudTopSpec>;
#[doc = "Register `aud_top` writer"]
pub type W = crate::W<AudTopSpec>;
#[doc = "Field `ckg_en` reader - Audio Clock Gate Enable"]
pub type CkgEnR = crate::BitReader;
#[doc = "Field `ckg_en` writer - Audio Clock Gate Enable"]
pub type CkgEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dac_itf_inv_sel` reader - DAC Interface Invert Select"]
pub type DacItfInvSelR = crate::BitReader;
#[doc = "Field `dac_itf_inv_sel` writer - DAC Interface Invert Select"]
pub type DacItfInvSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `adc_itf_inv_sel` reader - ADC Interface Invert Select"]
pub type AdcItfInvSelR = crate::BitReader;
#[doc = "Field `adc_itf_inv_sel` writer - ADC Interface Invert Select"]
pub type AdcItfInvSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pdm_itf_inv_sel` reader - PDM Interface Invert Select"]
pub type PdmItfInvSelR = crate::BitReader;
#[doc = "Field `pdm_itf_inv_sel` writer - PDM Interface Invert Select"]
pub type PdmItfInvSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dac_rate` reader - DAC Sample Rate selection (e.g., 8k to 48k)"]
pub type DacRateR = crate::FieldReader;
#[doc = "Field `dac_rate` writer - DAC Sample Rate selection (e.g., 8k to 48k)"]
pub type DacRateW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `adc_rate` reader - ADC Sample Rate selection (e.g., 8k to 48k)"]
pub type AdcRateR = crate::FieldReader;
#[doc = "Field `adc_rate` writer - ADC Sample Rate selection (e.g., 8k to 48k)"]
pub type AdcRateW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bit 0 - Audio Clock Gate Enable"]
    #[inline(always)]
    pub fn ckg_en(&self) -> CkgEnR {
        CkgEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DAC Interface Invert Select"]
    #[inline(always)]
    pub fn dac_itf_inv_sel(&self) -> DacItfInvSelR {
        DacItfInvSelR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ADC Interface Invert Select"]
    #[inline(always)]
    pub fn adc_itf_inv_sel(&self) -> AdcItfInvSelR {
        AdcItfInvSelR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - PDM Interface Invert Select"]
    #[inline(always)]
    pub fn pdm_itf_inv_sel(&self) -> PdmItfInvSelR {
        PdmItfInvSelR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 24:26 - DAC Sample Rate selection (e.g., 8k to 48k)"]
    #[inline(always)]
    pub fn dac_rate(&self) -> DacRateR {
        DacRateR::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 28:30 - ADC Sample Rate selection (e.g., 8k to 48k)"]
    #[inline(always)]
    pub fn adc_rate(&self) -> AdcRateR {
        AdcRateR::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Audio Clock Gate Enable"]
    #[inline(always)]
    pub fn ckg_en(&mut self) -> CkgEnW<'_, AudTopSpec> {
        CkgEnW::new(self, 0)
    }
    #[doc = "Bit 1 - DAC Interface Invert Select"]
    #[inline(always)]
    pub fn dac_itf_inv_sel(&mut self) -> DacItfInvSelW<'_, AudTopSpec> {
        DacItfInvSelW::new(self, 1)
    }
    #[doc = "Bit 2 - ADC Interface Invert Select"]
    #[inline(always)]
    pub fn adc_itf_inv_sel(&mut self) -> AdcItfInvSelW<'_, AudTopSpec> {
        AdcItfInvSelW::new(self, 2)
    }
    #[doc = "Bit 3 - PDM Interface Invert Select"]
    #[inline(always)]
    pub fn pdm_itf_inv_sel(&mut self) -> PdmItfInvSelW<'_, AudTopSpec> {
        PdmItfInvSelW::new(self, 3)
    }
    #[doc = "Bits 24:26 - DAC Sample Rate selection (e.g., 8k to 48k)"]
    #[inline(always)]
    pub fn dac_rate(&mut self) -> DacRateW<'_, AudTopSpec> {
        DacRateW::new(self, 24)
    }
    #[doc = "Bits 28:30 - ADC Sample Rate selection (e.g., 8k to 48k)"]
    #[inline(always)]
    pub fn adc_rate(&mut self) -> AdcRateW<'_, AudTopSpec> {
        AdcRateW::new(self, 28)
    }
}
#[doc = "Audio Top Level Configuration: Clock gating and sample rate management\n\nYou can [`read`](crate::Reg::read) this register and get [`aud_top::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aud_top::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AudTopSpec;
impl crate::RegisterSpec for AudTopSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aud_top::R`](R) reader structure"]
impl crate::Readable for AudTopSpec {}
#[doc = "`write(|w| ..)` method takes [`aud_top::W`](W) writer structure"]
impl crate::Writable for AudTopSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets aud_top to value 0"]
impl crate::Resettable for AudTopSpec {}
