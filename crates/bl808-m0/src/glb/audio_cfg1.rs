#[doc = "Register `AUDIO_CFG1` reader"]
pub type R = crate::R<AudioCfg1Spec>;
#[doc = "Register `AUDIO_CFG1` writer"]
pub type W = crate::W<AudioCfg1Spec>;
#[doc = "Field `reg_padc_clk_div` reader - "]
pub type RegPadcClkDivR = crate::FieldReader<u16>;
#[doc = "Field `reg_padc_clk_div` writer - "]
pub type RegPadcClkDivW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `reg_padc_clk_en` reader - "]
pub type RegPadcClkEnR = crate::BitReader;
#[doc = "Field `reg_padc_clk_en` writer - "]
pub type RegPadcClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn reg_padc_clk_div(&self) -> RegPadcClkDivR {
        RegPadcClkDivR::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn reg_padc_clk_en(&self) -> RegPadcClkEnR {
        RegPadcClkEnR::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn reg_padc_clk_div(&mut self) -> RegPadcClkDivW<'_, AudioCfg1Spec> {
        RegPadcClkDivW::new(self, 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn reg_padc_clk_en(&mut self) -> RegPadcClkEnW<'_, AudioCfg1Spec> {
        RegPadcClkEnW::new(self, 10)
    }
}
#[doc = "Audio Clock Configuration 1 (PADC)\n\nYou can [`read`](crate::Reg::read) this register and get [`audio_cfg1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`audio_cfg1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AudioCfg1Spec;
impl crate::RegisterSpec for AudioCfg1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`audio_cfg1::R`](R) reader structure"]
impl crate::Readable for AudioCfg1Spec {}
#[doc = "`write(|w| ..)` method takes [`audio_cfg1::W`](W) writer structure"]
impl crate::Writable for AudioCfg1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AUDIO_CFG1 to value 0"]
impl crate::Resettable for AudioCfg1Spec {}
