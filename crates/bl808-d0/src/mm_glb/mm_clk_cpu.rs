#[doc = "Register `MM_CLK_CPU` reader"]
pub type R = crate::R<MmClkCpuSpec>;
#[doc = "Register `MM_CLK_CPU` writer"]
pub type W = crate::W<MmClkCpuSpec>;
#[doc = "Field `reg_cpu_clk_div` reader - CPU clock divider factor"]
pub type RegCpuClkDivR = crate::FieldReader;
#[doc = "Field `reg_cpu_clk_div` writer - CPU clock divider factor"]
pub type RegCpuClkDivW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_cnn_clk_div_en` reader - Enable CNN clock divider"]
pub type RegCnnClkDivEnR = crate::BitReader;
#[doc = "Field `reg_cnn_clk_div_en` writer - Enable CNN clock divider"]
pub type RegCnnClkDivEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_cnn_clk_sel` reader - CNN clock selection"]
pub type RegCnnClkSelR = crate::FieldReader;
#[doc = "Field `reg_cnn_clk_sel` writer - CNN clock selection"]
pub type RegCnnClkSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `reg_cnn_clk_div` reader - CNN clock divider factor"]
pub type RegCnnClkDivR = crate::FieldReader;
#[doc = "Field `reg_cnn_clk_div` writer - CNN clock divider factor"]
pub type RegCnnClkDivW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `reg_bclk2x_div` reader - BCLK 2X divider factor"]
pub type RegBclk2xDivR = crate::FieldReader;
#[doc = "Field `reg_bclk2x_div` writer - BCLK 2X divider factor"]
pub type RegBclk2xDivW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_bclk1x_div` reader - BCLK 1X divider factor"]
pub type RegBclk1xDivR = crate::FieldReader;
#[doc = "Field `reg_bclk1x_div` writer - BCLK 1X divider factor"]
pub type RegBclk1xDivW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - CPU clock divider factor"]
    #[inline(always)]
    pub fn reg_cpu_clk_div(&self) -> RegCpuClkDivR {
        RegCpuClkDivR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - Enable CNN clock divider"]
    #[inline(always)]
    pub fn reg_cnn_clk_div_en(&self) -> RegCnnClkDivEnR {
        RegCnnClkDivEnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:10 - CNN clock selection"]
    #[inline(always)]
    pub fn reg_cnn_clk_sel(&self) -> RegCnnClkSelR {
        RegCnnClkSelR::new(((self.bits >> 9) & 3) as u8)
    }
    #[doc = "Bits 12:14 - CNN clock divider factor"]
    #[inline(always)]
    pub fn reg_cnn_clk_div(&self) -> RegCnnClkDivR {
        RegCnnClkDivR::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 16:23 - BCLK 2X divider factor"]
    #[inline(always)]
    pub fn reg_bclk2x_div(&self) -> RegBclk2xDivR {
        RegBclk2xDivR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - BCLK 1X divider factor"]
    #[inline(always)]
    pub fn reg_bclk1x_div(&self) -> RegBclk1xDivR {
        RegBclk1xDivR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - CPU clock divider factor"]
    #[inline(always)]
    pub fn reg_cpu_clk_div(&mut self) -> RegCpuClkDivW<'_, MmClkCpuSpec> {
        RegCpuClkDivW::new(self, 0)
    }
    #[doc = "Bit 8 - Enable CNN clock divider"]
    #[inline(always)]
    pub fn reg_cnn_clk_div_en(&mut self) -> RegCnnClkDivEnW<'_, MmClkCpuSpec> {
        RegCnnClkDivEnW::new(self, 8)
    }
    #[doc = "Bits 9:10 - CNN clock selection"]
    #[inline(always)]
    pub fn reg_cnn_clk_sel(&mut self) -> RegCnnClkSelW<'_, MmClkCpuSpec> {
        RegCnnClkSelW::new(self, 9)
    }
    #[doc = "Bits 12:14 - CNN clock divider factor"]
    #[inline(always)]
    pub fn reg_cnn_clk_div(&mut self) -> RegCnnClkDivW<'_, MmClkCpuSpec> {
        RegCnnClkDivW::new(self, 12)
    }
    #[doc = "Bits 16:23 - BCLK 2X divider factor"]
    #[inline(always)]
    pub fn reg_bclk2x_div(&mut self) -> RegBclk2xDivW<'_, MmClkCpuSpec> {
        RegBclk2xDivW::new(self, 16)
    }
    #[doc = "Bits 24:31 - BCLK 1X divider factor"]
    #[inline(always)]
    pub fn reg_bclk1x_div(&mut self) -> RegBclk1xDivW<'_, MmClkCpuSpec> {
        RegBclk1xDivW::new(self, 24)
    }
}
#[doc = "Multimedia CPU and CNN Clock Dividers\n\nYou can [`read`](crate::Reg::read) this register and get [`mm_clk_cpu::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mm_clk_cpu::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MmClkCpuSpec;
impl crate::RegisterSpec for MmClkCpuSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mm_clk_cpu::R`](R) reader structure"]
impl crate::Readable for MmClkCpuSpec {}
#[doc = "`write(|w| ..)` method takes [`mm_clk_cpu::W`](W) writer structure"]
impl crate::Writable for MmClkCpuSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MM_CLK_CPU to value 0x0100"]
impl crate::Resettable for MmClkCpuSpec {
    const RESET_VALUE: u32 = 0x0100;
}
