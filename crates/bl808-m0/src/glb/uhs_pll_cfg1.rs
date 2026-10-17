#[doc = "Register `UHS_PLL_CFG1` reader"]
pub type R = crate::R<UhsPllCfg1Spec>;
#[doc = "Register `UHS_PLL_CFG1` writer"]
pub type W = crate::W<UhsPllCfg1Spec>;
#[doc = "Field `uhspll_even_div_ratio` reader - "]
pub type UhspllEvenDivRatioR = crate::FieldReader;
#[doc = "Field `uhspll_even_div_ratio` writer - "]
pub type UhspllEvenDivRatioW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `uhspll_even_div_en` reader - "]
pub type UhspllEvenDivEnR = crate::BitReader;
#[doc = "Field `uhspll_even_div_en` writer - "]
pub type UhspllEvenDivEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `uhspll_refdiv_ratio` reader - "]
pub type UhspllRefdivRatioR = crate::FieldReader;
#[doc = "Field `uhspll_refdiv_ratio` writer - "]
pub type UhspllRefdivRatioW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `uhspll_refclk_sel` reader - "]
pub type UhspllRefclkSelR = crate::FieldReader;
#[doc = "Field `uhspll_refclk_sel` writer - "]
pub type UhspllRefclkSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `uhspll_vg11_sel` reader - "]
pub type UhspllVg11SelR = crate::FieldReader;
#[doc = "Field `uhspll_vg11_sel` writer - "]
pub type UhspllVg11SelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `uhspll_vg13_sel` reader - "]
pub type UhspllVg13SelR = crate::FieldReader;
#[doc = "Field `uhspll_vg13_sel` writer - "]
pub type UhspllVg13SelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:6"]
    #[inline(always)]
    pub fn uhspll_even_div_ratio(&self) -> UhspllEvenDivRatioR {
        UhspllEvenDivRatioR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn uhspll_even_div_en(&self) -> UhspllEvenDivEnR {
        UhspllEvenDivEnR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn uhspll_refdiv_ratio(&self) -> UhspllRefdivRatioR {
        UhspllRefdivRatioR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn uhspll_refclk_sel(&self) -> UhspllRefclkSelR {
        UhspllRefclkSelR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    pub fn uhspll_vg11_sel(&self) -> UhspllVg11SelR {
        UhspllVg11SelR::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn uhspll_vg13_sel(&self) -> UhspllVg13SelR {
        UhspllVg13SelR::new(((self.bits >> 24) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6"]
    #[inline(always)]
    pub fn uhspll_even_div_ratio(&mut self) -> UhspllEvenDivRatioW<'_, UhsPllCfg1Spec> {
        UhspllEvenDivRatioW::new(self, 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn uhspll_even_div_en(&mut self) -> UhspllEvenDivEnW<'_, UhsPllCfg1Spec> {
        UhspllEvenDivEnW::new(self, 7)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn uhspll_refdiv_ratio(&mut self) -> UhspllRefdivRatioW<'_, UhsPllCfg1Spec> {
        UhspllRefdivRatioW::new(self, 8)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn uhspll_refclk_sel(&mut self) -> UhspllRefclkSelW<'_, UhsPllCfg1Spec> {
        UhspllRefclkSelW::new(self, 16)
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    pub fn uhspll_vg11_sel(&mut self) -> UhspllVg11SelW<'_, UhsPllCfg1Spec> {
        UhspllVg11SelW::new(self, 20)
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn uhspll_vg13_sel(&mut self) -> UhspllVg13SelW<'_, UhsPllCfg1Spec> {
        UhspllVg13SelW::new(self, 24)
    }
}
#[doc = "UHS PLL Configuration 1 - Divider and Clock Select\n\nYou can [`read`](crate::Reg::read) this register and get [`uhs_pll_cfg1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uhs_pll_cfg1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UhsPllCfg1Spec;
impl crate::RegisterSpec for UhsPllCfg1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uhs_pll_cfg1::R`](R) reader structure"]
impl crate::Readable for UhsPllCfg1Spec {}
#[doc = "`write(|w| ..)` method takes [`uhs_pll_cfg1::W`](W) writer structure"]
impl crate::Writable for UhsPllCfg1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UHS_PLL_CFG1 to value 0"]
impl crate::Resettable for UhsPllCfg1Spec {}
