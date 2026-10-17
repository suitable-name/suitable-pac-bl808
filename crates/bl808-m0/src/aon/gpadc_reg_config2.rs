#[doc = "Register `GPADC_REG_CONFIG2` reader"]
pub type R = crate::R<GpadcRegConfig2Spec>;
#[doc = "Register `GPADC_REG_CONFIG2` writer"]
pub type W = crate::W<GpadcRegConfig2Spec>;
#[doc = "Field `gpadc_diff_mode` reader - "]
pub type GpadcDiffModeR = crate::BitReader;
#[doc = "Field `gpadc_diff_mode` writer - "]
pub type GpadcDiffModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `gpadc_vref_sel` reader - "]
pub type GpadcVrefSelR = crate::BitReader;
#[doc = "Field `gpadc_vref_sel` writer - "]
pub type GpadcVrefSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `gpadc_vbat_en` reader - "]
pub type GpadcVbatEnR = crate::BitReader;
#[doc = "Field `gpadc_vbat_en` writer - "]
pub type GpadcVbatEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `gpadc_tsext_sel` reader - "]
pub type GpadcTsextSelR = crate::BitReader;
#[doc = "Field `gpadc_tsext_sel` writer - "]
pub type GpadcTsextSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `gpadc_ts_en` reader - "]
pub type GpadcTsEnR = crate::BitReader;
#[doc = "Field `gpadc_ts_en` writer - "]
pub type GpadcTsEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `gpadc_pga_vcm` reader - "]
pub type GpadcPgaVcmR = crate::FieldReader;
#[doc = "Field `gpadc_pga_vcm` writer - "]
pub type GpadcPgaVcmW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `gpadc_pga_os_cal` reader - "]
pub type GpadcPgaOsCalR = crate::FieldReader;
#[doc = "Field `gpadc_pga_os_cal` writer - "]
pub type GpadcPgaOsCalW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `gpadc_pga_en` reader - "]
pub type GpadcPgaEnR = crate::BitReader;
#[doc = "Field `gpadc_pga_en` writer - "]
pub type GpadcPgaEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `gpadc_pga_vcmi_en` reader - "]
pub type GpadcPgaVcmiEnR = crate::BitReader;
#[doc = "Field `gpadc_pga_vcmi_en` writer - "]
pub type GpadcPgaVcmiEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `gpadc_chop_mode` reader - "]
pub type GpadcChopModeR = crate::FieldReader;
#[doc = "Field `gpadc_chop_mode` writer - "]
pub type GpadcChopModeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `gpadc_bias_sel` reader - "]
pub type GpadcBiasSelR = crate::BitReader;
#[doc = "Field `gpadc_bias_sel` writer - "]
pub type GpadcBiasSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `gpadc_test_en` reader - "]
pub type GpadcTestEnR = crate::BitReader;
#[doc = "Field `gpadc_test_en` writer - "]
pub type GpadcTestEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `gpadc_test_sel` reader - "]
pub type GpadcTestSelR = crate::FieldReader;
#[doc = "Field `gpadc_test_sel` writer - "]
pub type GpadcTestSelW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `gpadc_pga2_gain` reader - "]
pub type GpadcPga2GainR = crate::FieldReader;
#[doc = "Field `gpadc_pga2_gain` writer - "]
pub type GpadcPga2GainW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `gpadc_pga1_gain` reader - "]
pub type GpadcPga1GainR = crate::FieldReader;
#[doc = "Field `gpadc_pga1_gain` writer - "]
pub type GpadcPga1GainW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `gpadc_dly_sel` reader - "]
pub type GpadcDlySelR = crate::FieldReader;
#[doc = "Field `gpadc_dly_sel` writer - "]
pub type GpadcDlySelW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `gpadc_tsvbe_low` reader - "]
pub type GpadcTsvbeLowR = crate::BitReader;
#[doc = "Field `gpadc_tsvbe_low` writer - "]
pub type GpadcTsvbeLowW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn gpadc_diff_mode(&self) -> GpadcDiffModeR {
        GpadcDiffModeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn gpadc_vref_sel(&self) -> GpadcVrefSelR {
        GpadcVrefSelR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn gpadc_vbat_en(&self) -> GpadcVbatEnR {
        GpadcVbatEnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn gpadc_tsext_sel(&self) -> GpadcTsextSelR {
        GpadcTsextSelR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn gpadc_ts_en(&self) -> GpadcTsEnR {
        GpadcTsEnR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 7:8"]
    #[inline(always)]
    pub fn gpadc_pga_vcm(&self) -> GpadcPgaVcmR {
        GpadcPgaVcmR::new(((self.bits >> 7) & 3) as u8)
    }
    #[doc = "Bits 9:12"]
    #[inline(always)]
    pub fn gpadc_pga_os_cal(&self) -> GpadcPgaOsCalR {
        GpadcPgaOsCalR::new(((self.bits >> 9) & 0x0f) as u8)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn gpadc_pga_en(&self) -> GpadcPgaEnR {
        GpadcPgaEnR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn gpadc_pga_vcmi_en(&self) -> GpadcPgaVcmiEnR {
        GpadcPgaVcmiEnR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 15:16"]
    #[inline(always)]
    pub fn gpadc_chop_mode(&self) -> GpadcChopModeR {
        GpadcChopModeR::new(((self.bits >> 15) & 3) as u8)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn gpadc_bias_sel(&self) -> GpadcBiasSelR {
        GpadcBiasSelR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn gpadc_test_en(&self) -> GpadcTestEnR {
        GpadcTestEnR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 19:21"]
    #[inline(always)]
    pub fn gpadc_test_sel(&self) -> GpadcTestSelR {
        GpadcTestSelR::new(((self.bits >> 19) & 7) as u8)
    }
    #[doc = "Bits 22:24"]
    #[inline(always)]
    pub fn gpadc_pga2_gain(&self) -> GpadcPga2GainR {
        GpadcPga2GainR::new(((self.bits >> 22) & 7) as u8)
    }
    #[doc = "Bits 25:27"]
    #[inline(always)]
    pub fn gpadc_pga1_gain(&self) -> GpadcPga1GainR {
        GpadcPga1GainR::new(((self.bits >> 25) & 7) as u8)
    }
    #[doc = "Bits 28:30"]
    #[inline(always)]
    pub fn gpadc_dly_sel(&self) -> GpadcDlySelR {
        GpadcDlySelR::new(((self.bits >> 28) & 7) as u8)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn gpadc_tsvbe_low(&self) -> GpadcTsvbeLowR {
        GpadcTsvbeLowR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn gpadc_diff_mode(&mut self) -> GpadcDiffModeW<'_, GpadcRegConfig2Spec> {
        GpadcDiffModeW::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn gpadc_vref_sel(&mut self) -> GpadcVrefSelW<'_, GpadcRegConfig2Spec> {
        GpadcVrefSelW::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn gpadc_vbat_en(&mut self) -> GpadcVbatEnW<'_, GpadcRegConfig2Spec> {
        GpadcVbatEnW::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn gpadc_tsext_sel(&mut self) -> GpadcTsextSelW<'_, GpadcRegConfig2Spec> {
        GpadcTsextSelW::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn gpadc_ts_en(&mut self) -> GpadcTsEnW<'_, GpadcRegConfig2Spec> {
        GpadcTsEnW::new(self, 6)
    }
    #[doc = "Bits 7:8"]
    #[inline(always)]
    pub fn gpadc_pga_vcm(&mut self) -> GpadcPgaVcmW<'_, GpadcRegConfig2Spec> {
        GpadcPgaVcmW::new(self, 7)
    }
    #[doc = "Bits 9:12"]
    #[inline(always)]
    pub fn gpadc_pga_os_cal(&mut self) -> GpadcPgaOsCalW<'_, GpadcRegConfig2Spec> {
        GpadcPgaOsCalW::new(self, 9)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn gpadc_pga_en(&mut self) -> GpadcPgaEnW<'_, GpadcRegConfig2Spec> {
        GpadcPgaEnW::new(self, 13)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn gpadc_pga_vcmi_en(&mut self) -> GpadcPgaVcmiEnW<'_, GpadcRegConfig2Spec> {
        GpadcPgaVcmiEnW::new(self, 14)
    }
    #[doc = "Bits 15:16"]
    #[inline(always)]
    pub fn gpadc_chop_mode(&mut self) -> GpadcChopModeW<'_, GpadcRegConfig2Spec> {
        GpadcChopModeW::new(self, 15)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn gpadc_bias_sel(&mut self) -> GpadcBiasSelW<'_, GpadcRegConfig2Spec> {
        GpadcBiasSelW::new(self, 17)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn gpadc_test_en(&mut self) -> GpadcTestEnW<'_, GpadcRegConfig2Spec> {
        GpadcTestEnW::new(self, 18)
    }
    #[doc = "Bits 19:21"]
    #[inline(always)]
    pub fn gpadc_test_sel(&mut self) -> GpadcTestSelW<'_, GpadcRegConfig2Spec> {
        GpadcTestSelW::new(self, 19)
    }
    #[doc = "Bits 22:24"]
    #[inline(always)]
    pub fn gpadc_pga2_gain(&mut self) -> GpadcPga2GainW<'_, GpadcRegConfig2Spec> {
        GpadcPga2GainW::new(self, 22)
    }
    #[doc = "Bits 25:27"]
    #[inline(always)]
    pub fn gpadc_pga1_gain(&mut self) -> GpadcPga1GainW<'_, GpadcRegConfig2Spec> {
        GpadcPga1GainW::new(self, 25)
    }
    #[doc = "Bits 28:30"]
    #[inline(always)]
    pub fn gpadc_dly_sel(&mut self) -> GpadcDlySelW<'_, GpadcRegConfig2Spec> {
        GpadcDlySelW::new(self, 28)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn gpadc_tsvbe_low(&mut self) -> GpadcTsvbeLowW<'_, GpadcRegConfig2Spec> {
        GpadcTsvbeLowW::new(self, 31)
    }
}
#[doc = "GPADC Configuration Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`gpadc_reg_config2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpadc_reg_config2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpadcRegConfig2Spec;
impl crate::RegisterSpec for GpadcRegConfig2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpadc_reg_config2::R`](R) reader structure"]
impl crate::Readable for GpadcRegConfig2Spec {}
#[doc = "`write(|w| ..)` method takes [`gpadc_reg_config2::W`](W) writer structure"]
impl crate::Writable for GpadcRegConfig2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPADC_REG_CONFIG2 to value 0"]
impl crate::Resettable for GpadcRegConfig2Spec {}
