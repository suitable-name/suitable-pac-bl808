#[doc = "Register `DCDC_TOP_0` reader"]
pub type R = crate::R<DcdcTop0Spec>;
#[doc = "Register `DCDC_TOP_0` writer"]
pub type W = crate::W<DcdcTop0Spec>;
#[doc = "Field `dcdc11_sstart_time_aon` reader - "]
pub type Dcdc11SstartTimeAonR = crate::FieldReader;
#[doc = "Field `dcdc11_sstart_time_aon` writer - "]
pub type Dcdc11SstartTimeAonW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `dcdc11_stby_lp_cur_aon` reader - "]
pub type Dcdc11StbyLpCurAonR = crate::FieldReader;
#[doc = "Field `dcdc11_stby_lp_cur_aon` writer - "]
pub type Dcdc11StbyLpCurAonW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `dcdc11_vc_clamp_vth_aon` reader - "]
pub type Dcdc11VcClampVthAonR = crate::FieldReader;
#[doc = "Field `dcdc11_vc_clamp_vth_aon` writer - "]
pub type Dcdc11VcClampVthAonW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `dcdc11_vout_sel_aon` reader - "]
pub type Dcdc11VoutSelAonR = crate::FieldReader;
#[doc = "Field `dcdc11_vout_sel_aon` writer - "]
pub type Dcdc11VoutSelAonW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `dcdc11_vout_trim_aon` reader - "]
pub type Dcdc11VoutTrimAonR = crate::FieldReader;
#[doc = "Field `dcdc11_vout_trim_aon` writer - "]
pub type Dcdc11VoutTrimAonW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `dcdc11_vpfm_aon` reader - "]
pub type Dcdc11VpfmAonR = crate::FieldReader;
#[doc = "Field `dcdc11_vpfm_aon` writer - "]
pub type Dcdc11VpfmAonW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `dcdc11_zvs_td_opt_aon` reader - "]
pub type Dcdc11ZvsTdOptAonR = crate::FieldReader;
#[doc = "Field `dcdc11_zvs_td_opt_aon` writer - "]
pub type Dcdc11ZvsTdOptAonW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `dcdc11_vstby_aon` reader - "]
pub type Dcdc11VstbyAonR = crate::FieldReader;
#[doc = "Field `dcdc11_vstby_aon` writer - "]
pub type Dcdc11VstbyAonW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn dcdc11_sstart_time_aon(&self) -> Dcdc11SstartTimeAonR {
        Dcdc11SstartTimeAonR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    pub fn dcdc11_stby_lp_cur_aon(&self) -> Dcdc11StbyLpCurAonR {
        Dcdc11StbyLpCurAonR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn dcdc11_vc_clamp_vth_aon(&self) -> Dcdc11VcClampVthAonR {
        Dcdc11VcClampVthAonR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 11:15"]
    #[inline(always)]
    pub fn dcdc11_vout_sel_aon(&self) -> Dcdc11VoutSelAonR {
        Dcdc11VoutSelAonR::new(((self.bits >> 11) & 0x1f) as u8)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn dcdc11_vout_trim_aon(&self) -> Dcdc11VoutTrimAonR {
        Dcdc11VoutTrimAonR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23"]
    #[inline(always)]
    pub fn dcdc11_vpfm_aon(&self) -> Dcdc11VpfmAonR {
        Dcdc11VpfmAonR::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:26"]
    #[inline(always)]
    pub fn dcdc11_zvs_td_opt_aon(&self) -> Dcdc11ZvsTdOptAonR {
        Dcdc11ZvsTdOptAonR::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 28:29"]
    #[inline(always)]
    pub fn dcdc11_vstby_aon(&self) -> Dcdc11VstbyAonR {
        Dcdc11VstbyAonR::new(((self.bits >> 28) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn dcdc11_sstart_time_aon(&mut self) -> Dcdc11SstartTimeAonW<'_, DcdcTop0Spec> {
        Dcdc11SstartTimeAonW::new(self, 0)
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    pub fn dcdc11_stby_lp_cur_aon(&mut self) -> Dcdc11StbyLpCurAonW<'_, DcdcTop0Spec> {
        Dcdc11StbyLpCurAonW::new(self, 4)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn dcdc11_vc_clamp_vth_aon(&mut self) -> Dcdc11VcClampVthAonW<'_, DcdcTop0Spec> {
        Dcdc11VcClampVthAonW::new(self, 8)
    }
    #[doc = "Bits 11:15"]
    #[inline(always)]
    pub fn dcdc11_vout_sel_aon(&mut self) -> Dcdc11VoutSelAonW<'_, DcdcTop0Spec> {
        Dcdc11VoutSelAonW::new(self, 11)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn dcdc11_vout_trim_aon(&mut self) -> Dcdc11VoutTrimAonW<'_, DcdcTop0Spec> {
        Dcdc11VoutTrimAonW::new(self, 16)
    }
    #[doc = "Bits 20:23"]
    #[inline(always)]
    pub fn dcdc11_vpfm_aon(&mut self) -> Dcdc11VpfmAonW<'_, DcdcTop0Spec> {
        Dcdc11VpfmAonW::new(self, 20)
    }
    #[doc = "Bits 24:26"]
    #[inline(always)]
    pub fn dcdc11_zvs_td_opt_aon(&mut self) -> Dcdc11ZvsTdOptAonW<'_, DcdcTop0Spec> {
        Dcdc11ZvsTdOptAonW::new(self, 24)
    }
    #[doc = "Bits 28:29"]
    #[inline(always)]
    pub fn dcdc11_vstby_aon(&mut self) -> Dcdc11VstbyAonW<'_, DcdcTop0Spec> {
        Dcdc11VstbyAonW::new(self, 28)
    }
}
#[doc = "DCDC11 (1.1V) Configuration 0\n\nYou can [`read`](crate::Reg::read) this register and get [`dcdc_top_0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcdc_top_0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DcdcTop0Spec;
impl crate::RegisterSpec for DcdcTop0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dcdc_top_0::R`](R) reader structure"]
impl crate::Readable for DcdcTop0Spec {}
#[doc = "`write(|w| ..)` method takes [`dcdc_top_0::W`](W) writer structure"]
impl crate::Writable for DcdcTop0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DCDC_TOP_0 to value 0"]
impl crate::Resettable for DcdcTop0Spec {}
