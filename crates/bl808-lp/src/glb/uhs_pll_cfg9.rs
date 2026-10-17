#[doc = "Register `UHS_PLL_CFG9` reader"]
pub type R = crate::R<UhsPllCfg9Spec>;
#[doc = "Register `UHS_PLL_CFG9` writer"]
pub type W = crate::W<UhsPllCfg9Spec>;
#[doc = "Field `uhspll_ssc_en` reader - "]
pub type UhspllSscEnR = crate::BitReader;
#[doc = "Field `uhspll_ssc_en` writer - "]
pub type UhspllSscEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `uhspll_ssc_cnt` reader - "]
pub type UhspllSscCntR = crate::FieldReader;
#[doc = "Field `uhspll_ssc_cnt` writer - "]
pub type UhspllSscCntW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `uhspll_ssc_gain` reader - "]
pub type UhspllSscGainR = crate::FieldReader;
#[doc = "Field `uhspll_ssc_gain` writer - "]
pub type UhspllSscGainW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `uhspll_ssc_start_gate_en` reader - "]
pub type UhspllSscStartGateEnR = crate::BitReader;
#[doc = "Field `uhspll_ssc_start_gate_en` writer - "]
pub type UhspllSscStartGateEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn uhspll_ssc_en(&self) -> UhspllSscEnR {
        UhspllSscEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 4:11"]
    #[inline(always)]
    pub fn uhspll_ssc_cnt(&self) -> UhspllSscCntR {
        UhspllSscCntR::new(((self.bits >> 4) & 0xff) as u8)
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    pub fn uhspll_ssc_gain(&self) -> UhspllSscGainR {
        UhspllSscGainR::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn uhspll_ssc_start_gate_en(&self) -> UhspllSscStartGateEnR {
        UhspllSscStartGateEnR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn uhspll_ssc_en(&mut self) -> UhspllSscEnW<'_, UhsPllCfg9Spec> {
        UhspllSscEnW::new(self, 0)
    }
    #[doc = "Bits 4:11"]
    #[inline(always)]
    pub fn uhspll_ssc_cnt(&mut self) -> UhspllSscCntW<'_, UhsPllCfg9Spec> {
        UhspllSscCntW::new(self, 4)
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    pub fn uhspll_ssc_gain(&mut self) -> UhspllSscGainW<'_, UhsPllCfg9Spec> {
        UhspllSscGainW::new(self, 12)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn uhspll_ssc_start_gate_en(&mut self) -> UhspllSscStartGateEnW<'_, UhsPllCfg9Spec> {
        UhspllSscStartGateEnW::new(self, 16)
    }
}
#[doc = "UHS PLL Configuration 9 - SSC\n\nYou can [`read`](crate::Reg::read) this register and get [`uhs_pll_cfg9::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uhs_pll_cfg9::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UhsPllCfg9Spec;
impl crate::RegisterSpec for UhsPllCfg9Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uhs_pll_cfg9::R`](R) reader structure"]
impl crate::Readable for UhsPllCfg9Spec {}
#[doc = "`write(|w| ..)` method takes [`uhs_pll_cfg9::W`](W) writer structure"]
impl crate::Writable for UhsPllCfg9Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UHS_PLL_CFG9 to value 0"]
impl crate::Resettable for UhsPllCfg9Spec {}
