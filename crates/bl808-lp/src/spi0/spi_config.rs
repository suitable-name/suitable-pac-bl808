#[doc = "Register `SPI_CONFIG` reader"]
pub type R = crate::R<SpiConfigSpec>;
#[doc = "Register `SPI_CONFIG` writer"]
pub type W = crate::W<SpiConfigSpec>;
#[doc = "Field `cr_spi_m_en` reader - Master mode enable"]
pub type CrSpiMEnR = crate::BitReader;
#[doc = "Field `cr_spi_m_en` writer - Master mode enable"]
pub type CrSpiMEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_spi_s_en` reader - Slave mode enable"]
pub type CrSpiSEnR = crate::BitReader;
#[doc = "Field `cr_spi_s_en` writer - Slave mode enable"]
pub type CrSpiSEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_spi_frame_size` reader - Frame size: 0:8-bit, 1:16-bit, 2:24-bit, 3:32-bit"]
pub type CrSpiFrameSizeR = crate::FieldReader;
#[doc = "Field `cr_spi_frame_size` writer - Frame size: 0:8-bit, 1:16-bit, 2:24-bit, 3:32-bit"]
pub type CrSpiFrameSizeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `cr_spi_sclk_pol` reader - Clock polarity (CPOL)"]
pub type CrSpiSclkPolR = crate::BitReader;
#[doc = "Field `cr_spi_sclk_pol` writer - Clock polarity (CPOL)"]
pub type CrSpiSclkPolW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_spi_sclk_ph` reader - Clock phase (CPHA)"]
pub type CrSpiSclkPhR = crate::BitReader;
#[doc = "Field `cr_spi_sclk_ph` writer - Clock phase (CPHA)"]
pub type CrSpiSclkPhW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_spi_bit_inv` reader - Bit endianness: 0:MSB first, 1:LSB first"]
pub type CrSpiBitInvR = crate::BitReader;
#[doc = "Field `cr_spi_bit_inv` writer - Bit endianness: 0:MSB first, 1:LSB first"]
pub type CrSpiBitInvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_spi_byte_inv` reader - Byte endianness inversion"]
pub type CrSpiByteInvR = crate::BitReader;
#[doc = "Field `cr_spi_byte_inv` writer - Byte endianness inversion"]
pub type CrSpiByteInvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_spi_rxd_ignr_en` reader - RX data ignore enable"]
pub type CrSpiRxdIgnrEnR = crate::BitReader;
#[doc = "Field `cr_spi_rxd_ignr_en` writer - RX data ignore enable"]
pub type CrSpiRxdIgnrEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_spi_m_cont_en` reader - Master continuous transfer enable"]
pub type CrSpiMContEnR = crate::BitReader;
#[doc = "Field `cr_spi_m_cont_en` writer - Master continuous transfer enable"]
pub type CrSpiMContEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_spi_s_3pin_mode` reader - Slave 3-pin mode enable"]
pub type CrSpiS3pinModeR = crate::BitReader;
#[doc = "Field `cr_spi_s_3pin_mode` writer - Slave 3-pin mode enable"]
pub type CrSpiS3pinModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_spi_deg_en` reader - Deglitch function enable"]
pub type CrSpiDegEnR = crate::BitReader;
#[doc = "Field `cr_spi_deg_en` writer - Deglitch function enable"]
pub type CrSpiDegEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_spi_deg_cnt` reader - Deglitch cycle count"]
pub type CrSpiDegCntR = crate::FieldReader;
#[doc = "Field `cr_spi_deg_cnt` writer - Deglitch cycle count"]
pub type CrSpiDegCntW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0 - Master mode enable"]
    #[inline(always)]
    pub fn cr_spi_m_en(&self) -> CrSpiMEnR {
        CrSpiMEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Slave mode enable"]
    #[inline(always)]
    pub fn cr_spi_s_en(&self) -> CrSpiSEnR {
        CrSpiSEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Frame size: 0:8-bit, 1:16-bit, 2:24-bit, 3:32-bit"]
    #[inline(always)]
    pub fn cr_spi_frame_size(&self) -> CrSpiFrameSizeR {
        CrSpiFrameSizeR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - Clock polarity (CPOL)"]
    #[inline(always)]
    pub fn cr_spi_sclk_pol(&self) -> CrSpiSclkPolR {
        CrSpiSclkPolR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Clock phase (CPHA)"]
    #[inline(always)]
    pub fn cr_spi_sclk_ph(&self) -> CrSpiSclkPhR {
        CrSpiSclkPhR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Bit endianness: 0:MSB first, 1:LSB first"]
    #[inline(always)]
    pub fn cr_spi_bit_inv(&self) -> CrSpiBitInvR {
        CrSpiBitInvR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Byte endianness inversion"]
    #[inline(always)]
    pub fn cr_spi_byte_inv(&self) -> CrSpiByteInvR {
        CrSpiByteInvR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - RX data ignore enable"]
    #[inline(always)]
    pub fn cr_spi_rxd_ignr_en(&self) -> CrSpiRxdIgnrEnR {
        CrSpiRxdIgnrEnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Master continuous transfer enable"]
    #[inline(always)]
    pub fn cr_spi_m_cont_en(&self) -> CrSpiMContEnR {
        CrSpiMContEnR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Slave 3-pin mode enable"]
    #[inline(always)]
    pub fn cr_spi_s_3pin_mode(&self) -> CrSpiS3pinModeR {
        CrSpiS3pinModeR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Deglitch function enable"]
    #[inline(always)]
    pub fn cr_spi_deg_en(&self) -> CrSpiDegEnR {
        CrSpiDegEnR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:15 - Deglitch cycle count"]
    #[inline(always)]
    pub fn cr_spi_deg_cnt(&self) -> CrSpiDegCntR {
        CrSpiDegCntR::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Master mode enable"]
    #[inline(always)]
    pub fn cr_spi_m_en(&mut self) -> CrSpiMEnW<'_, SpiConfigSpec> {
        CrSpiMEnW::new(self, 0)
    }
    #[doc = "Bit 1 - Slave mode enable"]
    #[inline(always)]
    pub fn cr_spi_s_en(&mut self) -> CrSpiSEnW<'_, SpiConfigSpec> {
        CrSpiSEnW::new(self, 1)
    }
    #[doc = "Bits 2:3 - Frame size: 0:8-bit, 1:16-bit, 2:24-bit, 3:32-bit"]
    #[inline(always)]
    pub fn cr_spi_frame_size(&mut self) -> CrSpiFrameSizeW<'_, SpiConfigSpec> {
        CrSpiFrameSizeW::new(self, 2)
    }
    #[doc = "Bit 4 - Clock polarity (CPOL)"]
    #[inline(always)]
    pub fn cr_spi_sclk_pol(&mut self) -> CrSpiSclkPolW<'_, SpiConfigSpec> {
        CrSpiSclkPolW::new(self, 4)
    }
    #[doc = "Bit 5 - Clock phase (CPHA)"]
    #[inline(always)]
    pub fn cr_spi_sclk_ph(&mut self) -> CrSpiSclkPhW<'_, SpiConfigSpec> {
        CrSpiSclkPhW::new(self, 5)
    }
    #[doc = "Bit 6 - Bit endianness: 0:MSB first, 1:LSB first"]
    #[inline(always)]
    pub fn cr_spi_bit_inv(&mut self) -> CrSpiBitInvW<'_, SpiConfigSpec> {
        CrSpiBitInvW::new(self, 6)
    }
    #[doc = "Bit 7 - Byte endianness inversion"]
    #[inline(always)]
    pub fn cr_spi_byte_inv(&mut self) -> CrSpiByteInvW<'_, SpiConfigSpec> {
        CrSpiByteInvW::new(self, 7)
    }
    #[doc = "Bit 8 - RX data ignore enable"]
    #[inline(always)]
    pub fn cr_spi_rxd_ignr_en(&mut self) -> CrSpiRxdIgnrEnW<'_, SpiConfigSpec> {
        CrSpiRxdIgnrEnW::new(self, 8)
    }
    #[doc = "Bit 9 - Master continuous transfer enable"]
    #[inline(always)]
    pub fn cr_spi_m_cont_en(&mut self) -> CrSpiMContEnW<'_, SpiConfigSpec> {
        CrSpiMContEnW::new(self, 9)
    }
    #[doc = "Bit 10 - Slave 3-pin mode enable"]
    #[inline(always)]
    pub fn cr_spi_s_3pin_mode(&mut self) -> CrSpiS3pinModeW<'_, SpiConfigSpec> {
        CrSpiS3pinModeW::new(self, 10)
    }
    #[doc = "Bit 11 - Deglitch function enable"]
    #[inline(always)]
    pub fn cr_spi_deg_en(&mut self) -> CrSpiDegEnW<'_, SpiConfigSpec> {
        CrSpiDegEnW::new(self, 11)
    }
    #[doc = "Bits 12:15 - Deglitch cycle count"]
    #[inline(always)]
    pub fn cr_spi_deg_cnt(&mut self) -> CrSpiDegCntW<'_, SpiConfigSpec> {
        CrSpiDegCntW::new(self, 12)
    }
}
#[doc = "SPI Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`spi_config::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_config::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SpiConfigSpec;
impl crate::RegisterSpec for SpiConfigSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi_config::R`](R) reader structure"]
impl crate::Readable for SpiConfigSpec {}
#[doc = "`write(|w| ..)` method takes [`spi_config::W`](W) writer structure"]
impl crate::Writable for SpiConfigSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPI_CONFIG to value 0"]
impl crate::Resettable for SpiConfigSpec {}
