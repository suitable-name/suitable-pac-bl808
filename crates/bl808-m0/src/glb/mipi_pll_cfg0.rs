#[doc = "Register `MIPI_PLL_CFG0` reader"]
pub type R = crate::R<MipiPllCfg0Spec>;
#[doc = "Register `MIPI_PLL_CFG0` writer"]
pub type W = crate::W<MipiPllCfg0Spec>;
#[doc = "Field `mipipll_sdm_rstb` reader - "]
pub type MipipllSdmRstbR = crate::BitReader;
#[doc = "Field `mipipll_sdm_rstb` writer - "]
pub type MipipllSdmRstbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `mipipll_fbdv_rstb` reader - "]
pub type MipipllFbdvRstbR = crate::BitReader;
#[doc = "Field `mipipll_fbdv_rstb` writer - "]
pub type MipipllFbdvRstbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pu_mipipll_fbdv` reader - "]
pub type PuMipipllFbdvR = crate::BitReader;
#[doc = "Field `pu_mipipll_fbdv` writer - "]
pub type PuMipipllFbdvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pu_mipipll_cp` reader - "]
pub type PuMipipllCpR = crate::BitReader;
#[doc = "Field `pu_mipipll_cp` writer - "]
pub type PuMipipllCpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pu_mipipll_sfreg` reader - "]
pub type PuMipipllSfregR = crate::BitReader;
#[doc = "Field `pu_mipipll_sfreg` writer - "]
pub type PuMipipllSfregW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pu_mipipll` reader - "]
pub type PuMipipllR = crate::BitReader;
#[doc = "Field `pu_mipipll` writer - "]
pub type PuMipipllW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn mipipll_sdm_rstb(&self) -> MipipllSdmRstbR {
        MipipllSdmRstbR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn mipipll_fbdv_rstb(&self) -> MipipllFbdvRstbR {
        MipipllFbdvRstbR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn pu_mipipll_fbdv(&self) -> PuMipipllFbdvR {
        PuMipipllFbdvR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn pu_mipipll_cp(&self) -> PuMipipllCpR {
        PuMipipllCpR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn pu_mipipll_sfreg(&self) -> PuMipipllSfregR {
        PuMipipllSfregR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn pu_mipipll(&self) -> PuMipipllR {
        PuMipipllR::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn mipipll_sdm_rstb(&mut self) -> MipipllSdmRstbW<'_, MipiPllCfg0Spec> {
        MipipllSdmRstbW::new(self, 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn mipipll_fbdv_rstb(&mut self) -> MipipllFbdvRstbW<'_, MipiPllCfg0Spec> {
        MipipllFbdvRstbW::new(self, 2)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn pu_mipipll_fbdv(&mut self) -> PuMipipllFbdvW<'_, MipiPllCfg0Spec> {
        PuMipipllFbdvW::new(self, 5)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn pu_mipipll_cp(&mut self) -> PuMipipllCpW<'_, MipiPllCfg0Spec> {
        PuMipipllCpW::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn pu_mipipll_sfreg(&mut self) -> PuMipipllSfregW<'_, MipiPllCfg0Spec> {
        PuMipipllSfregW::new(self, 9)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn pu_mipipll(&mut self) -> PuMipipllW<'_, MipiPllCfg0Spec> {
        PuMipipllW::new(self, 10)
    }
}
#[doc = "MIPI PLL Configuration 0 - Power and Reset\n\nYou can [`read`](crate::Reg::read) this register and get [`mipi_pll_cfg0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mipi_pll_cfg0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MipiPllCfg0Spec;
impl crate::RegisterSpec for MipiPllCfg0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mipi_pll_cfg0::R`](R) reader structure"]
impl crate::Readable for MipiPllCfg0Spec {}
#[doc = "`write(|w| ..)` method takes [`mipi_pll_cfg0::W`](W) writer structure"]
impl crate::Writable for MipiPllCfg0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MIPI_PLL_CFG0 to value 0"]
impl crate::Resettable for MipiPllCfg0Spec {}
