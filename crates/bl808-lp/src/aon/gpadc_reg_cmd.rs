#[doc = "Register `GPADC_REG_CMD` reader"]
pub type R = crate::R<GpadcRegCmdSpec>;
#[doc = "Register `GPADC_REG_CMD` writer"]
pub type W = crate::W<GpadcRegCmdSpec>;
#[doc = "Field `gpadc_global_en` reader - "]
pub type GpadcGlobalEnR = crate::BitReader;
#[doc = "Field `gpadc_global_en` writer - "]
pub type GpadcGlobalEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `gpadc_conv_start` reader - "]
pub type GpadcConvStartR = crate::BitReader;
#[doc = "Field `gpadc_conv_start` writer - "]
pub type GpadcConvStartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `gpadc_soft_rst` reader - "]
pub type GpadcSoftRstR = crate::BitReader;
#[doc = "Field `gpadc_soft_rst` writer - "]
pub type GpadcSoftRstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `gpadc_neg_sel` reader - "]
pub type GpadcNegSelR = crate::FieldReader;
#[doc = "Field `gpadc_neg_sel` writer - "]
pub type GpadcNegSelW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `gpadc_pos_sel` reader - "]
pub type GpadcPosSelR = crate::FieldReader;
#[doc = "Field `gpadc_pos_sel` writer - "]
pub type GpadcPosSelW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `gpadc_neg_gnd` reader - "]
pub type GpadcNegGndR = crate::BitReader;
#[doc = "Field `gpadc_neg_gnd` writer - "]
pub type GpadcNegGndW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `gpadc_micbias_en` reader - "]
pub type GpadcMicbiasEnR = crate::BitReader;
#[doc = "Field `gpadc_micbias_en` writer - "]
pub type GpadcMicbiasEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `gpadc_micpga_en` reader - "]
pub type GpadcMicpgaEnR = crate::BitReader;
#[doc = "Field `gpadc_micpga_en` writer - "]
pub type GpadcMicpgaEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `gpadc_byp_micboost` reader - "]
pub type GpadcBypMicboostR = crate::BitReader;
#[doc = "Field `gpadc_byp_micboost` writer - "]
pub type GpadcBypMicboostW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `gpadc_rcal_en` reader - "]
pub type GpadcRcalEnR = crate::BitReader;
#[doc = "Field `gpadc_rcal_en` writer - "]
pub type GpadcRcalEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `gpadc_dwa_en` reader - "]
pub type GpadcDwaEnR = crate::BitReader;
#[doc = "Field `gpadc_dwa_en` writer - "]
pub type GpadcDwaEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `gpadc_mic2_diff` reader - "]
pub type GpadcMic2DiffR = crate::BitReader;
#[doc = "Field `gpadc_mic2_diff` writer - "]
pub type GpadcMic2DiffW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `gpadc_mic1_diff` reader - "]
pub type GpadcMic1DiffR = crate::BitReader;
#[doc = "Field `gpadc_mic1_diff` writer - "]
pub type GpadcMic1DiffW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `gpadc_mic_pga2_gain` reader - "]
pub type GpadcMicPga2GainR = crate::FieldReader;
#[doc = "Field `gpadc_mic_pga2_gain` writer - "]
pub type GpadcMicPga2GainW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `gpadc_micboost_32db_en` reader - "]
pub type GpadcMicboost32dbEnR = crate::BitReader;
#[doc = "Field `gpadc_micboost_32db_en` writer - "]
pub type GpadcMicboost32dbEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `gpadc_chip_sen_pu` reader - "]
pub type GpadcChipSenPuR = crate::BitReader;
#[doc = "Field `gpadc_chip_sen_pu` writer - "]
pub type GpadcChipSenPuW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `gpadc_sen_sel` reader - "]
pub type GpadcSenSelR = crate::FieldReader;
#[doc = "Field `gpadc_sen_sel` writer - "]
pub type GpadcSenSelW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `gpadc_sen_test_en` reader - "]
pub type GpadcSenTestEnR = crate::BitReader;
#[doc = "Field `gpadc_sen_test_en` writer - "]
pub type GpadcSenTestEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn gpadc_global_en(&self) -> GpadcGlobalEnR {
        GpadcGlobalEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn gpadc_conv_start(&self) -> GpadcConvStartR {
        GpadcConvStartR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn gpadc_soft_rst(&self) -> GpadcSoftRstR {
        GpadcSoftRstR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:7"]
    #[inline(always)]
    pub fn gpadc_neg_sel(&self) -> GpadcNegSelR {
        GpadcNegSelR::new(((self.bits >> 3) & 0x1f) as u8)
    }
    #[doc = "Bits 8:12"]
    #[inline(always)]
    pub fn gpadc_pos_sel(&self) -> GpadcPosSelR {
        GpadcPosSelR::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn gpadc_neg_gnd(&self) -> GpadcNegGndR {
        GpadcNegGndR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn gpadc_micbias_en(&self) -> GpadcMicbiasEnR {
        GpadcMicbiasEnR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn gpadc_micpga_en(&self) -> GpadcMicpgaEnR {
        GpadcMicpgaEnR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn gpadc_byp_micboost(&self) -> GpadcBypMicboostR {
        GpadcBypMicboostR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn gpadc_rcal_en(&self) -> GpadcRcalEnR {
        GpadcRcalEnR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn gpadc_dwa_en(&self) -> GpadcDwaEnR {
        GpadcDwaEnR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn gpadc_mic2_diff(&self) -> GpadcMic2DiffR {
        GpadcMic2DiffR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn gpadc_mic1_diff(&self) -> GpadcMic1DiffR {
        GpadcMic1DiffR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 21:22"]
    #[inline(always)]
    pub fn gpadc_mic_pga2_gain(&self) -> GpadcMicPga2GainR {
        GpadcMicPga2GainR::new(((self.bits >> 21) & 3) as u8)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn gpadc_micboost_32db_en(&self) -> GpadcMicboost32dbEnR {
        GpadcMicboost32dbEnR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn gpadc_chip_sen_pu(&self) -> GpadcChipSenPuR {
        GpadcChipSenPuR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:30"]
    #[inline(always)]
    pub fn gpadc_sen_sel(&self) -> GpadcSenSelR {
        GpadcSenSelR::new(((self.bits >> 28) & 7) as u8)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn gpadc_sen_test_en(&self) -> GpadcSenTestEnR {
        GpadcSenTestEnR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn gpadc_global_en(&mut self) -> GpadcGlobalEnW<'_, GpadcRegCmdSpec> {
        GpadcGlobalEnW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn gpadc_conv_start(&mut self) -> GpadcConvStartW<'_, GpadcRegCmdSpec> {
        GpadcConvStartW::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn gpadc_soft_rst(&mut self) -> GpadcSoftRstW<'_, GpadcRegCmdSpec> {
        GpadcSoftRstW::new(self, 2)
    }
    #[doc = "Bits 3:7"]
    #[inline(always)]
    pub fn gpadc_neg_sel(&mut self) -> GpadcNegSelW<'_, GpadcRegCmdSpec> {
        GpadcNegSelW::new(self, 3)
    }
    #[doc = "Bits 8:12"]
    #[inline(always)]
    pub fn gpadc_pos_sel(&mut self) -> GpadcPosSelW<'_, GpadcRegCmdSpec> {
        GpadcPosSelW::new(self, 8)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn gpadc_neg_gnd(&mut self) -> GpadcNegGndW<'_, GpadcRegCmdSpec> {
        GpadcNegGndW::new(self, 13)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn gpadc_micbias_en(&mut self) -> GpadcMicbiasEnW<'_, GpadcRegCmdSpec> {
        GpadcMicbiasEnW::new(self, 14)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn gpadc_micpga_en(&mut self) -> GpadcMicpgaEnW<'_, GpadcRegCmdSpec> {
        GpadcMicpgaEnW::new(self, 15)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn gpadc_byp_micboost(&mut self) -> GpadcBypMicboostW<'_, GpadcRegCmdSpec> {
        GpadcBypMicboostW::new(self, 16)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn gpadc_rcal_en(&mut self) -> GpadcRcalEnW<'_, GpadcRegCmdSpec> {
        GpadcRcalEnW::new(self, 17)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn gpadc_dwa_en(&mut self) -> GpadcDwaEnW<'_, GpadcRegCmdSpec> {
        GpadcDwaEnW::new(self, 18)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn gpadc_mic2_diff(&mut self) -> GpadcMic2DiffW<'_, GpadcRegCmdSpec> {
        GpadcMic2DiffW::new(self, 19)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn gpadc_mic1_diff(&mut self) -> GpadcMic1DiffW<'_, GpadcRegCmdSpec> {
        GpadcMic1DiffW::new(self, 20)
    }
    #[doc = "Bits 21:22"]
    #[inline(always)]
    pub fn gpadc_mic_pga2_gain(&mut self) -> GpadcMicPga2GainW<'_, GpadcRegCmdSpec> {
        GpadcMicPga2GainW::new(self, 21)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn gpadc_micboost_32db_en(&mut self) -> GpadcMicboost32dbEnW<'_, GpadcRegCmdSpec> {
        GpadcMicboost32dbEnW::new(self, 23)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn gpadc_chip_sen_pu(&mut self) -> GpadcChipSenPuW<'_, GpadcRegCmdSpec> {
        GpadcChipSenPuW::new(self, 27)
    }
    #[doc = "Bits 28:30"]
    #[inline(always)]
    pub fn gpadc_sen_sel(&mut self) -> GpadcSenSelW<'_, GpadcRegCmdSpec> {
        GpadcSenSelW::new(self, 28)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn gpadc_sen_test_en(&mut self) -> GpadcSenTestEnW<'_, GpadcRegCmdSpec> {
        GpadcSenTestEnW::new(self, 31)
    }
}
#[doc = "GPADC Command Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpadc_reg_cmd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpadc_reg_cmd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpadcRegCmdSpec;
impl crate::RegisterSpec for GpadcRegCmdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpadc_reg_cmd::R`](R) reader structure"]
impl crate::Readable for GpadcRegCmdSpec {}
#[doc = "`write(|w| ..)` method takes [`gpadc_reg_cmd::W`](W) writer structure"]
impl crate::Writable for GpadcRegCmdSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPADC_REG_CMD to value 0"]
impl crate::Resettable for GpadcRegCmdSpec {}
