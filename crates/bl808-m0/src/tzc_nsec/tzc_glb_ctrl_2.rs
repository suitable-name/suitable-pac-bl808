#[doc = "Register `TZC_GLB_CTRL_2` reader"]
pub type R = crate::R<TzcGlbCtrl2Spec>;
#[doc = "Field `tzc_glb_pwron_rst_tzsid_lock` reader - "]
pub type TzcGlbPwronRstTzsidLockR = crate::BitReader;
#[doc = "Field `tzc_glb_cpu_reset_tzsid_lock` reader - "]
pub type TzcGlbCpuResetTzsidLockR = crate::BitReader;
#[doc = "Field `tzc_glb_sys_reset_tzsid_lock` reader - "]
pub type TzcGlbSysResetTzsidLockR = crate::BitReader;
#[doc = "Field `tzc_glb_cpu2_reset_tzsid_lock` reader - "]
pub type TzcGlbCpu2ResetTzsidLockR = crate::BitReader;
#[doc = "Field `tzc_glb_misc_tzsid_lock` reader - "]
pub type TzcGlbMiscTzsidLockR = crate::BitReader;
#[doc = "Field `tzc_glb_sram_tzsid_lock` reader - "]
pub type TzcGlbSramTzsidLockR = crate::BitReader;
#[doc = "Field `tzc_glb_swrst_tzsid_lock` reader - "]
pub type TzcGlbSwrstTzsidLockR = crate::BitReader;
#[doc = "Field `tzc_glb_bmx_tzsid_lock` reader - "]
pub type TzcGlbBmxTzsidLockR = crate::BitReader;
#[doc = "Field `tzc_glb_dbg_tzsid_lock` reader - "]
pub type TzcGlbDbgTzsidLockR = crate::BitReader;
#[doc = "Field `tzc_glb_mbist_tzsid_lock` reader - "]
pub type TzcGlbMbistTzsidLockR = crate::BitReader;
#[doc = "Field `tzc_glb_clk_tzsid_lock` reader - "]
pub type TzcGlbClkTzsidLockR = crate::BitReader;
#[doc = "Field `tzc_glb_int_tzsid_lock` reader - "]
pub type TzcGlbIntTzsidLockR = crate::BitReader;
#[doc = "Field `tzc_glb_pwr_tzsid_lock` reader - "]
pub type TzcGlbPwrTzsidLockR = crate::BitReader;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn tzc_glb_pwron_rst_tzsid_lock(&self) -> TzcGlbPwronRstTzsidLockR {
        TzcGlbPwronRstTzsidLockR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn tzc_glb_cpu_reset_tzsid_lock(&self) -> TzcGlbCpuResetTzsidLockR {
        TzcGlbCpuResetTzsidLockR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn tzc_glb_sys_reset_tzsid_lock(&self) -> TzcGlbSysResetTzsidLockR {
        TzcGlbSysResetTzsidLockR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn tzc_glb_cpu2_reset_tzsid_lock(&self) -> TzcGlbCpu2ResetTzsidLockR {
        TzcGlbCpu2ResetTzsidLockR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn tzc_glb_misc_tzsid_lock(&self) -> TzcGlbMiscTzsidLockR {
        TzcGlbMiscTzsidLockR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn tzc_glb_sram_tzsid_lock(&self) -> TzcGlbSramTzsidLockR {
        TzcGlbSramTzsidLockR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn tzc_glb_swrst_tzsid_lock(&self) -> TzcGlbSwrstTzsidLockR {
        TzcGlbSwrstTzsidLockR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn tzc_glb_bmx_tzsid_lock(&self) -> TzcGlbBmxTzsidLockR {
        TzcGlbBmxTzsidLockR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn tzc_glb_dbg_tzsid_lock(&self) -> TzcGlbDbgTzsidLockR {
        TzcGlbDbgTzsidLockR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn tzc_glb_mbist_tzsid_lock(&self) -> TzcGlbMbistTzsidLockR {
        TzcGlbMbistTzsidLockR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn tzc_glb_clk_tzsid_lock(&self) -> TzcGlbClkTzsidLockR {
        TzcGlbClkTzsidLockR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn tzc_glb_int_tzsid_lock(&self) -> TzcGlbIntTzsidLockR {
        TzcGlbIntTzsidLockR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn tzc_glb_pwr_tzsid_lock(&self) -> TzcGlbPwrTzsidLockR {
        TzcGlbPwrTzsidLockR::new(((self.bits >> 12) & 1) != 0)
    }
}
#[doc = "Global Control 2 (System Reset TZID Lock)\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_glb_ctrl_2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TzcGlbCtrl2Spec;
impl crate::RegisterSpec for TzcGlbCtrl2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tzc_glb_ctrl_2::R`](R) reader structure"]
impl crate::Readable for TzcGlbCtrl2Spec {}
#[doc = "`reset()` method sets TZC_GLB_CTRL_2 to value 0"]
impl crate::Resettable for TzcGlbCtrl2Spec {}
