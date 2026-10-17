#[doc = "Register `APMEMORY_PSRAM_STATUS` reader"]
pub type R = crate::R<ApmemoryPsramStatusSpec>;
#[doc = "Field `sts_ap_burst_length` reader - APMemory burst length status"]
pub type StsApBurstLengthR = crate::FieldReader;
#[doc = "Field `sts_ap_burst_type` reader - APMemory burst type status"]
pub type StsApBurstTypeR = crate::BitReader;
#[doc = "Field `sts_ap_rbx` reader - APMemory read-burst-exit status"]
pub type StsApRbxR = crate::BitReader;
#[doc = "Field `sts_ap_x16_mode` reader - APMemory X16 mode status"]
pub type StsApX16ModeR = crate::BitReader;
#[doc = "Field `sts_ap_pasr` reader - APMemory Partial Array Self Refresh status"]
pub type StsApPasrR = crate::FieldReader;
#[doc = "Field `sts_ap_w_latency_code` reader - APMemory write latency code status"]
pub type StsApWLatencyCodeR = crate::FieldReader;
#[doc = "Field `sts_ap_drive_st` reader - APMemory drive strength status"]
pub type StsApDriveStR = crate::FieldReader;
#[doc = "Field `sts_ap_rf` reader - APMemory refresh frequency status"]
pub type StsApRfR = crate::FieldReader;
#[doc = "Field `sts_ap_r_latency_code` reader - APMemory read latency code status"]
pub type StsApRLatencyCodeR = crate::FieldReader;
#[doc = "Field `sts_ap_r_latency_type` reader - APMemory read latency type status"]
pub type StsApRLatencyTypeR = crate::BitReader;
impl R {
    #[doc = "Bits 0:1 - APMemory burst length status"]
    #[inline(always)]
    pub fn sts_ap_burst_length(&self) -> StsApBurstLengthR {
        StsApBurstLengthR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 4 - APMemory burst type status"]
    #[inline(always)]
    pub fn sts_ap_burst_type(&self) -> StsApBurstTypeR {
        StsApBurstTypeR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - APMemory read-burst-exit status"]
    #[inline(always)]
    pub fn sts_ap_rbx(&self) -> StsApRbxR {
        StsApRbxR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - APMemory X16 mode status"]
    #[inline(always)]
    pub fn sts_ap_x16_mode(&self) -> StsApX16ModeR {
        StsApX16ModeR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 8:10 - APMemory Partial Array Self Refresh status"]
    #[inline(always)]
    pub fn sts_ap_pasr(&self) -> StsApPasrR {
        StsApPasrR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 12:14 - APMemory write latency code status"]
    #[inline(always)]
    pub fn sts_ap_w_latency_code(&self) -> StsApWLatencyCodeR {
        StsApWLatencyCodeR::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 16:17 - APMemory drive strength status"]
    #[inline(always)]
    pub fn sts_ap_drive_st(&self) -> StsApDriveStR {
        StsApDriveStR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - APMemory refresh frequency status"]
    #[inline(always)]
    pub fn sts_ap_rf(&self) -> StsApRfR {
        StsApRfR::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:22 - APMemory read latency code status"]
    #[inline(always)]
    pub fn sts_ap_r_latency_code(&self) -> StsApRLatencyCodeR {
        StsApRLatencyCodeR::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bit 24 - APMemory read latency type status"]
    #[inline(always)]
    pub fn sts_ap_r_latency_type(&self) -> StsApRLatencyTypeR {
        StsApRLatencyTypeR::new(((self.bits >> 24) & 1) != 0)
    }
}
#[doc = "APMemory Specific Status\n\nYou can [`read`](crate::Reg::read) this register and get [`apmemory_psram_status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ApmemoryPsramStatusSpec;
impl crate::RegisterSpec for ApmemoryPsramStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apmemory_psram_status::R`](R) reader structure"]
impl crate::Readable for ApmemoryPsramStatusSpec {}
#[doc = "`reset()` method sets APMEMORY_PSRAM_STATUS to value 0x0021_2011"]
impl crate::Resettable for ApmemoryPsramStatusSpec {
    const RESET_VALUE: u32 = 0x0021_2011;
}
