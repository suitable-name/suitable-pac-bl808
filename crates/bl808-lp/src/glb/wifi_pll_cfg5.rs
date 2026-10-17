#[doc = "Register `WIFI_PLL_CFG5` reader"]
pub type R = crate::R<WifiPllCfg5Spec>;
#[doc = "Register `WIFI_PLL_CFG5` writer"]
pub type W = crate::W<WifiPllCfg5Spec>;
#[doc = "Field `wifipll_vco_speed` reader - "]
pub type WifipllVcoSpeedR = crate::FieldReader;
#[doc = "Field `wifipll_vco_speed` writer - "]
pub type WifipllVcoSpeedW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `wifipll_vco_div1_en` reader - "]
pub type WifipllVcoDiv1EnR = crate::BitReader;
#[doc = "Field `wifipll_vco_div1_en` writer - "]
pub type WifipllVcoDiv1EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `wifipll_vco_div2_en` reader - "]
pub type WifipllVcoDiv2EnR = crate::BitReader;
#[doc = "Field `wifipll_vco_div2_en` writer - "]
pub type WifipllVcoDiv2EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `wifipll_vco_div3_en` reader - "]
pub type WifipllVcoDiv3EnR = crate::BitReader;
#[doc = "Field `wifipll_vco_div3_en` writer - "]
pub type WifipllVcoDiv3EnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn wifipll_vco_speed(&self) -> WifipllVcoSpeedR {
        WifipllVcoSpeedR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn wifipll_vco_div1_en(&self) -> WifipllVcoDiv1EnR {
        WifipllVcoDiv1EnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn wifipll_vco_div2_en(&self) -> WifipllVcoDiv2EnR {
        WifipllVcoDiv2EnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn wifipll_vco_div3_en(&self) -> WifipllVcoDiv3EnR {
        WifipllVcoDiv3EnR::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn wifipll_vco_speed(&mut self) -> WifipllVcoSpeedW<'_, WifiPllCfg5Spec> {
        WifipllVcoSpeedW::new(self, 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn wifipll_vco_div1_en(&mut self) -> WifipllVcoDiv1EnW<'_, WifiPllCfg5Spec> {
        WifipllVcoDiv1EnW::new(self, 4)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn wifipll_vco_div2_en(&mut self) -> WifipllVcoDiv2EnW<'_, WifiPllCfg5Spec> {
        WifipllVcoDiv2EnW::new(self, 8)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn wifipll_vco_div3_en(&mut self) -> WifipllVcoDiv3EnW<'_, WifiPllCfg5Spec> {
        WifipllVcoDiv3EnW::new(self, 12)
    }
}
#[doc = "WIFI PLL Configuration 5 - VCO\n\nYou can [`read`](crate::Reg::read) this register and get [`wifi_pll_cfg5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wifi_pll_cfg5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WifiPllCfg5Spec;
impl crate::RegisterSpec for WifiPllCfg5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wifi_pll_cfg5::R`](R) reader structure"]
impl crate::Readable for WifiPllCfg5Spec {}
#[doc = "`write(|w| ..)` method takes [`wifi_pll_cfg5::W`](W) writer structure"]
impl crate::Writable for WifiPllCfg5Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets WIFI_PLL_CFG5 to value 0"]
impl crate::Resettable for WifiPllCfg5Spec {}
