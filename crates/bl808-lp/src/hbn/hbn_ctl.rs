#[doc = "Register `HBN_CTL` reader"]
pub type R = crate::R<HbnCtlSpec>;
#[doc = "Register `HBN_CTL` writer"]
pub type W = crate::W<HbnCtlSpec>;
#[doc = "Field `rtc_ctl` reader - "]
pub type RtcCtlR = crate::FieldReader;
#[doc = "Field `rtc_ctl` writer - "]
pub type RtcCtlW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `rtc_dly_option` reader - "]
pub type RtcDlyOptionR = crate::BitReader;
#[doc = "Field `rtc_dly_option` writer - "]
pub type RtcDlyOptionW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pu_ldo18io_aon` reader - "]
pub type PuLdo18ioAonR = crate::BitReader;
#[doc = "Field `pu_ldo18io_aon` writer - "]
pub type PuLdo18ioAonW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `hbn_mode` writer - Hibernate mode trigger"]
pub type HbnModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `trap_mode` reader - "]
pub type TrapModeR = crate::BitReader;
#[doc = "Field `pwrdn_hbn_core` reader - "]
pub type PwrdnHbnCoreR = crate::BitReader;
#[doc = "Field `pwrdn_hbn_core` writer - "]
pub type PwrdnHbnCoreW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sw_rst` reader - "]
pub type SwRstR = crate::BitReader;
#[doc = "Field `sw_rst` writer - "]
pub type SwRstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `hbn_dis_pwr_off_ldo11` reader - "]
pub type HbnDisPwrOffLdo11R = crate::BitReader;
#[doc = "Field `hbn_dis_pwr_off_ldo11` writer - "]
pub type HbnDisPwrOffLdo11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `hbn_dis_pwr_off_ldo11_rt` reader - "]
pub type HbnDisPwrOffLdo11RtR = crate::BitReader;
#[doc = "Field `hbn_dis_pwr_off_ldo11_rt` writer - "]
pub type HbnDisPwrOffLdo11RtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `hbn_ldo11_rt_vout_sel` reader - "]
pub type HbnLdo11RtVoutSelR = crate::FieldReader;
#[doc = "Field `hbn_ldo11_rt_vout_sel` writer - "]
pub type HbnLdo11RtVoutSelW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `hbn_ldo11_aon_vout_sel` reader - "]
pub type HbnLdo11AonVoutSelR = crate::FieldReader;
#[doc = "Field `hbn_ldo11_aon_vout_sel` writer - "]
pub type HbnLdo11AonVoutSelW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `pu_dcdc_aon` reader - "]
pub type PuDcdcAonR = crate::BitReader;
#[doc = "Field `pu_dcdc_aon` writer - "]
pub type PuDcdcAonW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pu_dcdc18_aon` reader - "]
pub type PuDcdc18AonR = crate::BitReader;
#[doc = "Field `pu_dcdc18_aon` writer - "]
pub type PuDcdc18AonW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pwr_on_option` reader - "]
pub type PwrOnOptionR = crate::BitReader;
#[doc = "Field `pwr_on_option` writer - "]
pub type PwrOnOptionW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sram_slp_option` reader - "]
pub type SramSlpOptionR = crate::BitReader;
#[doc = "Field `sram_slp_option` writer - "]
pub type SramSlpOptionW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sram_slp` reader - "]
pub type SramSlpR = crate::BitReader;
#[doc = "Field `hbn_state` reader - "]
pub type HbnStateR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn rtc_ctl(&self) -> RtcCtlR {
        RtcCtlR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn rtc_dly_option(&self) -> RtcDlyOptionR {
        RtcDlyOptionR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn pu_ldo18io_aon(&self) -> PuLdo18ioAonR {
        PuLdo18ioAonR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn trap_mode(&self) -> TrapModeR {
        TrapModeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn pwrdn_hbn_core(&self) -> PwrdnHbnCoreR {
        PwrdnHbnCoreR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn sw_rst(&self) -> SwRstR {
        SwRstR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn hbn_dis_pwr_off_ldo11(&self) -> HbnDisPwrOffLdo11R {
        HbnDisPwrOffLdo11R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn hbn_dis_pwr_off_ldo11_rt(&self) -> HbnDisPwrOffLdo11RtR {
        HbnDisPwrOffLdo11RtR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 15:18"]
    #[inline(always)]
    pub fn hbn_ldo11_rt_vout_sel(&self) -> HbnLdo11RtVoutSelR {
        HbnLdo11RtVoutSelR::new(((self.bits >> 15) & 0x0f) as u8)
    }
    #[doc = "Bits 19:22"]
    #[inline(always)]
    pub fn hbn_ldo11_aon_vout_sel(&self) -> HbnLdo11AonVoutSelR {
        HbnLdo11AonVoutSelR::new(((self.bits >> 19) & 0x0f) as u8)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn pu_dcdc_aon(&self) -> PuDcdcAonR {
        PuDcdcAonR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn pu_dcdc18_aon(&self) -> PuDcdc18AonR {
        PuDcdc18AonR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn pwr_on_option(&self) -> PwrOnOptionR {
        PwrOnOptionR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn sram_slp_option(&self) -> SramSlpOptionR {
        SramSlpOptionR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn sram_slp(&self) -> SramSlpR {
        SramSlpR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn hbn_state(&self) -> HbnStateR {
        HbnStateR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn rtc_ctl(&mut self) -> RtcCtlW<'_, HbnCtlSpec> {
        RtcCtlW::new(self, 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn rtc_dly_option(&mut self) -> RtcDlyOptionW<'_, HbnCtlSpec> {
        RtcDlyOptionW::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn pu_ldo18io_aon(&mut self) -> PuLdo18ioAonW<'_, HbnCtlSpec> {
        PuLdo18ioAonW::new(self, 5)
    }
    #[doc = "Bit 7 - Hibernate mode trigger"]
    #[inline(always)]
    pub fn hbn_mode(&mut self) -> HbnModeW<'_, HbnCtlSpec> {
        HbnModeW::new(self, 7)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn pwrdn_hbn_core(&mut self) -> PwrdnHbnCoreW<'_, HbnCtlSpec> {
        PwrdnHbnCoreW::new(self, 9)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn sw_rst(&mut self) -> SwRstW<'_, HbnCtlSpec> {
        SwRstW::new(self, 12)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn hbn_dis_pwr_off_ldo11(&mut self) -> HbnDisPwrOffLdo11W<'_, HbnCtlSpec> {
        HbnDisPwrOffLdo11W::new(self, 13)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn hbn_dis_pwr_off_ldo11_rt(&mut self) -> HbnDisPwrOffLdo11RtW<'_, HbnCtlSpec> {
        HbnDisPwrOffLdo11RtW::new(self, 14)
    }
    #[doc = "Bits 15:18"]
    #[inline(always)]
    pub fn hbn_ldo11_rt_vout_sel(&mut self) -> HbnLdo11RtVoutSelW<'_, HbnCtlSpec> {
        HbnLdo11RtVoutSelW::new(self, 15)
    }
    #[doc = "Bits 19:22"]
    #[inline(always)]
    pub fn hbn_ldo11_aon_vout_sel(&mut self) -> HbnLdo11AonVoutSelW<'_, HbnCtlSpec> {
        HbnLdo11AonVoutSelW::new(self, 19)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn pu_dcdc_aon(&mut self) -> PuDcdcAonW<'_, HbnCtlSpec> {
        PuDcdcAonW::new(self, 23)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn pu_dcdc18_aon(&mut self) -> PuDcdc18AonW<'_, HbnCtlSpec> {
        PuDcdc18AonW::new(self, 24)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn pwr_on_option(&mut self) -> PwrOnOptionW<'_, HbnCtlSpec> {
        PwrOnOptionW::new(self, 25)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn sram_slp_option(&mut self) -> SramSlpOptionW<'_, HbnCtlSpec> {
        SramSlpOptionW::new(self, 26)
    }
}
#[doc = "HBN Main Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hbn_ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hbn_ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HbnCtlSpec;
impl crate::RegisterSpec for HbnCtlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hbn_ctl::R`](R) reader structure"]
impl crate::Readable for HbnCtlSpec {}
#[doc = "`write(|w| ..)` method takes [`hbn_ctl::W`](W) writer structure"]
impl crate::Writable for HbnCtlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HBN_CTL to value 0x01d5_0020"]
impl crate::Resettable for HbnCtlSpec {
    const RESET_VALUE: u32 = 0x01d5_0020;
}
