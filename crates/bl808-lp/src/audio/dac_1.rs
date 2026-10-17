#[doc = "Register `dac_1` reader"]
pub type R = crate::R<Dac1Spec>;
#[doc = "Register `dac_1` writer"]
pub type W = crate::W<Dac1Spec>;
#[doc = "Field `dac_dem_bypass` reader - "]
pub type DacDemBypassR = crate::BitReader;
#[doc = "Field `dac_dem_bypass` writer - "]
pub type DacDemBypassW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dac_dem_out_swap` reader - "]
pub type DacDemOutSwapR = crate::BitReader;
#[doc = "Field `dac_dem_out_swap` writer - "]
pub type DacDemOutSwapW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dac_dsm_order` reader - "]
pub type DacDsmOrderR = crate::BitReader;
#[doc = "Field `dac_dsm_order` writer - "]
pub type DacDsmOrderW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dac_dsm_scaling_factor` reader - "]
pub type DacDsmScalingFactorR = crate::FieldReader;
#[doc = "Field `dac_dsm_scaling_factor` writer - "]
pub type DacDsmScalingFactorW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `dac_dsm_scaling_en` reader - "]
pub type DacDsmScalingEnR = crate::BitReader;
#[doc = "Field `dac_dsm_scaling_en` writer - "]
pub type DacDsmScalingEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dac_dsm_dither_amp` reader - "]
pub type DacDsmDitherAmpR = crate::FieldReader;
#[doc = "Field `dac_dsm_dither_amp` writer - "]
pub type DacDsmDitherAmpW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `dac_dsm_dither_en` reader - "]
pub type DacDsmDitherEnR = crate::BitReader;
#[doc = "Field `dac_dsm_dither_en` writer - "]
pub type DacDsmDitherEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dac_dsm_dither_prbs_mode` reader - "]
pub type DacDsmDitherPrbsModeR = crate::FieldReader;
#[doc = "Field `dac_dsm_dither_prbs_mode` writer - "]
pub type DacDsmDitherPrbsModeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `mix_1_sel` reader - "]
pub type Mix1SelR = crate::FieldReader;
#[doc = "Field `mix_1_sel` writer - "]
pub type Mix1SelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `mix_1_mode` reader - "]
pub type Mix1ModeR = crate::FieldReader;
#[doc = "Field `mix_1_mode` writer - "]
pub type Mix1ModeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `mix_1_att_mode1` reader - "]
pub type Mix1AttMode1R = crate::FieldReader;
#[doc = "Field `mix_1_att_mode1` writer - "]
pub type Mix1AttMode1W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `mix_1_att_mode2` reader - "]
pub type Mix1AttMode2R = crate::FieldReader;
#[doc = "Field `mix_1_att_mode2` writer - "]
pub type Mix1AttMode2W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn dac_dem_bypass(&self) -> DacDemBypassR {
        DacDemBypassR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn dac_dem_out_swap(&self) -> DacDemOutSwapR {
        DacDemOutSwapR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn dac_dsm_order(&self) -> DacDsmOrderR {
        DacDsmOrderR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:9"]
    #[inline(always)]
    pub fn dac_dsm_scaling_factor(&self) -> DacDsmScalingFactorR {
        DacDsmScalingFactorR::new(((self.bits >> 6) & 0x0f) as u8)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn dac_dsm_scaling_en(&self) -> DacDsmScalingEnR {
        DacDsmScalingEnR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:13"]
    #[inline(always)]
    pub fn dac_dsm_dither_amp(&self) -> DacDsmDitherAmpR {
        DacDsmDitherAmpR::new(((self.bits >> 11) & 7) as u8)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn dac_dsm_dither_en(&self) -> DacDsmDitherEnR {
        DacDsmDitherEnR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 15:16"]
    #[inline(always)]
    pub fn dac_dsm_dither_prbs_mode(&self) -> DacDsmDitherPrbsModeR {
        DacDsmDitherPrbsModeR::new(((self.bits >> 15) & 3) as u8)
    }
    #[doc = "Bits 21:22"]
    #[inline(always)]
    pub fn mix_1_sel(&self) -> Mix1SelR {
        Mix1SelR::new(((self.bits >> 21) & 3) as u8)
    }
    #[doc = "Bits 23:24"]
    #[inline(always)]
    pub fn mix_1_mode(&self) -> Mix1ModeR {
        Mix1ModeR::new(((self.bits >> 23) & 3) as u8)
    }
    #[doc = "Bits 25:27"]
    #[inline(always)]
    pub fn mix_1_att_mode1(&self) -> Mix1AttMode1R {
        Mix1AttMode1R::new(((self.bits >> 25) & 7) as u8)
    }
    #[doc = "Bits 28:30"]
    #[inline(always)]
    pub fn mix_1_att_mode2(&self) -> Mix1AttMode2R {
        Mix1AttMode2R::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn dac_dem_bypass(&mut self) -> DacDemBypassW<'_, Dac1Spec> {
        DacDemBypassW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn dac_dem_out_swap(&mut self) -> DacDemOutSwapW<'_, Dac1Spec> {
        DacDemOutSwapW::new(self, 1)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn dac_dsm_order(&mut self) -> DacDsmOrderW<'_, Dac1Spec> {
        DacDsmOrderW::new(self, 5)
    }
    #[doc = "Bits 6:9"]
    #[inline(always)]
    pub fn dac_dsm_scaling_factor(&mut self) -> DacDsmScalingFactorW<'_, Dac1Spec> {
        DacDsmScalingFactorW::new(self, 6)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn dac_dsm_scaling_en(&mut self) -> DacDsmScalingEnW<'_, Dac1Spec> {
        DacDsmScalingEnW::new(self, 10)
    }
    #[doc = "Bits 11:13"]
    #[inline(always)]
    pub fn dac_dsm_dither_amp(&mut self) -> DacDsmDitherAmpW<'_, Dac1Spec> {
        DacDsmDitherAmpW::new(self, 11)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn dac_dsm_dither_en(&mut self) -> DacDsmDitherEnW<'_, Dac1Spec> {
        DacDsmDitherEnW::new(self, 14)
    }
    #[doc = "Bits 15:16"]
    #[inline(always)]
    pub fn dac_dsm_dither_prbs_mode(&mut self) -> DacDsmDitherPrbsModeW<'_, Dac1Spec> {
        DacDsmDitherPrbsModeW::new(self, 15)
    }
    #[doc = "Bits 21:22"]
    #[inline(always)]
    pub fn mix_1_sel(&mut self) -> Mix1SelW<'_, Dac1Spec> {
        Mix1SelW::new(self, 21)
    }
    #[doc = "Bits 23:24"]
    #[inline(always)]
    pub fn mix_1_mode(&mut self) -> Mix1ModeW<'_, Dac1Spec> {
        Mix1ModeW::new(self, 23)
    }
    #[doc = "Bits 25:27"]
    #[inline(always)]
    pub fn mix_1_att_mode1(&mut self) -> Mix1AttMode1W<'_, Dac1Spec> {
        Mix1AttMode1W::new(self, 25)
    }
    #[doc = "Bits 28:30"]
    #[inline(always)]
    pub fn mix_1_att_mode2(&mut self) -> Mix1AttMode2W<'_, Dac1Spec> {
        Mix1AttMode2W::new(self, 28)
    }
}
#[doc = "DAC Configuration 1: Modulation and order settings\n\nYou can [`read`](crate::Reg::read) this register and get [`dac_1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dac_1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dac1Spec;
impl crate::RegisterSpec for Dac1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dac_1::R`](R) reader structure"]
impl crate::Readable for Dac1Spec {}
#[doc = "`write(|w| ..)` method takes [`dac_1::W`](W) writer structure"]
impl crate::Writable for Dac1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets dac_1 to value 0"]
impl crate::Resettable for Dac1Spec {}
