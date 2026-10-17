#[doc = "Register `TZC_GLB_CTRL_2` reader"]
pub type R = crate::R<TzcGlbCtrl2Spec>;
#[doc = "Register `TZC_GLB_CTRL_2` writer"]
pub type W = crate::W<TzcGlbCtrl2Spec>;
#[doc = "Field `tzc_glb_pwron_rst_tzsid_lock` reader - Power-on reset TZSID lock"]
pub type TzcGlbPwronRstTzsidLockR = crate::BitReader;
#[doc = "Field `tzc_glb_pwron_rst_tzsid_lock` writer - Power-on reset TZSID lock"]
pub type TzcGlbPwronRstTzsidLockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_glb_cpu_reset_tzsid_lock` reader - CPU reset TZSID lock"]
pub type TzcGlbCpuResetTzsidLockR = crate::BitReader;
#[doc = "Field `tzc_glb_cpu_reset_tzsid_lock` writer - CPU reset TZSID lock"]
pub type TzcGlbCpuResetTzsidLockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_glb_sys_reset_tzsid_lock` reader - System reset TZSID lock"]
pub type TzcGlbSysResetTzsidLockR = crate::BitReader;
#[doc = "Field `tzc_glb_sys_reset_tzsid_lock` writer - System reset TZSID lock"]
pub type TzcGlbSysResetTzsidLockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_glb_cpu2_reset_tzsid_lock` reader - CPU2 reset TZSID lock"]
pub type TzcGlbCpu2ResetTzsidLockR = crate::BitReader;
#[doc = "Field `tzc_glb_cpu2_reset_tzsid_lock` writer - CPU2 reset TZSID lock"]
pub type TzcGlbCpu2ResetTzsidLockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_glb_misc_tzsid_lock` reader - Miscellaneous TZSID lock"]
pub type TzcGlbMiscTzsidLockR = crate::BitReader;
#[doc = "Field `tzc_glb_misc_tzsid_lock` writer - Miscellaneous TZSID lock"]
pub type TzcGlbMiscTzsidLockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_glb_sram_tzsid_lock` reader - SRAM TZSID lock"]
pub type TzcGlbSramTzsidLockR = crate::BitReader;
#[doc = "Field `tzc_glb_sram_tzsid_lock` writer - SRAM TZSID lock"]
pub type TzcGlbSramTzsidLockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_glb_swrst_tzsid_lock` reader - Software reset TZSID lock"]
pub type TzcGlbSwrstTzsidLockR = crate::BitReader;
#[doc = "Field `tzc_glb_swrst_tzsid_lock` writer - Software reset TZSID lock"]
pub type TzcGlbSwrstTzsidLockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_glb_bmx_tzsid_lock` reader - BMX TZSID lock"]
pub type TzcGlbBmxTzsidLockR = crate::BitReader;
#[doc = "Field `tzc_glb_bmx_tzsid_lock` writer - BMX TZSID lock"]
pub type TzcGlbBmxTzsidLockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_glb_dbg_tzsid_lock` reader - Debug TZSID lock"]
pub type TzcGlbDbgTzsidLockR = crate::BitReader;
#[doc = "Field `tzc_glb_dbg_tzsid_lock` writer - Debug TZSID lock"]
pub type TzcGlbDbgTzsidLockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_glb_mbist_tzsid_lock` reader - MBIST TZSID lock"]
pub type TzcGlbMbistTzsidLockR = crate::BitReader;
#[doc = "Field `tzc_glb_mbist_tzsid_lock` writer - MBIST TZSID lock"]
pub type TzcGlbMbistTzsidLockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_glb_clk_tzsid_lock` reader - Clock TZSID lock"]
pub type TzcGlbClkTzsidLockR = crate::BitReader;
#[doc = "Field `tzc_glb_clk_tzsid_lock` writer - Clock TZSID lock"]
pub type TzcGlbClkTzsidLockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_glb_int_tzsid_lock` reader - Interrupt TZSID lock"]
pub type TzcGlbIntTzsidLockR = crate::BitReader;
#[doc = "Field `tzc_glb_int_tzsid_lock` writer - Interrupt TZSID lock"]
pub type TzcGlbIntTzsidLockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_glb_pwr_tzsid_lock` reader - Power TZSID lock"]
pub type TzcGlbPwrTzsidLockR = crate::BitReader;
#[doc = "Field `tzc_glb_pwr_tzsid_lock` writer - Power TZSID lock"]
pub type TzcGlbPwrTzsidLockW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Power-on reset TZSID lock"]
    #[inline(always)]
    pub fn tzc_glb_pwron_rst_tzsid_lock(&self) -> TzcGlbPwronRstTzsidLockR {
        TzcGlbPwronRstTzsidLockR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CPU reset TZSID lock"]
    #[inline(always)]
    pub fn tzc_glb_cpu_reset_tzsid_lock(&self) -> TzcGlbCpuResetTzsidLockR {
        TzcGlbCpuResetTzsidLockR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - System reset TZSID lock"]
    #[inline(always)]
    pub fn tzc_glb_sys_reset_tzsid_lock(&self) -> TzcGlbSysResetTzsidLockR {
        TzcGlbSysResetTzsidLockR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - CPU2 reset TZSID lock"]
    #[inline(always)]
    pub fn tzc_glb_cpu2_reset_tzsid_lock(&self) -> TzcGlbCpu2ResetTzsidLockR {
        TzcGlbCpu2ResetTzsidLockR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Miscellaneous TZSID lock"]
    #[inline(always)]
    pub fn tzc_glb_misc_tzsid_lock(&self) -> TzcGlbMiscTzsidLockR {
        TzcGlbMiscTzsidLockR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - SRAM TZSID lock"]
    #[inline(always)]
    pub fn tzc_glb_sram_tzsid_lock(&self) -> TzcGlbSramTzsidLockR {
        TzcGlbSramTzsidLockR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Software reset TZSID lock"]
    #[inline(always)]
    pub fn tzc_glb_swrst_tzsid_lock(&self) -> TzcGlbSwrstTzsidLockR {
        TzcGlbSwrstTzsidLockR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - BMX TZSID lock"]
    #[inline(always)]
    pub fn tzc_glb_bmx_tzsid_lock(&self) -> TzcGlbBmxTzsidLockR {
        TzcGlbBmxTzsidLockR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Debug TZSID lock"]
    #[inline(always)]
    pub fn tzc_glb_dbg_tzsid_lock(&self) -> TzcGlbDbgTzsidLockR {
        TzcGlbDbgTzsidLockR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - MBIST TZSID lock"]
    #[inline(always)]
    pub fn tzc_glb_mbist_tzsid_lock(&self) -> TzcGlbMbistTzsidLockR {
        TzcGlbMbistTzsidLockR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Clock TZSID lock"]
    #[inline(always)]
    pub fn tzc_glb_clk_tzsid_lock(&self) -> TzcGlbClkTzsidLockR {
        TzcGlbClkTzsidLockR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Interrupt TZSID lock"]
    #[inline(always)]
    pub fn tzc_glb_int_tzsid_lock(&self) -> TzcGlbIntTzsidLockR {
        TzcGlbIntTzsidLockR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Power TZSID lock"]
    #[inline(always)]
    pub fn tzc_glb_pwr_tzsid_lock(&self) -> TzcGlbPwrTzsidLockR {
        TzcGlbPwrTzsidLockR::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Power-on reset TZSID lock"]
    #[inline(always)]
    pub fn tzc_glb_pwron_rst_tzsid_lock(
        &mut self,
    ) -> TzcGlbPwronRstTzsidLockW<'_, TzcGlbCtrl2Spec> {
        TzcGlbPwronRstTzsidLockW::new(self, 0)
    }
    #[doc = "Bit 1 - CPU reset TZSID lock"]
    #[inline(always)]
    pub fn tzc_glb_cpu_reset_tzsid_lock(
        &mut self,
    ) -> TzcGlbCpuResetTzsidLockW<'_, TzcGlbCtrl2Spec> {
        TzcGlbCpuResetTzsidLockW::new(self, 1)
    }
    #[doc = "Bit 2 - System reset TZSID lock"]
    #[inline(always)]
    pub fn tzc_glb_sys_reset_tzsid_lock(
        &mut self,
    ) -> TzcGlbSysResetTzsidLockW<'_, TzcGlbCtrl2Spec> {
        TzcGlbSysResetTzsidLockW::new(self, 2)
    }
    #[doc = "Bit 3 - CPU2 reset TZSID lock"]
    #[inline(always)]
    pub fn tzc_glb_cpu2_reset_tzsid_lock(
        &mut self,
    ) -> TzcGlbCpu2ResetTzsidLockW<'_, TzcGlbCtrl2Spec> {
        TzcGlbCpu2ResetTzsidLockW::new(self, 3)
    }
    #[doc = "Bit 4 - Miscellaneous TZSID lock"]
    #[inline(always)]
    pub fn tzc_glb_misc_tzsid_lock(&mut self) -> TzcGlbMiscTzsidLockW<'_, TzcGlbCtrl2Spec> {
        TzcGlbMiscTzsidLockW::new(self, 4)
    }
    #[doc = "Bit 5 - SRAM TZSID lock"]
    #[inline(always)]
    pub fn tzc_glb_sram_tzsid_lock(&mut self) -> TzcGlbSramTzsidLockW<'_, TzcGlbCtrl2Spec> {
        TzcGlbSramTzsidLockW::new(self, 5)
    }
    #[doc = "Bit 6 - Software reset TZSID lock"]
    #[inline(always)]
    pub fn tzc_glb_swrst_tzsid_lock(&mut self) -> TzcGlbSwrstTzsidLockW<'_, TzcGlbCtrl2Spec> {
        TzcGlbSwrstTzsidLockW::new(self, 6)
    }
    #[doc = "Bit 7 - BMX TZSID lock"]
    #[inline(always)]
    pub fn tzc_glb_bmx_tzsid_lock(&mut self) -> TzcGlbBmxTzsidLockW<'_, TzcGlbCtrl2Spec> {
        TzcGlbBmxTzsidLockW::new(self, 7)
    }
    #[doc = "Bit 8 - Debug TZSID lock"]
    #[inline(always)]
    pub fn tzc_glb_dbg_tzsid_lock(&mut self) -> TzcGlbDbgTzsidLockW<'_, TzcGlbCtrl2Spec> {
        TzcGlbDbgTzsidLockW::new(self, 8)
    }
    #[doc = "Bit 9 - MBIST TZSID lock"]
    #[inline(always)]
    pub fn tzc_glb_mbist_tzsid_lock(&mut self) -> TzcGlbMbistTzsidLockW<'_, TzcGlbCtrl2Spec> {
        TzcGlbMbistTzsidLockW::new(self, 9)
    }
    #[doc = "Bit 10 - Clock TZSID lock"]
    #[inline(always)]
    pub fn tzc_glb_clk_tzsid_lock(&mut self) -> TzcGlbClkTzsidLockW<'_, TzcGlbCtrl2Spec> {
        TzcGlbClkTzsidLockW::new(self, 10)
    }
    #[doc = "Bit 11 - Interrupt TZSID lock"]
    #[inline(always)]
    pub fn tzc_glb_int_tzsid_lock(&mut self) -> TzcGlbIntTzsidLockW<'_, TzcGlbCtrl2Spec> {
        TzcGlbIntTzsidLockW::new(self, 11)
    }
    #[doc = "Bit 12 - Power TZSID lock"]
    #[inline(always)]
    pub fn tzc_glb_pwr_tzsid_lock(&mut self) -> TzcGlbPwrTzsidLockW<'_, TzcGlbCtrl2Spec> {
        TzcGlbPwrTzsidLockW::new(self, 12)
    }
}
#[doc = "Global TrustZone Control Register 2 (Lock)\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_glb_ctrl_2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tzc_glb_ctrl_2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TzcGlbCtrl2Spec;
impl crate::RegisterSpec for TzcGlbCtrl2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tzc_glb_ctrl_2::R`](R) reader structure"]
impl crate::Readable for TzcGlbCtrl2Spec {}
#[doc = "`write(|w| ..)` method takes [`tzc_glb_ctrl_2::W`](W) writer structure"]
impl crate::Writable for TzcGlbCtrl2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TZC_GLB_CTRL_2 to value 0"]
impl crate::Resettable for TzcGlbCtrl2Spec {}
