#[doc = "Register `SPI_FIFO_RDATA` reader"]
pub type R = crate::R<SpiFifoRdataSpec>;
#[doc = "Register `SPI_FIFO_RDATA` writer"]
pub type W = crate::W<SpiFifoRdataSpec>;
#[doc = "Field `spi_fifo_rdata` reader - Pop data from RX FIFO"]
pub type SpiFifoRdataR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Pop data from RX FIFO"]
    #[inline(always)]
    pub fn spi_fifo_rdata(&self) -> SpiFifoRdataR {
        SpiFifoRdataR::new(self.bits)
    }
}
impl W {}
#[doc = "SPI RX FIFO Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`spi_fifo_rdata::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_fifo_rdata::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SpiFifoRdataSpec;
impl crate::RegisterSpec for SpiFifoRdataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi_fifo_rdata::R`](R) reader structure"]
impl crate::Readable for SpiFifoRdataSpec {}
#[doc = "`write(|w| ..)` method takes [`spi_fifo_rdata::W`](W) writer structure"]
impl crate::Writable for SpiFifoRdataSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPI_FIFO_RDATA to value 0"]
impl crate::Resettable for SpiFifoRdataSpec {}
