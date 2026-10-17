#[doc = "Register `MM_CLK_CTRL_PERI3` reader"]
pub type R = crate::R<MmClkCtrlPeri3Spec>;
#[doc = "Register `MM_CLK_CTRL_PERI3` writer"]
pub type W = crate::W<MmClkCtrlPeri3Spec>;
#[doc = "Field `reg_i2c1_clk_div` reader - "]
pub type RegI2c1ClkDivR = crate::FieldReader;
#[doc = "Field `reg_i2c1_clk_div` writer - "]
pub type RegI2c1ClkDivW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_i2c1_clk_div_en` reader - "]
pub type RegI2c1ClkDivEnR = crate::BitReader;
#[doc = "Field `reg_i2c1_clk_div_en` writer - "]
pub type RegI2c1ClkDivEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_i2c1_clk_en` reader - "]
pub type RegI2c1ClkEnR = crate::BitReader;
#[doc = "Field `reg_i2c1_clk_en` writer - "]
pub type RegI2c1ClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_uart1_clk_div_en` reader - "]
pub type RegUart1ClkDivEnR = crate::BitReader;
#[doc = "Field `reg_uart1_clk_div_en` writer - "]
pub type RegUart1ClkDivEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_uart1_clk_div` reader - "]
pub type RegUart1ClkDivR = crate::FieldReader;
#[doc = "Field `reg_uart1_clk_div` writer - "]
pub type RegUart1ClkDivW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn reg_i2c1_clk_div(&self) -> RegI2c1ClkDivR {
        RegI2c1ClkDivR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn reg_i2c1_clk_div_en(&self) -> RegI2c1ClkDivEnR {
        RegI2c1ClkDivEnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn reg_i2c1_clk_en(&self) -> RegI2c1ClkEnR {
        RegI2c1ClkEnR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn reg_uart1_clk_div_en(&self) -> RegUart1ClkDivEnR {
        RegUart1ClkDivEnR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:19"]
    #[inline(always)]
    pub fn reg_uart1_clk_div(&self) -> RegUart1ClkDivR {
        RegUart1ClkDivR::new(((self.bits >> 17) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn reg_i2c1_clk_div(&mut self) -> RegI2c1ClkDivW<'_, MmClkCtrlPeri3Spec> {
        RegI2c1ClkDivW::new(self, 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn reg_i2c1_clk_div_en(&mut self) -> RegI2c1ClkDivEnW<'_, MmClkCtrlPeri3Spec> {
        RegI2c1ClkDivEnW::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn reg_i2c1_clk_en(&mut self) -> RegI2c1ClkEnW<'_, MmClkCtrlPeri3Spec> {
        RegI2c1ClkEnW::new(self, 9)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn reg_uart1_clk_div_en(&mut self) -> RegUart1ClkDivEnW<'_, MmClkCtrlPeri3Spec> {
        RegUart1ClkDivEnW::new(self, 16)
    }
    #[doc = "Bits 17:19"]
    #[inline(always)]
    pub fn reg_uart1_clk_div(&mut self) -> RegUart1ClkDivW<'_, MmClkCtrlPeri3Spec> {
        RegUart1ClkDivW::new(self, 17)
    }
}
#[doc = "MM Peripheral Clock Control 1\n\nYou can [`read`](crate::Reg::read) this register and get [`mm_clk_ctrl_peri3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mm_clk_ctrl_peri3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MmClkCtrlPeri3Spec;
impl crate::RegisterSpec for MmClkCtrlPeri3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mm_clk_ctrl_peri3::R`](R) reader structure"]
impl crate::Readable for MmClkCtrlPeri3Spec {}
#[doc = "`write(|w| ..)` method takes [`mm_clk_ctrl_peri3::W`](W) writer structure"]
impl crate::Writable for MmClkCtrlPeri3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MM_CLK_CTRL_PERI3 to value 0x0001_0300"]
impl crate::Resettable for MmClkCtrlPeri3Spec {
    const RESET_VALUE: u32 = 0x0001_0300;
}
