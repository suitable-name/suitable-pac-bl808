#[doc = "Register `CPU_PLL_CFG0` reader"]
pub type R = crate::R<CpuPllCfg0Spec>;
#[doc = "Register `CPU_PLL_CFG0` writer"]
pub type W = crate::W<CpuPllCfg0Spec>;
#[doc = "Field `cpupll_sdm_rstb` reader - "]
pub type CpupllSdmRstbR = crate::BitReader;
#[doc = "Field `cpupll_sdm_rstb` writer - "]
pub type CpupllSdmRstbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cpupll_postdiv_rstb` reader - "]
pub type CpupllPostdivRstbR = crate::BitReader;
#[doc = "Field `cpupll_postdiv_rstb` writer - "]
pub type CpupllPostdivRstbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cpupll_fbdv_rstb` reader - "]
pub type CpupllFbdvRstbR = crate::BitReader;
#[doc = "Field `cpupll_fbdv_rstb` writer - "]
pub type CpupllFbdvRstbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cpupll_refdiv_rstb` reader - "]
pub type CpupllRefdivRstbR = crate::BitReader;
#[doc = "Field `cpupll_refdiv_rstb` writer - "]
pub type CpupllRefdivRstbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pu_cpupll_postdiv` reader - "]
pub type PuCpupllPostdivR = crate::BitReader;
#[doc = "Field `pu_cpupll_postdiv` writer - "]
pub type PuCpupllPostdivW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pu_cpupll_fbdv` reader - "]
pub type PuCpupllFbdvR = crate::BitReader;
#[doc = "Field `pu_cpupll_fbdv` writer - "]
pub type PuCpupllFbdvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pu_cpupll_clamp_op` reader - "]
pub type PuCpupllClampOpR = crate::BitReader;
#[doc = "Field `pu_cpupll_clamp_op` writer - "]
pub type PuCpupllClampOpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pu_cpupll_pfd` reader - "]
pub type PuCpupllPfdR = crate::BitReader;
#[doc = "Field `pu_cpupll_pfd` writer - "]
pub type PuCpupllPfdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pu_cpupll_cp` reader - "]
pub type PuCpupllCpR = crate::BitReader;
#[doc = "Field `pu_cpupll_cp` writer - "]
pub type PuCpupllCpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pu_cpupll_sfreg` reader - "]
pub type PuCpupllSfregR = crate::BitReader;
#[doc = "Field `pu_cpupll_sfreg` writer - "]
pub type PuCpupllSfregW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pu_cpupll` reader - "]
pub type PuCpupllR = crate::BitReader;
#[doc = "Field `pu_cpupll` writer - "]
pub type PuCpupllW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pu_cpupll_clktree` reader - "]
pub type PuCpupllClktreeR = crate::BitReader;
#[doc = "Field `pu_cpupll_clktree` writer - "]
pub type PuCpupllClktreeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cpupll_sdm_rstb(&self) -> CpupllSdmRstbR {
        CpupllSdmRstbR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cpupll_postdiv_rstb(&self) -> CpupllPostdivRstbR {
        CpupllPostdivRstbR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn cpupll_fbdv_rstb(&self) -> CpupllFbdvRstbR {
        CpupllFbdvRstbR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn cpupll_refdiv_rstb(&self) -> CpupllRefdivRstbR {
        CpupllRefdivRstbR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn pu_cpupll_postdiv(&self) -> PuCpupllPostdivR {
        PuCpupllPostdivR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn pu_cpupll_fbdv(&self) -> PuCpupllFbdvR {
        PuCpupllFbdvR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn pu_cpupll_clamp_op(&self) -> PuCpupllClampOpR {
        PuCpupllClampOpR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn pu_cpupll_pfd(&self) -> PuCpupllPfdR {
        PuCpupllPfdR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn pu_cpupll_cp(&self) -> PuCpupllCpR {
        PuCpupllCpR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn pu_cpupll_sfreg(&self) -> PuCpupllSfregR {
        PuCpupllSfregR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn pu_cpupll(&self) -> PuCpupllR {
        PuCpupllR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn pu_cpupll_clktree(&self) -> PuCpupllClktreeR {
        PuCpupllClktreeR::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cpupll_sdm_rstb(&mut self) -> CpupllSdmRstbW<'_, CpuPllCfg0Spec> {
        CpupllSdmRstbW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cpupll_postdiv_rstb(&mut self) -> CpupllPostdivRstbW<'_, CpuPllCfg0Spec> {
        CpupllPostdivRstbW::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn cpupll_fbdv_rstb(&mut self) -> CpupllFbdvRstbW<'_, CpuPllCfg0Spec> {
        CpupllFbdvRstbW::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn cpupll_refdiv_rstb(&mut self) -> CpupllRefdivRstbW<'_, CpuPllCfg0Spec> {
        CpupllRefdivRstbW::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn pu_cpupll_postdiv(&mut self) -> PuCpupllPostdivW<'_, CpuPllCfg0Spec> {
        PuCpupllPostdivW::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn pu_cpupll_fbdv(&mut self) -> PuCpupllFbdvW<'_, CpuPllCfg0Spec> {
        PuCpupllFbdvW::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn pu_cpupll_clamp_op(&mut self) -> PuCpupllClampOpW<'_, CpuPllCfg0Spec> {
        PuCpupllClampOpW::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn pu_cpupll_pfd(&mut self) -> PuCpupllPfdW<'_, CpuPllCfg0Spec> {
        PuCpupllPfdW::new(self, 7)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn pu_cpupll_cp(&mut self) -> PuCpupllCpW<'_, CpuPllCfg0Spec> {
        PuCpupllCpW::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn pu_cpupll_sfreg(&mut self) -> PuCpupllSfregW<'_, CpuPllCfg0Spec> {
        PuCpupllSfregW::new(self, 9)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn pu_cpupll(&mut self) -> PuCpupllW<'_, CpuPllCfg0Spec> {
        PuCpupllW::new(self, 10)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn pu_cpupll_clktree(&mut self) -> PuCpupllClktreeW<'_, CpuPllCfg0Spec> {
        PuCpupllClktreeW::new(self, 11)
    }
}
#[doc = "CPU PLL Configuration 0 - Resets and Power Up\n\nYou can [`read`](crate::Reg::read) this register and get [`cpu_pll_cfg0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpu_pll_cfg0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpuPllCfg0Spec;
impl crate::RegisterSpec for CpuPllCfg0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpu_pll_cfg0::R`](R) reader structure"]
impl crate::Readable for CpuPllCfg0Spec {}
#[doc = "`write(|w| ..)` method takes [`cpu_pll_cfg0::W`](W) writer structure"]
impl crate::Writable for CpuPllCfg0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CPU_PLL_CFG0 to value 0x09ef"]
impl crate::Resettable for CpuPllCfg0Spec {
    const RESET_VALUE: u32 = 0x09ef;
}
