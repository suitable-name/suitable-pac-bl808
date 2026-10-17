#[doc = "Register `UHS_PLL_CFG8` reader"]
pub type R = crate::R<UhsPllCfg8Spec>;
#[doc = "Register `UHS_PLL_CFG8` writer"]
pub type W = crate::W<UhsPllCfg8Spec>;
#[doc = "Field `uhspll_dc_tp_out_en` reader - "]
pub type UhspllDcTpOutEnR = crate::BitReader;
#[doc = "Field `uhspll_dc_tp_out_en` writer - "]
pub type UhspllDcTpOutEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `uhspll_ten` reader - "]
pub type UhspllTenR = crate::BitReader;
#[doc = "Field `uhspll_ten` writer - "]
pub type UhspllTenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `uhspll_ten_sfreg` reader - "]
pub type UhspllTenSfregR = crate::BitReader;
#[doc = "Field `uhspll_ten_sfreg` writer - "]
pub type UhspllTenSfregW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `uhspll_dten_ckin` reader - "]
pub type UhspllDtenCkinR = crate::BitReader;
#[doc = "Field `uhspll_dten_ckin` writer - "]
pub type UhspllDtenCkinW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `uhspll_dten_fref` reader - "]
pub type UhspllDtenFrefR = crate::BitReader;
#[doc = "Field `uhspll_dten_fref` writer - "]
pub type UhspllDtenFrefW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `uhspll_dten_fsdm` reader - "]
pub type UhspllDtenFsdmR = crate::BitReader;
#[doc = "Field `uhspll_dten_fsdm` writer - "]
pub type UhspllDtenFsdmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `uhspll_dten_pupll` reader - "]
pub type UhspllDtenPupllR = crate::BitReader;
#[doc = "Field `uhspll_dten_pupll` writer - "]
pub type UhspllDtenPupllW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `uhspll_dten_pll_locked` reader - "]
pub type UhspllDtenPllLockedR = crate::BitReader;
#[doc = "Field `uhspll_dten_pll_locked` writer - "]
pub type UhspllDtenPllLockedW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `uhspll_dtest_pull_down` reader - "]
pub type UhspllDtestPullDownR = crate::BitReader;
#[doc = "Field `uhspll_dtest_pull_down` writer - "]
pub type UhspllDtestPullDownW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn uhspll_dc_tp_out_en(&self) -> UhspllDcTpOutEnR {
        UhspllDcTpOutEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn uhspll_ten(&self) -> UhspllTenR {
        UhspllTenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn uhspll_ten_sfreg(&self) -> UhspllTenSfregR {
        UhspllTenSfregR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn uhspll_dten_ckin(&self) -> UhspllDtenCkinR {
        UhspllDtenCkinR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn uhspll_dten_fref(&self) -> UhspllDtenFrefR {
        UhspllDtenFrefR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn uhspll_dten_fsdm(&self) -> UhspllDtenFsdmR {
        UhspllDtenFsdmR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn uhspll_dten_pupll(&self) -> UhspllDtenPupllR {
        UhspllDtenPupllR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn uhspll_dten_pll_locked(&self) -> UhspllDtenPllLockedR {
        UhspllDtenPllLockedR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn uhspll_dtest_pull_down(&self) -> UhspllDtestPullDownR {
        UhspllDtestPullDownR::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn uhspll_dc_tp_out_en(&mut self) -> UhspllDcTpOutEnW<'_, UhsPllCfg8Spec> {
        UhspllDcTpOutEnW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn uhspll_ten(&mut self) -> UhspllTenW<'_, UhsPllCfg8Spec> {
        UhspllTenW::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn uhspll_ten_sfreg(&mut self) -> UhspllTenSfregW<'_, UhsPllCfg8Spec> {
        UhspllTenSfregW::new(self, 2)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn uhspll_dten_ckin(&mut self) -> UhspllDtenCkinW<'_, UhsPllCfg8Spec> {
        UhspllDtenCkinW::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn uhspll_dten_fref(&mut self) -> UhspllDtenFrefW<'_, UhsPllCfg8Spec> {
        UhspllDtenFrefW::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn uhspll_dten_fsdm(&mut self) -> UhspllDtenFsdmW<'_, UhsPllCfg8Spec> {
        UhspllDtenFsdmW::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn uhspll_dten_pupll(&mut self) -> UhspllDtenPupllW<'_, UhsPllCfg8Spec> {
        UhspllDtenPupllW::new(self, 7)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn uhspll_dten_pll_locked(&mut self) -> UhspllDtenPllLockedW<'_, UhsPllCfg8Spec> {
        UhspllDtenPllLockedW::new(self, 8)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn uhspll_dtest_pull_down(&mut self) -> UhspllDtestPullDownW<'_, UhsPllCfg8Spec> {
        UhspllDtestPullDownW::new(self, 10)
    }
}
#[doc = "UHS PLL Configuration 8 - Test\n\nYou can [`read`](crate::Reg::read) this register and get [`uhs_pll_cfg8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uhs_pll_cfg8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UhsPllCfg8Spec;
impl crate::RegisterSpec for UhsPllCfg8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uhs_pll_cfg8::R`](R) reader structure"]
impl crate::Readable for UhsPllCfg8Spec {}
#[doc = "`write(|w| ..)` method takes [`uhs_pll_cfg8::W`](W) writer structure"]
impl crate::Writable for UhsPllCfg8Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UHS_PLL_CFG8 to value 0"]
impl crate::Resettable for UhsPllCfg8Spec {}
