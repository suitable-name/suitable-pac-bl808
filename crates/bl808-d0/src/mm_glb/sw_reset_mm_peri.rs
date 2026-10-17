#[doc = "Register `SW_RESET_MM_PERI` reader"]
pub type R = crate::R<SwResetMmPeriSpec>;
#[doc = "Register `SW_RESET_MM_PERI` writer"]
pub type W = crate::W<SwResetMmPeriSpec>;
#[doc = "Field `swrst_mm_misc` reader - "]
pub type SwrstMmMiscR = crate::BitReader;
#[doc = "Field `swrst_mm_misc` writer - "]
pub type SwrstMmMiscW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `swrst_dma` reader - "]
pub type SwrstDmaR = crate::BitReader;
#[doc = "Field `swrst_dma` writer - "]
pub type SwrstDmaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `swrst_uart0` reader - "]
pub type SwrstUart0R = crate::BitReader;
#[doc = "Field `swrst_uart0` writer - "]
pub type SwrstUart0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `swrst_i2c0` reader - "]
pub type SwrstI2c0R = crate::BitReader;
#[doc = "Field `swrst_i2c0` writer - "]
pub type SwrstI2c0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `swrst_i2c1` reader - "]
pub type SwrstI2c1R = crate::BitReader;
#[doc = "Field `swrst_i2c1` writer - "]
pub type SwrstI2c1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `swrst_ipc` reader - "]
pub type SwrstIpcR = crate::BitReader;
#[doc = "Field `swrst_ipc` writer - "]
pub type SwrstIpcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `swrst_dma2d` reader - "]
pub type SwrstDma2dR = crate::BitReader;
#[doc = "Field `swrst_dma2d` writer - "]
pub type SwrstDma2dW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `swrst_spi` reader - "]
pub type SwrstSpiR = crate::BitReader;
#[doc = "Field `swrst_spi` writer - "]
pub type SwrstSpiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `swrst_timer` reader - "]
pub type SwrstTimerR = crate::BitReader;
#[doc = "Field `swrst_timer` writer - "]
pub type SwrstTimerW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `swrst_i2s0` reader - "]
pub type SwrstI2s0R = crate::BitReader;
#[doc = "Field `swrst_i2s0` writer - "]
pub type SwrstI2s0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `swrst_i2s1` reader - "]
pub type SwrstI2s1R = crate::BitReader;
#[doc = "Field `swrst_i2s1` writer - "]
pub type SwrstI2s1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `swrst_pdm0` reader - "]
pub type SwrstPdm0R = crate::BitReader;
#[doc = "Field `swrst_pdm0` writer - "]
pub type SwrstPdm0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `swrst_pdm1` reader - "]
pub type SwrstPdm1R = crate::BitReader;
#[doc = "Field `swrst_pdm1` writer - "]
pub type SwrstPdm1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `swrst_uart1` reader - "]
pub type SwrstUart1R = crate::BitReader;
#[doc = "Field `swrst_uart1` writer - "]
pub type SwrstUart1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `swrst_puhs` reader - "]
pub type SwrstPuhsR = crate::BitReader;
#[doc = "Field `swrst_puhs` writer - "]
pub type SwrstPuhsW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn swrst_mm_misc(&self) -> SwrstMmMiscR {
        SwrstMmMiscR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn swrst_dma(&self) -> SwrstDmaR {
        SwrstDmaR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn swrst_uart0(&self) -> SwrstUart0R {
        SwrstUart0R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn swrst_i2c0(&self) -> SwrstI2c0R {
        SwrstI2c0R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn swrst_i2c1(&self) -> SwrstI2c1R {
        SwrstI2c1R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn swrst_ipc(&self) -> SwrstIpcR {
        SwrstIpcR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn swrst_dma2d(&self) -> SwrstDma2dR {
        SwrstDma2dR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn swrst_spi(&self) -> SwrstSpiR {
        SwrstSpiR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn swrst_timer(&self) -> SwrstTimerR {
        SwrstTimerR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn swrst_i2s0(&self) -> SwrstI2s0R {
        SwrstI2s0R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn swrst_i2s1(&self) -> SwrstI2s1R {
        SwrstI2s1R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn swrst_pdm0(&self) -> SwrstPdm0R {
        SwrstPdm0R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn swrst_pdm1(&self) -> SwrstPdm1R {
        SwrstPdm1R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn swrst_uart1(&self) -> SwrstUart1R {
        SwrstUart1R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn swrst_puhs(&self) -> SwrstPuhsR {
        SwrstPuhsR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn swrst_mm_misc(&mut self) -> SwrstMmMiscW<'_, SwResetMmPeriSpec> {
        SwrstMmMiscW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn swrst_dma(&mut self) -> SwrstDmaW<'_, SwResetMmPeriSpec> {
        SwrstDmaW::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn swrst_uart0(&mut self) -> SwrstUart0W<'_, SwResetMmPeriSpec> {
        SwrstUart0W::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn swrst_i2c0(&mut self) -> SwrstI2c0W<'_, SwResetMmPeriSpec> {
        SwrstI2c0W::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn swrst_i2c1(&mut self) -> SwrstI2c1W<'_, SwResetMmPeriSpec> {
        SwrstI2c1W::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn swrst_ipc(&mut self) -> SwrstIpcW<'_, SwResetMmPeriSpec> {
        SwrstIpcW::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn swrst_dma2d(&mut self) -> SwrstDma2dW<'_, SwResetMmPeriSpec> {
        SwrstDma2dW::new(self, 6)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn swrst_spi(&mut self) -> SwrstSpiW<'_, SwResetMmPeriSpec> {
        SwrstSpiW::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn swrst_timer(&mut self) -> SwrstTimerW<'_, SwResetMmPeriSpec> {
        SwrstTimerW::new(self, 9)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn swrst_i2s0(&mut self) -> SwrstI2s0W<'_, SwResetMmPeriSpec> {
        SwrstI2s0W::new(self, 10)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn swrst_i2s1(&mut self) -> SwrstI2s1W<'_, SwResetMmPeriSpec> {
        SwrstI2s1W::new(self, 11)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn swrst_pdm0(&mut self) -> SwrstPdm0W<'_, SwResetMmPeriSpec> {
        SwrstPdm0W::new(self, 12)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn swrst_pdm1(&mut self) -> SwrstPdm1W<'_, SwResetMmPeriSpec> {
        SwrstPdm1W::new(self, 13)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn swrst_uart1(&mut self) -> SwrstUart1W<'_, SwResetMmPeriSpec> {
        SwrstUart1W::new(self, 14)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn swrst_puhs(&mut self) -> SwrstPuhsW<'_, SwResetMmPeriSpec> {
        SwrstPuhsW::new(self, 15)
    }
}
#[doc = "MM Peripheral Software Reset\n\nYou can [`read`](crate::Reg::read) this register and get [`sw_reset_mm_peri::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sw_reset_mm_peri::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SwResetMmPeriSpec;
impl crate::RegisterSpec for SwResetMmPeriSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sw_reset_mm_peri::R`](R) reader structure"]
impl crate::Readable for SwResetMmPeriSpec {}
#[doc = "`write(|w| ..)` method takes [`sw_reset_mm_peri::W`](W) writer structure"]
impl crate::Writable for SwResetMmPeriSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SW_RESET_MM_PERI to value 0"]
impl crate::Resettable for SwResetMmPeriSpec {}
