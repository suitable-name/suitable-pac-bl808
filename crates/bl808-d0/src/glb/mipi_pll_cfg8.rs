#[doc = "Register `MIPI_PLL_CFG8` reader"]
pub type R = crate::R<MipiPllCfg8Spec>;
#[doc = "Register `MIPI_PLL_CFG8` writer"]
pub type W = crate::W<MipiPllCfg8Spec>;
#[doc = "Field `mipipll_dc_tp_out_en` reader - "]
pub type MipipllDcTpOutEnR = crate::BitReader;
#[doc = "Field `mipipll_dc_tp_out_en` writer - "]
pub type MipipllDcTpOutEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `mipipll_ten` reader - "]
pub type MipipllTenR = crate::BitReader;
#[doc = "Field `mipipll_ten` writer - "]
pub type MipipllTenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `mipipll_ten_sfreg` reader - "]
pub type MipipllTenSfregR = crate::BitReader;
#[doc = "Field `mipipll_ten_sfreg` writer - "]
pub type MipipllTenSfregW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `mipipll_dten_ckin` reader - "]
pub type MipipllDtenCkinR = crate::BitReader;
#[doc = "Field `mipipll_dten_ckin` writer - "]
pub type MipipllDtenCkinW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `mipipll_dten_fref` reader - "]
pub type MipipllDtenFrefR = crate::BitReader;
#[doc = "Field `mipipll_dten_fref` writer - "]
pub type MipipllDtenFrefW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `mipipll_dten_fsdm` reader - "]
pub type MipipllDtenFsdmR = crate::BitReader;
#[doc = "Field `mipipll_dten_fsdm` writer - "]
pub type MipipllDtenFsdmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `mipipll_dten_pupll` reader - "]
pub type MipipllDtenPupllR = crate::BitReader;
#[doc = "Field `mipipll_dten_pupll` writer - "]
pub type MipipllDtenPupllW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `mipipll_dten_pll_locked` reader - "]
pub type MipipllDtenPllLockedR = crate::BitReader;
#[doc = "Field `mipipll_dten_pll_locked` writer - "]
pub type MipipllDtenPllLockedW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `mipipll_dtest_pull_down` reader - "]
pub type MipipllDtestPullDownR = crate::BitReader;
#[doc = "Field `mipipll_dtest_pull_down` writer - "]
pub type MipipllDtestPullDownW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn mipipll_dc_tp_out_en(&self) -> MipipllDcTpOutEnR {
        MipipllDcTpOutEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn mipipll_ten(&self) -> MipipllTenR {
        MipipllTenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn mipipll_ten_sfreg(&self) -> MipipllTenSfregR {
        MipipllTenSfregR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn mipipll_dten_ckin(&self) -> MipipllDtenCkinR {
        MipipllDtenCkinR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn mipipll_dten_fref(&self) -> MipipllDtenFrefR {
        MipipllDtenFrefR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn mipipll_dten_fsdm(&self) -> MipipllDtenFsdmR {
        MipipllDtenFsdmR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn mipipll_dten_pupll(&self) -> MipipllDtenPupllR {
        MipipllDtenPupllR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn mipipll_dten_pll_locked(&self) -> MipipllDtenPllLockedR {
        MipipllDtenPllLockedR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn mipipll_dtest_pull_down(&self) -> MipipllDtestPullDownR {
        MipipllDtestPullDownR::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn mipipll_dc_tp_out_en(&mut self) -> MipipllDcTpOutEnW<'_, MipiPllCfg8Spec> {
        MipipllDcTpOutEnW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn mipipll_ten(&mut self) -> MipipllTenW<'_, MipiPllCfg8Spec> {
        MipipllTenW::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn mipipll_ten_sfreg(&mut self) -> MipipllTenSfregW<'_, MipiPllCfg8Spec> {
        MipipllTenSfregW::new(self, 2)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn mipipll_dten_ckin(&mut self) -> MipipllDtenCkinW<'_, MipiPllCfg8Spec> {
        MipipllDtenCkinW::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn mipipll_dten_fref(&mut self) -> MipipllDtenFrefW<'_, MipiPllCfg8Spec> {
        MipipllDtenFrefW::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn mipipll_dten_fsdm(&mut self) -> MipipllDtenFsdmW<'_, MipiPllCfg8Spec> {
        MipipllDtenFsdmW::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn mipipll_dten_pupll(&mut self) -> MipipllDtenPupllW<'_, MipiPllCfg8Spec> {
        MipipllDtenPupllW::new(self, 7)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn mipipll_dten_pll_locked(&mut self) -> MipipllDtenPllLockedW<'_, MipiPllCfg8Spec> {
        MipipllDtenPllLockedW::new(self, 8)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn mipipll_dtest_pull_down(&mut self) -> MipipllDtestPullDownW<'_, MipiPllCfg8Spec> {
        MipipllDtestPullDownW::new(self, 10)
    }
}
#[doc = "MIPI PLL Configuration 8 - Test\n\nYou can [`read`](crate::Reg::read) this register and get [`mipi_pll_cfg8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mipi_pll_cfg8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MipiPllCfg8Spec;
impl crate::RegisterSpec for MipiPllCfg8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mipi_pll_cfg8::R`](R) reader structure"]
impl crate::Readable for MipiPllCfg8Spec {}
#[doc = "`write(|w| ..)` method takes [`mipi_pll_cfg8::W`](W) writer structure"]
impl crate::Writable for MipiPllCfg8Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MIPI_PLL_CFG8 to value 0"]
impl crate::Resettable for MipiPllCfg8Spec {}
