#[doc = "Register `XTAL_CFG` reader"]
pub type R = crate::R<XtalCfgSpec>;
#[doc = "Register `XTAL_CFG` writer"]
pub type W = crate::W<XtalCfgSpec>;
#[doc = "Field `xtal_bk_aon` reader - "]
pub type XtalBkAonR = crate::FieldReader;
#[doc = "Field `xtal_bk_aon` writer - "]
pub type XtalBkAonW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `xtal_capcode_extra_aon` reader - "]
pub type XtalCapcodeExtraAonR = crate::BitReader;
#[doc = "Field `xtal_capcode_extra_aon` writer - "]
pub type XtalCapcodeExtraAonW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `xtal_ext_sel_aon` reader - "]
pub type XtalExtSelAonR = crate::BitReader;
#[doc = "Field `xtal_ext_sel_aon` writer - "]
pub type XtalExtSelAonW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `xtal_buf_en_aon` reader - "]
pub type XtalBufEnAonR = crate::FieldReader;
#[doc = "Field `xtal_buf_en_aon` writer - "]
pub type XtalBufEnAonW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `xtal_buf_hp_aon` reader - "]
pub type XtalBufHpAonR = crate::FieldReader;
#[doc = "Field `xtal_buf_hp_aon` writer - "]
pub type XtalBufHpAonW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `xtal_fast_startup_aon` reader - "]
pub type XtalFastStartupAonR = crate::BitReader;
#[doc = "Field `xtal_fast_startup_aon` writer - "]
pub type XtalFastStartupAonW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `xtal_sleep_aon` reader - "]
pub type XtalSleepAonR = crate::BitReader;
#[doc = "Field `xtal_sleep_aon` writer - "]
pub type XtalSleepAonW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `xtal_amp_ctrl_aon` reader - "]
pub type XtalAmpCtrlAonR = crate::FieldReader;
#[doc = "Field `xtal_amp_ctrl_aon` writer - "]
pub type XtalAmpCtrlAonW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `xtal_capcode_out_aon` reader - "]
pub type XtalCapcodeOutAonR = crate::FieldReader;
#[doc = "Field `xtal_capcode_out_aon` writer - "]
pub type XtalCapcodeOutAonW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `xtal_capcode_in_aon` reader - "]
pub type XtalCapcodeInAonR = crate::FieldReader;
#[doc = "Field `xtal_capcode_in_aon` writer - "]
pub type XtalCapcodeInAonW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `xtal_gm_boost_aon` reader - "]
pub type XtalGmBoostAonR = crate::FieldReader;
#[doc = "Field `xtal_gm_boost_aon` writer - "]
pub type XtalGmBoostAonW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `xtal_rdy_sel_aon` reader - "]
pub type XtalRdySelAonR = crate::FieldReader;
#[doc = "Field `xtal_rdy_sel_aon` writer - "]
pub type XtalRdySelAonW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn xtal_bk_aon(&self) -> XtalBkAonR {
        XtalBkAonR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn xtal_capcode_extra_aon(&self) -> XtalCapcodeExtraAonR {
        XtalCapcodeExtraAonR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn xtal_ext_sel_aon(&self) -> XtalExtSelAonR {
        XtalExtSelAonR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn xtal_buf_en_aon(&self) -> XtalBufEnAonR {
        XtalBufEnAonR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn xtal_buf_hp_aon(&self) -> XtalBufHpAonR {
        XtalBufHpAonR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn xtal_fast_startup_aon(&self) -> XtalFastStartupAonR {
        XtalFastStartupAonR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn xtal_sleep_aon(&self) -> XtalSleepAonR {
        XtalSleepAonR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn xtal_amp_ctrl_aon(&self) -> XtalAmpCtrlAonR {
        XtalAmpCtrlAonR::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:21"]
    #[inline(always)]
    pub fn xtal_capcode_out_aon(&self) -> XtalCapcodeOutAonR {
        XtalCapcodeOutAonR::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 22:27"]
    #[inline(always)]
    pub fn xtal_capcode_in_aon(&self) -> XtalCapcodeInAonR {
        XtalCapcodeInAonR::new(((self.bits >> 22) & 0x3f) as u8)
    }
    #[doc = "Bits 28:29"]
    #[inline(always)]
    pub fn xtal_gm_boost_aon(&self) -> XtalGmBoostAonR {
        XtalGmBoostAonR::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31"]
    #[inline(always)]
    pub fn xtal_rdy_sel_aon(&self) -> XtalRdySelAonR {
        XtalRdySelAonR::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn xtal_bk_aon(&mut self) -> XtalBkAonW<'_, XtalCfgSpec> {
        XtalBkAonW::new(self, 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn xtal_capcode_extra_aon(&mut self) -> XtalCapcodeExtraAonW<'_, XtalCfgSpec> {
        XtalCapcodeExtraAonW::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn xtal_ext_sel_aon(&mut self) -> XtalExtSelAonW<'_, XtalCfgSpec> {
        XtalExtSelAonW::new(self, 3)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn xtal_buf_en_aon(&mut self) -> XtalBufEnAonW<'_, XtalCfgSpec> {
        XtalBufEnAonW::new(self, 4)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn xtal_buf_hp_aon(&mut self) -> XtalBufHpAonW<'_, XtalCfgSpec> {
        XtalBufHpAonW::new(self, 8)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn xtal_fast_startup_aon(&mut self) -> XtalFastStartupAonW<'_, XtalCfgSpec> {
        XtalFastStartupAonW::new(self, 12)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn xtal_sleep_aon(&mut self) -> XtalSleepAonW<'_, XtalCfgSpec> {
        XtalSleepAonW::new(self, 13)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn xtal_amp_ctrl_aon(&mut self) -> XtalAmpCtrlAonW<'_, XtalCfgSpec> {
        XtalAmpCtrlAonW::new(self, 14)
    }
    #[doc = "Bits 16:21"]
    #[inline(always)]
    pub fn xtal_capcode_out_aon(&mut self) -> XtalCapcodeOutAonW<'_, XtalCfgSpec> {
        XtalCapcodeOutAonW::new(self, 16)
    }
    #[doc = "Bits 22:27"]
    #[inline(always)]
    pub fn xtal_capcode_in_aon(&mut self) -> XtalCapcodeInAonW<'_, XtalCfgSpec> {
        XtalCapcodeInAonW::new(self, 22)
    }
    #[doc = "Bits 28:29"]
    #[inline(always)]
    pub fn xtal_gm_boost_aon(&mut self) -> XtalGmBoostAonW<'_, XtalCfgSpec> {
        XtalGmBoostAonW::new(self, 28)
    }
    #[doc = "Bits 30:31"]
    #[inline(always)]
    pub fn xtal_rdy_sel_aon(&mut self) -> XtalRdySelAonW<'_, XtalCfgSpec> {
        XtalRdySelAonW::new(self, 30)
    }
}
#[doc = "Crystal Oscillator Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`xtal_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xtal_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct XtalCfgSpec;
impl crate::RegisterSpec for XtalCfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`xtal_cfg::R`](R) reader structure"]
impl crate::Readable for XtalCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`xtal_cfg::W`](W) writer structure"]
impl crate::Writable for XtalCfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets XTAL_CFG to value 0"]
impl crate::Resettable for XtalCfgSpec {}
