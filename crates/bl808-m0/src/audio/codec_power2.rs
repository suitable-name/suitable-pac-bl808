#[doc = "Register `codec_power2` reader"]
pub type R = crate::R<CodecPower2Spec>;
#[doc = "Register `codec_power2` writer"]
pub type W = crate::W<CodecPower2Spec>;
#[doc = "Field `hpbg_chop_sel` reader - "]
pub type HpbgChopSelR = crate::FieldReader;
#[doc = "Field `hpbg_chop_sel` writer - "]
pub type HpbgChopSelW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `hpbg_chop_en` reader - "]
pub type HpbgChopEnR = crate::BitReader;
#[doc = "Field `hpbg_chop_en` writer - "]
pub type HpbgChopEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `hpbg_pu` reader - "]
pub type HpbgPuR = crate::BitReader;
#[doc = "Field `hpbg_pu` writer - "]
pub type HpbgPuW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `hpbg_filt_rsel` reader - "]
pub type HpbgFiltRselR = crate::FieldReader;
#[doc = "Field `hpbg_filt_rsel` writer - "]
pub type HpbgFiltRselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `hpbg_bypass` reader - "]
pub type HpbgBypassR = crate::BitReader;
#[doc = "Field `hpbg_bypass` writer - "]
pub type HpbgBypassW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `hpbg_adjust_fine` reader - "]
pub type HpbgAdjustFineR = crate::FieldReader;
#[doc = "Field `hpbg_adjust_fine` writer - "]
pub type HpbgAdjustFineW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `hpbg_adjust_coarse` reader - "]
pub type HpbgAdjustCoarseR = crate::FieldReader;
#[doc = "Field `hpbg_adjust_coarse` writer - "]
pub type HpbgAdjustCoarseW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `hpbg_compst_en` reader - "]
pub type HpbgCompstEnR = crate::BitReader;
#[doc = "Field `hpbg_compst_en` writer - "]
pub type HpbgCompstEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ldo25_sstart_en` reader - "]
pub type Ldo25SstartEnR = crate::BitReader;
#[doc = "Field `ldo25_sstart_en` writer - "]
pub type Ldo25SstartEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ldo25_sstart_delay` reader - "]
pub type Ldo25SstartDelayR = crate::FieldReader;
#[doc = "Field `ldo25_sstart_delay` writer - "]
pub type Ldo25SstartDelayW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ldo25_pulldown_sel` reader - "]
pub type Ldo25PulldownSelR = crate::BitReader;
#[doc = "Field `ldo25_pulldown_sel` writer - "]
pub type Ldo25PulldownSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ldo25_pulldown` reader - "]
pub type Ldo25PulldownR = crate::BitReader;
#[doc = "Field `ldo25_pulldown` writer - "]
pub type Ldo25PulldownW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ldo25_pu` reader - "]
pub type Ldo25PuR = crate::BitReader;
#[doc = "Field `ldo25_pu` writer - "]
pub type Ldo25PuW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ldo25_chop_sel` reader - "]
pub type Ldo25ChopSelR = crate::FieldReader;
#[doc = "Field `ldo25_chop_sel` writer - "]
pub type Ldo25ChopSelW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `ldo25_chop_en` reader - "]
pub type Ldo25ChopEnR = crate::BitReader;
#[doc = "Field `ldo25_chop_en` writer - "]
pub type Ldo25ChopEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ldo25_cc_sel` reader - "]
pub type Ldo25CcSelR = crate::FieldReader;
#[doc = "Field `ldo25_cc_sel` writer - "]
pub type Ldo25CcSelW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn hpbg_chop_sel(&self) -> HpbgChopSelR {
        HpbgChopSelR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn hpbg_chop_en(&self) -> HpbgChopEnR {
        HpbgChopEnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn hpbg_pu(&self) -> HpbgPuR {
        HpbgPuR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6"]
    #[inline(always)]
    pub fn hpbg_filt_rsel(&self) -> HpbgFiltRselR {
        HpbgFiltRselR::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn hpbg_bypass(&self) -> HpbgBypassR {
        HpbgBypassR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn hpbg_adjust_fine(&self) -> HpbgAdjustFineR {
        HpbgAdjustFineR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn hpbg_adjust_coarse(&self) -> HpbgAdjustCoarseR {
        HpbgAdjustCoarseR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn hpbg_compst_en(&self) -> HpbgCompstEnR {
        HpbgCompstEnR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn ldo25_sstart_en(&self) -> Ldo25SstartEnR {
        Ldo25SstartEnR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    pub fn ldo25_sstart_delay(&self) -> Ldo25SstartDelayR {
        Ldo25SstartDelayR::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn ldo25_pulldown_sel(&self) -> Ldo25PulldownSelR {
        Ldo25PulldownSelR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn ldo25_pulldown(&self) -> Ldo25PulldownR {
        Ldo25PulldownR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn ldo25_pu(&self) -> Ldo25PuR {
        Ldo25PuR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bits 24:26"]
    #[inline(always)]
    pub fn ldo25_chop_sel(&self) -> Ldo25ChopSelR {
        Ldo25ChopSelR::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn ldo25_chop_en(&self) -> Ldo25ChopEnR {
        Ldo25ChopEnR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:30"]
    #[inline(always)]
    pub fn ldo25_cc_sel(&self) -> Ldo25CcSelR {
        Ldo25CcSelR::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn hpbg_chop_sel(&mut self) -> HpbgChopSelW<'_, CodecPower2Spec> {
        HpbgChopSelW::new(self, 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn hpbg_chop_en(&mut self) -> HpbgChopEnW<'_, CodecPower2Spec> {
        HpbgChopEnW::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn hpbg_pu(&mut self) -> HpbgPuW<'_, CodecPower2Spec> {
        HpbgPuW::new(self, 4)
    }
    #[doc = "Bits 5:6"]
    #[inline(always)]
    pub fn hpbg_filt_rsel(&mut self) -> HpbgFiltRselW<'_, CodecPower2Spec> {
        HpbgFiltRselW::new(self, 5)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn hpbg_bypass(&mut self) -> HpbgBypassW<'_, CodecPower2Spec> {
        HpbgBypassW::new(self, 7)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn hpbg_adjust_fine(&mut self) -> HpbgAdjustFineW<'_, CodecPower2Spec> {
        HpbgAdjustFineW::new(self, 8)
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn hpbg_adjust_coarse(&mut self) -> HpbgAdjustCoarseW<'_, CodecPower2Spec> {
        HpbgAdjustCoarseW::new(self, 12)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn hpbg_compst_en(&mut self) -> HpbgCompstEnW<'_, CodecPower2Spec> {
        HpbgCompstEnW::new(self, 16)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn ldo25_sstart_en(&mut self) -> Ldo25SstartEnW<'_, CodecPower2Spec> {
        Ldo25SstartEnW::new(self, 17)
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    pub fn ldo25_sstart_delay(&mut self) -> Ldo25SstartDelayW<'_, CodecPower2Spec> {
        Ldo25SstartDelayW::new(self, 18)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn ldo25_pulldown_sel(&mut self) -> Ldo25PulldownSelW<'_, CodecPower2Spec> {
        Ldo25PulldownSelW::new(self, 20)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn ldo25_pulldown(&mut self) -> Ldo25PulldownW<'_, CodecPower2Spec> {
        Ldo25PulldownW::new(self, 21)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn ldo25_pu(&mut self) -> Ldo25PuW<'_, CodecPower2Spec> {
        Ldo25PuW::new(self, 22)
    }
    #[doc = "Bits 24:26"]
    #[inline(always)]
    pub fn ldo25_chop_sel(&mut self) -> Ldo25ChopSelW<'_, CodecPower2Spec> {
        Ldo25ChopSelW::new(self, 24)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn ldo25_chop_en(&mut self) -> Ldo25ChopEnW<'_, CodecPower2Spec> {
        Ldo25ChopEnW::new(self, 27)
    }
    #[doc = "Bits 28:30"]
    #[inline(always)]
    pub fn ldo25_cc_sel(&mut self) -> Ldo25CcSelW<'_, CodecPower2Spec> {
        Ldo25CcSelW::new(self, 28)
    }
}
#[doc = "Analog Codec Power Control 2: Bandgap and LDO settings\n\nYou can [`read`](crate::Reg::read) this register and get [`codec_power2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`codec_power2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CodecPower2Spec;
impl crate::RegisterSpec for CodecPower2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`codec_power2::R`](R) reader structure"]
impl crate::Readable for CodecPower2Spec {}
#[doc = "`write(|w| ..)` method takes [`codec_power2::W`](W) writer structure"]
impl crate::Writable for CodecPower2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets codec_power2 to value 0"]
impl crate::Resettable for CodecPower2Spec {}
