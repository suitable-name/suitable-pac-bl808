#[doc = "Register `WIFI_PLL_CFG3` reader"]
pub type R = crate::R<WifiPllCfg3Spec>;
#[doc = "Register `WIFI_PLL_CFG3` writer"]
pub type W = crate::W<WifiPllCfg3Spec>;
#[doc = "Field `wifipll_c4_en` reader - "]
pub type WifipllC4EnR = crate::BitReader;
#[doc = "Field `wifipll_c4_en` writer - "]
pub type WifipllC4EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `wifipll_r4` reader - "]
pub type WifipllR4R = crate::FieldReader;
#[doc = "Field `wifipll_r4` writer - "]
pub type WifipllR4W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `wifipll_r4_short` reader - "]
pub type WifipllR4ShortR = crate::BitReader;
#[doc = "Field `wifipll_r4_short` writer - "]
pub type WifipllR4ShortW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `wifipll_c3` reader - "]
pub type WifipllC3R = crate::FieldReader;
#[doc = "Field `wifipll_c3` writer - "]
pub type WifipllC3W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `wifipll_cz` reader - "]
pub type WifipllCzR = crate::FieldReader;
#[doc = "Field `wifipll_cz` writer - "]
pub type WifipllCzW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `wifipll_rz` reader - "]
pub type WifipllRzR = crate::FieldReader;
#[doc = "Field `wifipll_rz` writer - "]
pub type WifipllRzW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn wifipll_c4_en(&self) -> WifipllC4EnR {
        WifipllC4EnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn wifipll_r4(&self) -> WifipllR4R {
        WifipllR4R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn wifipll_r4_short(&self) -> WifipllR4ShortR {
        WifipllR4ShortR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn wifipll_c3(&self) -> WifipllC3R {
        WifipllC3R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn wifipll_cz(&self) -> WifipllCzR {
        WifipllCzR::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:18"]
    #[inline(always)]
    pub fn wifipll_rz(&self) -> WifipllRzR {
        WifipllRzR::new(((self.bits >> 16) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn wifipll_c4_en(&mut self) -> WifipllC4EnW<'_, WifiPllCfg3Spec> {
        WifipllC4EnW::new(self, 0)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn wifipll_r4(&mut self) -> WifipllR4W<'_, WifiPllCfg3Spec> {
        WifipllR4W::new(self, 4)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn wifipll_r4_short(&mut self) -> WifipllR4ShortW<'_, WifiPllCfg3Spec> {
        WifipllR4ShortW::new(self, 8)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn wifipll_c3(&mut self) -> WifipllC3W<'_, WifiPllCfg3Spec> {
        WifipllC3W::new(self, 12)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn wifipll_cz(&mut self) -> WifipllCzW<'_, WifiPllCfg3Spec> {
        WifipllCzW::new(self, 14)
    }
    #[doc = "Bits 16:18"]
    #[inline(always)]
    pub fn wifipll_rz(&mut self) -> WifipllRzW<'_, WifiPllCfg3Spec> {
        WifipllRzW::new(self, 16)
    }
}
#[doc = "WIFI PLL Configuration 3 - Loop Filter\n\nYou can [`read`](crate::Reg::read) this register and get [`wifi_pll_cfg3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wifi_pll_cfg3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WifiPllCfg3Spec;
impl crate::RegisterSpec for WifiPllCfg3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wifi_pll_cfg3::R`](R) reader structure"]
impl crate::Readable for WifiPllCfg3Spec {}
#[doc = "`write(|w| ..)` method takes [`wifi_pll_cfg3::W`](W) writer structure"]
impl crate::Writable for WifiPllCfg3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets WIFI_PLL_CFG3 to value 0"]
impl crate::Resettable for WifiPllCfg3Spec {}
