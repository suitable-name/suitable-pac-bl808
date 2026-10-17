#[doc = "Register `TZC_MM_CTRL_0` reader"]
pub type R = crate::R<TzcMmCtrl0Spec>;
#[doc = "Field `tzc_mm_pwron_rst_tzsid_en` reader - "]
pub type TzcMmPwronRstTzsidEnR = crate::FieldReader;
#[doc = "Field `tzc_mm_cpu0_reset_tzsid_en` reader - "]
pub type TzcMmCpu0ResetTzsidEnR = crate::FieldReader;
#[doc = "Field `tzc_mm_sys_reset_tzsid_en` reader - "]
pub type TzcMmSysResetTzsidEnR = crate::FieldReader;
#[doc = "Field `tzc_mm_cpu0_tzsid_en` reader - "]
pub type TzcMmCpu0TzsidEnR = crate::FieldReader;
#[doc = "Field `tzc_mm_sram_tzsid_en` reader - "]
pub type TzcMmSramTzsidEnR = crate::FieldReader;
#[doc = "Field `tzc_mm_swrst_tzsid_en` reader - "]
pub type TzcMmSwrstTzsidEnR = crate::FieldReader;
#[doc = "Field `tzc_mm_clk_tzsid_en` reader - "]
pub type TzcMmClkTzsidEnR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn tzc_mm_pwron_rst_tzsid_en(&self) -> TzcMmPwronRstTzsidEnR {
        TzcMmPwronRstTzsidEnR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn tzc_mm_cpu0_reset_tzsid_en(&self) -> TzcMmCpu0ResetTzsidEnR {
        TzcMmCpu0ResetTzsidEnR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn tzc_mm_sys_reset_tzsid_en(&self) -> TzcMmSysResetTzsidEnR {
        TzcMmSysResetTzsidEnR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn tzc_mm_cpu0_tzsid_en(&self) -> TzcMmCpu0TzsidEnR {
        TzcMmCpu0TzsidEnR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn tzc_mm_sram_tzsid_en(&self) -> TzcMmSramTzsidEnR {
        TzcMmSramTzsidEnR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn tzc_mm_swrst_tzsid_en(&self) -> TzcMmSwrstTzsidEnR {
        TzcMmSwrstTzsidEnR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    pub fn tzc_mm_clk_tzsid_en(&self) -> TzcMmClkTzsidEnR {
        TzcMmClkTzsidEnR::new(((self.bits >> 20) & 3) as u8)
    }
}
#[doc = "MM Control 0 (MM System Reset TZID Enable)\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_mm_ctrl_0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TzcMmCtrl0Spec;
impl crate::RegisterSpec for TzcMmCtrl0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tzc_mm_ctrl_0::R`](R) reader structure"]
impl crate::Readable for TzcMmCtrl0Spec {}
#[doc = "`reset()` method sets TZC_MM_CTRL_0 to value 0"]
impl crate::Resettable for TzcMmCtrl0Spec {}
