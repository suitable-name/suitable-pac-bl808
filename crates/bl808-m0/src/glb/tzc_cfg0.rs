#[doc = "Register `TZC_CFG0` reader"]
pub type R = crate::R<TzcCfg0Spec>;
#[doc = "Register `TZC_CFG0` writer"]
pub type W = crate::W<TzcCfg0Spec>;
#[doc = "Field `tzc_glb_pwron_rst_lock` reader - "]
pub type TzcGlbPwronRstLockR = crate::BitReader;
#[doc = "Field `tzc_glb_pwron_rst_lock` writer - "]
pub type TzcGlbPwronRstLockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_glb_cpu_reset_lock` reader - "]
pub type TzcGlbCpuResetLockR = crate::BitReader;
#[doc = "Field `tzc_glb_cpu_reset_lock` writer - "]
pub type TzcGlbCpuResetLockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_glb_sys_reset_lock` reader - "]
pub type TzcGlbSysResetLockR = crate::BitReader;
#[doc = "Field `tzc_glb_sys_reset_lock` writer - "]
pub type TzcGlbSysResetLockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_glb_cpu2_reset_lock` reader - "]
pub type TzcGlbCpu2ResetLockR = crate::BitReader;
#[doc = "Field `tzc_glb_cpu2_reset_lock` writer - "]
pub type TzcGlbCpu2ResetLockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_glb_pwr_lock` reader - "]
pub type TzcGlbPwrLockR = crate::BitReader;
#[doc = "Field `tzc_glb_pwr_lock` writer - "]
pub type TzcGlbPwrLockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_glb_int_lock` reader - "]
pub type TzcGlbIntLockR = crate::BitReader;
#[doc = "Field `tzc_glb_int_lock` writer - "]
pub type TzcGlbIntLockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_glb_cpupll_lock` reader - "]
pub type TzcGlbCpupllLockR = crate::BitReader;
#[doc = "Field `tzc_glb_cpupll_lock` writer - "]
pub type TzcGlbCpupllLockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_glb_misc_lock` reader - "]
pub type TzcGlbMiscLockR = crate::BitReader;
#[doc = "Field `tzc_glb_misc_lock` writer - "]
pub type TzcGlbMiscLockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_glb_sram_lock` reader - "]
pub type TzcGlbSramLockR = crate::BitReader;
#[doc = "Field `tzc_glb_sram_lock` writer - "]
pub type TzcGlbSramLockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_glb_swrst_lock` reader - "]
pub type TzcGlbSwrstLockR = crate::BitReader;
#[doc = "Field `tzc_glb_swrst_lock` writer - "]
pub type TzcGlbSwrstLockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_glb_bmx_lock` reader - "]
pub type TzcGlbBmxLockR = crate::BitReader;
#[doc = "Field `tzc_glb_bmx_lock` writer - "]
pub type TzcGlbBmxLockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_glb_dbg_lock` reader - "]
pub type TzcGlbDbgLockR = crate::BitReader;
#[doc = "Field `tzc_glb_dbg_lock` writer - "]
pub type TzcGlbDbgLockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_glb_mbist_lock` reader - "]
pub type TzcGlbMbistLockR = crate::BitReader;
#[doc = "Field `tzc_glb_mbist_lock` writer - "]
pub type TzcGlbMbistLockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_glb_clk_lock` reader - "]
pub type TzcGlbClkLockR = crate::BitReader;
#[doc = "Field `tzc_glb_clk_lock` writer - "]
pub type TzcGlbClkLockW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn tzc_glb_pwron_rst_lock(&self) -> TzcGlbPwronRstLockR {
        TzcGlbPwronRstLockR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn tzc_glb_cpu_reset_lock(&self) -> TzcGlbCpuResetLockR {
        TzcGlbCpuResetLockR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn tzc_glb_sys_reset_lock(&self) -> TzcGlbSysResetLockR {
        TzcGlbSysResetLockR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn tzc_glb_cpu2_reset_lock(&self) -> TzcGlbCpu2ResetLockR {
        TzcGlbCpu2ResetLockR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn tzc_glb_pwr_lock(&self) -> TzcGlbPwrLockR {
        TzcGlbPwrLockR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn tzc_glb_int_lock(&self) -> TzcGlbIntLockR {
        TzcGlbIntLockR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn tzc_glb_cpupll_lock(&self) -> TzcGlbCpupllLockR {
        TzcGlbCpupllLockR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn tzc_glb_misc_lock(&self) -> TzcGlbMiscLockR {
        TzcGlbMiscLockR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn tzc_glb_sram_lock(&self) -> TzcGlbSramLockR {
        TzcGlbSramLockR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn tzc_glb_swrst_lock(&self) -> TzcGlbSwrstLockR {
        TzcGlbSwrstLockR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn tzc_glb_bmx_lock(&self) -> TzcGlbBmxLockR {
        TzcGlbBmxLockR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn tzc_glb_dbg_lock(&self) -> TzcGlbDbgLockR {
        TzcGlbDbgLockR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn tzc_glb_mbist_lock(&self) -> TzcGlbMbistLockR {
        TzcGlbMbistLockR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn tzc_glb_clk_lock(&self) -> TzcGlbClkLockR {
        TzcGlbClkLockR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn tzc_glb_pwron_rst_lock(&mut self) -> TzcGlbPwronRstLockW<'_, TzcCfg0Spec> {
        TzcGlbPwronRstLockW::new(self, 12)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn tzc_glb_cpu_reset_lock(&mut self) -> TzcGlbCpuResetLockW<'_, TzcCfg0Spec> {
        TzcGlbCpuResetLockW::new(self, 13)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn tzc_glb_sys_reset_lock(&mut self) -> TzcGlbSysResetLockW<'_, TzcCfg0Spec> {
        TzcGlbSysResetLockW::new(self, 14)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn tzc_glb_cpu2_reset_lock(&mut self) -> TzcGlbCpu2ResetLockW<'_, TzcCfg0Spec> {
        TzcGlbCpu2ResetLockW::new(self, 15)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn tzc_glb_pwr_lock(&mut self) -> TzcGlbPwrLockW<'_, TzcCfg0Spec> {
        TzcGlbPwrLockW::new(self, 21)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn tzc_glb_int_lock(&mut self) -> TzcGlbIntLockW<'_, TzcCfg0Spec> {
        TzcGlbIntLockW::new(self, 22)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn tzc_glb_cpupll_lock(&mut self) -> TzcGlbCpupllLockW<'_, TzcCfg0Spec> {
        TzcGlbCpupllLockW::new(self, 24)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn tzc_glb_misc_lock(&mut self) -> TzcGlbMiscLockW<'_, TzcCfg0Spec> {
        TzcGlbMiscLockW::new(self, 25)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn tzc_glb_sram_lock(&mut self) -> TzcGlbSramLockW<'_, TzcCfg0Spec> {
        TzcGlbSramLockW::new(self, 26)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn tzc_glb_swrst_lock(&mut self) -> TzcGlbSwrstLockW<'_, TzcCfg0Spec> {
        TzcGlbSwrstLockW::new(self, 27)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn tzc_glb_bmx_lock(&mut self) -> TzcGlbBmxLockW<'_, TzcCfg0Spec> {
        TzcGlbBmxLockW::new(self, 28)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn tzc_glb_dbg_lock(&mut self) -> TzcGlbDbgLockW<'_, TzcCfg0Spec> {
        TzcGlbDbgLockW::new(self, 29)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn tzc_glb_mbist_lock(&mut self) -> TzcGlbMbistLockW<'_, TzcCfg0Spec> {
        TzcGlbMbistLockW::new(self, 30)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn tzc_glb_clk_lock(&mut self) -> TzcGlbClkLockW<'_, TzcCfg0Spec> {
        TzcGlbClkLockW::new(self, 31)
    }
}
#[doc = "TrustZone Controller Config 0 (Lock Bits)\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_cfg0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tzc_cfg0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TzcCfg0Spec;
impl crate::RegisterSpec for TzcCfg0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tzc_cfg0::R`](R) reader structure"]
impl crate::Readable for TzcCfg0Spec {}
#[doc = "`write(|w| ..)` method takes [`tzc_cfg0::W`](W) writer structure"]
impl crate::Writable for TzcCfg0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TZC_CFG0 to value 0"]
impl crate::Resettable for TzcCfg0Spec {}
