#[doc = "Register `AUDIO_PLL_CFG3` reader"]
pub type R = crate::R<AudioPllCfg3Spec>;
#[doc = "Register `AUDIO_PLL_CFG3` writer"]
pub type W = crate::W<AudioPllCfg3Spec>;
#[doc = "Field `aupll_c4_en` reader - C4 loop filter capacitor enable"]
pub type AupllC4EnR = crate::BitReader;
#[doc = "Field `aupll_c4_en` writer - C4 loop filter capacitor enable"]
pub type AupllC4EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `aupll_r4` reader - R4 resistor trimming"]
pub type AupllR4R = crate::FieldReader;
#[doc = "Field `aupll_r4` writer - R4 resistor trimming"]
pub type AupllR4W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `aupll_r4_short` reader - R4 short connection control"]
pub type AupllR4ShortR = crate::BitReader;
#[doc = "Field `aupll_r4_short` writer - R4 short connection control"]
pub type AupllR4ShortW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `aupll_c3` reader - C3 capacitor trimming"]
pub type AupllC3R = crate::FieldReader;
#[doc = "Field `aupll_c3` writer - C3 capacitor trimming"]
pub type AupllC3W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `aupll_cz` reader - CZ capacitor trimming"]
pub type AupllCzR = crate::FieldReader;
#[doc = "Field `aupll_cz` writer - CZ capacitor trimming"]
pub type AupllCzW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `aupll_rz` reader - RZ resistor trimming"]
pub type AupllRzR = crate::FieldReader;
#[doc = "Field `aupll_rz` writer - RZ resistor trimming"]
pub type AupllRzW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bit 0 - C4 loop filter capacitor enable"]
    #[inline(always)]
    pub fn aupll_c4_en(&self) -> AupllC4EnR {
        AupllC4EnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 4:5 - R4 resistor trimming"]
    #[inline(always)]
    pub fn aupll_r4(&self) -> AupllR4R {
        AupllR4R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 8 - R4 short connection control"]
    #[inline(always)]
    pub fn aupll_r4_short(&self) -> AupllR4ShortR {
        AupllR4ShortR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 12:13 - C3 capacitor trimming"]
    #[inline(always)]
    pub fn aupll_c3(&self) -> AupllC3R {
        AupllC3R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - CZ capacitor trimming"]
    #[inline(always)]
    pub fn aupll_cz(&self) -> AupllCzR {
        AupllCzR::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:18 - RZ resistor trimming"]
    #[inline(always)]
    pub fn aupll_rz(&self) -> AupllRzR {
        AupllRzR::new(((self.bits >> 16) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - C4 loop filter capacitor enable"]
    #[inline(always)]
    pub fn aupll_c4_en(&mut self) -> AupllC4EnW<'_, AudioPllCfg3Spec> {
        AupllC4EnW::new(self, 0)
    }
    #[doc = "Bits 4:5 - R4 resistor trimming"]
    #[inline(always)]
    pub fn aupll_r4(&mut self) -> AupllR4W<'_, AudioPllCfg3Spec> {
        AupllR4W::new(self, 4)
    }
    #[doc = "Bit 8 - R4 short connection control"]
    #[inline(always)]
    pub fn aupll_r4_short(&mut self) -> AupllR4ShortW<'_, AudioPllCfg3Spec> {
        AupllR4ShortW::new(self, 8)
    }
    #[doc = "Bits 12:13 - C3 capacitor trimming"]
    #[inline(always)]
    pub fn aupll_c3(&mut self) -> AupllC3W<'_, AudioPllCfg3Spec> {
        AupllC3W::new(self, 12)
    }
    #[doc = "Bits 14:15 - CZ capacitor trimming"]
    #[inline(always)]
    pub fn aupll_cz(&mut self) -> AupllCzW<'_, AudioPllCfg3Spec> {
        AupllCzW::new(self, 14)
    }
    #[doc = "Bits 16:18 - RZ resistor trimming"]
    #[inline(always)]
    pub fn aupll_rz(&mut self) -> AupllRzW<'_, AudioPllCfg3Spec> {
        AupllRzW::new(self, 16)
    }
}
#[doc = "Audio PLL Configuration 3 - Loop Filter\n\nYou can [`read`](crate::Reg::read) this register and get [`audio_pll_cfg3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`audio_pll_cfg3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AudioPllCfg3Spec;
impl crate::RegisterSpec for AudioPllCfg3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`audio_pll_cfg3::R`](R) reader structure"]
impl crate::Readable for AudioPllCfg3Spec {}
#[doc = "`write(|w| ..)` method takes [`audio_pll_cfg3::W`](W) writer structure"]
impl crate::Writable for AudioPllCfg3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AUDIO_PLL_CFG3 to value 0x0005_a021"]
impl crate::Resettable for AudioPllCfg3Spec {
    const RESET_VALUE: u32 = 0x0005_a021;
}
