#[doc = "Register `AUDIO_PLL_CFG1` reader"]
pub type R = crate::R<AudioPllCfg1Spec>;
#[doc = "Register `AUDIO_PLL_CFG1` writer"]
pub type W = crate::W<AudioPllCfg1Spec>;
#[doc = "Field `aupll_postdiv` reader - Post divider ratio"]
pub type AupllPostdivR = crate::FieldReader;
#[doc = "Field `aupll_postdiv` writer - Post divider ratio"]
pub type AupllPostdivW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `aupll_refdiv_ratio` reader - Reference clock divider ratio"]
pub type AupllRefdivRatioR = crate::FieldReader;
#[doc = "Field `aupll_refdiv_ratio` writer - Reference clock divider ratio"]
pub type AupllRefdivRatioW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `aupll_refclk_sel` reader - Reference clock source selection"]
pub type AupllRefclkSelR = crate::FieldReader;
#[doc = "Field `aupll_refclk_sel` writer - Reference clock source selection"]
pub type AupllRefclkSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `aupll_vg11_sel` reader - VG11 selection"]
pub type AupllVg11SelR = crate::FieldReader;
#[doc = "Field `aupll_vg11_sel` writer - VG11 selection"]
pub type AupllVg11SelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `aupll_vg13_sel` reader - VG13 selection"]
pub type AupllVg13SelR = crate::FieldReader;
#[doc = "Field `aupll_vg13_sel` writer - VG13 selection"]
pub type AupllVg13SelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:6 - Post divider ratio"]
    #[inline(always)]
    pub fn aupll_postdiv(&self) -> AupllPostdivR {
        AupllPostdivR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:11 - Reference clock divider ratio"]
    #[inline(always)]
    pub fn aupll_refdiv_ratio(&self) -> AupllRefdivRatioR {
        AupllRefdivRatioR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:17 - Reference clock source selection"]
    #[inline(always)]
    pub fn aupll_refclk_sel(&self) -> AupllRefclkSelR {
        AupllRefclkSelR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 20:21 - VG11 selection"]
    #[inline(always)]
    pub fn aupll_vg11_sel(&self) -> AupllVg11SelR {
        AupllVg11SelR::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 24:25 - VG13 selection"]
    #[inline(always)]
    pub fn aupll_vg13_sel(&self) -> AupllVg13SelR {
        AupllVg13SelR::new(((self.bits >> 24) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Post divider ratio"]
    #[inline(always)]
    pub fn aupll_postdiv(&mut self) -> AupllPostdivW<'_, AudioPllCfg1Spec> {
        AupllPostdivW::new(self, 0)
    }
    #[doc = "Bits 8:11 - Reference clock divider ratio"]
    #[inline(always)]
    pub fn aupll_refdiv_ratio(&mut self) -> AupllRefdivRatioW<'_, AudioPllCfg1Spec> {
        AupllRefdivRatioW::new(self, 8)
    }
    #[doc = "Bits 16:17 - Reference clock source selection"]
    #[inline(always)]
    pub fn aupll_refclk_sel(&mut self) -> AupllRefclkSelW<'_, AudioPllCfg1Spec> {
        AupllRefclkSelW::new(self, 16)
    }
    #[doc = "Bits 20:21 - VG11 selection"]
    #[inline(always)]
    pub fn aupll_vg11_sel(&mut self) -> AupllVg11SelW<'_, AudioPllCfg1Spec> {
        AupllVg11SelW::new(self, 20)
    }
    #[doc = "Bits 24:25 - VG13 selection"]
    #[inline(always)]
    pub fn aupll_vg13_sel(&mut self) -> AupllVg13SelW<'_, AudioPllCfg1Spec> {
        AupllVg13SelW::new(self, 24)
    }
}
#[doc = "Audio PLL Configuration 1 - Dividers\n\nYou can [`read`](crate::Reg::read) this register and get [`audio_pll_cfg1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`audio_pll_cfg1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AudioPllCfg1Spec;
impl crate::RegisterSpec for AudioPllCfg1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`audio_pll_cfg1::R`](R) reader structure"]
impl crate::Readable for AudioPllCfg1Spec {}
#[doc = "`write(|w| ..)` method takes [`audio_pll_cfg1::W`](W) writer structure"]
impl crate::Writable for AudioPllCfg1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AUDIO_PLL_CFG1 to value 0x0110_0412"]
impl crate::Resettable for AudioPllCfg1Spec {
    const RESET_VALUE: u32 = 0x0110_0412;
}
