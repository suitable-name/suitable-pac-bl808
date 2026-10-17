#[doc = "Register `PRESET_VALUE_FOR_SDR50` reader"]
pub type R = crate::R<PresetValueForSdr50Spec>;
#[doc = "Field `SDCLK_FREQ_SEL_VAL` reader - "]
pub type SdclkFreqSelValR = crate::FieldReader<u16>;
#[doc = "Field `CLKGEN_SEL_VAL` reader - "]
pub type ClkgenSelValR = crate::BitReader;
#[doc = "Field `DRV_STRENGTH_VAL` reader - "]
pub type DrvStrengthValR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn sdclk_freq_sel_val(&self) -> SdclkFreqSelValR {
        SdclkFreqSelValR::new(self.bits & 0x03ff)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn clkgen_sel_val(&self) -> ClkgenSelValR {
        ClkgenSelValR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn drv_strength_val(&self) -> DrvStrengthValR {
        DrvStrengthValR::new(((self.bits >> 14) & 3) as u8)
    }
}
#[doc = "Preset Value for SDR50\n\nYou can [`read`](crate::Reg::read) this register and get [`preset_value_for_sdr50::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PresetValueForSdr50Spec;
impl crate::RegisterSpec for PresetValueForSdr50Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`preset_value_for_sdr50::R`](R) reader structure"]
impl crate::Readable for PresetValueForSdr50Spec {}
#[doc = "`reset()` method sets PRESET_VALUE_FOR_SDR50 to value 0"]
impl crate::Resettable for PresetValueForSdr50Spec {}
