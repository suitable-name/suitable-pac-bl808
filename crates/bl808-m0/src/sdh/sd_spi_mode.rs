#[doc = "Register `SD_SPI_MODE` reader"]
pub type R = crate::R<SdSpiModeSpec>;
#[doc = "Register `SD_SPI_MODE` writer"]
pub type W = crate::W<SdSpiModeSpec>;
#[doc = "Field `SPI_EN` reader - "]
pub type SpiEnR = crate::BitReader;
#[doc = "Field `SPI_EN` writer - "]
pub type SpiEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_ERR_TOKEN` reader - "]
pub type SpiErrTokenR = crate::FieldReader;
#[doc = "Field `SPI_ERR_TOKEN` writer - "]
pub type SpiErrTokenW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn spi_en(&self) -> SpiEnR {
        SpiEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:12"]
    #[inline(always)]
    pub fn spi_err_token(&self) -> SpiErrTokenR {
        SpiErrTokenR::new(((self.bits >> 8) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn spi_en(&mut self) -> SpiEnW<'_, SdSpiModeSpec> {
        SpiEnW::new(self, 0)
    }
    #[doc = "Bits 8:12"]
    #[inline(always)]
    pub fn spi_err_token(&mut self) -> SpiErrTokenW<'_, SdSpiModeSpec> {
        SpiErrTokenW::new(self, 8)
    }
}
#[doc = "SD legacy SPI mode control and error tokens\n\nYou can [`read`](crate::Reg::read) this register and get [`sd_spi_mode::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sd_spi_mode::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdSpiModeSpec;
impl crate::RegisterSpec for SdSpiModeSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`sd_spi_mode::R`](R) reader structure"]
impl crate::Readable for SdSpiModeSpec {}
#[doc = "`write(|w| ..)` method takes [`sd_spi_mode::W`](W) writer structure"]
impl crate::Writable for SdSpiModeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SD_SPI_MODE to value 0"]
impl crate::Resettable for SdSpiModeSpec {}
