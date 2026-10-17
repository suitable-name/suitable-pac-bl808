#[doc = "Register `MIPI_PLL_CFG1` reader"]
pub type R = crate::R<MipiPllCfg1Spec>;
#[doc = "Register `MIPI_PLL_CFG1` writer"]
pub type W = crate::W<MipiPllCfg1Spec>;
#[doc = "Field `mipipll_even_div_ratio` reader - "]
pub type MipipllEvenDivRatioR = crate::FieldReader;
#[doc = "Field `mipipll_even_div_ratio` writer - "]
pub type MipipllEvenDivRatioW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `mipipll_even_div_en` reader - "]
pub type MipipllEvenDivEnR = crate::BitReader;
#[doc = "Field `mipipll_even_div_en` writer - "]
pub type MipipllEvenDivEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `mipipll_refdiv_ratio` reader - "]
pub type MipipllRefdivRatioR = crate::FieldReader;
#[doc = "Field `mipipll_refdiv_ratio` writer - "]
pub type MipipllRefdivRatioW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `mipipll_refclk_sel` reader - "]
pub type MipipllRefclkSelR = crate::FieldReader;
#[doc = "Field `mipipll_refclk_sel` writer - "]
pub type MipipllRefclkSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `mipipll_vg11_sel` reader - "]
pub type MipipllVg11SelR = crate::FieldReader;
#[doc = "Field `mipipll_vg11_sel` writer - "]
pub type MipipllVg11SelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `mipipll_vg13_sel` reader - "]
pub type MipipllVg13SelR = crate::FieldReader;
#[doc = "Field `mipipll_vg13_sel` writer - "]
pub type MipipllVg13SelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:6"]
    #[inline(always)]
    pub fn mipipll_even_div_ratio(&self) -> MipipllEvenDivRatioR {
        MipipllEvenDivRatioR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn mipipll_even_div_en(&self) -> MipipllEvenDivEnR {
        MipipllEvenDivEnR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn mipipll_refdiv_ratio(&self) -> MipipllRefdivRatioR {
        MipipllRefdivRatioR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn mipipll_refclk_sel(&self) -> MipipllRefclkSelR {
        MipipllRefclkSelR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    pub fn mipipll_vg11_sel(&self) -> MipipllVg11SelR {
        MipipllVg11SelR::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn mipipll_vg13_sel(&self) -> MipipllVg13SelR {
        MipipllVg13SelR::new(((self.bits >> 24) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6"]
    #[inline(always)]
    pub fn mipipll_even_div_ratio(&mut self) -> MipipllEvenDivRatioW<'_, MipiPllCfg1Spec> {
        MipipllEvenDivRatioW::new(self, 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn mipipll_even_div_en(&mut self) -> MipipllEvenDivEnW<'_, MipiPllCfg1Spec> {
        MipipllEvenDivEnW::new(self, 7)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn mipipll_refdiv_ratio(&mut self) -> MipipllRefdivRatioW<'_, MipiPllCfg1Spec> {
        MipipllRefdivRatioW::new(self, 8)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn mipipll_refclk_sel(&mut self) -> MipipllRefclkSelW<'_, MipiPllCfg1Spec> {
        MipipllRefclkSelW::new(self, 16)
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    pub fn mipipll_vg11_sel(&mut self) -> MipipllVg11SelW<'_, MipiPllCfg1Spec> {
        MipipllVg11SelW::new(self, 20)
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn mipipll_vg13_sel(&mut self) -> MipipllVg13SelW<'_, MipiPllCfg1Spec> {
        MipipllVg13SelW::new(self, 24)
    }
}
#[doc = "MIPI PLL Configuration 1 - Divider and Clock Select\n\nYou can [`read`](crate::Reg::read) this register and get [`mipi_pll_cfg1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mipi_pll_cfg1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MipiPllCfg1Spec;
impl crate::RegisterSpec for MipiPllCfg1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mipi_pll_cfg1::R`](R) reader structure"]
impl crate::Readable for MipiPllCfg1Spec {}
#[doc = "`write(|w| ..)` method takes [`mipi_pll_cfg1::W`](W) writer structure"]
impl crate::Writable for MipiPllCfg1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MIPI_PLL_CFG1 to value 0"]
impl crate::Resettable for MipiPllCfg1Spec {}
