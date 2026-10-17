#[doc = "Register `WIFI_PLL_CFG13` reader"]
pub type R = crate::R<WifiPllCfg13Spec>;
#[doc = "Register `WIFI_PLL_CFG13` writer"]
pub type W = crate::W<WifiPllCfg13Spec>;
#[doc = "Field `wifipll_resv` reader - "]
pub type WifipllResvR = crate::FieldReader<u16>;
#[doc = "Field `wifipll_resv` writer - "]
pub type WifipllResvW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `usbpll_dl_ctrl` reader - "]
pub type UsbpllDlCtrlR = crate::BitReader;
#[doc = "Field `usbpll_dl_ctrl` writer - "]
pub type UsbpllDlCtrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `wifipll_dl_ctrl_30_bz_adc` reader - "]
pub type WifipllDlCtrl30BzAdcR = crate::BitReader;
#[doc = "Field `wifipll_dl_ctrl_30_bz_adc` writer - "]
pub type WifipllDlCtrl30BzAdcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `wifipll_dl_ctrl_30` reader - "]
pub type WifipllDlCtrl30R = crate::BitReader;
#[doc = "Field `wifipll_dl_ctrl_30` writer - "]
pub type WifipllDlCtrl30W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `wifipll_dl_ctrl_20` reader - "]
pub type WifipllDlCtrl20R = crate::BitReader;
#[doc = "Field `wifipll_dl_ctrl_20` writer - "]
pub type WifipllDlCtrl20W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `wifipll_dl_ctrl_12` reader - "]
pub type WifipllDlCtrl12R = crate::BitReader;
#[doc = "Field `wifipll_dl_ctrl_12` writer - "]
pub type WifipllDlCtrl12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `wifipll_dl_ctrl_10` reader - "]
pub type WifipllDlCtrl10R = crate::BitReader;
#[doc = "Field `wifipll_dl_ctrl_10` writer - "]
pub type WifipllDlCtrl10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `wifipll_dl_ctrl_8` reader - "]
pub type WifipllDlCtrl8R = crate::BitReader;
#[doc = "Field `wifipll_dl_ctrl_8` writer - "]
pub type WifipllDlCtrl8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `wifipll_dl_ctrl_6` reader - "]
pub type WifipllDlCtrl6R = crate::BitReader;
#[doc = "Field `wifipll_dl_ctrl_6` writer - "]
pub type WifipllDlCtrl6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `wifipll_dl_ctrl_5` reader - "]
pub type WifipllDlCtrl5R = crate::BitReader;
#[doc = "Field `wifipll_dl_ctrl_5` writer - "]
pub type WifipllDlCtrl5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `wifipll_dl_ctrl_4` reader - "]
pub type WifipllDlCtrl4R = crate::BitReader;
#[doc = "Field `wifipll_dl_ctrl_4` writer - "]
pub type WifipllDlCtrl4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `wifipll_dl_ctrl_2` reader - "]
pub type WifipllDlCtrl2R = crate::BitReader;
#[doc = "Field `wifipll_dl_ctrl_2` writer - "]
pub type WifipllDlCtrl2W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn wifipll_resv(&self) -> WifipllResvR {
        WifipllResvR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn usbpll_dl_ctrl(&self) -> UsbpllDlCtrlR {
        UsbpllDlCtrlR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn wifipll_dl_ctrl_30_bz_adc(&self) -> WifipllDlCtrl30BzAdcR {
        WifipllDlCtrl30BzAdcR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn wifipll_dl_ctrl_30(&self) -> WifipllDlCtrl30R {
        WifipllDlCtrl30R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn wifipll_dl_ctrl_20(&self) -> WifipllDlCtrl20R {
        WifipllDlCtrl20R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn wifipll_dl_ctrl_12(&self) -> WifipllDlCtrl12R {
        WifipllDlCtrl12R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn wifipll_dl_ctrl_10(&self) -> WifipllDlCtrl10R {
        WifipllDlCtrl10R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn wifipll_dl_ctrl_8(&self) -> WifipllDlCtrl8R {
        WifipllDlCtrl8R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn wifipll_dl_ctrl_6(&self) -> WifipllDlCtrl6R {
        WifipllDlCtrl6R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn wifipll_dl_ctrl_5(&self) -> WifipllDlCtrl5R {
        WifipllDlCtrl5R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn wifipll_dl_ctrl_4(&self) -> WifipllDlCtrl4R {
        WifipllDlCtrl4R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn wifipll_dl_ctrl_2(&self) -> WifipllDlCtrl2R {
        WifipllDlCtrl2R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn wifipll_resv(&mut self) -> WifipllResvW<'_, WifiPllCfg13Spec> {
        WifipllResvW::new(self, 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn usbpll_dl_ctrl(&mut self) -> UsbpllDlCtrlW<'_, WifiPllCfg13Spec> {
        UsbpllDlCtrlW::new(self, 21)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn wifipll_dl_ctrl_30_bz_adc(&mut self) -> WifipllDlCtrl30BzAdcW<'_, WifiPllCfg13Spec> {
        WifipllDlCtrl30BzAdcW::new(self, 22)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn wifipll_dl_ctrl_30(&mut self) -> WifipllDlCtrl30W<'_, WifiPllCfg13Spec> {
        WifipllDlCtrl30W::new(self, 23)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn wifipll_dl_ctrl_20(&mut self) -> WifipllDlCtrl20W<'_, WifiPllCfg13Spec> {
        WifipllDlCtrl20W::new(self, 24)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn wifipll_dl_ctrl_12(&mut self) -> WifipllDlCtrl12W<'_, WifiPllCfg13Spec> {
        WifipllDlCtrl12W::new(self, 25)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn wifipll_dl_ctrl_10(&mut self) -> WifipllDlCtrl10W<'_, WifiPllCfg13Spec> {
        WifipllDlCtrl10W::new(self, 26)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn wifipll_dl_ctrl_8(&mut self) -> WifipllDlCtrl8W<'_, WifiPllCfg13Spec> {
        WifipllDlCtrl8W::new(self, 27)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn wifipll_dl_ctrl_6(&mut self) -> WifipllDlCtrl6W<'_, WifiPllCfg13Spec> {
        WifipllDlCtrl6W::new(self, 28)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn wifipll_dl_ctrl_5(&mut self) -> WifipllDlCtrl5W<'_, WifiPllCfg13Spec> {
        WifipllDlCtrl5W::new(self, 29)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn wifipll_dl_ctrl_4(&mut self) -> WifipllDlCtrl4W<'_, WifiPllCfg13Spec> {
        WifipllDlCtrl4W::new(self, 30)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn wifipll_dl_ctrl_2(&mut self) -> WifipllDlCtrl2W<'_, WifiPllCfg13Spec> {
        WifipllDlCtrl2W::new(self, 31)
    }
}
#[doc = "WIFI PLL Configuration 13 - Reserved and DL Control\n\nYou can [`read`](crate::Reg::read) this register and get [`wifi_pll_cfg13::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wifi_pll_cfg13::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WifiPllCfg13Spec;
impl crate::RegisterSpec for WifiPllCfg13Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wifi_pll_cfg13::R`](R) reader structure"]
impl crate::Readable for WifiPllCfg13Spec {}
#[doc = "`write(|w| ..)` method takes [`wifi_pll_cfg13::W`](W) writer structure"]
impl crate::Writable for WifiPllCfg13Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets WIFI_PLL_CFG13 to value 0"]
impl crate::Resettable for WifiPllCfg13Spec {}
