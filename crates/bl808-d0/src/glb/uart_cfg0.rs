#[doc = "Register `UART_CFG0` reader"]
pub type R = crate::R<UartCfg0Spec>;
#[doc = "Register `UART_CFG0` writer"]
pub type W = crate::W<UartCfg0Spec>;
#[doc = "Field `uart_clk_div` reader - "]
pub type UartClkDivR = crate::FieldReader;
#[doc = "Field `uart_clk_div` writer - "]
pub type UartClkDivW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `uart_clk_en` reader - "]
pub type UartClkEnR = crate::BitReader;
#[doc = "Field `uart_clk_en` writer - "]
pub type UartClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `hbn_uart_clk_sel` reader - "]
pub type HbnUartClkSelR = crate::BitReader;
#[doc = "Field `hbn_uart_clk_sel` writer - "]
pub type HbnUartClkSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `hbn_uart_clk_sel2` reader - "]
pub type HbnUartClkSel2R = crate::BitReader;
#[doc = "Field `hbn_uart_clk_sel2` writer - "]
pub type HbnUartClkSel2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `uart2_io_sel` reader - "]
pub type Uart2IoSelR = crate::BitReader;
#[doc = "Field `uart2_io_sel` writer - "]
pub type Uart2IoSelW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn uart_clk_div(&self) -> UartClkDivR {
        UartClkDivR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn uart_clk_en(&self) -> UartClkEnR {
        UartClkEnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn hbn_uart_clk_sel(&self) -> HbnUartClkSelR {
        HbnUartClkSelR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn hbn_uart_clk_sel2(&self) -> HbnUartClkSel2R {
        HbnUartClkSel2R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn uart2_io_sel(&self) -> Uart2IoSelR {
        Uart2IoSelR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn uart_clk_div(&mut self) -> UartClkDivW<'_, UartCfg0Spec> {
        UartClkDivW::new(self, 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn uart_clk_en(&mut self) -> UartClkEnW<'_, UartCfg0Spec> {
        UartClkEnW::new(self, 4)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn hbn_uart_clk_sel(&mut self) -> HbnUartClkSelW<'_, UartCfg0Spec> {
        HbnUartClkSelW::new(self, 7)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn hbn_uart_clk_sel2(&mut self) -> HbnUartClkSel2W<'_, UartCfg0Spec> {
        HbnUartClkSel2W::new(self, 22)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn uart2_io_sel(&mut self) -> Uart2IoSelW<'_, UartCfg0Spec> {
        Uart2IoSelW::new(self, 24)
    }
}
#[doc = "UART Clock Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`uart_cfg0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart_cfg0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UartCfg0Spec;
impl crate::RegisterSpec for UartCfg0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uart_cfg0::R`](R) reader structure"]
impl crate::Readable for UartCfg0Spec {}
#[doc = "`write(|w| ..)` method takes [`uart_cfg0::W`](W) writer structure"]
impl crate::Writable for UartCfg0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UART_CFG0 to value 0"]
impl crate::Resettable for UartCfg0Spec {}
