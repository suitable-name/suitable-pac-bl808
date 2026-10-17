#[doc = "Register `DIG_CLK_CFG1` reader"]
pub type R = crate::R<DigClkCfg1Spec>;
#[doc = "Register `DIG_CLK_CFG1` writer"]
pub type W = crate::W<DigClkCfg1Spec>;
#[doc = "Field `reg_mm_muxpll_160m_sel` reader - "]
pub type RegMmMuxpll160mSelR = crate::BitReader;
#[doc = "Field `reg_mm_muxpll_160m_sel` writer - "]
pub type RegMmMuxpll160mSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_mm_muxpll_240m_sel` reader - "]
pub type RegMmMuxpll240mSelR = crate::BitReader;
#[doc = "Field `reg_mm_muxpll_240m_sel` writer - "]
pub type RegMmMuxpll240mSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_mm_muxpll_320m_sel` reader - "]
pub type RegMmMuxpll320mSelR = crate::BitReader;
#[doc = "Field `reg_mm_muxpll_320m_sel` writer - "]
pub type RegMmMuxpll320mSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_top_muxpll_80m_sel` reader - "]
pub type RegTopMuxpll80mSelR = crate::FieldReader;
#[doc = "Field `reg_top_muxpll_80m_sel` writer - "]
pub type RegTopMuxpll80mSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `reg_top_muxpll_160m_sel` reader - "]
pub type RegTopMuxpll160mSelR = crate::FieldReader;
#[doc = "Field `reg_top_muxpll_160m_sel` writer - "]
pub type RegTopMuxpll160mSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn reg_mm_muxpll_160m_sel(&self) -> RegMmMuxpll160mSelR {
        RegMmMuxpll160mSelR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn reg_mm_muxpll_240m_sel(&self) -> RegMmMuxpll240mSelR {
        RegMmMuxpll240mSelR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn reg_mm_muxpll_320m_sel(&self) -> RegMmMuxpll320mSelR {
        RegMmMuxpll320mSelR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn reg_top_muxpll_80m_sel(&self) -> RegTopMuxpll80mSelR {
        RegTopMuxpll80mSelR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn reg_top_muxpll_160m_sel(&self) -> RegTopMuxpll160mSelR {
        RegTopMuxpll160mSelR::new(((self.bits >> 10) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn reg_mm_muxpll_160m_sel(&mut self) -> RegMmMuxpll160mSelW<'_, DigClkCfg1Spec> {
        RegMmMuxpll160mSelW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn reg_mm_muxpll_240m_sel(&mut self) -> RegMmMuxpll240mSelW<'_, DigClkCfg1Spec> {
        RegMmMuxpll240mSelW::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn reg_mm_muxpll_320m_sel(&mut self) -> RegMmMuxpll320mSelW<'_, DigClkCfg1Spec> {
        RegMmMuxpll320mSelW::new(self, 2)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn reg_top_muxpll_80m_sel(&mut self) -> RegTopMuxpll80mSelW<'_, DigClkCfg1Spec> {
        RegTopMuxpll80mSelW::new(self, 8)
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn reg_top_muxpll_160m_sel(&mut self) -> RegTopMuxpll160mSelW<'_, DigClkCfg1Spec> {
        RegTopMuxpll160mSelW::new(self, 10)
    }
}
#[doc = "Digital Clock Config 1 (MM/TOP Mux)\n\nYou can [`read`](crate::Reg::read) this register and get [`dig_clk_cfg1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dig_clk_cfg1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DigClkCfg1Spec;
impl crate::RegisterSpec for DigClkCfg1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dig_clk_cfg1::R`](R) reader structure"]
impl crate::Readable for DigClkCfg1Spec {}
#[doc = "`write(|w| ..)` method takes [`dig_clk_cfg1::W`](W) writer structure"]
impl crate::Writable for DigClkCfg1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DIG_CLK_CFG1 to value 0"]
impl crate::Resettable for DigClkCfg1Spec {}
