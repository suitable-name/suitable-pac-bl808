#[doc = "Register `dac_drc_1` reader"]
pub type R = crate::R<DacDrc1Spec>;
#[doc = "Register `dac_drc_1` writer"]
pub type W = crate::W<DacDrc1Spec>;
#[doc = "Field `drc_release_time` reader - "]
pub type DrcReleaseTimeR = crate::FieldReader;
#[doc = "Field `drc_release_time` writer - "]
pub type DrcReleaseTimeW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `drc_attack_time` reader - "]
pub type DrcAttackTimeR = crate::FieldReader;
#[doc = "Field `drc_attack_time` writer - "]
pub type DrcAttackTimeW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `drc_makeup_gain` reader - "]
pub type DrcMakeupGainR = crate::FieldReader;
#[doc = "Field `drc_makeup_gain` writer - "]
pub type DrcMakeupGainW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `drc_slope_mode_e` reader - "]
pub type DrcSlopeModeER = crate::FieldReader;
#[doc = "Field `drc_slope_mode_e` writer - "]
pub type DrcSlopeModeEW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `drc_slope_mode_c` reader - "]
pub type DrcSlopeModeCR = crate::FieldReader;
#[doc = "Field `drc_slope_mode_c` writer - "]
pub type DrcSlopeModeCW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `drc_threshold_e` reader - "]
pub type DrcThresholdER = crate::FieldReader;
#[doc = "Field `drc_threshold_e` writer - "]
pub type DrcThresholdEW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `drc_threshold_c` reader - "]
pub type DrcThresholdCR = crate::FieldReader;
#[doc = "Field `drc_threshold_c` writer - "]
pub type DrcThresholdCW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `drc_threshold_l` reader - "]
pub type DrcThresholdLR = crate::FieldReader;
#[doc = "Field `drc_threshold_l` writer - "]
pub type DrcThresholdLW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn drc_release_time(&self) -> DrcReleaseTimeR {
        DrcReleaseTimeR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn drc_attack_time(&self) -> DrcAttackTimeR {
        DrcAttackTimeR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn drc_makeup_gain(&self) -> DrcMakeupGainR {
        DrcMakeupGainR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn drc_slope_mode_e(&self) -> DrcSlopeModeER {
        DrcSlopeModeER::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn drc_slope_mode_c(&self) -> DrcSlopeModeCR {
        DrcSlopeModeCR::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:21"]
    #[inline(always)]
    pub fn drc_threshold_e(&self) -> DrcThresholdER {
        DrcThresholdER::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 22:26"]
    #[inline(always)]
    pub fn drc_threshold_c(&self) -> DrcThresholdCR {
        DrcThresholdCR::new(((self.bits >> 22) & 0x1f) as u8)
    }
    #[doc = "Bits 27:31"]
    #[inline(always)]
    pub fn drc_threshold_l(&self) -> DrcThresholdLR {
        DrcThresholdLR::new(((self.bits >> 27) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn drc_release_time(&mut self) -> DrcReleaseTimeW<'_, DacDrc1Spec> {
        DrcReleaseTimeW::new(self, 0)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn drc_attack_time(&mut self) -> DrcAttackTimeW<'_, DacDrc1Spec> {
        DrcAttackTimeW::new(self, 4)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn drc_makeup_gain(&mut self) -> DrcMakeupGainW<'_, DacDrc1Spec> {
        DrcMakeupGainW::new(self, 8)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn drc_slope_mode_e(&mut self) -> DrcSlopeModeEW<'_, DacDrc1Spec> {
        DrcSlopeModeEW::new(self, 12)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn drc_slope_mode_c(&mut self) -> DrcSlopeModeCW<'_, DacDrc1Spec> {
        DrcSlopeModeCW::new(self, 14)
    }
    #[doc = "Bits 16:21"]
    #[inline(always)]
    pub fn drc_threshold_e(&mut self) -> DrcThresholdEW<'_, DacDrc1Spec> {
        DrcThresholdEW::new(self, 16)
    }
    #[doc = "Bits 22:26"]
    #[inline(always)]
    pub fn drc_threshold_c(&mut self) -> DrcThresholdCW<'_, DacDrc1Spec> {
        DrcThresholdCW::new(self, 22)
    }
    #[doc = "Bits 27:31"]
    #[inline(always)]
    pub fn drc_threshold_l(&mut self) -> DrcThresholdLW<'_, DacDrc1Spec> {
        DrcThresholdLW::new(self, 27)
    }
}
#[doc = "DAC Dynamic Range Compression 1: Slope and threshold configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`dac_drc_1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dac_drc_1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DacDrc1Spec;
impl crate::RegisterSpec for DacDrc1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dac_drc_1::R`](R) reader structure"]
impl crate::Readable for DacDrc1Spec {}
#[doc = "`write(|w| ..)` method takes [`dac_drc_1::W`](W) writer structure"]
impl crate::Writable for DacDrc1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets dac_drc_1 to value 0"]
impl crate::Resettable for DacDrc1Spec {}
