#[doc = "Register `AUDIO_PLL_CFG10` reader"]
pub type R = crate::R<AudioPllCfg10Spec>;
#[doc = "Register `AUDIO_PLL_CFG10` writer"]
pub type W = crate::W<AudioPllCfg10Spec>;
#[doc = "Field `aupll_ssc_en` reader - "]
pub type AupllSscEnR = crate::BitReader;
#[doc = "Field `aupll_ssc_en` writer - "]
pub type AupllSscEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `aupll_ssc_cnt` reader - "]
pub type AupllSscCntR = crate::FieldReader;
#[doc = "Field `aupll_ssc_cnt` writer - "]
pub type AupllSscCntW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `aupll_ssc_gain` reader - "]
pub type AupllSscGainR = crate::FieldReader;
#[doc = "Field `aupll_ssc_gain` writer - "]
pub type AupllSscGainW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `aupll_ssc_start_gate_en` reader - "]
pub type AupllSscStartGateEnR = crate::BitReader;
#[doc = "Field `aupll_ssc_start_gate_en` writer - "]
pub type AupllSscStartGateEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `aupll_ssc_start` reader - "]
pub type AupllSscStartR = crate::BitReader;
#[doc = "Field `aupll_ssc_start` writer - "]
pub type AupllSscStartW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn aupll_ssc_en(&self) -> AupllSscEnR {
        AupllSscEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 4:11"]
    #[inline(always)]
    pub fn aupll_ssc_cnt(&self) -> AupllSscCntR {
        AupllSscCntR::new(((self.bits >> 4) & 0xff) as u8)
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    pub fn aupll_ssc_gain(&self) -> AupllSscGainR {
        AupllSscGainR::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn aupll_ssc_start_gate_en(&self) -> AupllSscStartGateEnR {
        AupllSscStartGateEnR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn aupll_ssc_start(&self) -> AupllSscStartR {
        AupllSscStartR::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn aupll_ssc_en(&mut self) -> AupllSscEnW<'_, AudioPllCfg10Spec> {
        AupllSscEnW::new(self, 0)
    }
    #[doc = "Bits 4:11"]
    #[inline(always)]
    pub fn aupll_ssc_cnt(&mut self) -> AupllSscCntW<'_, AudioPllCfg10Spec> {
        AupllSscCntW::new(self, 4)
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    pub fn aupll_ssc_gain(&mut self) -> AupllSscGainW<'_, AudioPllCfg10Spec> {
        AupllSscGainW::new(self, 12)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn aupll_ssc_start_gate_en(&mut self) -> AupllSscStartGateEnW<'_, AudioPllCfg10Spec> {
        AupllSscStartGateEnW::new(self, 16)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn aupll_ssc_start(&mut self) -> AupllSscStartW<'_, AudioPllCfg10Spec> {
        AupllSscStartW::new(self, 20)
    }
}
#[doc = "Audio PLL Configuration 10 - SSC Control\n\nYou can [`read`](crate::Reg::read) this register and get [`audio_pll_cfg10::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`audio_pll_cfg10::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AudioPllCfg10Spec;
impl crate::RegisterSpec for AudioPllCfg10Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`audio_pll_cfg10::R`](R) reader structure"]
impl crate::Readable for AudioPllCfg10Spec {}
#[doc = "`write(|w| ..)` method takes [`audio_pll_cfg10::W`](W) writer structure"]
impl crate::Writable for AudioPllCfg10Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AUDIO_PLL_CFG10 to value 0x0010_4640"]
impl crate::Resettable for AudioPllCfg10Spec {
    const RESET_VALUE: u32 = 0x0010_4640;
}
