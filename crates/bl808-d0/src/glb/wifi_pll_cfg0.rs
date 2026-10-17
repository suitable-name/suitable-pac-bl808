#[doc = "Register `WIFI_PLL_CFG0` reader"]
pub type R = crate::R<WifiPllCfg0Spec>;
#[doc = "Register `WIFI_PLL_CFG0` writer"]
pub type W = crate::W<WifiPllCfg0Spec>;
#[doc = "Field `wifipll_sdm_rstb` reader - "]
pub type WifipllSdmRstbR = crate::BitReader;
#[doc = "Field `wifipll_sdm_rstb` writer - "]
pub type WifipllSdmRstbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `wifipll_postdiv_rstb` reader - "]
pub type WifipllPostdivRstbR = crate::BitReader;
#[doc = "Field `wifipll_postdiv_rstb` writer - "]
pub type WifipllPostdivRstbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `wifipll_fbdv_rstb` reader - "]
pub type WifipllFbdvRstbR = crate::BitReader;
#[doc = "Field `wifipll_fbdv_rstb` writer - "]
pub type WifipllFbdvRstbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `wifipll_refdiv_rstb` reader - "]
pub type WifipllRefdivRstbR = crate::BitReader;
#[doc = "Field `wifipll_refdiv_rstb` writer - "]
pub type WifipllRefdivRstbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pu_wifipll_postdiv` reader - "]
pub type PuWifipllPostdivR = crate::BitReader;
#[doc = "Field `pu_wifipll_postdiv` writer - "]
pub type PuWifipllPostdivW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pu_wifipll_fbdv` reader - "]
pub type PuWifipllFbdvR = crate::BitReader;
#[doc = "Field `pu_wifipll_fbdv` writer - "]
pub type PuWifipllFbdvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pu_wifipll_clamp_op` reader - "]
pub type PuWifipllClampOpR = crate::BitReader;
#[doc = "Field `pu_wifipll_clamp_op` writer - "]
pub type PuWifipllClampOpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pu_wifipll_pfd` reader - "]
pub type PuWifipllPfdR = crate::BitReader;
#[doc = "Field `pu_wifipll_pfd` writer - "]
pub type PuWifipllPfdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pu_wifipll_cp` reader - "]
pub type PuWifipllCpR = crate::BitReader;
#[doc = "Field `pu_wifipll_cp` writer - "]
pub type PuWifipllCpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pu_wifipll_sfreg` reader - "]
pub type PuWifipllSfregR = crate::BitReader;
#[doc = "Field `pu_wifipll_sfreg` writer - "]
pub type PuWifipllSfregW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pu_wifipll` reader - "]
pub type PuWifipllR = crate::BitReader;
#[doc = "Field `pu_wifipll` writer - "]
pub type PuWifipllW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pu_wifipll_clktree` reader - "]
pub type PuWifipllClktreeR = crate::BitReader;
#[doc = "Field `pu_wifipll_clktree` writer - "]
pub type PuWifipllClktreeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn wifipll_sdm_rstb(&self) -> WifipllSdmRstbR {
        WifipllSdmRstbR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn wifipll_postdiv_rstb(&self) -> WifipllPostdivRstbR {
        WifipllPostdivRstbR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn wifipll_fbdv_rstb(&self) -> WifipllFbdvRstbR {
        WifipllFbdvRstbR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn wifipll_refdiv_rstb(&self) -> WifipllRefdivRstbR {
        WifipllRefdivRstbR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn pu_wifipll_postdiv(&self) -> PuWifipllPostdivR {
        PuWifipllPostdivR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn pu_wifipll_fbdv(&self) -> PuWifipllFbdvR {
        PuWifipllFbdvR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn pu_wifipll_clamp_op(&self) -> PuWifipllClampOpR {
        PuWifipllClampOpR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn pu_wifipll_pfd(&self) -> PuWifipllPfdR {
        PuWifipllPfdR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn pu_wifipll_cp(&self) -> PuWifipllCpR {
        PuWifipllCpR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn pu_wifipll_sfreg(&self) -> PuWifipllSfregR {
        PuWifipllSfregR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn pu_wifipll(&self) -> PuWifipllR {
        PuWifipllR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn pu_wifipll_clktree(&self) -> PuWifipllClktreeR {
        PuWifipllClktreeR::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn wifipll_sdm_rstb(&mut self) -> WifipllSdmRstbW<'_, WifiPllCfg0Spec> {
        WifipllSdmRstbW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn wifipll_postdiv_rstb(&mut self) -> WifipllPostdivRstbW<'_, WifiPllCfg0Spec> {
        WifipllPostdivRstbW::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn wifipll_fbdv_rstb(&mut self) -> WifipllFbdvRstbW<'_, WifiPllCfg0Spec> {
        WifipllFbdvRstbW::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn wifipll_refdiv_rstb(&mut self) -> WifipllRefdivRstbW<'_, WifiPllCfg0Spec> {
        WifipllRefdivRstbW::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn pu_wifipll_postdiv(&mut self) -> PuWifipllPostdivW<'_, WifiPllCfg0Spec> {
        PuWifipllPostdivW::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn pu_wifipll_fbdv(&mut self) -> PuWifipllFbdvW<'_, WifiPllCfg0Spec> {
        PuWifipllFbdvW::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn pu_wifipll_clamp_op(&mut self) -> PuWifipllClampOpW<'_, WifiPllCfg0Spec> {
        PuWifipllClampOpW::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn pu_wifipll_pfd(&mut self) -> PuWifipllPfdW<'_, WifiPllCfg0Spec> {
        PuWifipllPfdW::new(self, 7)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn pu_wifipll_cp(&mut self) -> PuWifipllCpW<'_, WifiPllCfg0Spec> {
        PuWifipllCpW::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn pu_wifipll_sfreg(&mut self) -> PuWifipllSfregW<'_, WifiPllCfg0Spec> {
        PuWifipllSfregW::new(self, 9)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn pu_wifipll(&mut self) -> PuWifipllW<'_, WifiPllCfg0Spec> {
        PuWifipllW::new(self, 10)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn pu_wifipll_clktree(&mut self) -> PuWifipllClktreeW<'_, WifiPllCfg0Spec> {
        PuWifipllClktreeW::new(self, 11)
    }
}
#[doc = "WIFI PLL Configuration 0 - Power and Reset\n\nYou can [`read`](crate::Reg::read) this register and get [`wifi_pll_cfg0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wifi_pll_cfg0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WifiPllCfg0Spec;
impl crate::RegisterSpec for WifiPllCfg0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wifi_pll_cfg0::R`](R) reader structure"]
impl crate::Readable for WifiPllCfg0Spec {}
#[doc = "`write(|w| ..)` method takes [`wifi_pll_cfg0::W`](W) writer structure"]
impl crate::Writable for WifiPllCfg0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets WIFI_PLL_CFG0 to value 0"]
impl crate::Resettable for WifiPllCfg0Spec {}
