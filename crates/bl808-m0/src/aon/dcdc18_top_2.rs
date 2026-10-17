#[doc = "Register `DCDC18_TOP_2` reader"]
pub type R = crate::R<Dcdc18Top2Spec>;
#[doc = "Register `DCDC18_TOP_2` writer"]
pub type W = crate::W<Dcdc18Top2Spec>;
#[doc = "Field `dcdc18_cfb_sel_aon` reader - "]
pub type Dcdc18CfbSelAonR = crate::FieldReader;
#[doc = "Field `dcdc18_cfb_sel_aon` writer - "]
pub type Dcdc18CfbSelAonW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `dcdc18_chf_sel_aon` reader - "]
pub type Dcdc18ChfSelAonR = crate::FieldReader;
#[doc = "Field `dcdc18_chf_sel_aon` writer - "]
pub type Dcdc18ChfSelAonW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `dcdc18_comp_gm_sel_aon` reader - "]
pub type Dcdc18CompGmSelAonR = crate::FieldReader;
#[doc = "Field `dcdc18_comp_gm_sel_aon` writer - "]
pub type Dcdc18CompGmSelAonW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `dcdc18_cs_delay_aon` reader - "]
pub type Dcdc18CsDelayAonR = crate::FieldReader;
#[doc = "Field `dcdc18_cs_delay_aon` writer - "]
pub type Dcdc18CsDelayAonW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `dcdc18_drv_sr_aon` reader - "]
pub type Dcdc18DrvSrAonR = crate::FieldReader;
#[doc = "Field `dcdc18_drv_sr_aon` writer - "]
pub type Dcdc18DrvSrAonW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `dcdc18_en_antiring_aon` reader - "]
pub type Dcdc18EnAntiringAonR = crate::BitReader;
#[doc = "Field `dcdc18_en_antiring_aon` writer - "]
pub type Dcdc18EnAntiringAonW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dcdc18_en_osc_inhibit_t2_aon` reader - "]
pub type Dcdc18EnOscInhibitT2AonR = crate::BitReader;
#[doc = "Field `dcdc18_en_osc_inhibit_t2_aon` writer - "]
pub type Dcdc18EnOscInhibitT2AonW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dcdc18_en_slow_osc_aon` reader - "]
pub type Dcdc18EnSlowOscAonR = crate::BitReader;
#[doc = "Field `dcdc18_en_slow_osc_aon` writer - "]
pub type Dcdc18EnSlowOscAonW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dcdc18_en_stby_lp_aon` reader - "]
pub type Dcdc18EnStbyLpAonR = crate::BitReader;
#[doc = "Field `dcdc18_en_stby_lp_aon` writer - "]
pub type Dcdc18EnStbyLpAonW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dcdc18_en_stop_osc_aon` reader - "]
pub type Dcdc18EnStopOscAonR = crate::BitReader;
#[doc = "Field `dcdc18_en_stop_osc_aon` writer - "]
pub type Dcdc18EnStopOscAonW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dcdc18_force_en_cs_zvs_aon` reader - "]
pub type Dcdc18ForceEnCsZvsAonR = crate::BitReader;
#[doc = "Field `dcdc18_force_en_cs_zvs_aon` writer - "]
pub type Dcdc18ForceEnCsZvsAonW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dcdc18_isense_trim_aon` reader - "]
pub type Dcdc18IsenseTrimAonR = crate::FieldReader;
#[doc = "Field `dcdc18_isense_trim_aon` writer - "]
pub type Dcdc18IsenseTrimAonW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn dcdc18_cfb_sel_aon(&self) -> Dcdc18CfbSelAonR {
        Dcdc18CfbSelAonR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn dcdc18_chf_sel_aon(&self) -> Dcdc18ChfSelAonR {
        Dcdc18ChfSelAonR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    pub fn dcdc18_comp_gm_sel_aon(&self) -> Dcdc18CompGmSelAonR {
        Dcdc18CompGmSelAonR::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 16:18"]
    #[inline(always)]
    pub fn dcdc18_cs_delay_aon(&self) -> Dcdc18CsDelayAonR {
        Dcdc18CsDelayAonR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    pub fn dcdc18_drv_sr_aon(&self) -> Dcdc18DrvSrAonR {
        Dcdc18DrvSrAonR::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn dcdc18_en_antiring_aon(&self) -> Dcdc18EnAntiringAonR {
        Dcdc18EnAntiringAonR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn dcdc18_en_osc_inhibit_t2_aon(&self) -> Dcdc18EnOscInhibitT2AonR {
        Dcdc18EnOscInhibitT2AonR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn dcdc18_en_slow_osc_aon(&self) -> Dcdc18EnSlowOscAonR {
        Dcdc18EnSlowOscAonR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn dcdc18_en_stby_lp_aon(&self) -> Dcdc18EnStbyLpAonR {
        Dcdc18EnStbyLpAonR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn dcdc18_en_stop_osc_aon(&self) -> Dcdc18EnStopOscAonR {
        Dcdc18EnStopOscAonR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn dcdc18_force_en_cs_zvs_aon(&self) -> Dcdc18ForceEnCsZvsAonR {
        Dcdc18ForceEnCsZvsAonR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:30"]
    #[inline(always)]
    pub fn dcdc18_isense_trim_aon(&self) -> Dcdc18IsenseTrimAonR {
        Dcdc18IsenseTrimAonR::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn dcdc18_cfb_sel_aon(&mut self) -> Dcdc18CfbSelAonW<'_, Dcdc18Top2Spec> {
        Dcdc18CfbSelAonW::new(self, 4)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn dcdc18_chf_sel_aon(&mut self) -> Dcdc18ChfSelAonW<'_, Dcdc18Top2Spec> {
        Dcdc18ChfSelAonW::new(self, 8)
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    pub fn dcdc18_comp_gm_sel_aon(&mut self) -> Dcdc18CompGmSelAonW<'_, Dcdc18Top2Spec> {
        Dcdc18CompGmSelAonW::new(self, 12)
    }
    #[doc = "Bits 16:18"]
    #[inline(always)]
    pub fn dcdc18_cs_delay_aon(&mut self) -> Dcdc18CsDelayAonW<'_, Dcdc18Top2Spec> {
        Dcdc18CsDelayAonW::new(self, 16)
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    pub fn dcdc18_drv_sr_aon(&mut self) -> Dcdc18DrvSrAonW<'_, Dcdc18Top2Spec> {
        Dcdc18DrvSrAonW::new(self, 20)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn dcdc18_en_antiring_aon(&mut self) -> Dcdc18EnAntiringAonW<'_, Dcdc18Top2Spec> {
        Dcdc18EnAntiringAonW::new(self, 22)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn dcdc18_en_osc_inhibit_t2_aon(&mut self) -> Dcdc18EnOscInhibitT2AonW<'_, Dcdc18Top2Spec> {
        Dcdc18EnOscInhibitT2AonW::new(self, 23)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn dcdc18_en_slow_osc_aon(&mut self) -> Dcdc18EnSlowOscAonW<'_, Dcdc18Top2Spec> {
        Dcdc18EnSlowOscAonW::new(self, 24)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn dcdc18_en_stby_lp_aon(&mut self) -> Dcdc18EnStbyLpAonW<'_, Dcdc18Top2Spec> {
        Dcdc18EnStbyLpAonW::new(self, 25)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn dcdc18_en_stop_osc_aon(&mut self) -> Dcdc18EnStopOscAonW<'_, Dcdc18Top2Spec> {
        Dcdc18EnStopOscAonW::new(self, 26)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn dcdc18_force_en_cs_zvs_aon(&mut self) -> Dcdc18ForceEnCsZvsAonW<'_, Dcdc18Top2Spec> {
        Dcdc18ForceEnCsZvsAonW::new(self, 27)
    }
    #[doc = "Bits 28:30"]
    #[inline(always)]
    pub fn dcdc18_isense_trim_aon(&mut self) -> Dcdc18IsenseTrimAonW<'_, Dcdc18Top2Spec> {
        Dcdc18IsenseTrimAonW::new(self, 28)
    }
}
#[doc = "DCDC18 (1.8V) Configuration 2\n\nYou can [`read`](crate::Reg::read) this register and get [`dcdc18_top_2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcdc18_top_2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dcdc18Top2Spec;
impl crate::RegisterSpec for Dcdc18Top2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dcdc18_top_2::R`](R) reader structure"]
impl crate::Readable for Dcdc18Top2Spec {}
#[doc = "`write(|w| ..)` method takes [`dcdc18_top_2::W`](W) writer structure"]
impl crate::Writable for Dcdc18Top2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DCDC18_TOP_2 to value 0"]
impl crate::Resettable for Dcdc18Top2Spec {}
