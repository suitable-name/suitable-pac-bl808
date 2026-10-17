#[doc = "Register `TZC_MM_CTRL_2` reader"]
pub type R = crate::R<TzcMmCtrl2Spec>;
#[doc = "Field `tzc_mm_pwron_rst_tzsid_lock` reader - "]
pub type TzcMmPwronRstTzsidLockR = crate::BitReader;
#[doc = "Field `tzc_mm_cpu0_reset_tzsid_lock` reader - "]
pub type TzcMmCpu0ResetTzsidLockR = crate::BitReader;
#[doc = "Field `tzc_mm_sys_reset_tzsid_lock` reader - "]
pub type TzcMmSysResetTzsidLockR = crate::BitReader;
#[doc = "Field `tzc_mm_cpu0_tzsid_lock` reader - "]
pub type TzcMmCpu0TzsidLockR = crate::BitReader;
#[doc = "Field `tzc_mm_sram_tzsid_lock` reader - "]
pub type TzcMmSramTzsidLockR = crate::BitReader;
#[doc = "Field `tzc_mm_swrst_tzsid_lock` reader - "]
pub type TzcMmSwrstTzsidLockR = crate::BitReader;
#[doc = "Field `tzc_mm_clk_tzsid_lock` reader - "]
pub type TzcMmClkTzsidLockR = crate::BitReader;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn tzc_mm_pwron_rst_tzsid_lock(&self) -> TzcMmPwronRstTzsidLockR {
        TzcMmPwronRstTzsidLockR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn tzc_mm_cpu0_reset_tzsid_lock(&self) -> TzcMmCpu0ResetTzsidLockR {
        TzcMmCpu0ResetTzsidLockR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn tzc_mm_sys_reset_tzsid_lock(&self) -> TzcMmSysResetTzsidLockR {
        TzcMmSysResetTzsidLockR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn tzc_mm_cpu0_tzsid_lock(&self) -> TzcMmCpu0TzsidLockR {
        TzcMmCpu0TzsidLockR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn tzc_mm_sram_tzsid_lock(&self) -> TzcMmSramTzsidLockR {
        TzcMmSramTzsidLockR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn tzc_mm_swrst_tzsid_lock(&self) -> TzcMmSwrstTzsidLockR {
        TzcMmSwrstTzsidLockR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn tzc_mm_clk_tzsid_lock(&self) -> TzcMmClkTzsidLockR {
        TzcMmClkTzsidLockR::new(((self.bits >> 10) & 1) != 0)
    }
}
#[doc = "MM Control 2 (MM System Reset TZID Lock)\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_mm_ctrl_2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TzcMmCtrl2Spec;
impl crate::RegisterSpec for TzcMmCtrl2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tzc_mm_ctrl_2::R`](R) reader structure"]
impl crate::Readable for TzcMmCtrl2Spec {}
#[doc = "`reset()` method sets TZC_MM_CTRL_2 to value 0"]
impl crate::Resettable for TzcMmCtrl2Spec {}
