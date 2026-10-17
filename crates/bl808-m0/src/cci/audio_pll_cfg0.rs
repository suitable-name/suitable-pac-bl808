#[doc = "Register `AUDIO_PLL_CFG0` reader"]
pub type R = crate::R<AudioPllCfg0Spec>;
#[doc = "Register `AUDIO_PLL_CFG0` writer"]
pub type W = crate::W<AudioPllCfg0Spec>;
#[doc = "Field `aupll_sdm_rstb` reader - "]
pub type AupllSdmRstbR = crate::BitReader;
#[doc = "Field `aupll_sdm_rstb` writer - "]
pub type AupllSdmRstbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `aupll_postdiv_rstb` reader - "]
pub type AupllPostdivRstbR = crate::BitReader;
#[doc = "Field `aupll_postdiv_rstb` writer - "]
pub type AupllPostdivRstbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `aupll_fbdv_rstb` reader - "]
pub type AupllFbdvRstbR = crate::BitReader;
#[doc = "Field `aupll_fbdv_rstb` writer - "]
pub type AupllFbdvRstbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `aupll_refdiv_rstb` reader - "]
pub type AupllRefdivRstbR = crate::BitReader;
#[doc = "Field `aupll_refdiv_rstb` writer - "]
pub type AupllRefdivRstbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pu_aupll_postdiv` reader - Power up post divider"]
pub type PuAupllPostdivR = crate::BitReader;
#[doc = "Field `pu_aupll_postdiv` writer - Power up post divider"]
pub type PuAupllPostdivW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pu_aupll_fbdv` reader - Power up feedback divider"]
pub type PuAupllFbdvR = crate::BitReader;
#[doc = "Field `pu_aupll_fbdv` writer - Power up feedback divider"]
pub type PuAupllFbdvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pu_aupll_clamp_op` reader - Power up clamp operation"]
pub type PuAupllClampOpR = crate::BitReader;
#[doc = "Field `pu_aupll_clamp_op` writer - Power up clamp operation"]
pub type PuAupllClampOpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pu_aupll_pfd` reader - Power up Phase Frequency Detector (PFD)"]
pub type PuAupllPfdR = crate::BitReader;
#[doc = "Field `pu_aupll_pfd` writer - Power up Phase Frequency Detector (PFD)"]
pub type PuAupllPfdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pu_aupll_cp` reader - Power up Charge Pump (CP)"]
pub type PuAupllCpR = crate::BitReader;
#[doc = "Field `pu_aupll_cp` writer - Power up Charge Pump (CP)"]
pub type PuAupllCpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pu_aupll_sfreg` reader - Power up SF regulator"]
pub type PuAupllSfregR = crate::BitReader;
#[doc = "Field `pu_aupll_sfreg` writer - Power up SF regulator"]
pub type PuAupllSfregW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pu_aupll` reader - Power up Audio PLL"]
pub type PuAupllR = crate::BitReader;
#[doc = "Field `pu_aupll` writer - Power up Audio PLL"]
pub type PuAupllW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pu_aupll_clktree` reader - Power up clock tree"]
pub type PuAupllClktreeR = crate::BitReader;
#[doc = "Field `pu_aupll_clktree` writer - Power up clock tree"]
pub type PuAupllClktreeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn aupll_sdm_rstb(&self) -> AupllSdmRstbR {
        AupllSdmRstbR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn aupll_postdiv_rstb(&self) -> AupllPostdivRstbR {
        AupllPostdivRstbR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn aupll_fbdv_rstb(&self) -> AupllFbdvRstbR {
        AupllFbdvRstbR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn aupll_refdiv_rstb(&self) -> AupllRefdivRstbR {
        AupllRefdivRstbR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Power up post divider"]
    #[inline(always)]
    pub fn pu_aupll_postdiv(&self) -> PuAupllPostdivR {
        PuAupllPostdivR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Power up feedback divider"]
    #[inline(always)]
    pub fn pu_aupll_fbdv(&self) -> PuAupllFbdvR {
        PuAupllFbdvR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Power up clamp operation"]
    #[inline(always)]
    pub fn pu_aupll_clamp_op(&self) -> PuAupllClampOpR {
        PuAupllClampOpR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Power up Phase Frequency Detector (PFD)"]
    #[inline(always)]
    pub fn pu_aupll_pfd(&self) -> PuAupllPfdR {
        PuAupllPfdR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Power up Charge Pump (CP)"]
    #[inline(always)]
    pub fn pu_aupll_cp(&self) -> PuAupllCpR {
        PuAupllCpR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Power up SF regulator"]
    #[inline(always)]
    pub fn pu_aupll_sfreg(&self) -> PuAupllSfregR {
        PuAupllSfregR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Power up Audio PLL"]
    #[inline(always)]
    pub fn pu_aupll(&self) -> PuAupllR {
        PuAupllR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Power up clock tree"]
    #[inline(always)]
    pub fn pu_aupll_clktree(&self) -> PuAupllClktreeR {
        PuAupllClktreeR::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn aupll_sdm_rstb(&mut self) -> AupllSdmRstbW<'_, AudioPllCfg0Spec> {
        AupllSdmRstbW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn aupll_postdiv_rstb(&mut self) -> AupllPostdivRstbW<'_, AudioPllCfg0Spec> {
        AupllPostdivRstbW::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn aupll_fbdv_rstb(&mut self) -> AupllFbdvRstbW<'_, AudioPllCfg0Spec> {
        AupllFbdvRstbW::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn aupll_refdiv_rstb(&mut self) -> AupllRefdivRstbW<'_, AudioPllCfg0Spec> {
        AupllRefdivRstbW::new(self, 3)
    }
    #[doc = "Bit 4 - Power up post divider"]
    #[inline(always)]
    pub fn pu_aupll_postdiv(&mut self) -> PuAupllPostdivW<'_, AudioPllCfg0Spec> {
        PuAupllPostdivW::new(self, 4)
    }
    #[doc = "Bit 5 - Power up feedback divider"]
    #[inline(always)]
    pub fn pu_aupll_fbdv(&mut self) -> PuAupllFbdvW<'_, AudioPllCfg0Spec> {
        PuAupllFbdvW::new(self, 5)
    }
    #[doc = "Bit 6 - Power up clamp operation"]
    #[inline(always)]
    pub fn pu_aupll_clamp_op(&mut self) -> PuAupllClampOpW<'_, AudioPllCfg0Spec> {
        PuAupllClampOpW::new(self, 6)
    }
    #[doc = "Bit 7 - Power up Phase Frequency Detector (PFD)"]
    #[inline(always)]
    pub fn pu_aupll_pfd(&mut self) -> PuAupllPfdW<'_, AudioPllCfg0Spec> {
        PuAupllPfdW::new(self, 7)
    }
    #[doc = "Bit 8 - Power up Charge Pump (CP)"]
    #[inline(always)]
    pub fn pu_aupll_cp(&mut self) -> PuAupllCpW<'_, AudioPllCfg0Spec> {
        PuAupllCpW::new(self, 8)
    }
    #[doc = "Bit 9 - Power up SF regulator"]
    #[inline(always)]
    pub fn pu_aupll_sfreg(&mut self) -> PuAupllSfregW<'_, AudioPllCfg0Spec> {
        PuAupllSfregW::new(self, 9)
    }
    #[doc = "Bit 10 - Power up Audio PLL"]
    #[inline(always)]
    pub fn pu_aupll(&mut self) -> PuAupllW<'_, AudioPllCfg0Spec> {
        PuAupllW::new(self, 10)
    }
    #[doc = "Bit 11 - Power up clock tree"]
    #[inline(always)]
    pub fn pu_aupll_clktree(&mut self) -> PuAupllClktreeW<'_, AudioPllCfg0Spec> {
        PuAupllClktreeW::new(self, 11)
    }
}
#[doc = "Audio PLL Configuration 0 - Resets and Power Up\n\nYou can [`read`](crate::Reg::read) this register and get [`audio_pll_cfg0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`audio_pll_cfg0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AudioPllCfg0Spec;
impl crate::RegisterSpec for AudioPllCfg0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`audio_pll_cfg0::R`](R) reader structure"]
impl crate::Readable for AudioPllCfg0Spec {}
#[doc = "`write(|w| ..)` method takes [`audio_pll_cfg0::W`](W) writer structure"]
impl crate::Writable for AudioPllCfg0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AUDIO_PLL_CFG0 to value 0x09ff"]
impl crate::Resettable for AudioPllCfg0Spec {
    const RESET_VALUE: u32 = 0x09ff;
}
