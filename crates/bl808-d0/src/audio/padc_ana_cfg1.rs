#[doc = "Register `padc_ana_cfg1` reader"]
pub type R = crate::R<PadcAnaCfg1Spec>;
#[doc = "Register `padc_ana_cfg1` writer"]
pub type W = crate::W<PadcAnaCfg1Spec>;
#[doc = "Field `padc_pga_chop_cksel` reader - "]
pub type PadcPgaChopCkselR = crate::BitReader;
#[doc = "Field `padc_pga_chop_cksel` writer - "]
pub type PadcPgaChopCkselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `padc_pga_chop_freq` reader - "]
pub type PadcPgaChopFreqR = crate::FieldReader;
#[doc = "Field `padc_pga_chop_freq` writer - "]
pub type PadcPgaChopFreqW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `padc_pga_chop_en` reader - "]
pub type PadcPgaChopEnR = crate::BitReader;
#[doc = "Field `padc_pga_chop_en` writer - "]
pub type PadcPgaChopEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `padc_pga_noisctrl` reader - "]
pub type PadcPgaNoisctrlR = crate::FieldReader;
#[doc = "Field `padc_pga_noisctrl` writer - "]
pub type PadcPgaNoisctrlW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `padc_ictrl_pga_buf` reader - "]
pub type PadcIctrlPgaBufR = crate::FieldReader;
#[doc = "Field `padc_ictrl_pga_buf` writer - "]
pub type PadcIctrlPgaBufW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `padc_ictrl_pga_ia` reader - "]
pub type PadcIctrlPgaIaR = crate::FieldReader;
#[doc = "Field `padc_ictrl_pga_ia` writer - "]
pub type PadcIctrlPgaIaW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `padc_pga_force` reader - "]
pub type PadcPgaForceR = crate::BitReader;
#[doc = "Field `padc_pga_force` writer - "]
pub type PadcPgaForceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `padc_pga_autobps_buf` reader - "]
pub type PadcPgaAutobpsBufR = crate::BitReader;
#[doc = "Field `padc_pga_autobps_buf` writer - "]
pub type PadcPgaAutobpsBufW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `padc_pga_autobps_ia` reader - "]
pub type PadcPgaAutobpsIaR = crate::BitReader;
#[doc = "Field `padc_pga_autobps_ia` writer - "]
pub type PadcPgaAutobpsIaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `padc_pga_mode` reader - "]
pub type PadcPgaModeR = crate::FieldReader;
#[doc = "Field `padc_pga_mode` writer - "]
pub type PadcPgaModeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `padc_afevcm_rout` reader - "]
pub type PadcAfevcmRoutR = crate::FieldReader;
#[doc = "Field `padc_afevcm_rout` writer - "]
pub type PadcAfevcmRoutW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `padc_afevcm_sel` reader - "]
pub type PadcAfevcmSelR = crate::BitReader;
#[doc = "Field `padc_afevcm_sel` writer - "]
pub type PadcAfevcmSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `padc_ckb_en` reader - "]
pub type PadcCkbEnR = crate::BitReader;
#[doc = "Field `padc_ckb_en` writer - "]
pub type PadcCkbEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `padc_sel_edge` reader - "]
pub type PadcSelEdgeR = crate::BitReader;
#[doc = "Field `padc_sel_edge` writer - "]
pub type PadcSelEdgeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn padc_pga_chop_cksel(&self) -> PadcPgaChopCkselR {
        PadcPgaChopCkselR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3"]
    #[inline(always)]
    pub fn padc_pga_chop_freq(&self) -> PadcPgaChopFreqR {
        PadcPgaChopFreqR::new(((self.bits >> 1) & 7) as u8)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn padc_pga_chop_en(&self) -> PadcPgaChopEnR {
        PadcPgaChopEnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6"]
    #[inline(always)]
    pub fn padc_pga_noisctrl(&self) -> PadcPgaNoisctrlR {
        PadcPgaNoisctrlR::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bits 7:8"]
    #[inline(always)]
    pub fn padc_ictrl_pga_buf(&self) -> PadcIctrlPgaBufR {
        PadcIctrlPgaBufR::new(((self.bits >> 7) & 3) as u8)
    }
    #[doc = "Bits 9:10"]
    #[inline(always)]
    pub fn padc_ictrl_pga_ia(&self) -> PadcIctrlPgaIaR {
        PadcIctrlPgaIaR::new(((self.bits >> 9) & 3) as u8)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn padc_pga_force(&self) -> PadcPgaForceR {
        PadcPgaForceR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn padc_pga_autobps_buf(&self) -> PadcPgaAutobpsBufR {
        PadcPgaAutobpsBufR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn padc_pga_autobps_ia(&self) -> PadcPgaAutobpsIaR {
        PadcPgaAutobpsIaR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn padc_pga_mode(&self) -> PadcPgaModeR {
        PadcPgaModeR::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn padc_afevcm_rout(&self) -> PadcAfevcmRoutR {
        PadcAfevcmRoutR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn padc_afevcm_sel(&self) -> PadcAfevcmSelR {
        PadcAfevcmSelR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn padc_ckb_en(&self) -> PadcCkbEnR {
        PadcCkbEnR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn padc_sel_edge(&self) -> PadcSelEdgeR {
        PadcSelEdgeR::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn padc_pga_chop_cksel(&mut self) -> PadcPgaChopCkselW<'_, PadcAnaCfg1Spec> {
        PadcPgaChopCkselW::new(self, 0)
    }
    #[doc = "Bits 1:3"]
    #[inline(always)]
    pub fn padc_pga_chop_freq(&mut self) -> PadcPgaChopFreqW<'_, PadcAnaCfg1Spec> {
        PadcPgaChopFreqW::new(self, 1)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn padc_pga_chop_en(&mut self) -> PadcPgaChopEnW<'_, PadcAnaCfg1Spec> {
        PadcPgaChopEnW::new(self, 4)
    }
    #[doc = "Bits 5:6"]
    #[inline(always)]
    pub fn padc_pga_noisctrl(&mut self) -> PadcPgaNoisctrlW<'_, PadcAnaCfg1Spec> {
        PadcPgaNoisctrlW::new(self, 5)
    }
    #[doc = "Bits 7:8"]
    #[inline(always)]
    pub fn padc_ictrl_pga_buf(&mut self) -> PadcIctrlPgaBufW<'_, PadcAnaCfg1Spec> {
        PadcIctrlPgaBufW::new(self, 7)
    }
    #[doc = "Bits 9:10"]
    #[inline(always)]
    pub fn padc_ictrl_pga_ia(&mut self) -> PadcIctrlPgaIaW<'_, PadcAnaCfg1Spec> {
        PadcIctrlPgaIaW::new(self, 9)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn padc_pga_force(&mut self) -> PadcPgaForceW<'_, PadcAnaCfg1Spec> {
        PadcPgaForceW::new(self, 11)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn padc_pga_autobps_buf(&mut self) -> PadcPgaAutobpsBufW<'_, PadcAnaCfg1Spec> {
        PadcPgaAutobpsBufW::new(self, 12)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn padc_pga_autobps_ia(&mut self) -> PadcPgaAutobpsIaW<'_, PadcAnaCfg1Spec> {
        PadcPgaAutobpsIaW::new(self, 13)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn padc_pga_mode(&mut self) -> PadcPgaModeW<'_, PadcAnaCfg1Spec> {
        PadcPgaModeW::new(self, 14)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn padc_afevcm_rout(&mut self) -> PadcAfevcmRoutW<'_, PadcAnaCfg1Spec> {
        PadcAfevcmRoutW::new(self, 16)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn padc_afevcm_sel(&mut self) -> PadcAfevcmSelW<'_, PadcAnaCfg1Spec> {
        PadcAfevcmSelW::new(self, 18)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn padc_ckb_en(&mut self) -> PadcCkbEnW<'_, PadcAnaCfg1Spec> {
        PadcCkbEnW::new(self, 19)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn padc_sel_edge(&mut self) -> PadcSelEdgeW<'_, PadcAnaCfg1Spec> {
        PadcSelEdgeW::new(self, 20)
    }
}
#[doc = "Passthrough ADC Analog Config 1: Chopping and bias configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`padc_ana_cfg1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`padc_ana_cfg1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PadcAnaCfg1Spec;
impl crate::RegisterSpec for PadcAnaCfg1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`padc_ana_cfg1::R`](R) reader structure"]
impl crate::Readable for PadcAnaCfg1Spec {}
#[doc = "`write(|w| ..)` method takes [`padc_ana_cfg1::W`](W) writer structure"]
impl crate::Writable for PadcAnaCfg1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets padc_ana_cfg1 to value 0"]
impl crate::Resettable for PadcAnaCfg1Spec {}
