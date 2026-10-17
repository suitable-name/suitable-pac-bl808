#[doc = "Register `TZC_GLB_CTRL_0` reader"]
pub type R = crate::R<TzcGlbCtrl0Spec>;
#[doc = "Register `TZC_GLB_CTRL_0` writer"]
pub type W = crate::W<TzcGlbCtrl0Spec>;
#[doc = "Field `tzc_glb_pwron_rst_tzsid_en` reader - Power-on reset TZSID enable"]
pub type TzcGlbPwronRstTzsidEnR = crate::FieldReader;
#[doc = "Field `tzc_glb_pwron_rst_tzsid_en` writer - Power-on reset TZSID enable"]
pub type TzcGlbPwronRstTzsidEnW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `tzc_glb_cpu_reset_tzsid_en` reader - CPU reset TZSID enable"]
pub type TzcGlbCpuResetTzsidEnR = crate::FieldReader;
#[doc = "Field `tzc_glb_cpu_reset_tzsid_en` writer - CPU reset TZSID enable"]
pub type TzcGlbCpuResetTzsidEnW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `tzc_glb_sys_reset_tzsid_en` reader - System reset TZSID enable"]
pub type TzcGlbSysResetTzsidEnR = crate::FieldReader;
#[doc = "Field `tzc_glb_sys_reset_tzsid_en` writer - System reset TZSID enable"]
pub type TzcGlbSysResetTzsidEnW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `tzc_glb_cpu2_reset_tzsid_en` reader - CPU2 reset TZSID enable"]
pub type TzcGlbCpu2ResetTzsidEnR = crate::FieldReader;
#[doc = "Field `tzc_glb_cpu2_reset_tzsid_en` writer - CPU2 reset TZSID enable"]
pub type TzcGlbCpu2ResetTzsidEnW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `tzc_glb_misc_tzsid_en` reader - Miscellaneous TZSID enable"]
pub type TzcGlbMiscTzsidEnR = crate::FieldReader;
#[doc = "Field `tzc_glb_misc_tzsid_en` writer - Miscellaneous TZSID enable"]
pub type TzcGlbMiscTzsidEnW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `tzc_glb_sram_tzsid_en` reader - SRAM TZSID enable"]
pub type TzcGlbSramTzsidEnR = crate::FieldReader;
#[doc = "Field `tzc_glb_sram_tzsid_en` writer - SRAM TZSID enable"]
pub type TzcGlbSramTzsidEnW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `tzc_glb_swrst_tzsid_en` reader - Software reset TZSID enable"]
pub type TzcGlbSwrstTzsidEnR = crate::FieldReader;
#[doc = "Field `tzc_glb_swrst_tzsid_en` writer - Software reset TZSID enable"]
pub type TzcGlbSwrstTzsidEnW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `tzc_glb_bmx_tzsid_en` reader - BMX TZSID enable"]
pub type TzcGlbBmxTzsidEnR = crate::FieldReader;
#[doc = "Field `tzc_glb_bmx_tzsid_en` writer - BMX TZSID enable"]
pub type TzcGlbBmxTzsidEnW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `tzc_glb_dbg_tzsid_en` reader - Debug TZSID enable"]
pub type TzcGlbDbgTzsidEnR = crate::FieldReader;
#[doc = "Field `tzc_glb_dbg_tzsid_en` writer - Debug TZSID enable"]
pub type TzcGlbDbgTzsidEnW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `tzc_glb_mbist_tzsid_en` reader - MBIST TZSID enable"]
pub type TzcGlbMbistTzsidEnR = crate::FieldReader;
#[doc = "Field `tzc_glb_mbist_tzsid_en` writer - MBIST TZSID enable"]
pub type TzcGlbMbistTzsidEnW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `tzc_glb_clk_tzsid_en` reader - Clock TZSID enable"]
pub type TzcGlbClkTzsidEnR = crate::FieldReader;
#[doc = "Field `tzc_glb_clk_tzsid_en` writer - Clock TZSID enable"]
pub type TzcGlbClkTzsidEnW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `tzc_glb_int_tzsid_en` reader - Interrupt TZSID enable"]
pub type TzcGlbIntTzsidEnR = crate::FieldReader;
#[doc = "Field `tzc_glb_int_tzsid_en` writer - Interrupt TZSID enable"]
pub type TzcGlbIntTzsidEnW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `tzc_glb_pwr_tzsid_en` reader - Power TZSID enable"]
pub type TzcGlbPwrTzsidEnR = crate::FieldReader;
#[doc = "Field `tzc_glb_pwr_tzsid_en` writer - Power TZSID enable"]
pub type TzcGlbPwrTzsidEnW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - Power-on reset TZSID enable"]
    #[inline(always)]
    pub fn tzc_glb_pwron_rst_tzsid_en(&self) -> TzcGlbPwronRstTzsidEnR {
        TzcGlbPwronRstTzsidEnR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - CPU reset TZSID enable"]
    #[inline(always)]
    pub fn tzc_glb_cpu_reset_tzsid_en(&self) -> TzcGlbCpuResetTzsidEnR {
        TzcGlbCpuResetTzsidEnR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - System reset TZSID enable"]
    #[inline(always)]
    pub fn tzc_glb_sys_reset_tzsid_en(&self) -> TzcGlbSysResetTzsidEnR {
        TzcGlbSysResetTzsidEnR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - CPU2 reset TZSID enable"]
    #[inline(always)]
    pub fn tzc_glb_cpu2_reset_tzsid_en(&self) -> TzcGlbCpu2ResetTzsidEnR {
        TzcGlbCpu2ResetTzsidEnR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Miscellaneous TZSID enable"]
    #[inline(always)]
    pub fn tzc_glb_misc_tzsid_en(&self) -> TzcGlbMiscTzsidEnR {
        TzcGlbMiscTzsidEnR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - SRAM TZSID enable"]
    #[inline(always)]
    pub fn tzc_glb_sram_tzsid_en(&self) -> TzcGlbSramTzsidEnR {
        TzcGlbSramTzsidEnR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Software reset TZSID enable"]
    #[inline(always)]
    pub fn tzc_glb_swrst_tzsid_en(&self) -> TzcGlbSwrstTzsidEnR {
        TzcGlbSwrstTzsidEnR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - BMX TZSID enable"]
    #[inline(always)]
    pub fn tzc_glb_bmx_tzsid_en(&self) -> TzcGlbBmxTzsidEnR {
        TzcGlbBmxTzsidEnR::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Debug TZSID enable"]
    #[inline(always)]
    pub fn tzc_glb_dbg_tzsid_en(&self) -> TzcGlbDbgTzsidEnR {
        TzcGlbDbgTzsidEnR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - MBIST TZSID enable"]
    #[inline(always)]
    pub fn tzc_glb_mbist_tzsid_en(&self) -> TzcGlbMbistTzsidEnR {
        TzcGlbMbistTzsidEnR::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - Clock TZSID enable"]
    #[inline(always)]
    pub fn tzc_glb_clk_tzsid_en(&self) -> TzcGlbClkTzsidEnR {
        TzcGlbClkTzsidEnR::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - Interrupt TZSID enable"]
    #[inline(always)]
    pub fn tzc_glb_int_tzsid_en(&self) -> TzcGlbIntTzsidEnR {
        TzcGlbIntTzsidEnR::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - Power TZSID enable"]
    #[inline(always)]
    pub fn tzc_glb_pwr_tzsid_en(&self) -> TzcGlbPwrTzsidEnR {
        TzcGlbPwrTzsidEnR::new(((self.bits >> 24) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Power-on reset TZSID enable"]
    #[inline(always)]
    pub fn tzc_glb_pwron_rst_tzsid_en(&mut self) -> TzcGlbPwronRstTzsidEnW<'_, TzcGlbCtrl0Spec> {
        TzcGlbPwronRstTzsidEnW::new(self, 0)
    }
    #[doc = "Bits 2:3 - CPU reset TZSID enable"]
    #[inline(always)]
    pub fn tzc_glb_cpu_reset_tzsid_en(&mut self) -> TzcGlbCpuResetTzsidEnW<'_, TzcGlbCtrl0Spec> {
        TzcGlbCpuResetTzsidEnW::new(self, 2)
    }
    #[doc = "Bits 4:5 - System reset TZSID enable"]
    #[inline(always)]
    pub fn tzc_glb_sys_reset_tzsid_en(&mut self) -> TzcGlbSysResetTzsidEnW<'_, TzcGlbCtrl0Spec> {
        TzcGlbSysResetTzsidEnW::new(self, 4)
    }
    #[doc = "Bits 6:7 - CPU2 reset TZSID enable"]
    #[inline(always)]
    pub fn tzc_glb_cpu2_reset_tzsid_en(&mut self) -> TzcGlbCpu2ResetTzsidEnW<'_, TzcGlbCtrl0Spec> {
        TzcGlbCpu2ResetTzsidEnW::new(self, 6)
    }
    #[doc = "Bits 8:9 - Miscellaneous TZSID enable"]
    #[inline(always)]
    pub fn tzc_glb_misc_tzsid_en(&mut self) -> TzcGlbMiscTzsidEnW<'_, TzcGlbCtrl0Spec> {
        TzcGlbMiscTzsidEnW::new(self, 8)
    }
    #[doc = "Bits 10:11 - SRAM TZSID enable"]
    #[inline(always)]
    pub fn tzc_glb_sram_tzsid_en(&mut self) -> TzcGlbSramTzsidEnW<'_, TzcGlbCtrl0Spec> {
        TzcGlbSramTzsidEnW::new(self, 10)
    }
    #[doc = "Bits 12:13 - Software reset TZSID enable"]
    #[inline(always)]
    pub fn tzc_glb_swrst_tzsid_en(&mut self) -> TzcGlbSwrstTzsidEnW<'_, TzcGlbCtrl0Spec> {
        TzcGlbSwrstTzsidEnW::new(self, 12)
    }
    #[doc = "Bits 14:15 - BMX TZSID enable"]
    #[inline(always)]
    pub fn tzc_glb_bmx_tzsid_en(&mut self) -> TzcGlbBmxTzsidEnW<'_, TzcGlbCtrl0Spec> {
        TzcGlbBmxTzsidEnW::new(self, 14)
    }
    #[doc = "Bits 16:17 - Debug TZSID enable"]
    #[inline(always)]
    pub fn tzc_glb_dbg_tzsid_en(&mut self) -> TzcGlbDbgTzsidEnW<'_, TzcGlbCtrl0Spec> {
        TzcGlbDbgTzsidEnW::new(self, 16)
    }
    #[doc = "Bits 18:19 - MBIST TZSID enable"]
    #[inline(always)]
    pub fn tzc_glb_mbist_tzsid_en(&mut self) -> TzcGlbMbistTzsidEnW<'_, TzcGlbCtrl0Spec> {
        TzcGlbMbistTzsidEnW::new(self, 18)
    }
    #[doc = "Bits 20:21 - Clock TZSID enable"]
    #[inline(always)]
    pub fn tzc_glb_clk_tzsid_en(&mut self) -> TzcGlbClkTzsidEnW<'_, TzcGlbCtrl0Spec> {
        TzcGlbClkTzsidEnW::new(self, 20)
    }
    #[doc = "Bits 22:23 - Interrupt TZSID enable"]
    #[inline(always)]
    pub fn tzc_glb_int_tzsid_en(&mut self) -> TzcGlbIntTzsidEnW<'_, TzcGlbCtrl0Spec> {
        TzcGlbIntTzsidEnW::new(self, 22)
    }
    #[doc = "Bits 24:25 - Power TZSID enable"]
    #[inline(always)]
    pub fn tzc_glb_pwr_tzsid_en(&mut self) -> TzcGlbPwrTzsidEnW<'_, TzcGlbCtrl0Spec> {
        TzcGlbPwrTzsidEnW::new(self, 24)
    }
}
#[doc = "Global TrustZone Control Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_glb_ctrl_0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tzc_glb_ctrl_0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TzcGlbCtrl0Spec;
impl crate::RegisterSpec for TzcGlbCtrl0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tzc_glb_ctrl_0::R`](R) reader structure"]
impl crate::Readable for TzcGlbCtrl0Spec {}
#[doc = "`write(|w| ..)` method takes [`tzc_glb_ctrl_0::W`](W) writer structure"]
impl crate::Writable for TzcGlbCtrl0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TZC_GLB_CTRL_0 to value 0x03ff_ffff"]
impl crate::Resettable for TzcGlbCtrl0Spec {
    const RESET_VALUE: u32 = 0x03ff_ffff;
}
