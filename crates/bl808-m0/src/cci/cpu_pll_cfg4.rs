#[doc = "Register `CPU_PLL_CFG4` reader"]
pub type R = crate::R<CpuPllCfg4Spec>;
#[doc = "Register `CPU_PLL_CFG4` writer"]
pub type W = crate::W<CpuPllCfg4Spec>;
#[doc = "Field `cpupll_sel_sample_clk` reader - "]
pub type CpupllSelSampleClkR = crate::FieldReader;
#[doc = "Field `cpupll_sel_sample_clk` writer - "]
pub type CpupllSelSampleClkW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `cpupll_sel_fb_clk` reader - "]
pub type CpupllSelFbClkR = crate::FieldReader;
#[doc = "Field `cpupll_sel_fb_clk` writer - "]
pub type CpupllSelFbClkW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `cpupll_sdmclk_sel` reader - "]
pub type CpupllSdmclkSelR = crate::BitReader;
#[doc = "Field `cpupll_sdmclk_sel` writer - "]
pub type CpupllSdmclkSelW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn cpupll_sel_sample_clk(&self) -> CpupllSelSampleClkR {
        CpupllSelSampleClkR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn cpupll_sel_fb_clk(&self) -> CpupllSelFbClkR {
        CpupllSelFbClkR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn cpupll_sdmclk_sel(&self) -> CpupllSdmclkSelR {
        CpupllSdmclkSelR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn cpupll_sel_sample_clk(&mut self) -> CpupllSelSampleClkW<'_, CpuPllCfg4Spec> {
        CpupllSelSampleClkW::new(self, 0)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn cpupll_sel_fb_clk(&mut self) -> CpupllSelFbClkW<'_, CpuPllCfg4Spec> {
        CpupllSelFbClkW::new(self, 4)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn cpupll_sdmclk_sel(&mut self) -> CpupllSdmclkSelW<'_, CpuPllCfg4Spec> {
        CpupllSdmclkSelW::new(self, 8)
    }
}
#[doc = "CPU PLL Configuration 4 - Clock Select\n\nYou can [`read`](crate::Reg::read) this register and get [`cpu_pll_cfg4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpu_pll_cfg4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpuPllCfg4Spec;
impl crate::RegisterSpec for CpuPllCfg4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpu_pll_cfg4::R`](R) reader structure"]
impl crate::Readable for CpuPllCfg4Spec {}
#[doc = "`write(|w| ..)` method takes [`cpu_pll_cfg4::W`](W) writer structure"]
impl crate::Writable for CpuPllCfg4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CPU_PLL_CFG4 to value 0x0111"]
impl crate::Resettable for CpuPllCfg4Spec {
    const RESET_VALUE: u32 = 0x0111;
}
