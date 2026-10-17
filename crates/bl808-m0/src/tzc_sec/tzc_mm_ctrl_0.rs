#[doc = "Register `TZC_MM_CTRL_0` reader"]
pub type R = crate::R<TzcMmCtrl0Spec>;
#[doc = "Register `TZC_MM_CTRL_0` writer"]
pub type W = crate::W<TzcMmCtrl0Spec>;
#[doc = "Field `tzc_mm_pwron_rst_tzsid_en` reader - MM Power-on reset TZSID enable"]
pub type TzcMmPwronRstTzsidEnR = crate::FieldReader;
#[doc = "Field `tzc_mm_pwron_rst_tzsid_en` writer - MM Power-on reset TZSID enable"]
pub type TzcMmPwronRstTzsidEnW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `tzc_mm_cpu0_reset_tzsid_en` reader - MM CPU0 reset TZSID enable"]
pub type TzcMmCpu0ResetTzsidEnR = crate::FieldReader;
#[doc = "Field `tzc_mm_cpu0_reset_tzsid_en` writer - MM CPU0 reset TZSID enable"]
pub type TzcMmCpu0ResetTzsidEnW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `tzc_mm_sys_reset_tzsid_en` reader - MM system reset TZSID enable"]
pub type TzcMmSysResetTzsidEnR = crate::FieldReader;
#[doc = "Field `tzc_mm_sys_reset_tzsid_en` writer - MM system reset TZSID enable"]
pub type TzcMmSysResetTzsidEnW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `tzc_mm_cpu0_tzsid_en` reader - MM CPU0 TZSID enable"]
pub type TzcMmCpu0TzsidEnR = crate::FieldReader;
#[doc = "Field `tzc_mm_cpu0_tzsid_en` writer - MM CPU0 TZSID enable"]
pub type TzcMmCpu0TzsidEnW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `tzc_mm_sram_tzsid_en` reader - MM SRAM TZSID enable"]
pub type TzcMmSramTzsidEnR = crate::FieldReader;
#[doc = "Field `tzc_mm_sram_tzsid_en` writer - MM SRAM TZSID enable"]
pub type TzcMmSramTzsidEnW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `tzc_mm_swrst_tzsid_en` reader - MM software reset TZSID enable"]
pub type TzcMmSwrstTzsidEnR = crate::FieldReader;
#[doc = "Field `tzc_mm_swrst_tzsid_en` writer - MM software reset TZSID enable"]
pub type TzcMmSwrstTzsidEnW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `tzc_mm_clk_tzsid_en` reader - MM clock TZSID enable"]
pub type TzcMmClkTzsidEnR = crate::FieldReader;
#[doc = "Field `tzc_mm_clk_tzsid_en` writer - MM clock TZSID enable"]
pub type TzcMmClkTzsidEnW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - MM Power-on reset TZSID enable"]
    #[inline(always)]
    pub fn tzc_mm_pwron_rst_tzsid_en(&self) -> TzcMmPwronRstTzsidEnR {
        TzcMmPwronRstTzsidEnR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - MM CPU0 reset TZSID enable"]
    #[inline(always)]
    pub fn tzc_mm_cpu0_reset_tzsid_en(&self) -> TzcMmCpu0ResetTzsidEnR {
        TzcMmCpu0ResetTzsidEnR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - MM system reset TZSID enable"]
    #[inline(always)]
    pub fn tzc_mm_sys_reset_tzsid_en(&self) -> TzcMmSysResetTzsidEnR {
        TzcMmSysResetTzsidEnR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - MM CPU0 TZSID enable"]
    #[inline(always)]
    pub fn tzc_mm_cpu0_tzsid_en(&self) -> TzcMmCpu0TzsidEnR {
        TzcMmCpu0TzsidEnR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 10:11 - MM SRAM TZSID enable"]
    #[inline(always)]
    pub fn tzc_mm_sram_tzsid_en(&self) -> TzcMmSramTzsidEnR {
        TzcMmSramTzsidEnR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - MM software reset TZSID enable"]
    #[inline(always)]
    pub fn tzc_mm_swrst_tzsid_en(&self) -> TzcMmSwrstTzsidEnR {
        TzcMmSwrstTzsidEnR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 20:21 - MM clock TZSID enable"]
    #[inline(always)]
    pub fn tzc_mm_clk_tzsid_en(&self) -> TzcMmClkTzsidEnR {
        TzcMmClkTzsidEnR::new(((self.bits >> 20) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - MM Power-on reset TZSID enable"]
    #[inline(always)]
    pub fn tzc_mm_pwron_rst_tzsid_en(&mut self) -> TzcMmPwronRstTzsidEnW<'_, TzcMmCtrl0Spec> {
        TzcMmPwronRstTzsidEnW::new(self, 0)
    }
    #[doc = "Bits 2:3 - MM CPU0 reset TZSID enable"]
    #[inline(always)]
    pub fn tzc_mm_cpu0_reset_tzsid_en(&mut self) -> TzcMmCpu0ResetTzsidEnW<'_, TzcMmCtrl0Spec> {
        TzcMmCpu0ResetTzsidEnW::new(self, 2)
    }
    #[doc = "Bits 4:5 - MM system reset TZSID enable"]
    #[inline(always)]
    pub fn tzc_mm_sys_reset_tzsid_en(&mut self) -> TzcMmSysResetTzsidEnW<'_, TzcMmCtrl0Spec> {
        TzcMmSysResetTzsidEnW::new(self, 4)
    }
    #[doc = "Bits 6:7 - MM CPU0 TZSID enable"]
    #[inline(always)]
    pub fn tzc_mm_cpu0_tzsid_en(&mut self) -> TzcMmCpu0TzsidEnW<'_, TzcMmCtrl0Spec> {
        TzcMmCpu0TzsidEnW::new(self, 6)
    }
    #[doc = "Bits 10:11 - MM SRAM TZSID enable"]
    #[inline(always)]
    pub fn tzc_mm_sram_tzsid_en(&mut self) -> TzcMmSramTzsidEnW<'_, TzcMmCtrl0Spec> {
        TzcMmSramTzsidEnW::new(self, 10)
    }
    #[doc = "Bits 12:13 - MM software reset TZSID enable"]
    #[inline(always)]
    pub fn tzc_mm_swrst_tzsid_en(&mut self) -> TzcMmSwrstTzsidEnW<'_, TzcMmCtrl0Spec> {
        TzcMmSwrstTzsidEnW::new(self, 12)
    }
    #[doc = "Bits 20:21 - MM clock TZSID enable"]
    #[inline(always)]
    pub fn tzc_mm_clk_tzsid_en(&mut self) -> TzcMmClkTzsidEnW<'_, TzcMmCtrl0Spec> {
        TzcMmClkTzsidEnW::new(self, 20)
    }
}
#[doc = "Multimedia TrustZone Control Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_mm_ctrl_0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tzc_mm_ctrl_0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TzcMmCtrl0Spec;
impl crate::RegisterSpec for TzcMmCtrl0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tzc_mm_ctrl_0::R`](R) reader structure"]
impl crate::Readable for TzcMmCtrl0Spec {}
#[doc = "`write(|w| ..)` method takes [`tzc_mm_ctrl_0::W`](W) writer structure"]
impl crate::Writable for TzcMmCtrl0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TZC_MM_CTRL_0 to value 0x0030_33ff"]
impl crate::Resettable for TzcMmCtrl0Spec {
    const RESET_VALUE: u32 = 0x0030_33ff;
}
