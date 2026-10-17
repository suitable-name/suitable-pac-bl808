#[doc = "Register `XTAL_CFG3` reader"]
pub type R = crate::R<XtalCfg3Spec>;
#[doc = "Register `XTAL_CFG3` writer"]
pub type W = crate::W<XtalCfg3Spec>;
#[doc = "Field `wifi_xtal_clk_inv_en_aon` reader - "]
pub type WifiXtalClkInvEnAonR = crate::BitReader;
#[doc = "Field `wifi_xtal_clk_inv_en_aon` writer - "]
pub type WifiXtalClkInvEnAonW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `wifi_xtal_cml_en_aon` reader - "]
pub type WifiXtalCmlEnAonR = crate::BitReader;
#[doc = "Field `wifi_xtal_cml_en_aon` writer - "]
pub type WifiXtalCmlEnAonW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `wifi_xtal_cml_r_sel_aon` reader - "]
pub type WifiXtalCmlRSelAonR = crate::FieldReader;
#[doc = "Field `wifi_xtal_cml_r_sel_aon` writer - "]
pub type WifiXtalCmlRSelAonW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `wifi_xtal_clk_en_aon` reader - "]
pub type WifiXtalClkEnAonR = crate::BitReader;
#[doc = "Field `wifi_xtal_clk_en_aon` writer - "]
pub type WifiXtalClkEnAonW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `wifi_xtal_buf_drv_aon` reader - "]
pub type WifiXtalBufDrvAonR = crate::FieldReader;
#[doc = "Field `wifi_xtal_buf_drv_aon` writer - "]
pub type WifiXtalBufDrvAonW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn wifi_xtal_clk_inv_en_aon(&self) -> WifiXtalClkInvEnAonR {
        WifiXtalClkInvEnAonR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn wifi_xtal_cml_en_aon(&self) -> WifiXtalCmlEnAonR {
        WifiXtalCmlEnAonR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:18"]
    #[inline(always)]
    pub fn wifi_xtal_cml_r_sel_aon(&self) -> WifiXtalCmlRSelAonR {
        WifiXtalCmlRSelAonR::new(((self.bits >> 17) & 3) as u8)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn wifi_xtal_clk_en_aon(&self) -> WifiXtalClkEnAonR {
        WifiXtalClkEnAonR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 30:31"]
    #[inline(always)]
    pub fn wifi_xtal_buf_drv_aon(&self) -> WifiXtalBufDrvAonR {
        WifiXtalBufDrvAonR::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn wifi_xtal_clk_inv_en_aon(&mut self) -> WifiXtalClkInvEnAonW<'_, XtalCfg3Spec> {
        WifiXtalClkInvEnAonW::new(self, 12)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn wifi_xtal_cml_en_aon(&mut self) -> WifiXtalCmlEnAonW<'_, XtalCfg3Spec> {
        WifiXtalCmlEnAonW::new(self, 16)
    }
    #[doc = "Bits 17:18"]
    #[inline(always)]
    pub fn wifi_xtal_cml_r_sel_aon(&mut self) -> WifiXtalCmlRSelAonW<'_, XtalCfg3Spec> {
        WifiXtalCmlRSelAonW::new(self, 17)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn wifi_xtal_clk_en_aon(&mut self) -> WifiXtalClkEnAonW<'_, XtalCfg3Spec> {
        WifiXtalClkEnAonW::new(self, 20)
    }
    #[doc = "Bits 30:31"]
    #[inline(always)]
    pub fn wifi_xtal_buf_drv_aon(&mut self) -> WifiXtalBufDrvAonW<'_, XtalCfg3Spec> {
        WifiXtalBufDrvAonW::new(self, 30)
    }
}
#[doc = "Crystal Oscillator Configuration 3\n\nYou can [`read`](crate::Reg::read) this register and get [`xtal_cfg3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xtal_cfg3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct XtalCfg3Spec;
impl crate::RegisterSpec for XtalCfg3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`xtal_cfg3::R`](R) reader structure"]
impl crate::Readable for XtalCfg3Spec {}
#[doc = "`write(|w| ..)` method takes [`xtal_cfg3::W`](W) writer structure"]
impl crate::Writable for XtalCfg3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets XTAL_CFG3 to value 0"]
impl crate::Resettable for XtalCfg3Spec {}
