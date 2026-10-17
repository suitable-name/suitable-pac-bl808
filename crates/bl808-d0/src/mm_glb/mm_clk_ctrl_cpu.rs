#[doc = "Register `MM_CLK_CTRL_CPU` reader"]
pub type R = crate::R<MmClkCtrlCpuSpec>;
#[doc = "Register `MM_CLK_CTRL_CPU` writer"]
pub type W = crate::W<MmClkCtrlCpuSpec>;
#[doc = "Field `reg_pll_en` reader - Enable Multimedia PLL"]
pub type RegPllEnR = crate::BitReader;
#[doc = "Field `reg_pll_en` writer - Enable Multimedia PLL"]
pub type RegPllEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_cpu_clk_en` reader - Enable D0 CPU clock"]
pub type RegCpuClkEnR = crate::BitReader;
#[doc = "Field `reg_cpu_clk_en` writer - Enable D0 CPU clock"]
pub type RegCpuClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_bclk_en` reader - Enable MM bus clock"]
pub type RegBclkEnR = crate::BitReader;
#[doc = "Field `reg_bclk_en` writer - Enable MM bus clock"]
pub type RegBclkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_mm_cpu_clk_en` reader - Enable MM Subsystem CPU clock"]
pub type RegMmCpuClkEnR = crate::BitReader;
#[doc = "Field `reg_mm_cpu_clk_en` writer - Enable MM Subsystem CPU clock"]
pub type RegMmCpuClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_uart_clk_sel` reader - MM UART clock source selection"]
pub type RegUartClkSelR = crate::FieldReader;
#[doc = "Field `reg_uart_clk_sel` writer - MM UART clock source selection"]
pub type RegUartClkSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `reg_i2c_clk_sel` reader - MM I2C clock source selection"]
pub type RegI2cClkSelR = crate::BitReader;
#[doc = "Field `reg_i2c_clk_sel` writer - MM I2C clock source selection"]
pub type RegI2cClkSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_spi_clk_sel` reader - MM SPI clock source selection"]
pub type RegSpiClkSelR = crate::BitReader;
#[doc = "Field `reg_spi_clk_sel` writer - MM SPI clock source selection"]
pub type RegSpiClkSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_cpu_clk_sel` reader - D0 CPU clock source selection"]
pub type RegCpuClkSelR = crate::FieldReader;
#[doc = "Field `reg_cpu_clk_sel` writer - D0 CPU clock source selection"]
pub type RegCpuClkSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `reg_xclk_clk_sel` reader - XCLK selection"]
pub type RegXclkClkSelR = crate::BitReader;
#[doc = "Field `reg_xclk_clk_sel` writer - XCLK selection"]
pub type RegXclkClkSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_cpu_root_clk_sel` reader - CPU root clock selection"]
pub type RegCpuRootClkSelR = crate::BitReader;
#[doc = "Field `reg_cpu_root_clk_sel` writer - CPU root clock selection"]
pub type RegCpuRootClkSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_mmcpu0_clk_en` reader - Enable MM CPU0 clock"]
pub type RegMmcpu0ClkEnR = crate::BitReader;
#[doc = "Field `reg_mmcpu0_clk_en` writer - Enable MM CPU0 clock"]
pub type RegMmcpu0ClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_bclk1x_sel` reader - BCLK 1X selection"]
pub type RegBclk1xSelR = crate::FieldReader;
#[doc = "Field `reg_bclk1x_sel` writer - BCLK 1X selection"]
pub type RegBclk1xSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `reg_bclk2x_div_act_pulse` writer - Write 1 to trigger BCLK2X divider update"]
pub type RegBclk2xDivActPulseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_bclk2x_div_bypass` reader - Bypass BCLK2X divider"]
pub type RegBclk2xDivBypassR = crate::BitReader;
#[doc = "Field `reg_bclk2x_div_bypass` writer - Bypass BCLK2X divider"]
pub type RegBclk2xDivBypassW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sts_bclk2x_prot_done` reader - BCLK2X protection done status"]
pub type StsBclk2xProtDoneR = crate::BitReader;
#[doc = "Field `reg_bclk2x_sw_done_cnt` reader - Software done count for BCLK2X"]
pub type RegBclk2xSwDoneCntR = crate::FieldReader;
#[doc = "Field `reg_bclk2x_sw_done_cnt` writer - Software done count for BCLK2X"]
pub type RegBclk2xSwDoneCntW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `cpu_clk_sw_state` reader - CPU clock switch FSM state"]
pub type CpuClkSwStateR = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - Enable Multimedia PLL"]
    #[inline(always)]
    pub fn reg_pll_en(&self) -> RegPllEnR {
        RegPllEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable D0 CPU clock"]
    #[inline(always)]
    pub fn reg_cpu_clk_en(&self) -> RegCpuClkEnR {
        RegCpuClkEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable MM bus clock"]
    #[inline(always)]
    pub fn reg_bclk_en(&self) -> RegBclkEnR {
        RegBclkEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable MM Subsystem CPU clock"]
    #[inline(always)]
    pub fn reg_mm_cpu_clk_en(&self) -> RegMmCpuClkEnR {
        RegMmCpuClkEnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - MM UART clock source selection"]
    #[inline(always)]
    pub fn reg_uart_clk_sel(&self) -> RegUartClkSelR {
        RegUartClkSelR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - MM I2C clock source selection"]
    #[inline(always)]
    pub fn reg_i2c_clk_sel(&self) -> RegI2cClkSelR {
        RegI2cClkSelR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - MM SPI clock source selection"]
    #[inline(always)]
    pub fn reg_spi_clk_sel(&self) -> RegSpiClkSelR {
        RegSpiClkSelR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - D0 CPU clock source selection"]
    #[inline(always)]
    pub fn reg_cpu_clk_sel(&self) -> RegCpuClkSelR {
        RegCpuClkSelR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - XCLK selection"]
    #[inline(always)]
    pub fn reg_xclk_clk_sel(&self) -> RegXclkClkSelR {
        RegXclkClkSelR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - CPU root clock selection"]
    #[inline(always)]
    pub fn reg_cpu_root_clk_sel(&self) -> RegCpuRootClkSelR {
        RegCpuRootClkSelR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Enable MM CPU0 clock"]
    #[inline(always)]
    pub fn reg_mmcpu0_clk_en(&self) -> RegMmcpu0ClkEnR {
        RegMmcpu0ClkEnR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:14 - BCLK 1X selection"]
    #[inline(always)]
    pub fn reg_bclk1x_sel(&self) -> RegBclk1xSelR {
        RegBclk1xSelR::new(((self.bits >> 13) & 3) as u8)
    }
    #[doc = "Bit 19 - Bypass BCLK2X divider"]
    #[inline(always)]
    pub fn reg_bclk2x_div_bypass(&self) -> RegBclk2xDivBypassR {
        RegBclk2xDivBypassR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - BCLK2X protection done status"]
    #[inline(always)]
    pub fn sts_bclk2x_prot_done(&self) -> StsBclk2xProtDoneR {
        StsBclk2xProtDoneR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 24:27 - Software done count for BCLK2X"]
    #[inline(always)]
    pub fn reg_bclk2x_sw_done_cnt(&self) -> RegBclk2xSwDoneCntR {
        RegBclk2xSwDoneCntR::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:30 - CPU clock switch FSM state"]
    #[inline(always)]
    pub fn cpu_clk_sw_state(&self) -> CpuClkSwStateR {
        CpuClkSwStateR::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Enable Multimedia PLL"]
    #[inline(always)]
    pub fn reg_pll_en(&mut self) -> RegPllEnW<'_, MmClkCtrlCpuSpec> {
        RegPllEnW::new(self, 0)
    }
    #[doc = "Bit 1 - Enable D0 CPU clock"]
    #[inline(always)]
    pub fn reg_cpu_clk_en(&mut self) -> RegCpuClkEnW<'_, MmClkCtrlCpuSpec> {
        RegCpuClkEnW::new(self, 1)
    }
    #[doc = "Bit 2 - Enable MM bus clock"]
    #[inline(always)]
    pub fn reg_bclk_en(&mut self) -> RegBclkEnW<'_, MmClkCtrlCpuSpec> {
        RegBclkEnW::new(self, 2)
    }
    #[doc = "Bit 3 - Enable MM Subsystem CPU clock"]
    #[inline(always)]
    pub fn reg_mm_cpu_clk_en(&mut self) -> RegMmCpuClkEnW<'_, MmClkCtrlCpuSpec> {
        RegMmCpuClkEnW::new(self, 3)
    }
    #[doc = "Bits 4:5 - MM UART clock source selection"]
    #[inline(always)]
    pub fn reg_uart_clk_sel(&mut self) -> RegUartClkSelW<'_, MmClkCtrlCpuSpec> {
        RegUartClkSelW::new(self, 4)
    }
    #[doc = "Bit 6 - MM I2C clock source selection"]
    #[inline(always)]
    pub fn reg_i2c_clk_sel(&mut self) -> RegI2cClkSelW<'_, MmClkCtrlCpuSpec> {
        RegI2cClkSelW::new(self, 6)
    }
    #[doc = "Bit 7 - MM SPI clock source selection"]
    #[inline(always)]
    pub fn reg_spi_clk_sel(&mut self) -> RegSpiClkSelW<'_, MmClkCtrlCpuSpec> {
        RegSpiClkSelW::new(self, 7)
    }
    #[doc = "Bits 8:9 - D0 CPU clock source selection"]
    #[inline(always)]
    pub fn reg_cpu_clk_sel(&mut self) -> RegCpuClkSelW<'_, MmClkCtrlCpuSpec> {
        RegCpuClkSelW::new(self, 8)
    }
    #[doc = "Bit 10 - XCLK selection"]
    #[inline(always)]
    pub fn reg_xclk_clk_sel(&mut self) -> RegXclkClkSelW<'_, MmClkCtrlCpuSpec> {
        RegXclkClkSelW::new(self, 10)
    }
    #[doc = "Bit 11 - CPU root clock selection"]
    #[inline(always)]
    pub fn reg_cpu_root_clk_sel(&mut self) -> RegCpuRootClkSelW<'_, MmClkCtrlCpuSpec> {
        RegCpuRootClkSelW::new(self, 11)
    }
    #[doc = "Bit 12 - Enable MM CPU0 clock"]
    #[inline(always)]
    pub fn reg_mmcpu0_clk_en(&mut self) -> RegMmcpu0ClkEnW<'_, MmClkCtrlCpuSpec> {
        RegMmcpu0ClkEnW::new(self, 12)
    }
    #[doc = "Bits 13:14 - BCLK 1X selection"]
    #[inline(always)]
    pub fn reg_bclk1x_sel(&mut self) -> RegBclk1xSelW<'_, MmClkCtrlCpuSpec> {
        RegBclk1xSelW::new(self, 13)
    }
    #[doc = "Bit 18 - Write 1 to trigger BCLK2X divider update"]
    #[inline(always)]
    pub fn reg_bclk2x_div_act_pulse(&mut self) -> RegBclk2xDivActPulseW<'_, MmClkCtrlCpuSpec> {
        RegBclk2xDivActPulseW::new(self, 18)
    }
    #[doc = "Bit 19 - Bypass BCLK2X divider"]
    #[inline(always)]
    pub fn reg_bclk2x_div_bypass(&mut self) -> RegBclk2xDivBypassW<'_, MmClkCtrlCpuSpec> {
        RegBclk2xDivBypassW::new(self, 19)
    }
    #[doc = "Bits 24:27 - Software done count for BCLK2X"]
    #[inline(always)]
    pub fn reg_bclk2x_sw_done_cnt(&mut self) -> RegBclk2xSwDoneCntW<'_, MmClkCtrlCpuSpec> {
        RegBclk2xSwDoneCntW::new(self, 24)
    }
}
#[doc = "Multimedia CPU Clock Control\n\nYou can [`read`](crate::Reg::read) this register and get [`mm_clk_ctrl_cpu::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mm_clk_ctrl_cpu::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MmClkCtrlCpuSpec;
impl crate::RegisterSpec for MmClkCtrlCpuSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mm_clk_ctrl_cpu::R`](R) reader structure"]
impl crate::Readable for MmClkCtrlCpuSpec {}
#[doc = "`write(|w| ..)` method takes [`mm_clk_ctrl_cpu::W`](W) writer structure"]
impl crate::Writable for MmClkCtrlCpuSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MM_CLK_CTRL_CPU to value 0x0510_000f"]
impl crate::Resettable for MmClkCtrlCpuSpec {
    const RESET_VALUE: u32 = 0x0510_000f;
}
