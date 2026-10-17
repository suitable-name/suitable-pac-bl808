#[doc = "Register `AUDIO_PLL_CFG7` reader"]
pub type R = crate::R<AudioPllCfg7Spec>;
#[doc = "Register `AUDIO_PLL_CFG7` writer"]
pub type W = crate::W<AudioPllCfg7Spec>;
#[doc = "Field `aupll_sdm_order_sel` reader - SDM modulator order selection"]
pub type AupllSdmOrderSelR = crate::BitReader;
#[doc = "Field `aupll_sdm_order_sel` writer - SDM modulator order selection"]
pub type AupllSdmOrderSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `aupll_sdm_sig_dith_sel` reader - SDM signal dithering selection"]
pub type AupllSdmSigDithSelR = crate::FieldReader;
#[doc = "Field `aupll_sdm_sig_dith_sel` writer - SDM signal dithering selection"]
pub type AupllSdmSigDithSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - SDM modulator order selection"]
    #[inline(always)]
    pub fn aupll_sdm_order_sel(&self) -> AupllSdmOrderSelR {
        AupllSdmOrderSelR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 16:17 - SDM signal dithering selection"]
    #[inline(always)]
    pub fn aupll_sdm_sig_dith_sel(&self) -> AupllSdmSigDithSelR {
        AupllSdmSigDithSelR::new(((self.bits >> 16) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - SDM modulator order selection"]
    #[inline(always)]
    pub fn aupll_sdm_order_sel(&mut self) -> AupllSdmOrderSelW<'_, AudioPllCfg7Spec> {
        AupllSdmOrderSelW::new(self, 0)
    }
    #[doc = "Bits 16:17 - SDM signal dithering selection"]
    #[inline(always)]
    pub fn aupll_sdm_sig_dith_sel(&mut self) -> AupllSdmSigDithSelW<'_, AudioPllCfg7Spec> {
        AupllSdmSigDithSelW::new(self, 16)
    }
}
#[doc = "Audio PLL Configuration 7 - SDM Control\n\nYou can [`read`](crate::Reg::read) this register and get [`audio_pll_cfg7::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`audio_pll_cfg7::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AudioPllCfg7Spec;
impl crate::RegisterSpec for AudioPllCfg7Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`audio_pll_cfg7::R`](R) reader structure"]
impl crate::Readable for AudioPllCfg7Spec {}
#[doc = "`write(|w| ..)` method takes [`audio_pll_cfg7::W`](W) writer structure"]
impl crate::Writable for AudioPllCfg7Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AUDIO_PLL_CFG7 to value 0x01"]
impl crate::Resettable for AudioPllCfg7Spec {
    const RESET_VALUE: u32 = 0x01;
}
