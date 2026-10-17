#[doc = "Register `SD_CAPABILITIES_3` reader"]
pub type R = crate::R<SdCapabilities3Spec>;
#[doc = "Field `SDR50_SUPPORT` reader - "]
pub type Sdr50SupportR = crate::BitReader;
#[doc = "Field `SDR104_SUPPORT` reader - "]
pub type Sdr104SupportR = crate::BitReader;
#[doc = "Field `DDR50_SUPPORT` reader - "]
pub type Ddr50SupportR = crate::BitReader;
#[doc = "Field `DRV_TYPE_A` reader - "]
pub type DrvTypeAR = crate::BitReader;
#[doc = "Field `DRV_TYPE_C` reader - "]
pub type DrvTypeCR = crate::BitReader;
#[doc = "Field `DRV_TYPE_D` reader - "]
pub type DrvTypeDR = crate::BitReader;
#[doc = "Field `TMR_RETUNE` reader - "]
pub type TmrRetuneR = crate::FieldReader;
#[doc = "Field `SDR50_TUNE` reader - "]
pub type Sdr50TuneR = crate::BitReader;
#[doc = "Field `RETUNE_MODES` reader - "]
pub type RetuneModesR = crate::FieldReader;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn sdr50_support(&self) -> Sdr50SupportR {
        Sdr50SupportR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn sdr104_support(&self) -> Sdr104SupportR {
        Sdr104SupportR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn ddr50_support(&self) -> Ddr50SupportR {
        Ddr50SupportR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn drv_type_a(&self) -> DrvTypeAR {
        DrvTypeAR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn drv_type_c(&self) -> DrvTypeCR {
        DrvTypeCR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn drv_type_d(&self) -> DrvTypeDR {
        DrvTypeDR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn tmr_retune(&self) -> TmrRetuneR {
        TmrRetuneR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn sdr50_tune(&self) -> Sdr50TuneR {
        Sdr50TuneR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn retune_modes(&self) -> RetuneModesR {
        RetuneModesR::new(((self.bits >> 14) & 3) as u8)
    }
}
#[doc = "Host Controller Capabilities Register 3 (UHS-I)\n\nYou can [`read`](crate::Reg::read) this register and get [`sd_capabilities_3::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdCapabilities3Spec;
impl crate::RegisterSpec for SdCapabilities3Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`sd_capabilities_3::R`](R) reader structure"]
impl crate::Readable for SdCapabilities3Spec {}
#[doc = "`reset()` method sets SD_CAPABILITIES_3 to value 0"]
impl crate::Resettable for SdCapabilities3Spec {}
