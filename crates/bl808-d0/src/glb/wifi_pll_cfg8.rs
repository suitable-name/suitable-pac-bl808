#[doc = "Register `WIFI_PLL_CFG8` reader"]
pub type R = crate::R<WifiPllCfg8Spec>;
#[doc = "Register `WIFI_PLL_CFG8` writer"]
pub type W = crate::W<WifiPllCfg8Spec>;
#[doc = "Field `wifipll_en_div2` reader - "]
pub type WifipllEnDiv2R = crate::BitReader;
#[doc = "Field `wifipll_en_div2` writer - "]
pub type WifipllEnDiv2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `wifipll_en_div4` reader - "]
pub type WifipllEnDiv4R = crate::BitReader;
#[doc = "Field `wifipll_en_div4` writer - "]
pub type WifipllEnDiv4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `wifipll_en_div5` reader - "]
pub type WifipllEnDiv5R = crate::BitReader;
#[doc = "Field `wifipll_en_div5` writer - "]
pub type WifipllEnDiv5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `wifipll_en_div6` reader - "]
pub type WifipllEnDiv6R = crate::BitReader;
#[doc = "Field `wifipll_en_div6` writer - "]
pub type WifipllEnDiv6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `wifipll_en_div8` reader - "]
pub type WifipllEnDiv8R = crate::BitReader;
#[doc = "Field `wifipll_en_div8` writer - "]
pub type WifipllEnDiv8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `wifipll_en_div10` reader - "]
pub type WifipllEnDiv10R = crate::BitReader;
#[doc = "Field `wifipll_en_div10` writer - "]
pub type WifipllEnDiv10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `wifipll_en_div12` reader - "]
pub type WifipllEnDiv12R = crate::BitReader;
#[doc = "Field `wifipll_en_div12` writer - "]
pub type WifipllEnDiv12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `wifipll_en_div20` reader - "]
pub type WifipllEnDiv20R = crate::BitReader;
#[doc = "Field `wifipll_en_div20` writer - "]
pub type WifipllEnDiv20W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `wifipll_en_div30` reader - "]
pub type WifipllEnDiv30R = crate::BitReader;
#[doc = "Field `wifipll_en_div30` writer - "]
pub type WifipllEnDiv30W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `wifipll_sel_div2_div4` reader - "]
pub type WifipllSelDiv2Div4R = crate::BitReader;
#[doc = "Field `wifipll_sel_div2_div4` writer - "]
pub type WifipllSelDiv2Div4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `en_wifipll_div30_bz_adc` reader - "]
pub type EnWifipllDiv30BzAdcR = crate::BitReader;
#[doc = "Field `en_wifipll_div30_bz_adc` writer - "]
pub type EnWifipllDiv30BzAdcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `wifipll_en_div2_hw` reader - "]
pub type WifipllEnDiv2HwR = crate::BitReader;
#[doc = "Field `wifipll_en_div2_hw` writer - "]
pub type WifipllEnDiv2HwW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `wifipll_en_ctrl_hw` reader - "]
pub type WifipllEnCtrlHwR = crate::BitReader;
#[doc = "Field `wifipll_en_ctrl_hw` writer - "]
pub type WifipllEnCtrlHwW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn wifipll_en_div2(&self) -> WifipllEnDiv2R {
        WifipllEnDiv2R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn wifipll_en_div4(&self) -> WifipllEnDiv4R {
        WifipllEnDiv4R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn wifipll_en_div5(&self) -> WifipllEnDiv5R {
        WifipllEnDiv5R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn wifipll_en_div6(&self) -> WifipllEnDiv6R {
        WifipllEnDiv6R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn wifipll_en_div8(&self) -> WifipllEnDiv8R {
        WifipllEnDiv8R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn wifipll_en_div10(&self) -> WifipllEnDiv10R {
        WifipllEnDiv10R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn wifipll_en_div12(&self) -> WifipllEnDiv12R {
        WifipllEnDiv12R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn wifipll_en_div20(&self) -> WifipllEnDiv20R {
        WifipllEnDiv20R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn wifipll_en_div30(&self) -> WifipllEnDiv30R {
        WifipllEnDiv30R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn wifipll_sel_div2_div4(&self) -> WifipllSelDiv2Div4R {
        WifipllSelDiv2Div4R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn en_wifipll_div30_bz_adc(&self) -> EnWifipllDiv30BzAdcR {
        EnWifipllDiv30BzAdcR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn wifipll_en_div2_hw(&self) -> WifipllEnDiv2HwR {
        WifipllEnDiv2HwR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn wifipll_en_ctrl_hw(&self) -> WifipllEnCtrlHwR {
        WifipllEnCtrlHwR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn wifipll_en_div2(&mut self) -> WifipllEnDiv2W<'_, WifiPllCfg8Spec> {
        WifipllEnDiv2W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn wifipll_en_div4(&mut self) -> WifipllEnDiv4W<'_, WifiPllCfg8Spec> {
        WifipllEnDiv4W::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn wifipll_en_div5(&mut self) -> WifipllEnDiv5W<'_, WifiPllCfg8Spec> {
        WifipllEnDiv5W::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn wifipll_en_div6(&mut self) -> WifipllEnDiv6W<'_, WifiPllCfg8Spec> {
        WifipllEnDiv6W::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn wifipll_en_div8(&mut self) -> WifipllEnDiv8W<'_, WifiPllCfg8Spec> {
        WifipllEnDiv8W::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn wifipll_en_div10(&mut self) -> WifipllEnDiv10W<'_, WifiPllCfg8Spec> {
        WifipllEnDiv10W::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn wifipll_en_div12(&mut self) -> WifipllEnDiv12W<'_, WifiPllCfg8Spec> {
        WifipllEnDiv12W::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn wifipll_en_div20(&mut self) -> WifipllEnDiv20W<'_, WifiPllCfg8Spec> {
        WifipllEnDiv20W::new(self, 7)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn wifipll_en_div30(&mut self) -> WifipllEnDiv30W<'_, WifiPllCfg8Spec> {
        WifipllEnDiv30W::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn wifipll_sel_div2_div4(&mut self) -> WifipllSelDiv2Div4W<'_, WifiPllCfg8Spec> {
        WifipllSelDiv2Div4W::new(self, 9)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn en_wifipll_div30_bz_adc(&mut self) -> EnWifipllDiv30BzAdcW<'_, WifiPllCfg8Spec> {
        EnWifipllDiv30BzAdcW::new(self, 10)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn wifipll_en_div2_hw(&mut self) -> WifipllEnDiv2HwW<'_, WifiPllCfg8Spec> {
        WifipllEnDiv2HwW::new(self, 12)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn wifipll_en_ctrl_hw(&mut self) -> WifipllEnCtrlHwW<'_, WifiPllCfg8Spec> {
        WifipllEnCtrlHwW::new(self, 31)
    }
}
#[doc = "WIFI PLL Configuration 8 - Output Divider Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`wifi_pll_cfg8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wifi_pll_cfg8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WifiPllCfg8Spec;
impl crate::RegisterSpec for WifiPllCfg8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wifi_pll_cfg8::R`](R) reader structure"]
impl crate::Readable for WifiPllCfg8Spec {}
#[doc = "`write(|w| ..)` method takes [`wifi_pll_cfg8::W`](W) writer structure"]
impl crate::Writable for WifiPllCfg8Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets WIFI_PLL_CFG8 to value 0"]
impl crate::Resettable for WifiPllCfg8Spec {}
