#[doc = "Register `TZ_MM_CLKRST` reader"]
pub type R = crate::R<TzMmClkrstSpec>;
#[doc = "Field `tzc_mm_swrst_lock` reader - "]
pub type TzcMmSwrstLockR = crate::BitReader;
#[doc = "Field `tzc_mm_sys_reset_lock` reader - "]
pub type TzcMmSysResetLockR = crate::BitReader;
#[doc = "Field `tzc_mm_pwron_rst_lock` reader - "]
pub type TzcMmPwronRstLockR = crate::BitReader;
#[doc = "Field `tzc_mm_cpu0_reset_lock` reader - "]
pub type TzcMmCpu0ResetLockR = crate::BitReader;
#[doc = "Field `tzc_mm_clk_lock` reader - "]
pub type TzcMmClkLockR = crate::BitReader;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn tzc_mm_swrst_lock(&self) -> TzcMmSwrstLockR {
        TzcMmSwrstLockR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn tzc_mm_sys_reset_lock(&self) -> TzcMmSysResetLockR {
        TzcMmSysResetLockR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn tzc_mm_pwron_rst_lock(&self) -> TzcMmPwronRstLockR {
        TzcMmPwronRstLockR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn tzc_mm_cpu0_reset_lock(&self) -> TzcMmCpu0ResetLockR {
        TzcMmCpu0ResetLockR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn tzc_mm_clk_lock(&self) -> TzcMmClkLockR {
        TzcMmClkLockR::new(((self.bits >> 4) & 1) != 0)
    }
}
#[doc = "TrustZone MM Clock/Reset Lock Status\n\nYou can [`read`](crate::Reg::read) this register and get [`tz_mm_clkrst::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TzMmClkrstSpec;
impl crate::RegisterSpec for TzMmClkrstSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tz_mm_clkrst::R`](R) reader structure"]
impl crate::Readable for TzMmClkrstSpec {}
#[doc = "`reset()` method sets TZ_MM_CLKRST to value 0"]
impl crate::Resettable for TzMmClkrstSpec {}
