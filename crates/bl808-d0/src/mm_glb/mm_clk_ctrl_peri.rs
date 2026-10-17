#[doc = "Register `MM_CLK_CTRL_PERI` reader"]
pub type R = crate::R<MmClkCtrlPeriSpec>;
#[doc = "Register `MM_CLK_CTRL_PERI` writer"]
pub type W = crate::W<MmClkCtrlPeriSpec>;
#[doc = "Field `reg_i2c0_clk_div` reader - "]
pub type RegI2c0ClkDivR = crate::FieldReader;
#[doc = "Field `reg_i2c0_clk_div` writer - "]
pub type RegI2c0ClkDivW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_i2c0_clk_div_en` reader - "]
pub type RegI2c0ClkDivEnR = crate::BitReader;
#[doc = "Field `reg_i2c0_clk_div_en` writer - "]
pub type RegI2c0ClkDivEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_i2c0_clk_en` reader - "]
pub type RegI2c0ClkEnR = crate::BitReader;
#[doc = "Field `reg_i2c0_clk_en` writer - "]
pub type RegI2c0ClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_uart0_clk_div_en` reader - "]
pub type RegUart0ClkDivEnR = crate::BitReader;
#[doc = "Field `reg_uart0_clk_div_en` writer - "]
pub type RegUart0ClkDivEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_uart0_clk_div` reader - "]
pub type RegUart0ClkDivR = crate::FieldReader;
#[doc = "Field `reg_uart0_clk_div` writer - "]
pub type RegUart0ClkDivW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `reg_spi_clk_div_en` reader - "]
pub type RegSpiClkDivEnR = crate::BitReader;
#[doc = "Field `reg_spi_clk_div_en` writer - "]
pub type RegSpiClkDivEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_spi_clk_div` reader - "]
pub type RegSpiClkDivR = crate::FieldReader;
#[doc = "Field `reg_spi_clk_div` writer - "]
pub type RegSpiClkDivW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn reg_i2c0_clk_div(&self) -> RegI2c0ClkDivR {
        RegI2c0ClkDivR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn reg_i2c0_clk_div_en(&self) -> RegI2c0ClkDivEnR {
        RegI2c0ClkDivEnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn reg_i2c0_clk_en(&self) -> RegI2c0ClkEnR {
        RegI2c0ClkEnR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn reg_uart0_clk_div_en(&self) -> RegUart0ClkDivEnR {
        RegUart0ClkDivEnR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:19"]
    #[inline(always)]
    pub fn reg_uart0_clk_div(&self) -> RegUart0ClkDivR {
        RegUart0ClkDivR::new(((self.bits >> 17) & 7) as u8)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn reg_spi_clk_div_en(&self) -> RegSpiClkDivEnR {
        RegSpiClkDivEnR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn reg_spi_clk_div(&self) -> RegSpiClkDivR {
        RegSpiClkDivR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn reg_i2c0_clk_div(&mut self) -> RegI2c0ClkDivW<'_, MmClkCtrlPeriSpec> {
        RegI2c0ClkDivW::new(self, 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn reg_i2c0_clk_div_en(&mut self) -> RegI2c0ClkDivEnW<'_, MmClkCtrlPeriSpec> {
        RegI2c0ClkDivEnW::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn reg_i2c0_clk_en(&mut self) -> RegI2c0ClkEnW<'_, MmClkCtrlPeriSpec> {
        RegI2c0ClkEnW::new(self, 9)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn reg_uart0_clk_div_en(&mut self) -> RegUart0ClkDivEnW<'_, MmClkCtrlPeriSpec> {
        RegUart0ClkDivEnW::new(self, 16)
    }
    #[doc = "Bits 17:19"]
    #[inline(always)]
    pub fn reg_uart0_clk_div(&mut self) -> RegUart0ClkDivW<'_, MmClkCtrlPeriSpec> {
        RegUart0ClkDivW::new(self, 17)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn reg_spi_clk_div_en(&mut self) -> RegSpiClkDivEnW<'_, MmClkCtrlPeriSpec> {
        RegSpiClkDivEnW::new(self, 23)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn reg_spi_clk_div(&mut self) -> RegSpiClkDivW<'_, MmClkCtrlPeriSpec> {
        RegSpiClkDivW::new(self, 24)
    }
}
#[doc = "MM Peripheral Clock Control 0\n\nYou can [`read`](crate::Reg::read) this register and get [`mm_clk_ctrl_peri::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mm_clk_ctrl_peri::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MmClkCtrlPeriSpec;
impl crate::RegisterSpec for MmClkCtrlPeriSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mm_clk_ctrl_peri::R`](R) reader structure"]
impl crate::Readable for MmClkCtrlPeriSpec {}
#[doc = "`write(|w| ..)` method takes [`mm_clk_ctrl_peri::W`](W) writer structure"]
impl crate::Writable for MmClkCtrlPeriSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MM_CLK_CTRL_PERI to value 0x0081_0300"]
impl crate::Resettable for MmClkCtrlPeriSpec {
    const RESET_VALUE: u32 = 0x0081_0300;
}
