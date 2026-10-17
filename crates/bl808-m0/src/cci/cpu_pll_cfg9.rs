#[doc = "Register `CPU_PLL_CFG9` reader"]
pub type R = crate::R<CpuPllCfg9Spec>;
#[doc = "Register `CPU_PLL_CFG9` writer"]
pub type W = crate::W<CpuPllCfg9Spec>;
#[doc = "Field `cpupll_dc_tp_out_en` reader - "]
pub type CpupllDcTpOutEnR = crate::BitReader;
#[doc = "Field `cpupll_dc_tp_out_en` writer - "]
pub type CpupllDcTpOutEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ten_cpupll` reader - "]
pub type TenCpupllR = crate::BitReader;
#[doc = "Field `ten_cpupll` writer - "]
pub type TenCpupllW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ten_cpupll_sfreg` reader - "]
pub type TenCpupllSfregR = crate::BitReader;
#[doc = "Field `ten_cpupll_sfreg` writer - "]
pub type TenCpupllSfregW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dten_cpupll_fin` reader - "]
pub type DtenCpupllFinR = crate::BitReader;
#[doc = "Field `dten_cpupll_fin` writer - "]
pub type DtenCpupllFinW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dten_cpupll_fref` reader - "]
pub type DtenCpupllFrefR = crate::BitReader;
#[doc = "Field `dten_cpupll_fref` writer - "]
pub type DtenCpupllFrefW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dten_cpupll_fsdm` reader - "]
pub type DtenCpupllFsdmR = crate::BitReader;
#[doc = "Field `dten_cpupll_fsdm` writer - "]
pub type DtenCpupllFsdmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dten_cpupll_div15` reader - "]
pub type DtenCpupllDiv15R = crate::BitReader;
#[doc = "Field `dten_cpupll_div15` writer - "]
pub type DtenCpupllDiv15W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dten_cpupll_div5` reader - "]
pub type DtenCpupllDiv5R = crate::BitReader;
#[doc = "Field `dten_cpupll_div5` writer - "]
pub type DtenCpupllDiv5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dten_cpupll_postdiv_clk` reader - "]
pub type DtenCpupllPostdivClkR = crate::BitReader;
#[doc = "Field `dten_cpupll_postdiv_clk` writer - "]
pub type DtenCpupllPostdivClkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dtest_cpupll_pulldown` reader - "]
pub type DtestCpupllPulldownR = crate::BitReader;
#[doc = "Field `dtest_cpupll_pulldown` writer - "]
pub type DtestCpupllPulldownW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cpupll_dc_tp_out_en(&self) -> CpupllDcTpOutEnR {
        CpupllDcTpOutEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn ten_cpupll(&self) -> TenCpupllR {
        TenCpupllR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn ten_cpupll_sfreg(&self) -> TenCpupllSfregR {
        TenCpupllSfregR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn dten_cpupll_fin(&self) -> DtenCpupllFinR {
        DtenCpupllFinR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn dten_cpupll_fref(&self) -> DtenCpupllFrefR {
        DtenCpupllFrefR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn dten_cpupll_fsdm(&self) -> DtenCpupllFsdmR {
        DtenCpupllFsdmR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn dten_cpupll_div15(&self) -> DtenCpupllDiv15R {
        DtenCpupllDiv15R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn dten_cpupll_div5(&self) -> DtenCpupllDiv5R {
        DtenCpupllDiv5R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn dten_cpupll_postdiv_clk(&self) -> DtenCpupllPostdivClkR {
        DtenCpupllPostdivClkR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn dtest_cpupll_pulldown(&self) -> DtestCpupllPulldownR {
        DtestCpupllPulldownR::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cpupll_dc_tp_out_en(&mut self) -> CpupllDcTpOutEnW<'_, CpuPllCfg9Spec> {
        CpupllDcTpOutEnW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn ten_cpupll(&mut self) -> TenCpupllW<'_, CpuPllCfg9Spec> {
        TenCpupllW::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn ten_cpupll_sfreg(&mut self) -> TenCpupllSfregW<'_, CpuPllCfg9Spec> {
        TenCpupllSfregW::new(self, 2)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn dten_cpupll_fin(&mut self) -> DtenCpupllFinW<'_, CpuPllCfg9Spec> {
        DtenCpupllFinW::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn dten_cpupll_fref(&mut self) -> DtenCpupllFrefW<'_, CpuPllCfg9Spec> {
        DtenCpupllFrefW::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn dten_cpupll_fsdm(&mut self) -> DtenCpupllFsdmW<'_, CpuPllCfg9Spec> {
        DtenCpupllFsdmW::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn dten_cpupll_div15(&mut self) -> DtenCpupllDiv15W<'_, CpuPllCfg9Spec> {
        DtenCpupllDiv15W::new(self, 7)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn dten_cpupll_div5(&mut self) -> DtenCpupllDiv5W<'_, CpuPllCfg9Spec> {
        DtenCpupllDiv5W::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn dten_cpupll_postdiv_clk(&mut self) -> DtenCpupllPostdivClkW<'_, CpuPllCfg9Spec> {
        DtenCpupllPostdivClkW::new(self, 9)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn dtest_cpupll_pulldown(&mut self) -> DtestCpupllPulldownW<'_, CpuPllCfg9Spec> {
        DtestCpupllPulldownW::new(self, 10)
    }
}
#[doc = "CPU PLL Configuration 9 - Test and Digital Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`cpu_pll_cfg9::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpu_pll_cfg9::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpuPllCfg9Spec;
impl crate::RegisterSpec for CpuPllCfg9Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpu_pll_cfg9::R`](R) reader structure"]
impl crate::Readable for CpuPllCfg9Spec {}
#[doc = "`write(|w| ..)` method takes [`cpu_pll_cfg9::W`](W) writer structure"]
impl crate::Writable for CpuPllCfg9Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CPU_PLL_CFG9 to value 0x0400"]
impl crate::Resettable for CpuPllCfg9Spec {
    const RESET_VALUE: u32 = 0x0400;
}
