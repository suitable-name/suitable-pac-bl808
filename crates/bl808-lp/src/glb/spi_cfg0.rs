#[doc = "Register `SPI_CFG0` reader"]
pub type R = crate::R<SpiCfg0Spec>;
#[doc = "Register `SPI_CFG0` writer"]
pub type W = crate::W<SpiCfg0Spec>;
#[doc = "Field `spi_clk_div` reader - "]
pub type SpiClkDivR = crate::FieldReader;
#[doc = "Field `spi_clk_div` writer - "]
pub type SpiClkDivW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `spi_clk_en` reader - "]
pub type SpiClkEnR = crate::BitReader;
#[doc = "Field `spi_clk_en` writer - "]
pub type SpiClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `spi_clk_sel` reader - "]
pub type SpiClkSelR = crate::BitReader;
#[doc = "Field `spi_clk_sel` writer - "]
pub type SpiClkSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `spi_swap_set` reader - "]
pub type SpiSwapSetR = crate::FieldReader;
#[doc = "Field `spi_swap_set` writer - "]
pub type SpiSwapSetW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn spi_clk_div(&self) -> SpiClkDivR {
        SpiClkDivR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn spi_clk_en(&self) -> SpiClkEnR {
        SpiClkEnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn spi_clk_sel(&self) -> SpiClkSelR {
        SpiClkSelR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn spi_swap_set(&self) -> SpiSwapSetR {
        SpiSwapSetR::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn spi_clk_div(&mut self) -> SpiClkDivW<'_, SpiCfg0Spec> {
        SpiClkDivW::new(self, 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn spi_clk_en(&mut self) -> SpiClkEnW<'_, SpiCfg0Spec> {
        SpiClkEnW::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn spi_clk_sel(&mut self) -> SpiClkSelW<'_, SpiCfg0Spec> {
        SpiClkSelW::new(self, 9)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn spi_swap_set(&mut self) -> SpiSwapSetW<'_, SpiCfg0Spec> {
        SpiSwapSetW::new(self, 16)
    }
}
#[doc = "SPI Clock Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`spi_cfg0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_cfg0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SpiCfg0Spec;
impl crate::RegisterSpec for SpiCfg0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi_cfg0::R`](R) reader structure"]
impl crate::Readable for SpiCfg0Spec {}
#[doc = "`write(|w| ..)` method takes [`spi_cfg0::W`](W) writer structure"]
impl crate::Writable for SpiCfg0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPI_CFG0 to value 0"]
impl crate::Resettable for SpiCfg0Spec {}
