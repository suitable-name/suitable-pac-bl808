#[doc = "Register `AUDIO_PLL_CFG9` reader"]
pub type R = crate::R<AudioPllCfg9Spec>;
#[doc = "Register `AUDIO_PLL_CFG9` writer"]
pub type W = crate::W<AudioPllCfg9Spec>;
#[doc = "Field `aupll_dc_tp_out_en` reader - "]
pub type AupllDcTpOutEnR = crate::BitReader;
#[doc = "Field `aupll_dc_tp_out_en` writer - "]
pub type AupllDcTpOutEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ten_aupll` reader - "]
pub type TenAupllR = crate::BitReader;
#[doc = "Field `ten_aupll` writer - "]
pub type TenAupllW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ten_aupll_sfreg` reader - "]
pub type TenAupllSfregR = crate::BitReader;
#[doc = "Field `ten_aupll_sfreg` writer - "]
pub type TenAupllSfregW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dten_aupll_fin` reader - "]
pub type DtenAupllFinR = crate::BitReader;
#[doc = "Field `dten_aupll_fin` writer - "]
pub type DtenAupllFinW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dten_aupll_fref` reader - "]
pub type DtenAupllFrefR = crate::BitReader;
#[doc = "Field `dten_aupll_fref` writer - "]
pub type DtenAupllFrefW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dten_aupll_fsdm` reader - "]
pub type DtenAupllFsdmR = crate::BitReader;
#[doc = "Field `dten_aupll_fsdm` writer - "]
pub type DtenAupllFsdmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dten_aupll_div15` reader - "]
pub type DtenAupllDiv15R = crate::BitReader;
#[doc = "Field `dten_aupll_div15` writer - "]
pub type DtenAupllDiv15W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dten_aupll_div5` reader - "]
pub type DtenAupllDiv5R = crate::BitReader;
#[doc = "Field `dten_aupll_div5` writer - "]
pub type DtenAupllDiv5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dten_aupll_postdiv_clk` reader - "]
pub type DtenAupllPostdivClkR = crate::BitReader;
#[doc = "Field `dten_aupll_postdiv_clk` writer - "]
pub type DtenAupllPostdivClkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dtest_aupll_pulldown` reader - "]
pub type DtestAupllPulldownR = crate::BitReader;
#[doc = "Field `dtest_aupll_pulldown` writer - "]
pub type DtestAupllPulldownW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn aupll_dc_tp_out_en(&self) -> AupllDcTpOutEnR {
        AupllDcTpOutEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn ten_aupll(&self) -> TenAupllR {
        TenAupllR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn ten_aupll_sfreg(&self) -> TenAupllSfregR {
        TenAupllSfregR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn dten_aupll_fin(&self) -> DtenAupllFinR {
        DtenAupllFinR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn dten_aupll_fref(&self) -> DtenAupllFrefR {
        DtenAupllFrefR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn dten_aupll_fsdm(&self) -> DtenAupllFsdmR {
        DtenAupllFsdmR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn dten_aupll_div15(&self) -> DtenAupllDiv15R {
        DtenAupllDiv15R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn dten_aupll_div5(&self) -> DtenAupllDiv5R {
        DtenAupllDiv5R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn dten_aupll_postdiv_clk(&self) -> DtenAupllPostdivClkR {
        DtenAupllPostdivClkR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn dtest_aupll_pulldown(&self) -> DtestAupllPulldownR {
        DtestAupllPulldownR::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn aupll_dc_tp_out_en(&mut self) -> AupllDcTpOutEnW<'_, AudioPllCfg9Spec> {
        AupllDcTpOutEnW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn ten_aupll(&mut self) -> TenAupllW<'_, AudioPllCfg9Spec> {
        TenAupllW::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn ten_aupll_sfreg(&mut self) -> TenAupllSfregW<'_, AudioPllCfg9Spec> {
        TenAupllSfregW::new(self, 2)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn dten_aupll_fin(&mut self) -> DtenAupllFinW<'_, AudioPllCfg9Spec> {
        DtenAupllFinW::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn dten_aupll_fref(&mut self) -> DtenAupllFrefW<'_, AudioPllCfg9Spec> {
        DtenAupllFrefW::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn dten_aupll_fsdm(&mut self) -> DtenAupllFsdmW<'_, AudioPllCfg9Spec> {
        DtenAupllFsdmW::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn dten_aupll_div15(&mut self) -> DtenAupllDiv15W<'_, AudioPllCfg9Spec> {
        DtenAupllDiv15W::new(self, 7)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn dten_aupll_div5(&mut self) -> DtenAupllDiv5W<'_, AudioPllCfg9Spec> {
        DtenAupllDiv5W::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn dten_aupll_postdiv_clk(&mut self) -> DtenAupllPostdivClkW<'_, AudioPllCfg9Spec> {
        DtenAupllPostdivClkW::new(self, 9)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn dtest_aupll_pulldown(&mut self) -> DtestAupllPulldownW<'_, AudioPllCfg9Spec> {
        DtestAupllPulldownW::new(self, 10)
    }
}
#[doc = "Audio PLL Configuration 9 - Test and Digital Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`audio_pll_cfg9::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`audio_pll_cfg9::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AudioPllCfg9Spec;
impl crate::RegisterSpec for AudioPllCfg9Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`audio_pll_cfg9::R`](R) reader structure"]
impl crate::Readable for AudioPllCfg9Spec {}
#[doc = "`write(|w| ..)` method takes [`audio_pll_cfg9::W`](W) writer structure"]
impl crate::Writable for AudioPllCfg9Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AUDIO_PLL_CFG9 to value 0x0400"]
impl crate::Resettable for AudioPllCfg9Spec {
    const RESET_VALUE: u32 = 0x0400;
}
