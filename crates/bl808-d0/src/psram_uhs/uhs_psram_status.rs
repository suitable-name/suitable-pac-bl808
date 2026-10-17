#[doc = "Register `UHS_PSRAM_STATUS` reader"]
pub type R = crate::R<UhsPsramStatusSpec>;
#[doc = "Field `sts_uhs_latency` reader - UHS latency status"]
pub type StsUhsLatencyR = crate::FieldReader;
#[doc = "Field `sts_uhs_drive_st` reader - UHS drive strength status"]
pub type StsUhsDriveStR = crate::FieldReader;
#[doc = "Field `sts_uhs_bl_16` reader - UHS BL16 status"]
pub type StsUhsBl16R = crate::BitReader;
#[doc = "Field `sts_uhs_bl_32` reader - UHS BL32 status"]
pub type StsUhsBl32R = crate::BitReader;
#[doc = "Field `sts_uhs_bl_64` reader - UHS BL64 status"]
pub type StsUhsBl64R = crate::BitReader;
impl R {
    #[doc = "Bits 0:2 - UHS latency status"]
    #[inline(always)]
    pub fn sts_uhs_latency(&self) -> StsUhsLatencyR {
        StsUhsLatencyR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:7 - UHS drive strength status"]
    #[inline(always)]
    pub fn sts_uhs_drive_st(&self) -> StsUhsDriveStR {
        StsUhsDriveStR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - UHS BL16 status"]
    #[inline(always)]
    pub fn sts_uhs_bl_16(&self) -> StsUhsBl16R {
        StsUhsBl16R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - UHS BL32 status"]
    #[inline(always)]
    pub fn sts_uhs_bl_32(&self) -> StsUhsBl32R {
        StsUhsBl32R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - UHS BL64 status"]
    #[inline(always)]
    pub fn sts_uhs_bl_64(&self) -> StsUhsBl64R {
        StsUhsBl64R::new(((self.bits >> 10) & 1) != 0)
    }
}
#[doc = "UHS PSRAM Device Parameters Status\n\nYou can [`read`](crate::Reg::read) this register and get [`uhs_psram_status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UhsPsramStatusSpec;
impl crate::RegisterSpec for UhsPsramStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uhs_psram_status::R`](R) reader structure"]
impl crate::Readable for UhsPsramStatusSpec {}
#[doc = "`reset()` method sets UHS_PSRAM_STATUS to value 0"]
impl crate::Resettable for UhsPsramStatusSpec {}
