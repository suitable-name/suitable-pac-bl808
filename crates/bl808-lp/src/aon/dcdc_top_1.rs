#[doc = "Register `DCDC_TOP_1` reader"]
pub type R = crate::R<DcdcTop1Spec>;
#[doc = "Register `DCDC_TOP_1` writer"]
pub type W = crate::W<DcdcTop1Spec>;
#[doc = "Field `dcdc11_nonoverlap_td_aon` reader - "]
pub type Dcdc11NonoverlapTdAonR = crate::FieldReader;
#[doc = "Field `dcdc11_nonoverlap_td_aon` writer - "]
pub type Dcdc11NonoverlapTdAonW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `dcdc11_ocp_out_aon` reader - "]
pub type Dcdc11OcpOutAonR = crate::BitReader;
#[doc = "Field `dcdc11_ocp_rst_aon` reader - "]
pub type Dcdc11OcpRstAonR = crate::BitReader;
#[doc = "Field `dcdc11_ocp_rst_aon` writer - "]
pub type Dcdc11OcpRstAonW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dcdc11_ocp_vth_aon` reader - "]
pub type Dcdc11OcpVthAonR = crate::FieldReader;
#[doc = "Field `dcdc11_ocp_vth_aon` writer - "]
pub type Dcdc11OcpVthAonW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `dcdc11_osc_2m_mode_aon` reader - "]
pub type Dcdc11Osc2mModeAonR = crate::BitReader;
#[doc = "Field `dcdc11_osc_2m_mode_aon` writer - "]
pub type Dcdc11Osc2mModeAonW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dcdc11_osc_freq_trim_aon` reader - "]
pub type Dcdc11OscFreqTrimAonR = crate::FieldReader;
#[doc = "Field `dcdc11_osc_freq_trim_aon` writer - "]
pub type Dcdc11OscFreqTrimAonW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `dcdc11_pulldown_aon` reader - "]
pub type Dcdc11PulldownAonR = crate::BitReader;
#[doc = "Field `dcdc11_pulldown_aon` writer - "]
pub type Dcdc11PulldownAonW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dcdc11_rc_sel_aon` reader - "]
pub type Dcdc11RcSelAonR = crate::FieldReader;
#[doc = "Field `dcdc11_rc_sel_aon` writer - "]
pub type Dcdc11RcSelAonW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `dcdc11_rdy_aon` reader - "]
pub type Dcdc11RdyAonR = crate::BitReader;
#[doc = "Field `dcdc11_slope_curr_sel_aon` reader - "]
pub type Dcdc11SlopeCurrSelAonR = crate::FieldReader;
#[doc = "Field `dcdc11_slope_curr_sel_aon` writer - "]
pub type Dcdc11SlopeCurrSelAonW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn dcdc11_nonoverlap_td_aon(&self) -> Dcdc11NonoverlapTdAonR {
        Dcdc11NonoverlapTdAonR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn dcdc11_ocp_out_aon(&self) -> Dcdc11OcpOutAonR {
        Dcdc11OcpOutAonR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn dcdc11_ocp_rst_aon(&self) -> Dcdc11OcpRstAonR {
        Dcdc11OcpRstAonR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn dcdc11_ocp_vth_aon(&self) -> Dcdc11OcpVthAonR {
        Dcdc11OcpVthAonR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn dcdc11_osc_2m_mode_aon(&self) -> Dcdc11Osc2mModeAonR {
        Dcdc11Osc2mModeAonR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn dcdc11_osc_freq_trim_aon(&self) -> Dcdc11OscFreqTrimAonR {
        Dcdc11OscFreqTrimAonR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn dcdc11_pulldown_aon(&self) -> Dcdc11PulldownAonR {
        Dcdc11PulldownAonR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 20:23"]
    #[inline(always)]
    pub fn dcdc11_rc_sel_aon(&self) -> Dcdc11RcSelAonR {
        Dcdc11RcSelAonR::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn dcdc11_rdy_aon(&self) -> Dcdc11RdyAonR {
        Dcdc11RdyAonR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 26:30"]
    #[inline(always)]
    pub fn dcdc11_slope_curr_sel_aon(&self) -> Dcdc11SlopeCurrSelAonR {
        Dcdc11SlopeCurrSelAonR::new(((self.bits >> 26) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn dcdc11_nonoverlap_td_aon(&mut self) -> Dcdc11NonoverlapTdAonW<'_, DcdcTop1Spec> {
        Dcdc11NonoverlapTdAonW::new(self, 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn dcdc11_ocp_rst_aon(&mut self) -> Dcdc11OcpRstAonW<'_, DcdcTop1Spec> {
        Dcdc11OcpRstAonW::new(self, 6)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn dcdc11_ocp_vth_aon(&mut self) -> Dcdc11OcpVthAonW<'_, DcdcTop1Spec> {
        Dcdc11OcpVthAonW::new(self, 8)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn dcdc11_osc_2m_mode_aon(&mut self) -> Dcdc11Osc2mModeAonW<'_, DcdcTop1Spec> {
        Dcdc11Osc2mModeAonW::new(self, 11)
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn dcdc11_osc_freq_trim_aon(&mut self) -> Dcdc11OscFreqTrimAonW<'_, DcdcTop1Spec> {
        Dcdc11OscFreqTrimAonW::new(self, 12)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn dcdc11_pulldown_aon(&mut self) -> Dcdc11PulldownAonW<'_, DcdcTop1Spec> {
        Dcdc11PulldownAonW::new(self, 16)
    }
    #[doc = "Bits 20:23"]
    #[inline(always)]
    pub fn dcdc11_rc_sel_aon(&mut self) -> Dcdc11RcSelAonW<'_, DcdcTop1Spec> {
        Dcdc11RcSelAonW::new(self, 20)
    }
    #[doc = "Bits 26:30"]
    #[inline(always)]
    pub fn dcdc11_slope_curr_sel_aon(&mut self) -> Dcdc11SlopeCurrSelAonW<'_, DcdcTop1Spec> {
        Dcdc11SlopeCurrSelAonW::new(self, 26)
    }
}
#[doc = "DCDC11 (1.1V) Configuration 1\n\nYou can [`read`](crate::Reg::read) this register and get [`dcdc_top_1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcdc_top_1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DcdcTop1Spec;
impl crate::RegisterSpec for DcdcTop1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dcdc_top_1::R`](R) reader structure"]
impl crate::Readable for DcdcTop1Spec {}
#[doc = "`write(|w| ..)` method takes [`dcdc_top_1::W`](W) writer structure"]
impl crate::Writable for DcdcTop1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DCDC_TOP_1 to value 0"]
impl crate::Resettable for DcdcTop1Spec {}
