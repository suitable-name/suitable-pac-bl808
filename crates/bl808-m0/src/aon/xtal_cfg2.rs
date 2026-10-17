#[doc = "Register `XTAL_CFG2` reader"]
pub type R = crate::R<XtalCfg2Spec>;
#[doc = "Register `XTAL_CFG2` writer"]
pub type W = crate::W<XtalCfg2Spec>;
#[doc = "Field `wifi_xtal_ldo33_bypass_aon` reader - "]
pub type WifiXtalLdo33BypassAonR = crate::BitReader;
#[doc = "Field `wifi_xtal_ldo33_bypass_aon` writer - "]
pub type WifiXtalLdo33BypassAonW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `wifi_xtal_ldo33_sel_aon` reader - "]
pub type WifiXtalLdo33SelAonR = crate::FieldReader;
#[doc = "Field `wifi_xtal_ldo33_sel_aon` writer - "]
pub type WifiXtalLdo33SelAonW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `wifi_xtal_ldo18_sel_aon` reader - "]
pub type WifiXtalLdo18SelAonR = crate::FieldReader;
#[doc = "Field `wifi_xtal_ldo18_sel_aon` writer - "]
pub type WifiXtalLdo18SelAonW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `wifi_xtal_ldo33_pu_aon` reader - "]
pub type WifiXtalLdo33PuAonR = crate::BitReader;
#[doc = "Field `wifi_xtal_ldo33_pu_aon` writer - "]
pub type WifiXtalLdo33PuAonW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `wifi_xtal_ldo18_pu_aon` reader - "]
pub type WifiXtalLdo18PuAonR = crate::BitReader;
#[doc = "Field `wifi_xtal_ldo18_pu_aon` writer - "]
pub type WifiXtalLdo18PuAonW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `wifi_xtal_reserve` reader - "]
pub type WifiXtalReserveR = crate::FieldReader;
#[doc = "Field `wifi_xtal_reserve` writer - "]
pub type WifiXtalReserveW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `wifi_xtal_ldo18_short_filter_aon` reader - "]
pub type WifiXtalLdo18ShortFilterAonR = crate::BitReader;
#[doc = "Field `wifi_xtal_ldo18_short_filter_aon` writer - "]
pub type WifiXtalLdo18ShortFilterAonW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `xtal_buf_drv_aon` reader - "]
pub type XtalBufDrvAonR = crate::FieldReader;
#[doc = "Field `xtal_buf_drv_aon` writer - "]
pub type XtalBufDrvAonW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn wifi_xtal_ldo33_bypass_aon(&self) -> WifiXtalLdo33BypassAonR {
        WifiXtalLdo33BypassAonR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3"]
    #[inline(always)]
    pub fn wifi_xtal_ldo33_sel_aon(&self) -> WifiXtalLdo33SelAonR {
        WifiXtalLdo33SelAonR::new(((self.bits >> 1) & 7) as u8)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn wifi_xtal_ldo18_sel_aon(&self) -> WifiXtalLdo18SelAonR {
        WifiXtalLdo18SelAonR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn wifi_xtal_ldo33_pu_aon(&self) -> WifiXtalLdo33PuAonR {
        WifiXtalLdo33PuAonR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn wifi_xtal_ldo18_pu_aon(&self) -> WifiXtalLdo18PuAonR {
        WifiXtalLdo18PuAonR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 10:13"]
    #[inline(always)]
    pub fn wifi_xtal_reserve(&self) -> WifiXtalReserveR {
        WifiXtalReserveR::new(((self.bits >> 10) & 0x0f) as u8)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn wifi_xtal_ldo18_short_filter_aon(&self) -> WifiXtalLdo18ShortFilterAonR {
        WifiXtalLdo18ShortFilterAonR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 30:31"]
    #[inline(always)]
    pub fn xtal_buf_drv_aon(&self) -> XtalBufDrvAonR {
        XtalBufDrvAonR::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn wifi_xtal_ldo33_bypass_aon(&mut self) -> WifiXtalLdo33BypassAonW<'_, XtalCfg2Spec> {
        WifiXtalLdo33BypassAonW::new(self, 0)
    }
    #[doc = "Bits 1:3"]
    #[inline(always)]
    pub fn wifi_xtal_ldo33_sel_aon(&mut self) -> WifiXtalLdo33SelAonW<'_, XtalCfg2Spec> {
        WifiXtalLdo33SelAonW::new(self, 1)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn wifi_xtal_ldo18_sel_aon(&mut self) -> WifiXtalLdo18SelAonW<'_, XtalCfg2Spec> {
        WifiXtalLdo18SelAonW::new(self, 4)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn wifi_xtal_ldo33_pu_aon(&mut self) -> WifiXtalLdo33PuAonW<'_, XtalCfg2Spec> {
        WifiXtalLdo33PuAonW::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn wifi_xtal_ldo18_pu_aon(&mut self) -> WifiXtalLdo18PuAonW<'_, XtalCfg2Spec> {
        WifiXtalLdo18PuAonW::new(self, 7)
    }
    #[doc = "Bits 10:13"]
    #[inline(always)]
    pub fn wifi_xtal_reserve(&mut self) -> WifiXtalReserveW<'_, XtalCfg2Spec> {
        WifiXtalReserveW::new(self, 10)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn wifi_xtal_ldo18_short_filter_aon(
        &mut self,
    ) -> WifiXtalLdo18ShortFilterAonW<'_, XtalCfg2Spec> {
        WifiXtalLdo18ShortFilterAonW::new(self, 16)
    }
    #[doc = "Bits 30:31"]
    #[inline(always)]
    pub fn xtal_buf_drv_aon(&mut self) -> XtalBufDrvAonW<'_, XtalCfg2Spec> {
        XtalBufDrvAonW::new(self, 30)
    }
}
#[doc = "Crystal Oscillator Configuration 2\n\nYou can [`read`](crate::Reg::read) this register and get [`xtal_cfg2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xtal_cfg2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct XtalCfg2Spec;
impl crate::RegisterSpec for XtalCfg2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`xtal_cfg2::R`](R) reader structure"]
impl crate::Readable for XtalCfg2Spec {}
#[doc = "`write(|w| ..)` method takes [`xtal_cfg2::W`](W) writer structure"]
impl crate::Writable for XtalCfg2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets XTAL_CFG2 to value 0"]
impl crate::Resettable for XtalCfg2Spec {}
