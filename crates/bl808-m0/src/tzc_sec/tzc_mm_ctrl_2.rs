#[doc = "Register `TZC_MM_CTRL_2` reader"]
pub type R = crate::R<TzcMmCtrl2Spec>;
#[doc = "Register `TZC_MM_CTRL_2` writer"]
pub type W = crate::W<TzcMmCtrl2Spec>;
#[doc = "Field `tzc_mm_pwron_rst_tzsid_lock` reader - MM Power-on reset TZSID lock"]
pub type TzcMmPwronRstTzsidLockR = crate::BitReader;
#[doc = "Field `tzc_mm_pwron_rst_tzsid_lock` writer - MM Power-on reset TZSID lock"]
pub type TzcMmPwronRstTzsidLockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_mm_cpu0_reset_tzsid_lock` reader - MM CPU0 reset TZSID lock"]
pub type TzcMmCpu0ResetTzsidLockR = crate::BitReader;
#[doc = "Field `tzc_mm_cpu0_reset_tzsid_lock` writer - MM CPU0 reset TZSID lock"]
pub type TzcMmCpu0ResetTzsidLockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_mm_sys_reset_tzsid_lock` reader - MM system reset TZSID lock"]
pub type TzcMmSysResetTzsidLockR = crate::BitReader;
#[doc = "Field `tzc_mm_sys_reset_tzsid_lock` writer - MM system reset TZSID lock"]
pub type TzcMmSysResetTzsidLockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_mm_cpu0_tzsid_lock` reader - MM CPU0 TZSID lock"]
pub type TzcMmCpu0TzsidLockR = crate::BitReader;
#[doc = "Field `tzc_mm_cpu0_tzsid_lock` writer - MM CPU0 TZSID lock"]
pub type TzcMmCpu0TzsidLockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_mm_sram_tzsid_lock` reader - MM SRAM TZSID lock"]
pub type TzcMmSramTzsidLockR = crate::BitReader;
#[doc = "Field `tzc_mm_sram_tzsid_lock` writer - MM SRAM TZSID lock"]
pub type TzcMmSramTzsidLockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_mm_swrst_tzsid_lock` reader - MM software reset TZSID lock"]
pub type TzcMmSwrstTzsidLockR = crate::BitReader;
#[doc = "Field `tzc_mm_swrst_tzsid_lock` writer - MM software reset TZSID lock"]
pub type TzcMmSwrstTzsidLockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_mm_clk_tzsid_lock` reader - MM clock TZSID lock"]
pub type TzcMmClkTzsidLockR = crate::BitReader;
#[doc = "Field `tzc_mm_clk_tzsid_lock` writer - MM clock TZSID lock"]
pub type TzcMmClkTzsidLockW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - MM Power-on reset TZSID lock"]
    #[inline(always)]
    pub fn tzc_mm_pwron_rst_tzsid_lock(&self) -> TzcMmPwronRstTzsidLockR {
        TzcMmPwronRstTzsidLockR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - MM CPU0 reset TZSID lock"]
    #[inline(always)]
    pub fn tzc_mm_cpu0_reset_tzsid_lock(&self) -> TzcMmCpu0ResetTzsidLockR {
        TzcMmCpu0ResetTzsidLockR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - MM system reset TZSID lock"]
    #[inline(always)]
    pub fn tzc_mm_sys_reset_tzsid_lock(&self) -> TzcMmSysResetTzsidLockR {
        TzcMmSysResetTzsidLockR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - MM CPU0 TZSID lock"]
    #[inline(always)]
    pub fn tzc_mm_cpu0_tzsid_lock(&self) -> TzcMmCpu0TzsidLockR {
        TzcMmCpu0TzsidLockR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - MM SRAM TZSID lock"]
    #[inline(always)]
    pub fn tzc_mm_sram_tzsid_lock(&self) -> TzcMmSramTzsidLockR {
        TzcMmSramTzsidLockR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - MM software reset TZSID lock"]
    #[inline(always)]
    pub fn tzc_mm_swrst_tzsid_lock(&self) -> TzcMmSwrstTzsidLockR {
        TzcMmSwrstTzsidLockR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 10 - MM clock TZSID lock"]
    #[inline(always)]
    pub fn tzc_mm_clk_tzsid_lock(&self) -> TzcMmClkTzsidLockR {
        TzcMmClkTzsidLockR::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - MM Power-on reset TZSID lock"]
    #[inline(always)]
    pub fn tzc_mm_pwron_rst_tzsid_lock(&mut self) -> TzcMmPwronRstTzsidLockW<'_, TzcMmCtrl2Spec> {
        TzcMmPwronRstTzsidLockW::new(self, 0)
    }
    #[doc = "Bit 1 - MM CPU0 reset TZSID lock"]
    #[inline(always)]
    pub fn tzc_mm_cpu0_reset_tzsid_lock(&mut self) -> TzcMmCpu0ResetTzsidLockW<'_, TzcMmCtrl2Spec> {
        TzcMmCpu0ResetTzsidLockW::new(self, 1)
    }
    #[doc = "Bit 2 - MM system reset TZSID lock"]
    #[inline(always)]
    pub fn tzc_mm_sys_reset_tzsid_lock(&mut self) -> TzcMmSysResetTzsidLockW<'_, TzcMmCtrl2Spec> {
        TzcMmSysResetTzsidLockW::new(self, 2)
    }
    #[doc = "Bit 3 - MM CPU0 TZSID lock"]
    #[inline(always)]
    pub fn tzc_mm_cpu0_tzsid_lock(&mut self) -> TzcMmCpu0TzsidLockW<'_, TzcMmCtrl2Spec> {
        TzcMmCpu0TzsidLockW::new(self, 3)
    }
    #[doc = "Bit 5 - MM SRAM TZSID lock"]
    #[inline(always)]
    pub fn tzc_mm_sram_tzsid_lock(&mut self) -> TzcMmSramTzsidLockW<'_, TzcMmCtrl2Spec> {
        TzcMmSramTzsidLockW::new(self, 5)
    }
    #[doc = "Bit 6 - MM software reset TZSID lock"]
    #[inline(always)]
    pub fn tzc_mm_swrst_tzsid_lock(&mut self) -> TzcMmSwrstTzsidLockW<'_, TzcMmCtrl2Spec> {
        TzcMmSwrstTzsidLockW::new(self, 6)
    }
    #[doc = "Bit 10 - MM clock TZSID lock"]
    #[inline(always)]
    pub fn tzc_mm_clk_tzsid_lock(&mut self) -> TzcMmClkTzsidLockW<'_, TzcMmCtrl2Spec> {
        TzcMmClkTzsidLockW::new(self, 10)
    }
}
#[doc = "Multimedia TrustZone Control Register 2 (Lock)\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_mm_ctrl_2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tzc_mm_ctrl_2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TzcMmCtrl2Spec;
impl crate::RegisterSpec for TzcMmCtrl2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tzc_mm_ctrl_2::R`](R) reader structure"]
impl crate::Readable for TzcMmCtrl2Spec {}
#[doc = "`write(|w| ..)` method takes [`tzc_mm_ctrl_2::W`](W) writer structure"]
impl crate::Writable for TzcMmCtrl2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TZC_MM_CTRL_2 to value 0"]
impl crate::Resettable for TzcMmCtrl2Spec {}
