#[doc = "Register `AUDIO_PLL_CFG2` reader"]
pub type R = crate::R<AudioPllCfg2Spec>;
#[doc = "Register `AUDIO_PLL_CFG2` writer"]
pub type W = crate::W<AudioPllCfg2Spec>;
#[doc = "Field `aupll_sel_cp_bias` reader - Select charge pump bias"]
pub type AupllSelCpBiasR = crate::BitReader;
#[doc = "Field `aupll_sel_cp_bias` writer - Select charge pump bias"]
pub type AupllSelCpBiasW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `aupll_icp_5u` reader - Charge pump current (5uA)"]
pub type AupllIcp5uR = crate::FieldReader;
#[doc = "Field `aupll_icp_5u` writer - Charge pump current (5uA)"]
pub type AupllIcp5uW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `aupll_icp_1u` reader - Charge pump current (1uA)"]
pub type AupllIcp1uR = crate::FieldReader;
#[doc = "Field `aupll_icp_1u` writer - Charge pump current (1uA)"]
pub type AupllIcp1uW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `aupll_int_frac_sw` reader - Integer/Fractional mode switch"]
pub type AupllIntFracSwR = crate::BitReader;
#[doc = "Field `aupll_int_frac_sw` writer - Integer/Fractional mode switch"]
pub type AupllIntFracSwW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `aupll_cp_startup_en` reader - Charge pump startup enable"]
pub type AupllCpStartupEnR = crate::BitReader;
#[doc = "Field `aupll_cp_startup_en` writer - Charge pump startup enable"]
pub type AupllCpStartupEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `aupll_cp_opamp_en` reader - Charge pump opamp enable"]
pub type AupllCpOpampEnR = crate::BitReader;
#[doc = "Field `aupll_cp_opamp_en` writer - Charge pump opamp enable"]
pub type AupllCpOpampEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Select charge pump bias"]
    #[inline(always)]
    pub fn aupll_sel_cp_bias(&self) -> AupllSelCpBiasR {
        AupllSelCpBiasR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 4:5 - Charge pump current (5uA)"]
    #[inline(always)]
    pub fn aupll_icp_5u(&self) -> AupllIcp5uR {
        AupllIcp5uR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Charge pump current (1uA)"]
    #[inline(always)]
    pub fn aupll_icp_1u(&self) -> AupllIcp1uR {
        AupllIcp1uR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8 - Integer/Fractional mode switch"]
    #[inline(always)]
    pub fn aupll_int_frac_sw(&self) -> AupllIntFracSwR {
        AupllIntFracSwR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Charge pump startup enable"]
    #[inline(always)]
    pub fn aupll_cp_startup_en(&self) -> AupllCpStartupEnR {
        AupllCpStartupEnR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Charge pump opamp enable"]
    #[inline(always)]
    pub fn aupll_cp_opamp_en(&self) -> AupllCpOpampEnR {
        AupllCpOpampEnR::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Select charge pump bias"]
    #[inline(always)]
    pub fn aupll_sel_cp_bias(&mut self) -> AupllSelCpBiasW<'_, AudioPllCfg2Spec> {
        AupllSelCpBiasW::new(self, 0)
    }
    #[doc = "Bits 4:5 - Charge pump current (5uA)"]
    #[inline(always)]
    pub fn aupll_icp_5u(&mut self) -> AupllIcp5uW<'_, AudioPllCfg2Spec> {
        AupllIcp5uW::new(self, 4)
    }
    #[doc = "Bits 6:7 - Charge pump current (1uA)"]
    #[inline(always)]
    pub fn aupll_icp_1u(&mut self) -> AupllIcp1uW<'_, AudioPllCfg2Spec> {
        AupllIcp1uW::new(self, 6)
    }
    #[doc = "Bit 8 - Integer/Fractional mode switch"]
    #[inline(always)]
    pub fn aupll_int_frac_sw(&mut self) -> AupllIntFracSwW<'_, AudioPllCfg2Spec> {
        AupllIntFracSwW::new(self, 8)
    }
    #[doc = "Bit 9 - Charge pump startup enable"]
    #[inline(always)]
    pub fn aupll_cp_startup_en(&mut self) -> AupllCpStartupEnW<'_, AudioPllCfg2Spec> {
        AupllCpStartupEnW::new(self, 9)
    }
    #[doc = "Bit 10 - Charge pump opamp enable"]
    #[inline(always)]
    pub fn aupll_cp_opamp_en(&mut self) -> AupllCpOpampEnW<'_, AudioPllCfg2Spec> {
        AupllCpOpampEnW::new(self, 10)
    }
}
#[doc = "Audio PLL Configuration 2 - Charge Pump and Control\n\nYou can [`read`](crate::Reg::read) this register and get [`audio_pll_cfg2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`audio_pll_cfg2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AudioPllCfg2Spec;
impl crate::RegisterSpec for AudioPllCfg2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`audio_pll_cfg2::R`](R) reader structure"]
impl crate::Readable for AudioPllCfg2Spec {}
#[doc = "`write(|w| ..)` method takes [`audio_pll_cfg2::W`](W) writer structure"]
impl crate::Writable for AudioPllCfg2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AUDIO_PLL_CFG2 to value 0x0741"]
impl crate::Resettable for AudioPllCfg2Spec {
    const RESET_VALUE: u32 = 0x0741;
}
