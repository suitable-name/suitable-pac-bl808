#[doc = "Register `DPI_CONFIG` reader"]
pub type R = crate::R<DpiConfigSpec>;
#[doc = "Register `DPI_CONFIG` writer"]
pub type W = crate::W<DpiConfigSpec>;
#[doc = "Field `reg_dpi_en` reader - Enable DPI controller"]
pub type RegDpiEnR = crate::BitReader;
#[doc = "Field `reg_dpi_en` writer - Enable DPI controller"]
pub type RegDpiEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_dpi_h_pol` reader - HSYNC Polarity: 0: Active Low, 1: Active High"]
pub type RegDpiHPolR = crate::BitReader;
#[doc = "Field `reg_dpi_h_pol` writer - HSYNC Polarity: 0: Active Low, 1: Active High"]
pub type RegDpiHPolW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_dpi_v_pol` reader - VSYNC Polarity: 0: Active Low, 1: Active High"]
pub type RegDpiVPolR = crate::BitReader;
#[doc = "Field `reg_dpi_v_pol` writer - VSYNC Polarity: 0: Active Low, 1: Active High"]
pub type RegDpiVPolW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_dpi_de_pol` reader - Data Enable (DE) Polarity"]
pub type RegDpiDePolR = crate::BitReader;
#[doc = "Field `reg_dpi_de_pol` writer - Data Enable (DE) Polarity"]
pub type RegDpiDePolW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_pix_clk_inv` reader - Invert Pixel Clock (PCLK)"]
pub type RegPixClkInvR = crate::BitReader;
#[doc = "Field `reg_pix_clk_inv` writer - Invert Pixel Clock (PCLK)"]
pub type RegPixClkInvW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable DPI controller"]
    #[inline(always)]
    pub fn reg_dpi_en(&self) -> RegDpiEnR {
        RegDpiEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - HSYNC Polarity: 0: Active Low, 1: Active High"]
    #[inline(always)]
    pub fn reg_dpi_h_pol(&self) -> RegDpiHPolR {
        RegDpiHPolR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - VSYNC Polarity: 0: Active Low, 1: Active High"]
    #[inline(always)]
    pub fn reg_dpi_v_pol(&self) -> RegDpiVPolR {
        RegDpiVPolR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Data Enable (DE) Polarity"]
    #[inline(always)]
    pub fn reg_dpi_de_pol(&self) -> RegDpiDePolR {
        RegDpiDePolR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Invert Pixel Clock (PCLK)"]
    #[inline(always)]
    pub fn reg_pix_clk_inv(&self) -> RegPixClkInvR {
        RegPixClkInvR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable DPI controller"]
    #[inline(always)]
    pub fn reg_dpi_en(&mut self) -> RegDpiEnW<'_, DpiConfigSpec> {
        RegDpiEnW::new(self, 0)
    }
    #[doc = "Bit 1 - HSYNC Polarity: 0: Active Low, 1: Active High"]
    #[inline(always)]
    pub fn reg_dpi_h_pol(&mut self) -> RegDpiHPolW<'_, DpiConfigSpec> {
        RegDpiHPolW::new(self, 1)
    }
    #[doc = "Bit 2 - VSYNC Polarity: 0: Active Low, 1: Active High"]
    #[inline(always)]
    pub fn reg_dpi_v_pol(&mut self) -> RegDpiVPolW<'_, DpiConfigSpec> {
        RegDpiVPolW::new(self, 2)
    }
    #[doc = "Bit 3 - Data Enable (DE) Polarity"]
    #[inline(always)]
    pub fn reg_dpi_de_pol(&mut self) -> RegDpiDePolW<'_, DpiConfigSpec> {
        RegDpiDePolW::new(self, 3)
    }
    #[doc = "Bit 4 - Invert Pixel Clock (PCLK)"]
    #[inline(always)]
    pub fn reg_pix_clk_inv(&mut self) -> RegPixClkInvW<'_, DpiConfigSpec> {
        RegPixClkInvW::new(self, 4)
    }
}
#[doc = "DPI Global Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dpi_config::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dpi_config::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DpiConfigSpec;
impl crate::RegisterSpec for DpiConfigSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dpi_config::R`](R) reader structure"]
impl crate::Readable for DpiConfigSpec {}
#[doc = "`write(|w| ..)` method takes [`dpi_config::W`](W) writer structure"]
impl crate::Writable for DpiConfigSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DPI_CONFIG to value 0"]
impl crate::Resettable for DpiConfigSpec {}
