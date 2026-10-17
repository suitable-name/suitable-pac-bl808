#[doc = "Register `SD_CLOCK_CTRL` reader"]
pub type R = crate::R<SdClockCtrlSpec>;
#[doc = "Register `SD_CLOCK_CTRL` writer"]
pub type W = crate::W<SdClockCtrlSpec>;
#[doc = "Field `INT_CLK_EN` reader - "]
pub type IntClkEnR = crate::BitReader;
#[doc = "Field `INT_CLK_EN` writer - "]
pub type IntClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INT_CLK_STABLE` reader - "]
pub type IntClkStableR = crate::BitReader;
#[doc = "Field `SD_CLK_EN` reader - "]
pub type SdClkEnR = crate::BitReader;
#[doc = "Field `SD_CLK_EN` writer - "]
pub type SdClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_GEN_SEL` reader - "]
pub type ClkGenSelR = crate::BitReader;
#[doc = "Field `CLK_GEN_SEL` writer - "]
pub type ClkGenSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SD_FREQ_SEL_HI` reader - "]
pub type SdFreqSelHiR = crate::FieldReader;
#[doc = "Field `SD_FREQ_SEL_HI` writer - "]
pub type SdFreqSelHiW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SD_FREQ_SEL_LO` reader - "]
pub type SdFreqSelLoR = crate::FieldReader;
#[doc = "Field `SD_FREQ_SEL_LO` writer - "]
pub type SdFreqSelLoW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn int_clk_en(&self) -> IntClkEnR {
        IntClkEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn int_clk_stable(&self) -> IntClkStableR {
        IntClkStableR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn sd_clk_en(&self) -> SdClkEnR {
        SdClkEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn clk_gen_sel(&self) -> ClkGenSelR {
        ClkGenSelR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn sd_freq_sel_hi(&self) -> SdFreqSelHiR {
        SdFreqSelHiR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn sd_freq_sel_lo(&self) -> SdFreqSelLoR {
        SdFreqSelLoR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn int_clk_en(&mut self) -> IntClkEnW<'_, SdClockCtrlSpec> {
        IntClkEnW::new(self, 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn sd_clk_en(&mut self) -> SdClkEnW<'_, SdClockCtrlSpec> {
        SdClkEnW::new(self, 2)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn clk_gen_sel(&mut self) -> ClkGenSelW<'_, SdClockCtrlSpec> {
        ClkGenSelW::new(self, 5)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn sd_freq_sel_hi(&mut self) -> SdFreqSelHiW<'_, SdClockCtrlSpec> {
        SdFreqSelHiW::new(self, 6)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn sd_freq_sel_lo(&mut self) -> SdFreqSelLoW<'_, SdClockCtrlSpec> {
        SdFreqSelLoW::new(self, 8)
    }
}
#[doc = "Clock Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sd_clock_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sd_clock_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdClockCtrlSpec;
impl crate::RegisterSpec for SdClockCtrlSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`sd_clock_ctrl::R`](R) reader structure"]
impl crate::Readable for SdClockCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`sd_clock_ctrl::W`](W) writer structure"]
impl crate::Writable for SdClockCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SD_CLOCK_CTRL to value 0"]
impl crate::Resettable for SdClockCtrlSpec {}
