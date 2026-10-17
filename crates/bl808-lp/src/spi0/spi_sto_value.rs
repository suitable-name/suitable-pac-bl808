#[doc = "Register `SPI_STO_VALUE` reader"]
pub type R = crate::R<SpiStoValueSpec>;
#[doc = "Register `SPI_STO_VALUE` writer"]
pub type W = crate::W<SpiStoValueSpec>;
#[doc = "Field `cr_spi_sto_value` reader - Slave-mode timeout limit (cycles)"]
pub type CrSpiStoValueR = crate::FieldReader<u16>;
#[doc = "Field `cr_spi_sto_value` writer - Slave-mode timeout limit (cycles)"]
pub type CrSpiStoValueW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - Slave-mode timeout limit (cycles)"]
    #[inline(always)]
    pub fn cr_spi_sto_value(&self) -> CrSpiStoValueR {
        CrSpiStoValueR::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Slave-mode timeout limit (cycles)"]
    #[inline(always)]
    pub fn cr_spi_sto_value(&mut self) -> CrSpiStoValueW<'_, SpiStoValueSpec> {
        CrSpiStoValueW::new(self, 0)
    }
}
#[doc = "SPI Slave Timeout Value\n\nYou can [`read`](crate::Reg::read) this register and get [`spi_sto_value::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_sto_value::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SpiStoValueSpec;
impl crate::RegisterSpec for SpiStoValueSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi_sto_value::R`](R) reader structure"]
impl crate::Readable for SpiStoValueSpec {}
#[doc = "`write(|w| ..)` method takes [`spi_sto_value::W`](W) writer structure"]
impl crate::Writable for SpiStoValueSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPI_STO_VALUE to value 0x0fff"]
impl crate::Resettable for SpiStoValueSpec {
    const RESET_VALUE: u32 = 0x0fff;
}
