#[doc = "Register `CPU_PLL_CFG1` reader"]
pub type R = crate::R<CpuPllCfg1Spec>;
#[doc = "Register `CPU_PLL_CFG1` writer"]
pub type W = crate::W<CpuPllCfg1Spec>;
#[doc = "Field `cpupll_postdiv` reader - "]
pub type CpupllPostdivR = crate::FieldReader;
#[doc = "Field `cpupll_postdiv` writer - "]
pub type CpupllPostdivW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `cpupll_refdiv_ratio` reader - "]
pub type CpupllRefdivRatioR = crate::FieldReader;
#[doc = "Field `cpupll_refdiv_ratio` writer - "]
pub type CpupllRefdivRatioW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `cpupll_refclk_sel` reader - "]
pub type CpupllRefclkSelR = crate::FieldReader;
#[doc = "Field `cpupll_refclk_sel` writer - "]
pub type CpupllRefclkSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `cpupll_vg11_sel` reader - "]
pub type CpupllVg11SelR = crate::FieldReader;
#[doc = "Field `cpupll_vg11_sel` writer - "]
pub type CpupllVg11SelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `cpupll_vg13_sel` reader - "]
pub type CpupllVg13SelR = crate::FieldReader;
#[doc = "Field `cpupll_vg13_sel` writer - "]
pub type CpupllVg13SelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:6"]
    #[inline(always)]
    pub fn cpupll_postdiv(&self) -> CpupllPostdivR {
        CpupllPostdivR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn cpupll_refdiv_ratio(&self) -> CpupllRefdivRatioR {
        CpupllRefdivRatioR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn cpupll_refclk_sel(&self) -> CpupllRefclkSelR {
        CpupllRefclkSelR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    pub fn cpupll_vg11_sel(&self) -> CpupllVg11SelR {
        CpupllVg11SelR::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn cpupll_vg13_sel(&self) -> CpupllVg13SelR {
        CpupllVg13SelR::new(((self.bits >> 24) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6"]
    #[inline(always)]
    pub fn cpupll_postdiv(&mut self) -> CpupllPostdivW<'_, CpuPllCfg1Spec> {
        CpupllPostdivW::new(self, 0)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn cpupll_refdiv_ratio(&mut self) -> CpupllRefdivRatioW<'_, CpuPllCfg1Spec> {
        CpupllRefdivRatioW::new(self, 8)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn cpupll_refclk_sel(&mut self) -> CpupllRefclkSelW<'_, CpuPllCfg1Spec> {
        CpupllRefclkSelW::new(self, 16)
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    pub fn cpupll_vg11_sel(&mut self) -> CpupllVg11SelW<'_, CpuPllCfg1Spec> {
        CpupllVg11SelW::new(self, 20)
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn cpupll_vg13_sel(&mut self) -> CpupllVg13SelW<'_, CpuPllCfg1Spec> {
        CpupllVg13SelW::new(self, 24)
    }
}
#[doc = "CPU PLL Configuration 1 - Dividers\n\nYou can [`read`](crate::Reg::read) this register and get [`cpu_pll_cfg1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpu_pll_cfg1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpuPllCfg1Spec;
impl crate::RegisterSpec for CpuPllCfg1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpu_pll_cfg1::R`](R) reader structure"]
impl crate::Readable for CpuPllCfg1Spec {}
#[doc = "`write(|w| ..)` method takes [`cpu_pll_cfg1::W`](W) writer structure"]
impl crate::Writable for CpuPllCfg1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CPU_PLL_CFG1 to value 0x0110_0418"]
impl crate::Resettable for CpuPllCfg1Spec {
    const RESET_VALUE: u32 = 0x0110_0418;
}
