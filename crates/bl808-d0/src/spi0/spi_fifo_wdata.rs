#[doc = "Register `SPI_FIFO_WDATA` reader"]
pub type R = crate::R<SpiFifoWdataSpec>;
#[doc = "Register `SPI_FIFO_WDATA` writer"]
pub type W = crate::W<SpiFifoWdataSpec>;
#[doc = "Field `spi_fifo_wdata` writer - Push data into TX FIFO"]
pub type SpiFifoWdataW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - Push data into TX FIFO"]
    #[inline(always)]
    pub fn spi_fifo_wdata(&mut self) -> SpiFifoWdataW<'_, SpiFifoWdataSpec> {
        SpiFifoWdataW::new(self, 0)
    }
}
#[doc = "SPI TX FIFO Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`spi_fifo_wdata::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_fifo_wdata::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SpiFifoWdataSpec;
impl crate::RegisterSpec for SpiFifoWdataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi_fifo_wdata::R`](R) reader structure"]
impl crate::Readable for SpiFifoWdataSpec {}
#[doc = "`write(|w| ..)` method takes [`spi_fifo_wdata::W`](W) writer structure"]
impl crate::Writable for SpiFifoWdataSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPI_FIFO_WDATA to value 0"]
impl crate::Resettable for SpiFifoWdataSpec {}
