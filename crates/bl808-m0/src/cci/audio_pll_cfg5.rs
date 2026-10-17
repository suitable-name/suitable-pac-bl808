#[doc = "Register `AUDIO_PLL_CFG5` reader"]
pub type R = crate::R<AudioPllCfg5Spec>;
#[doc = "Register `AUDIO_PLL_CFG5` writer"]
pub type W = crate::W<AudioPllCfg5Spec>;
#[doc = "Field `aupll_vco_speed` reader - VCO speed control"]
pub type AupllVcoSpeedR = crate::FieldReader;
#[doc = "Field `aupll_vco_speed` writer - VCO speed control"]
pub type AupllVcoSpeedW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - VCO speed control"]
    #[inline(always)]
    pub fn aupll_vco_speed(&self) -> AupllVcoSpeedR {
        AupllVcoSpeedR::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - VCO speed control"]
    #[inline(always)]
    pub fn aupll_vco_speed(&mut self) -> AupllVcoSpeedW<'_, AudioPllCfg5Spec> {
        AupllVcoSpeedW::new(self, 0)
    }
}
#[doc = "Audio PLL Configuration 5 - VCO Speed\n\nYou can [`read`](crate::Reg::read) this register and get [`audio_pll_cfg5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`audio_pll_cfg5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AudioPllCfg5Spec;
impl crate::RegisterSpec for AudioPllCfg5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`audio_pll_cfg5::R`](R) reader structure"]
impl crate::Readable for AudioPllCfg5Spec {}
#[doc = "`write(|w| ..)` method takes [`audio_pll_cfg5::W`](W) writer structure"]
impl crate::Writable for AudioPllCfg5Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AUDIO_PLL_CFG5 to value 0x03"]
impl crate::Resettable for AudioPllCfg5Spec {
    const RESET_VALUE: u32 = 0x03;
}
