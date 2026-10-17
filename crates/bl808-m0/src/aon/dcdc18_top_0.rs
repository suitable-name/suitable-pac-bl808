#[doc = "Register `DCDC18_TOP_0` reader"]
pub type R = crate::R<Dcdc18Top0Spec>;
#[doc = "Register `DCDC18_TOP_0` writer"]
pub type W = crate::W<Dcdc18Top0Spec>;
#[doc = "Field `dcdc18_sstart_time_aon` reader - "]
pub type Dcdc18SstartTimeAonR = crate::FieldReader;
#[doc = "Field `dcdc18_sstart_time_aon` writer - "]
pub type Dcdc18SstartTimeAonW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `dcdc18_stby_lp_cur_aon` reader - "]
pub type Dcdc18StbyLpCurAonR = crate::FieldReader;
#[doc = "Field `dcdc18_stby_lp_cur_aon` writer - "]
pub type Dcdc18StbyLpCurAonW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `dcdc18_vc_clamp_vth_aon` reader - "]
pub type Dcdc18VcClampVthAonR = crate::FieldReader;
#[doc = "Field `dcdc18_vc_clamp_vth_aon` writer - "]
pub type Dcdc18VcClampVthAonW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `dcdc18_vout_sel_aon` reader - "]
pub type Dcdc18VoutSelAonR = crate::FieldReader;
#[doc = "Field `dcdc18_vout_sel_aon` writer - "]
pub type Dcdc18VoutSelAonW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `dcdc18_vout_trim_aon` reader - "]
pub type Dcdc18VoutTrimAonR = crate::FieldReader;
#[doc = "Field `dcdc18_vout_trim_aon` writer - "]
pub type Dcdc18VoutTrimAonW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `dcdc18_vpfm_aon` reader - "]
pub type Dcdc18VpfmAonR = crate::FieldReader;
#[doc = "Field `dcdc18_vpfm_aon` writer - "]
pub type Dcdc18VpfmAonW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `dcdc18_zvs_td_opt_aon` reader - "]
pub type Dcdc18ZvsTdOptAonR = crate::FieldReader;
#[doc = "Field `dcdc18_zvs_td_opt_aon` writer - "]
pub type Dcdc18ZvsTdOptAonW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `dcdc18_vstby_aon` reader - "]
pub type Dcdc18VstbyAonR = crate::FieldReader;
#[doc = "Field `dcdc18_vstby_aon` writer - "]
pub type Dcdc18VstbyAonW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn dcdc18_sstart_time_aon(&self) -> Dcdc18SstartTimeAonR {
        Dcdc18SstartTimeAonR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    pub fn dcdc18_stby_lp_cur_aon(&self) -> Dcdc18StbyLpCurAonR {
        Dcdc18StbyLpCurAonR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn dcdc18_vc_clamp_vth_aon(&self) -> Dcdc18VcClampVthAonR {
        Dcdc18VcClampVthAonR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 11:15"]
    #[inline(always)]
    pub fn dcdc18_vout_sel_aon(&self) -> Dcdc18VoutSelAonR {
        Dcdc18VoutSelAonR::new(((self.bits >> 11) & 0x1f) as u8)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn dcdc18_vout_trim_aon(&self) -> Dcdc18VoutTrimAonR {
        Dcdc18VoutTrimAonR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23"]
    #[inline(always)]
    pub fn dcdc18_vpfm_aon(&self) -> Dcdc18VpfmAonR {
        Dcdc18VpfmAonR::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:26"]
    #[inline(always)]
    pub fn dcdc18_zvs_td_opt_aon(&self) -> Dcdc18ZvsTdOptAonR {
        Dcdc18ZvsTdOptAonR::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 28:29"]
    #[inline(always)]
    pub fn dcdc18_vstby_aon(&self) -> Dcdc18VstbyAonR {
        Dcdc18VstbyAonR::new(((self.bits >> 28) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn dcdc18_sstart_time_aon(&mut self) -> Dcdc18SstartTimeAonW<'_, Dcdc18Top0Spec> {
        Dcdc18SstartTimeAonW::new(self, 0)
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    pub fn dcdc18_stby_lp_cur_aon(&mut self) -> Dcdc18StbyLpCurAonW<'_, Dcdc18Top0Spec> {
        Dcdc18StbyLpCurAonW::new(self, 4)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn dcdc18_vc_clamp_vth_aon(&mut self) -> Dcdc18VcClampVthAonW<'_, Dcdc18Top0Spec> {
        Dcdc18VcClampVthAonW::new(self, 8)
    }
    #[doc = "Bits 11:15"]
    #[inline(always)]
    pub fn dcdc18_vout_sel_aon(&mut self) -> Dcdc18VoutSelAonW<'_, Dcdc18Top0Spec> {
        Dcdc18VoutSelAonW::new(self, 11)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn dcdc18_vout_trim_aon(&mut self) -> Dcdc18VoutTrimAonW<'_, Dcdc18Top0Spec> {
        Dcdc18VoutTrimAonW::new(self, 16)
    }
    #[doc = "Bits 20:23"]
    #[inline(always)]
    pub fn dcdc18_vpfm_aon(&mut self) -> Dcdc18VpfmAonW<'_, Dcdc18Top0Spec> {
        Dcdc18VpfmAonW::new(self, 20)
    }
    #[doc = "Bits 24:26"]
    #[inline(always)]
    pub fn dcdc18_zvs_td_opt_aon(&mut self) -> Dcdc18ZvsTdOptAonW<'_, Dcdc18Top0Spec> {
        Dcdc18ZvsTdOptAonW::new(self, 24)
    }
    #[doc = "Bits 28:29"]
    #[inline(always)]
    pub fn dcdc18_vstby_aon(&mut self) -> Dcdc18VstbyAonW<'_, Dcdc18Top0Spec> {
        Dcdc18VstbyAonW::new(self, 28)
    }
}
#[doc = "DCDC18 (1.8V) Configuration 0\n\nYou can [`read`](crate::Reg::read) this register and get [`dcdc18_top_0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcdc18_top_0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dcdc18Top0Spec;
impl crate::RegisterSpec for Dcdc18Top0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dcdc18_top_0::R`](R) reader structure"]
impl crate::Readable for Dcdc18Top0Spec {}
#[doc = "`write(|w| ..)` method takes [`dcdc18_top_0::W`](W) writer structure"]
impl crate::Writable for Dcdc18Top0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DCDC18_TOP_0 to value 0"]
impl crate::Resettable for Dcdc18Top0Spec {}
