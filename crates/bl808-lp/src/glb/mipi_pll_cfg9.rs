#[doc = "Register `MIPI_PLL_CFG9` reader"]
pub type R = crate::R<MipiPllCfg9Spec>;
#[doc = "Register `MIPI_PLL_CFG9` writer"]
pub type W = crate::W<MipiPllCfg9Spec>;
#[doc = "Field `mipipll_ssc_en` reader - "]
pub type MipipllSscEnR = crate::BitReader;
#[doc = "Field `mipipll_ssc_en` writer - "]
pub type MipipllSscEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `mipipll_ssc_cnt` reader - "]
pub type MipipllSscCntR = crate::FieldReader;
#[doc = "Field `mipipll_ssc_cnt` writer - "]
pub type MipipllSscCntW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `mipipll_ssc_gain` reader - "]
pub type MipipllSscGainR = crate::FieldReader;
#[doc = "Field `mipipll_ssc_gain` writer - "]
pub type MipipllSscGainW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `mipipll_ssc_start_gate_en` reader - "]
pub type MipipllSscStartGateEnR = crate::BitReader;
#[doc = "Field `mipipll_ssc_start_gate_en` writer - "]
pub type MipipllSscStartGateEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn mipipll_ssc_en(&self) -> MipipllSscEnR {
        MipipllSscEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 4:11"]
    #[inline(always)]
    pub fn mipipll_ssc_cnt(&self) -> MipipllSscCntR {
        MipipllSscCntR::new(((self.bits >> 4) & 0xff) as u8)
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    pub fn mipipll_ssc_gain(&self) -> MipipllSscGainR {
        MipipllSscGainR::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn mipipll_ssc_start_gate_en(&self) -> MipipllSscStartGateEnR {
        MipipllSscStartGateEnR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn mipipll_ssc_en(&mut self) -> MipipllSscEnW<'_, MipiPllCfg9Spec> {
        MipipllSscEnW::new(self, 0)
    }
    #[doc = "Bits 4:11"]
    #[inline(always)]
    pub fn mipipll_ssc_cnt(&mut self) -> MipipllSscCntW<'_, MipiPllCfg9Spec> {
        MipipllSscCntW::new(self, 4)
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    pub fn mipipll_ssc_gain(&mut self) -> MipipllSscGainW<'_, MipiPllCfg9Spec> {
        MipipllSscGainW::new(self, 12)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn mipipll_ssc_start_gate_en(&mut self) -> MipipllSscStartGateEnW<'_, MipiPllCfg9Spec> {
        MipipllSscStartGateEnW::new(self, 16)
    }
}
#[doc = "MIPI PLL Configuration 9 - SSC\n\nYou can [`read`](crate::Reg::read) this register and get [`mipi_pll_cfg9::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mipi_pll_cfg9::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MipiPllCfg9Spec;
impl crate::RegisterSpec for MipiPllCfg9Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mipi_pll_cfg9::R`](R) reader structure"]
impl crate::Readable for MipiPllCfg9Spec {}
#[doc = "`write(|w| ..)` method takes [`mipi_pll_cfg9::W`](W) writer structure"]
impl crate::Writable for MipiPllCfg9Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MIPI_PLL_CFG9 to value 0"]
impl crate::Resettable for MipiPllCfg9Spec {}
