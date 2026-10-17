#[doc = "Register `TZC_GLB_CTRL_0` reader"]
pub type R = crate::R<TzcGlbCtrl0Spec>;
#[doc = "Field `tzc_glb_pwron_rst_tzsid_en` reader - "]
pub type TzcGlbPwronRstTzsidEnR = crate::FieldReader;
#[doc = "Field `tzc_glb_cpu_reset_tzsid_en` reader - "]
pub type TzcGlbCpuResetTzsidEnR = crate::FieldReader;
#[doc = "Field `tzc_glb_sys_reset_tzsid_en` reader - "]
pub type TzcGlbSysResetTzsidEnR = crate::FieldReader;
#[doc = "Field `tzc_glb_cpu2_reset_tzsid_en` reader - "]
pub type TzcGlbCpu2ResetTzsidEnR = crate::FieldReader;
#[doc = "Field `tzc_glb_misc_tzsid_en` reader - "]
pub type TzcGlbMiscTzsidEnR = crate::FieldReader;
#[doc = "Field `tzc_glb_sram_tzsid_en` reader - "]
pub type TzcGlbSramTzsidEnR = crate::FieldReader;
#[doc = "Field `tzc_glb_swrst_tzsid_en` reader - "]
pub type TzcGlbSwrstTzsidEnR = crate::FieldReader;
#[doc = "Field `tzc_glb_bmx_tzsid_en` reader - "]
pub type TzcGlbBmxTzsidEnR = crate::FieldReader;
#[doc = "Field `tzc_glb_dbg_tzsid_en` reader - "]
pub type TzcGlbDbgTzsidEnR = crate::FieldReader;
#[doc = "Field `tzc_glb_mbist_tzsid_en` reader - "]
pub type TzcGlbMbistTzsidEnR = crate::FieldReader;
#[doc = "Field `tzc_glb_clk_tzsid_en` reader - "]
pub type TzcGlbClkTzsidEnR = crate::FieldReader;
#[doc = "Field `tzc_glb_int_tzsid_en` reader - "]
pub type TzcGlbIntTzsidEnR = crate::FieldReader;
#[doc = "Field `tzc_glb_pwr_tzsid_en` reader - "]
pub type TzcGlbPwrTzsidEnR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn tzc_glb_pwron_rst_tzsid_en(&self) -> TzcGlbPwronRstTzsidEnR {
        TzcGlbPwronRstTzsidEnR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn tzc_glb_cpu_reset_tzsid_en(&self) -> TzcGlbCpuResetTzsidEnR {
        TzcGlbCpuResetTzsidEnR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn tzc_glb_sys_reset_tzsid_en(&self) -> TzcGlbSysResetTzsidEnR {
        TzcGlbSysResetTzsidEnR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn tzc_glb_cpu2_reset_tzsid_en(&self) -> TzcGlbCpu2ResetTzsidEnR {
        TzcGlbCpu2ResetTzsidEnR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn tzc_glb_misc_tzsid_en(&self) -> TzcGlbMiscTzsidEnR {
        TzcGlbMiscTzsidEnR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn tzc_glb_sram_tzsid_en(&self) -> TzcGlbSramTzsidEnR {
        TzcGlbSramTzsidEnR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn tzc_glb_swrst_tzsid_en(&self) -> TzcGlbSwrstTzsidEnR {
        TzcGlbSwrstTzsidEnR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn tzc_glb_bmx_tzsid_en(&self) -> TzcGlbBmxTzsidEnR {
        TzcGlbBmxTzsidEnR::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn tzc_glb_dbg_tzsid_en(&self) -> TzcGlbDbgTzsidEnR {
        TzcGlbDbgTzsidEnR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    pub fn tzc_glb_mbist_tzsid_en(&self) -> TzcGlbMbistTzsidEnR {
        TzcGlbMbistTzsidEnR::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    pub fn tzc_glb_clk_tzsid_en(&self) -> TzcGlbClkTzsidEnR {
        TzcGlbClkTzsidEnR::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23"]
    #[inline(always)]
    pub fn tzc_glb_int_tzsid_en(&self) -> TzcGlbIntTzsidEnR {
        TzcGlbIntTzsidEnR::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn tzc_glb_pwr_tzsid_en(&self) -> TzcGlbPwrTzsidEnR {
        TzcGlbPwrTzsidEnR::new(((self.bits >> 24) & 3) as u8)
    }
}
#[doc = "Global Control 0 (System Reset TZID Enable)\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_glb_ctrl_0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TzcGlbCtrl0Spec;
impl crate::RegisterSpec for TzcGlbCtrl0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tzc_glb_ctrl_0::R`](R) reader structure"]
impl crate::Readable for TzcGlbCtrl0Spec {}
#[doc = "`reset()` method sets TZC_GLB_CTRL_0 to value 0"]
impl crate::Resettable for TzcGlbCtrl0Spec {}
