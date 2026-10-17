#[doc = "Register `WIFI_PLL_CFG2` reader"]
pub type R = crate::R<WifiPllCfg2Spec>;
#[doc = "Register `WIFI_PLL_CFG2` writer"]
pub type W = crate::W<WifiPllCfg2Spec>;
#[doc = "Field `wifipll_sel_cp_bias` reader - "]
pub type WifipllSelCpBiasR = crate::BitReader;
#[doc = "Field `wifipll_sel_cp_bias` writer - "]
pub type WifipllSelCpBiasW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `wifipll_icp_5u` reader - "]
pub type WifipllIcp5uR = crate::FieldReader;
#[doc = "Field `wifipll_icp_5u` writer - "]
pub type WifipllIcp5uW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `wifipll_icp_1u` reader - "]
pub type WifipllIcp1uR = crate::FieldReader;
#[doc = "Field `wifipll_icp_1u` writer - "]
pub type WifipllIcp1uW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `wifipll_int_frac_sw` reader - "]
pub type WifipllIntFracSwR = crate::BitReader;
#[doc = "Field `wifipll_int_frac_sw` writer - "]
pub type WifipllIntFracSwW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `wifipll_cp_startup_en` reader - "]
pub type WifipllCpStartupEnR = crate::BitReader;
#[doc = "Field `wifipll_cp_startup_en` writer - "]
pub type WifipllCpStartupEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `wifipll_cp_opamp_en` reader - "]
pub type WifipllCpOpampEnR = crate::BitReader;
#[doc = "Field `wifipll_cp_opamp_en` writer - "]
pub type WifipllCpOpampEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn wifipll_sel_cp_bias(&self) -> WifipllSelCpBiasR {
        WifipllSelCpBiasR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn wifipll_icp_5u(&self) -> WifipllIcp5uR {
        WifipllIcp5uR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn wifipll_icp_1u(&self) -> WifipllIcp1uR {
        WifipllIcp1uR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn wifipll_int_frac_sw(&self) -> WifipllIntFracSwR {
        WifipllIntFracSwR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn wifipll_cp_startup_en(&self) -> WifipllCpStartupEnR {
        WifipllCpStartupEnR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn wifipll_cp_opamp_en(&self) -> WifipllCpOpampEnR {
        WifipllCpOpampEnR::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn wifipll_sel_cp_bias(&mut self) -> WifipllSelCpBiasW<'_, WifiPllCfg2Spec> {
        WifipllSelCpBiasW::new(self, 0)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn wifipll_icp_5u(&mut self) -> WifipllIcp5uW<'_, WifiPllCfg2Spec> {
        WifipllIcp5uW::new(self, 4)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn wifipll_icp_1u(&mut self) -> WifipllIcp1uW<'_, WifiPllCfg2Spec> {
        WifipllIcp1uW::new(self, 6)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn wifipll_int_frac_sw(&mut self) -> WifipllIntFracSwW<'_, WifiPllCfg2Spec> {
        WifipllIntFracSwW::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn wifipll_cp_startup_en(&mut self) -> WifipllCpStartupEnW<'_, WifiPllCfg2Spec> {
        WifipllCpStartupEnW::new(self, 9)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn wifipll_cp_opamp_en(&mut self) -> WifipllCpOpampEnW<'_, WifiPllCfg2Spec> {
        WifipllCpOpampEnW::new(self, 10)
    }
}
#[doc = "WIFI PLL Configuration 2 - Charge Pump\n\nYou can [`read`](crate::Reg::read) this register and get [`wifi_pll_cfg2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wifi_pll_cfg2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WifiPllCfg2Spec;
impl crate::RegisterSpec for WifiPllCfg2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wifi_pll_cfg2::R`](R) reader structure"]
impl crate::Readable for WifiPllCfg2Spec {}
#[doc = "`write(|w| ..)` method takes [`wifi_pll_cfg2::W`](W) writer structure"]
impl crate::Writable for WifiPllCfg2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets WIFI_PLL_CFG2 to value 0"]
impl crate::Resettable for WifiPllCfg2Spec {}
