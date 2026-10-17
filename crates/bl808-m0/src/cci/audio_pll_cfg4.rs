#[doc = "Register `AUDIO_PLL_CFG4` reader"]
pub type R = crate::R<AudioPllCfg4Spec>;
#[doc = "Register `AUDIO_PLL_CFG4` writer"]
pub type W = crate::W<AudioPllCfg4Spec>;
#[doc = "Field `aupll_sel_sample_clk` reader - Sample clock selection"]
pub type AupllSelSampleClkR = crate::FieldReader;
#[doc = "Field `aupll_sel_sample_clk` writer - Sample clock selection"]
pub type AupllSelSampleClkW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `aupll_sel_fb_clk` reader - Feedback clock selection"]
pub type AupllSelFbClkR = crate::FieldReader;
#[doc = "Field `aupll_sel_fb_clk` writer - Feedback clock selection"]
pub type AupllSelFbClkW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `aupll_sdmclk_sel` reader - SDM clock selection"]
pub type AupllSdmclkSelR = crate::BitReader;
#[doc = "Field `aupll_sdmclk_sel` writer - SDM clock selection"]
pub type AupllSdmclkSelW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - Sample clock selection"]
    #[inline(always)]
    pub fn aupll_sel_sample_clk(&self) -> AupllSelSampleClkR {
        AupllSelSampleClkR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:5 - Feedback clock selection"]
    #[inline(always)]
    pub fn aupll_sel_fb_clk(&self) -> AupllSelFbClkR {
        AupllSelFbClkR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 8 - SDM clock selection"]
    #[inline(always)]
    pub fn aupll_sdmclk_sel(&self) -> AupllSdmclkSelR {
        AupllSdmclkSelR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Sample clock selection"]
    #[inline(always)]
    pub fn aupll_sel_sample_clk(&mut self) -> AupllSelSampleClkW<'_, AudioPllCfg4Spec> {
        AupllSelSampleClkW::new(self, 0)
    }
    #[doc = "Bits 4:5 - Feedback clock selection"]
    #[inline(always)]
    pub fn aupll_sel_fb_clk(&mut self) -> AupllSelFbClkW<'_, AudioPllCfg4Spec> {
        AupllSelFbClkW::new(self, 4)
    }
    #[doc = "Bit 8 - SDM clock selection"]
    #[inline(always)]
    pub fn aupll_sdmclk_sel(&mut self) -> AupllSdmclkSelW<'_, AudioPllCfg4Spec> {
        AupllSdmclkSelW::new(self, 8)
    }
}
#[doc = "Audio PLL Configuration 4 - Clock Select\n\nYou can [`read`](crate::Reg::read) this register and get [`audio_pll_cfg4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`audio_pll_cfg4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AudioPllCfg4Spec;
impl crate::RegisterSpec for AudioPllCfg4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`audio_pll_cfg4::R`](R) reader structure"]
impl crate::Readable for AudioPllCfg4Spec {}
#[doc = "`write(|w| ..)` method takes [`audio_pll_cfg4::W`](W) writer structure"]
impl crate::Writable for AudioPllCfg4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AUDIO_PLL_CFG4 to value 0x0111"]
impl crate::Resettable for AudioPllCfg4Spec {
    const RESET_VALUE: u32 = 0x0111;
}
