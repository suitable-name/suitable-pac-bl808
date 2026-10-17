#[doc = "Register `DIG_CLK_CFG2` reader"]
pub type R = crate::R<DigClkCfg2Spec>;
#[doc = "Register `DIG_CLK_CFG2` writer"]
pub type W = crate::W<DigClkCfg2Spec>;
#[doc = "Field `chip_clk_out_0_sel` reader - "]
pub type ChipClkOut0SelR = crate::FieldReader;
#[doc = "Field `chip_clk_out_0_sel` writer - "]
pub type ChipClkOut0SelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `chip_clk_out_1_sel` reader - "]
pub type ChipClkOut1SelR = crate::FieldReader;
#[doc = "Field `chip_clk_out_1_sel` writer - "]
pub type ChipClkOut1SelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `chip_clk_out_2_sel` reader - "]
pub type ChipClkOut2SelR = crate::FieldReader;
#[doc = "Field `chip_clk_out_2_sel` writer - "]
pub type ChipClkOut2SelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `chip_clk_out_3_sel` reader - "]
pub type ChipClkOut3SelR = crate::FieldReader;
#[doc = "Field `chip_clk_out_3_sel` writer - "]
pub type ChipClkOut3SelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `chip_clk_out_0_en` reader - "]
pub type ChipClkOut0EnR = crate::BitReader;
#[doc = "Field `chip_clk_out_0_en` writer - "]
pub type ChipClkOut0EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `chip_clk_out_1_en` reader - "]
pub type ChipClkOut1EnR = crate::BitReader;
#[doc = "Field `chip_clk_out_1_en` writer - "]
pub type ChipClkOut1EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `chip_clk_out_2_en` reader - "]
pub type ChipClkOut2EnR = crate::BitReader;
#[doc = "Field `chip_clk_out_2_en` writer - "]
pub type ChipClkOut2EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `chip_clk_out_3_en` reader - "]
pub type ChipClkOut3EnR = crate::BitReader;
#[doc = "Field `chip_clk_out_3_en` writer - "]
pub type ChipClkOut3EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `gpio_tmr_clk_sel` reader - "]
pub type GpioTmrClkSelR = crate::FieldReader;
#[doc = "Field `gpio_tmr_clk_sel` writer - "]
pub type GpioTmrClkSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `gpio_mm_tmr_clk_sel` reader - "]
pub type GpioMmTmrClkSelR = crate::FieldReader;
#[doc = "Field `gpio_mm_tmr_clk_sel` writer - "]
pub type GpioMmTmrClkSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn chip_clk_out_0_sel(&self) -> ChipClkOut0SelR {
        ChipClkOut0SelR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn chip_clk_out_1_sel(&self) -> ChipClkOut1SelR {
        ChipClkOut1SelR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn chip_clk_out_2_sel(&self) -> ChipClkOut2SelR {
        ChipClkOut2SelR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn chip_clk_out_3_sel(&self) -> ChipClkOut3SelR {
        ChipClkOut3SelR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn chip_clk_out_0_en(&self) -> ChipClkOut0EnR {
        ChipClkOut0EnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn chip_clk_out_1_en(&self) -> ChipClkOut1EnR {
        ChipClkOut1EnR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn chip_clk_out_2_en(&self) -> ChipClkOut2EnR {
        ChipClkOut2EnR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn chip_clk_out_3_en(&self) -> ChipClkOut3EnR {
        ChipClkOut3EnR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn gpio_tmr_clk_sel(&self) -> GpioTmrClkSelR {
        GpioTmrClkSelR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn gpio_mm_tmr_clk_sel(&self) -> GpioMmTmrClkSelR {
        GpioMmTmrClkSelR::new(((self.bits >> 14) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn chip_clk_out_0_sel(&mut self) -> ChipClkOut0SelW<'_, DigClkCfg2Spec> {
        ChipClkOut0SelW::new(self, 0)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn chip_clk_out_1_sel(&mut self) -> ChipClkOut1SelW<'_, DigClkCfg2Spec> {
        ChipClkOut1SelW::new(self, 2)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn chip_clk_out_2_sel(&mut self) -> ChipClkOut2SelW<'_, DigClkCfg2Spec> {
        ChipClkOut2SelW::new(self, 4)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn chip_clk_out_3_sel(&mut self) -> ChipClkOut3SelW<'_, DigClkCfg2Spec> {
        ChipClkOut3SelW::new(self, 6)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn chip_clk_out_0_en(&mut self) -> ChipClkOut0EnW<'_, DigClkCfg2Spec> {
        ChipClkOut0EnW::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn chip_clk_out_1_en(&mut self) -> ChipClkOut1EnW<'_, DigClkCfg2Spec> {
        ChipClkOut1EnW::new(self, 9)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn chip_clk_out_2_en(&mut self) -> ChipClkOut2EnW<'_, DigClkCfg2Spec> {
        ChipClkOut2EnW::new(self, 10)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn chip_clk_out_3_en(&mut self) -> ChipClkOut3EnW<'_, DigClkCfg2Spec> {
        ChipClkOut3EnW::new(self, 11)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn gpio_tmr_clk_sel(&mut self) -> GpioTmrClkSelW<'_, DigClkCfg2Spec> {
        GpioTmrClkSelW::new(self, 12)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn gpio_mm_tmr_clk_sel(&mut self) -> GpioMmTmrClkSelW<'_, DigClkCfg2Spec> {
        GpioMmTmrClkSelW::new(self, 14)
    }
}
#[doc = "Digital Clock Config 2 (Chip Clock Output)\n\nYou can [`read`](crate::Reg::read) this register and get [`dig_clk_cfg2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dig_clk_cfg2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DigClkCfg2Spec;
impl crate::RegisterSpec for DigClkCfg2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dig_clk_cfg2::R`](R) reader structure"]
impl crate::Readable for DigClkCfg2Spec {}
#[doc = "`write(|w| ..)` method takes [`dig_clk_cfg2::W`](W) writer structure"]
impl crate::Writable for DigClkCfg2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DIG_CLK_CFG2 to value 0"]
impl crate::Resettable for DigClkCfg2Spec {}
