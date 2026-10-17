#[doc = "Register `AUDIO_PLL_CFG6` reader"]
pub type R = crate::R<AudioPllCfg6Spec>;
#[doc = "Register `AUDIO_PLL_CFG6` writer"]
pub type W = crate::W<AudioPllCfg6Spec>;
#[doc = "Field `aupll_sdmin` reader - SDM fractional input value"]
pub type AupllSdminR = crate::FieldReader<u32>;
#[doc = "Field `aupll_sdmin` writer - SDM fractional input value"]
pub type AupllSdminW<'a, REG> = crate::FieldWriter<'a, REG, 19, u32>;
#[doc = "Field `aupll_sdm_bypass` reader - SDM bypass enable"]
pub type AupllSdmBypassR = crate::BitReader;
#[doc = "Field `aupll_sdm_bypass` writer - SDM bypass enable"]
pub type AupllSdmBypassW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:18 - SDM fractional input value"]
    #[inline(always)]
    pub fn aupll_sdmin(&self) -> AupllSdminR {
        AupllSdminR::new(self.bits & 0x0007_ffff)
    }
    #[doc = "Bit 24 - SDM bypass enable"]
    #[inline(always)]
    pub fn aupll_sdm_bypass(&self) -> AupllSdmBypassR {
        AupllSdmBypassR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:18 - SDM fractional input value"]
    #[inline(always)]
    pub fn aupll_sdmin(&mut self) -> AupllSdminW<'_, AudioPllCfg6Spec> {
        AupllSdminW::new(self, 0)
    }
    #[doc = "Bit 24 - SDM bypass enable"]
    #[inline(always)]
    pub fn aupll_sdm_bypass(&mut self) -> AupllSdmBypassW<'_, AudioPllCfg6Spec> {
        AupllSdmBypassW::new(self, 24)
    }
}
#[doc = "Audio PLL Configuration 6 - SDM Input\n\nYou can [`read`](crate::Reg::read) this register and get [`audio_pll_cfg6::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`audio_pll_cfg6::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AudioPllCfg6Spec;
impl crate::RegisterSpec for AudioPllCfg6Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`audio_pll_cfg6::R`](R) reader structure"]
impl crate::Readable for AudioPllCfg6Spec {}
#[doc = "`write(|w| ..)` method takes [`audio_pll_cfg6::W`](W) writer structure"]
impl crate::Writable for AudioPllCfg6Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AUDIO_PLL_CFG6 to value 0x0001_61e5"]
impl crate::Resettable for AudioPllCfg6Spec {
    const RESET_VALUE: u32 = 0x0001_61e5;
}
