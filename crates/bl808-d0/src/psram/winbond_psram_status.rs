#[doc = "Register `WINBOND_PSRAM_STATUS` reader"]
pub type R = crate::R<WinbondPsramStatusSpec>;
#[doc = "Field `sts_wb_latency` reader - Winbond latency status"]
pub type StsWbLatencyR = crate::FieldReader;
#[doc = "Field `sts_wb_drive_st` reader - Winbond drive strength status"]
pub type StsWbDriveStR = crate::FieldReader;
#[doc = "Field `sts_wb_hybrid_en` reader - Winbond hybrid enable status"]
pub type StsWbHybridEnR = crate::BitReader;
#[doc = "Field `sts_wb_burst_length` reader - Winbond burst length status"]
pub type StsWbBurstLengthR = crate::FieldReader;
#[doc = "Field `sts_wb_fix_latency` reader - Winbond fixed latency status"]
pub type StsWbFixLatencyR = crate::BitReader;
#[doc = "Field `sts_wb_dpd_dis` reader - Winbond Deep Power Down disable status"]
pub type StsWbDpdDisR = crate::BitReader;
#[doc = "Field `sts_wb_pasr` reader - Winbond Partial Array Self Refresh status"]
pub type StsWbPasrR = crate::FieldReader;
#[doc = "Field `sts_wb_hybrid_slp` reader - Winbond hybrid sleep status"]
pub type StsWbHybridSlpR = crate::BitReader;
#[doc = "Field `sts_wb_mclk_type` reader - Winbond MCLK type status"]
pub type StsWbMclkTypeR = crate::BitReader;
impl R {
    #[doc = "Bits 0:3 - Winbond latency status"]
    #[inline(always)]
    pub fn sts_wb_latency(&self) -> StsWbLatencyR {
        StsWbLatencyR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:6 - Winbond drive strength status"]
    #[inline(always)]
    pub fn sts_wb_drive_st(&self) -> StsWbDriveStR {
        StsWbDriveStR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - Winbond hybrid enable status"]
    #[inline(always)]
    pub fn sts_wb_hybrid_en(&self) -> StsWbHybridEnR {
        StsWbHybridEnR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10 - Winbond burst length status"]
    #[inline(always)]
    pub fn sts_wb_burst_length(&self) -> StsWbBurstLengthR {
        StsWbBurstLengthR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 12 - Winbond fixed latency status"]
    #[inline(always)]
    pub fn sts_wb_fix_latency(&self) -> StsWbFixLatencyR {
        StsWbFixLatencyR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Winbond Deep Power Down disable status"]
    #[inline(always)]
    pub fn sts_wb_dpd_dis(&self) -> StsWbDpdDisR {
        StsWbDpdDisR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 16:20 - Winbond Partial Array Self Refresh status"]
    #[inline(always)]
    pub fn sts_wb_pasr(&self) -> StsWbPasrR {
        StsWbPasrR::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bit 24 - Winbond hybrid sleep status"]
    #[inline(always)]
    pub fn sts_wb_hybrid_slp(&self) -> StsWbHybridSlpR {
        StsWbHybridSlpR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 30 - Winbond MCLK type status"]
    #[inline(always)]
    pub fn sts_wb_mclk_type(&self) -> StsWbMclkTypeR {
        StsWbMclkTypeR::new(((self.bits >> 30) & 1) != 0)
    }
}
#[doc = "Winbond Specific Status\n\nYou can [`read`](crate::Reg::read) this register and get [`winbond_psram_status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WinbondPsramStatusSpec;
impl crate::RegisterSpec for WinbondPsramStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`winbond_psram_status::R`](R) reader structure"]
impl crate::Readable for WinbondPsramStatusSpec {}
#[doc = "`reset()` method sets WINBOND_PSRAM_STATUS to value 0x4000_3382"]
impl crate::Resettable for WinbondPsramStatusSpec {
    const RESET_VALUE: u32 = 0x4000_3382;
}
