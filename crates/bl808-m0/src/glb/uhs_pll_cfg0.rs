#[doc = "Register `UHS_PLL_CFG0` reader"]
pub type R = crate::R<UhsPllCfg0Spec>;
#[doc = "Register `UHS_PLL_CFG0` writer"]
pub type W = crate::W<UhsPllCfg0Spec>;
#[doc = "Field `uhspll_sdm_rstb` reader - "]
pub type UhspllSdmRstbR = crate::BitReader;
#[doc = "Field `uhspll_sdm_rstb` writer - "]
pub type UhspllSdmRstbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `uhspll_fbdv_rstb` reader - "]
pub type UhspllFbdvRstbR = crate::BitReader;
#[doc = "Field `uhspll_fbdv_rstb` writer - "]
pub type UhspllFbdvRstbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pu_uhspll_fbdv` reader - "]
pub type PuUhspllFbdvR = crate::BitReader;
#[doc = "Field `pu_uhspll_fbdv` writer - "]
pub type PuUhspllFbdvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pu_uhspll_cp` reader - "]
pub type PuUhspllCpR = crate::BitReader;
#[doc = "Field `pu_uhspll_cp` writer - "]
pub type PuUhspllCpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pu_uhspll_sfreg` reader - "]
pub type PuUhspllSfregR = crate::BitReader;
#[doc = "Field `pu_uhspll_sfreg` writer - "]
pub type PuUhspllSfregW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pu_uhspll` reader - "]
pub type PuUhspllR = crate::BitReader;
#[doc = "Field `pu_uhspll` writer - "]
pub type PuUhspllW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn uhspll_sdm_rstb(&self) -> UhspllSdmRstbR {
        UhspllSdmRstbR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn uhspll_fbdv_rstb(&self) -> UhspllFbdvRstbR {
        UhspllFbdvRstbR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn pu_uhspll_fbdv(&self) -> PuUhspllFbdvR {
        PuUhspllFbdvR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn pu_uhspll_cp(&self) -> PuUhspllCpR {
        PuUhspllCpR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn pu_uhspll_sfreg(&self) -> PuUhspllSfregR {
        PuUhspllSfregR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn pu_uhspll(&self) -> PuUhspllR {
        PuUhspllR::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn uhspll_sdm_rstb(&mut self) -> UhspllSdmRstbW<'_, UhsPllCfg0Spec> {
        UhspllSdmRstbW::new(self, 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn uhspll_fbdv_rstb(&mut self) -> UhspllFbdvRstbW<'_, UhsPllCfg0Spec> {
        UhspllFbdvRstbW::new(self, 2)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn pu_uhspll_fbdv(&mut self) -> PuUhspllFbdvW<'_, UhsPllCfg0Spec> {
        PuUhspllFbdvW::new(self, 5)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn pu_uhspll_cp(&mut self) -> PuUhspllCpW<'_, UhsPllCfg0Spec> {
        PuUhspllCpW::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn pu_uhspll_sfreg(&mut self) -> PuUhspllSfregW<'_, UhsPllCfg0Spec> {
        PuUhspllSfregW::new(self, 9)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn pu_uhspll(&mut self) -> PuUhspllW<'_, UhsPllCfg0Spec> {
        PuUhspllW::new(self, 10)
    }
}
#[doc = "UHS PLL Configuration 0 - Power and Reset\n\nYou can [`read`](crate::Reg::read) this register and get [`uhs_pll_cfg0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uhs_pll_cfg0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UhsPllCfg0Spec;
impl crate::RegisterSpec for UhsPllCfg0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uhs_pll_cfg0::R`](R) reader structure"]
impl crate::Readable for UhsPllCfg0Spec {}
#[doc = "`write(|w| ..)` method takes [`uhs_pll_cfg0::W`](W) writer structure"]
impl crate::Writable for UhsPllCfg0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UHS_PLL_CFG0 to value 0"]
impl crate::Resettable for UhsPllCfg0Spec {}
