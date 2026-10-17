#[doc = "Register `LDO11SOC_AND_DCTEST` reader"]
pub type R = crate::R<Ldo11socAndDctestSpec>;
#[doc = "Register `LDO11SOC_AND_DCTEST` writer"]
pub type W = crate::W<Ldo11socAndDctestSpec>;
#[doc = "Field `dcdc11_cfb_sel_aon` reader - "]
pub type Dcdc11CfbSelAonR = crate::FieldReader;
#[doc = "Field `dcdc11_cfb_sel_aon` writer - "]
pub type Dcdc11CfbSelAonW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `dcdc11_chf_sel_aon` reader - "]
pub type Dcdc11ChfSelAonR = crate::FieldReader;
#[doc = "Field `dcdc11_chf_sel_aon` writer - "]
pub type Dcdc11ChfSelAonW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `dcdc11_comp_gm_sel_aon` reader - "]
pub type Dcdc11CompGmSelAonR = crate::FieldReader;
#[doc = "Field `dcdc11_comp_gm_sel_aon` writer - "]
pub type Dcdc11CompGmSelAonW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `dcdc11_cs_delay_aon` reader - "]
pub type Dcdc11CsDelayAonR = crate::FieldReader;
#[doc = "Field `dcdc11_cs_delay_aon` writer - "]
pub type Dcdc11CsDelayAonW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `dcdc11_drv_sr_aon` reader - "]
pub type Dcdc11DrvSrAonR = crate::FieldReader;
#[doc = "Field `dcdc11_drv_sr_aon` writer - "]
pub type Dcdc11DrvSrAonW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `dcdc11_en_antiring_aon` reader - "]
pub type Dcdc11EnAntiringAonR = crate::BitReader;
#[doc = "Field `dcdc11_en_antiring_aon` writer - "]
pub type Dcdc11EnAntiringAonW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dcdc11_en_osc_inhibit_t2_aon` reader - "]
pub type Dcdc11EnOscInhibitT2AonR = crate::BitReader;
#[doc = "Field `dcdc11_en_osc_inhibit_t2_aon` writer - "]
pub type Dcdc11EnOscInhibitT2AonW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dcdc11_en_slow_osc_aon` reader - "]
pub type Dcdc11EnSlowOscAonR = crate::BitReader;
#[doc = "Field `dcdc11_en_slow_osc_aon` writer - "]
pub type Dcdc11EnSlowOscAonW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dcdc11_en_stby_lp_aon` reader - "]
pub type Dcdc11EnStbyLpAonR = crate::BitReader;
#[doc = "Field `dcdc11_en_stby_lp_aon` writer - "]
pub type Dcdc11EnStbyLpAonW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dcdc11_en_stop_osc_aon` reader - "]
pub type Dcdc11EnStopOscAonR = crate::BitReader;
#[doc = "Field `dcdc11_en_stop_osc_aon` writer - "]
pub type Dcdc11EnStopOscAonW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dcdc11_force_en_cs_zvs_aon` reader - "]
pub type Dcdc11ForceEnCsZvsAonR = crate::BitReader;
#[doc = "Field `dcdc11_force_en_cs_zvs_aon` writer - "]
pub type Dcdc11ForceEnCsZvsAonW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dcdc11_isense_trim_aon` reader - "]
pub type Dcdc11IsenseTrimAonR = crate::FieldReader;
#[doc = "Field `dcdc11_isense_trim_aon` writer - "]
pub type Dcdc11IsenseTrimAonW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn dcdc11_cfb_sel_aon(&self) -> Dcdc11CfbSelAonR {
        Dcdc11CfbSelAonR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn dcdc11_chf_sel_aon(&self) -> Dcdc11ChfSelAonR {
        Dcdc11ChfSelAonR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    pub fn dcdc11_comp_gm_sel_aon(&self) -> Dcdc11CompGmSelAonR {
        Dcdc11CompGmSelAonR::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 16:18"]
    #[inline(always)]
    pub fn dcdc11_cs_delay_aon(&self) -> Dcdc11CsDelayAonR {
        Dcdc11CsDelayAonR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    pub fn dcdc11_drv_sr_aon(&self) -> Dcdc11DrvSrAonR {
        Dcdc11DrvSrAonR::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn dcdc11_en_antiring_aon(&self) -> Dcdc11EnAntiringAonR {
        Dcdc11EnAntiringAonR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn dcdc11_en_osc_inhibit_t2_aon(&self) -> Dcdc11EnOscInhibitT2AonR {
        Dcdc11EnOscInhibitT2AonR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn dcdc11_en_slow_osc_aon(&self) -> Dcdc11EnSlowOscAonR {
        Dcdc11EnSlowOscAonR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn dcdc11_en_stby_lp_aon(&self) -> Dcdc11EnStbyLpAonR {
        Dcdc11EnStbyLpAonR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn dcdc11_en_stop_osc_aon(&self) -> Dcdc11EnStopOscAonR {
        Dcdc11EnStopOscAonR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn dcdc11_force_en_cs_zvs_aon(&self) -> Dcdc11ForceEnCsZvsAonR {
        Dcdc11ForceEnCsZvsAonR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:30"]
    #[inline(always)]
    pub fn dcdc11_isense_trim_aon(&self) -> Dcdc11IsenseTrimAonR {
        Dcdc11IsenseTrimAonR::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn dcdc11_cfb_sel_aon(&mut self) -> Dcdc11CfbSelAonW<'_, Ldo11socAndDctestSpec> {
        Dcdc11CfbSelAonW::new(self, 4)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn dcdc11_chf_sel_aon(&mut self) -> Dcdc11ChfSelAonW<'_, Ldo11socAndDctestSpec> {
        Dcdc11ChfSelAonW::new(self, 8)
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    pub fn dcdc11_comp_gm_sel_aon(&mut self) -> Dcdc11CompGmSelAonW<'_, Ldo11socAndDctestSpec> {
        Dcdc11CompGmSelAonW::new(self, 12)
    }
    #[doc = "Bits 16:18"]
    #[inline(always)]
    pub fn dcdc11_cs_delay_aon(&mut self) -> Dcdc11CsDelayAonW<'_, Ldo11socAndDctestSpec> {
        Dcdc11CsDelayAonW::new(self, 16)
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    pub fn dcdc11_drv_sr_aon(&mut self) -> Dcdc11DrvSrAonW<'_, Ldo11socAndDctestSpec> {
        Dcdc11DrvSrAonW::new(self, 20)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn dcdc11_en_antiring_aon(&mut self) -> Dcdc11EnAntiringAonW<'_, Ldo11socAndDctestSpec> {
        Dcdc11EnAntiringAonW::new(self, 22)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn dcdc11_en_osc_inhibit_t2_aon(
        &mut self,
    ) -> Dcdc11EnOscInhibitT2AonW<'_, Ldo11socAndDctestSpec> {
        Dcdc11EnOscInhibitT2AonW::new(self, 23)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn dcdc11_en_slow_osc_aon(&mut self) -> Dcdc11EnSlowOscAonW<'_, Ldo11socAndDctestSpec> {
        Dcdc11EnSlowOscAonW::new(self, 24)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn dcdc11_en_stby_lp_aon(&mut self) -> Dcdc11EnStbyLpAonW<'_, Ldo11socAndDctestSpec> {
        Dcdc11EnStbyLpAonW::new(self, 25)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn dcdc11_en_stop_osc_aon(&mut self) -> Dcdc11EnStopOscAonW<'_, Ldo11socAndDctestSpec> {
        Dcdc11EnStopOscAonW::new(self, 26)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn dcdc11_force_en_cs_zvs_aon(
        &mut self,
    ) -> Dcdc11ForceEnCsZvsAonW<'_, Ldo11socAndDctestSpec> {
        Dcdc11ForceEnCsZvsAonW::new(self, 27)
    }
    #[doc = "Bits 28:30"]
    #[inline(always)]
    pub fn dcdc11_isense_trim_aon(&mut self) -> Dcdc11IsenseTrimAonW<'_, Ldo11socAndDctestSpec> {
        Dcdc11IsenseTrimAonW::new(self, 28)
    }
}
#[doc = "LDO11SOC and DCDC Test Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`ldo11soc_and_dctest::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ldo11soc_and_dctest::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ldo11socAndDctestSpec;
impl crate::RegisterSpec for Ldo11socAndDctestSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ldo11soc_and_dctest::R`](R) reader structure"]
impl crate::Readable for Ldo11socAndDctestSpec {}
#[doc = "`write(|w| ..)` method takes [`ldo11soc_and_dctest::W`](W) writer structure"]
impl crate::Writable for Ldo11socAndDctestSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LDO11SOC_AND_DCTEST to value 0"]
impl crate::Resettable for Ldo11socAndDctestSpec {}
