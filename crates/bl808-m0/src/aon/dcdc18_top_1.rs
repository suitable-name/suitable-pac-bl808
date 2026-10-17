#[doc = "Register `DCDC18_TOP_1` reader"]
pub type R = crate::R<Dcdc18Top1Spec>;
#[doc = "Register `DCDC18_TOP_1` writer"]
pub type W = crate::W<Dcdc18Top1Spec>;
#[doc = "Field `dcdc18_nonoverlap_td_aon` reader - "]
pub type Dcdc18NonoverlapTdAonR = crate::FieldReader;
#[doc = "Field `dcdc18_nonoverlap_td_aon` writer - "]
pub type Dcdc18NonoverlapTdAonW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `dcdc18_ocp_out_aon` reader - "]
pub type Dcdc18OcpOutAonR = crate::BitReader;
#[doc = "Field `dcdc18_ocp_rst_aon` reader - "]
pub type Dcdc18OcpRstAonR = crate::BitReader;
#[doc = "Field `dcdc18_ocp_rst_aon` writer - "]
pub type Dcdc18OcpRstAonW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dcdc18_ocp_vth_aon` reader - "]
pub type Dcdc18OcpVthAonR = crate::FieldReader;
#[doc = "Field `dcdc18_ocp_vth_aon` writer - "]
pub type Dcdc18OcpVthAonW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `dcdc18_osc_2m_mode_aon` reader - "]
pub type Dcdc18Osc2mModeAonR = crate::BitReader;
#[doc = "Field `dcdc18_osc_2m_mode_aon` writer - "]
pub type Dcdc18Osc2mModeAonW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dcdc18_osc_freq_trim_aon` reader - "]
pub type Dcdc18OscFreqTrimAonR = crate::FieldReader;
#[doc = "Field `dcdc18_osc_freq_trim_aon` writer - "]
pub type Dcdc18OscFreqTrimAonW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `dcdc18_pulldown_aon` reader - "]
pub type Dcdc18PulldownAonR = crate::BitReader;
#[doc = "Field `dcdc18_pulldown_aon` writer - "]
pub type Dcdc18PulldownAonW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dcdc18_rc_sel_aon` reader - "]
pub type Dcdc18RcSelAonR = crate::FieldReader;
#[doc = "Field `dcdc18_rc_sel_aon` writer - "]
pub type Dcdc18RcSelAonW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `dcdc18_rdy_aon` reader - "]
pub type Dcdc18RdyAonR = crate::BitReader;
#[doc = "Field `dcdc18_slope_curr_sel_aon` reader - "]
pub type Dcdc18SlopeCurrSelAonR = crate::FieldReader;
#[doc = "Field `dcdc18_slope_curr_sel_aon` writer - "]
pub type Dcdc18SlopeCurrSelAonW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn dcdc18_nonoverlap_td_aon(&self) -> Dcdc18NonoverlapTdAonR {
        Dcdc18NonoverlapTdAonR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn dcdc18_ocp_out_aon(&self) -> Dcdc18OcpOutAonR {
        Dcdc18OcpOutAonR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn dcdc18_ocp_rst_aon(&self) -> Dcdc18OcpRstAonR {
        Dcdc18OcpRstAonR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn dcdc18_ocp_vth_aon(&self) -> Dcdc18OcpVthAonR {
        Dcdc18OcpVthAonR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn dcdc18_osc_2m_mode_aon(&self) -> Dcdc18Osc2mModeAonR {
        Dcdc18Osc2mModeAonR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn dcdc18_osc_freq_trim_aon(&self) -> Dcdc18OscFreqTrimAonR {
        Dcdc18OscFreqTrimAonR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn dcdc18_pulldown_aon(&self) -> Dcdc18PulldownAonR {
        Dcdc18PulldownAonR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 20:23"]
    #[inline(always)]
    pub fn dcdc18_rc_sel_aon(&self) -> Dcdc18RcSelAonR {
        Dcdc18RcSelAonR::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn dcdc18_rdy_aon(&self) -> Dcdc18RdyAonR {
        Dcdc18RdyAonR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 26:30"]
    #[inline(always)]
    pub fn dcdc18_slope_curr_sel_aon(&self) -> Dcdc18SlopeCurrSelAonR {
        Dcdc18SlopeCurrSelAonR::new(((self.bits >> 26) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn dcdc18_nonoverlap_td_aon(&mut self) -> Dcdc18NonoverlapTdAonW<'_, Dcdc18Top1Spec> {
        Dcdc18NonoverlapTdAonW::new(self, 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn dcdc18_ocp_rst_aon(&mut self) -> Dcdc18OcpRstAonW<'_, Dcdc18Top1Spec> {
        Dcdc18OcpRstAonW::new(self, 6)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn dcdc18_ocp_vth_aon(&mut self) -> Dcdc18OcpVthAonW<'_, Dcdc18Top1Spec> {
        Dcdc18OcpVthAonW::new(self, 8)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn dcdc18_osc_2m_mode_aon(&mut self) -> Dcdc18Osc2mModeAonW<'_, Dcdc18Top1Spec> {
        Dcdc18Osc2mModeAonW::new(self, 11)
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn dcdc18_osc_freq_trim_aon(&mut self) -> Dcdc18OscFreqTrimAonW<'_, Dcdc18Top1Spec> {
        Dcdc18OscFreqTrimAonW::new(self, 12)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn dcdc18_pulldown_aon(&mut self) -> Dcdc18PulldownAonW<'_, Dcdc18Top1Spec> {
        Dcdc18PulldownAonW::new(self, 16)
    }
    #[doc = "Bits 20:23"]
    #[inline(always)]
    pub fn dcdc18_rc_sel_aon(&mut self) -> Dcdc18RcSelAonW<'_, Dcdc18Top1Spec> {
        Dcdc18RcSelAonW::new(self, 20)
    }
    #[doc = "Bits 26:30"]
    #[inline(always)]
    pub fn dcdc18_slope_curr_sel_aon(&mut self) -> Dcdc18SlopeCurrSelAonW<'_, Dcdc18Top1Spec> {
        Dcdc18SlopeCurrSelAonW::new(self, 26)
    }
}
#[doc = "DCDC18 (1.8V) Configuration 1\n\nYou can [`read`](crate::Reg::read) this register and get [`dcdc18_top_1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcdc18_top_1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dcdc18Top1Spec;
impl crate::RegisterSpec for Dcdc18Top1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dcdc18_top_1::R`](R) reader structure"]
impl crate::Readable for Dcdc18Top1Spec {}
#[doc = "`write(|w| ..)` method takes [`dcdc18_top_1::W`](W) writer structure"]
impl crate::Writable for Dcdc18Top1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DCDC18_TOP_1 to value 0"]
impl crate::Resettable for Dcdc18Top1Spec {}
