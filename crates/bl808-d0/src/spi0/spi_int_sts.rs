#[doc = "Register `SPI_INT_STS` reader"]
pub type R = crate::R<SpiIntStsSpec>;
#[doc = "Register `SPI_INT_STS` writer"]
pub type W = crate::W<SpiIntStsSpec>;
#[doc = "Field `spi_end_int` reader - Transfer end interrupt status"]
pub type SpiEndIntR = crate::BitReader;
#[doc = "Field `spi_txf_int` reader - TX FIFO ready interrupt status"]
pub type SpiTxfIntR = crate::BitReader;
#[doc = "Field `spi_rxf_int` reader - RX FIFO ready interrupt status"]
pub type SpiRxfIntR = crate::BitReader;
#[doc = "Field `spi_sto_int` reader - Slave-mode timeout interrupt status"]
pub type SpiStoIntR = crate::BitReader;
#[doc = "Field `spi_txu_int` reader - TX underflow interrupt status"]
pub type SpiTxuIntR = crate::BitReader;
#[doc = "Field `spi_fer_int` reader - FIFO error interrupt status"]
pub type SpiFerIntR = crate::BitReader;
#[doc = "Field `cr_spi_end_mask` reader - Mask transfer end interrupt"]
pub type CrSpiEndMaskR = crate::BitReader;
#[doc = "Field `cr_spi_end_mask` writer - Mask transfer end interrupt"]
pub type CrSpiEndMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_spi_txf_mask` reader - Mask TX FIFO ready interrupt"]
pub type CrSpiTxfMaskR = crate::BitReader;
#[doc = "Field `cr_spi_txf_mask` writer - Mask TX FIFO ready interrupt"]
pub type CrSpiTxfMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_spi_rxf_mask` reader - Mask RX FIFO ready interrupt"]
pub type CrSpiRxfMaskR = crate::BitReader;
#[doc = "Field `cr_spi_rxf_mask` writer - Mask RX FIFO ready interrupt"]
pub type CrSpiRxfMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_spi_sto_mask` reader - Mask slave timeout interrupt"]
pub type CrSpiStoMaskR = crate::BitReader;
#[doc = "Field `cr_spi_sto_mask` writer - Mask slave timeout interrupt"]
pub type CrSpiStoMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_spi_txu_mask` reader - Mask TX underflow interrupt"]
pub type CrSpiTxuMaskR = crate::BitReader;
#[doc = "Field `cr_spi_txu_mask` writer - Mask TX underflow interrupt"]
pub type CrSpiTxuMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_spi_fer_mask` reader - Mask FIFO error interrupt"]
pub type CrSpiFerMaskR = crate::BitReader;
#[doc = "Field `cr_spi_fer_mask` writer - Mask FIFO error interrupt"]
pub type CrSpiFerMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_spi_end_clr` writer - Clear transfer end interrupt"]
pub type CrSpiEndClrW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `cr_spi_sto_clr` writer - Clear slave timeout interrupt"]
pub type CrSpiStoClrW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `cr_spi_txu_clr` writer - Clear TX underflow interrupt"]
pub type CrSpiTxuClrW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `cr_spi_end_en` reader - Enable transfer end interrupt"]
pub type CrSpiEndEnR = crate::BitReader;
#[doc = "Field `cr_spi_end_en` writer - Enable transfer end interrupt"]
pub type CrSpiEndEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_spi_txf_en` reader - Enable TX FIFO ready interrupt"]
pub type CrSpiTxfEnR = crate::BitReader;
#[doc = "Field `cr_spi_txf_en` writer - Enable TX FIFO ready interrupt"]
pub type CrSpiTxfEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_spi_rxf_en` reader - Enable RX FIFO ready interrupt"]
pub type CrSpiRxfEnR = crate::BitReader;
#[doc = "Field `cr_spi_rxf_en` writer - Enable RX FIFO ready interrupt"]
pub type CrSpiRxfEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_spi_sto_en` reader - Enable slave timeout interrupt"]
pub type CrSpiStoEnR = crate::BitReader;
#[doc = "Field `cr_spi_sto_en` writer - Enable slave timeout interrupt"]
pub type CrSpiStoEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_spi_txu_en` reader - Enable TX underflow interrupt"]
pub type CrSpiTxuEnR = crate::BitReader;
#[doc = "Field `cr_spi_txu_en` writer - Enable TX underflow interrupt"]
pub type CrSpiTxuEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_spi_fer_en` reader - Enable FIFO error interrupt"]
pub type CrSpiFerEnR = crate::BitReader;
#[doc = "Field `cr_spi_fer_en` writer - Enable FIFO error interrupt"]
pub type CrSpiFerEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Transfer end interrupt status"]
    #[inline(always)]
    pub fn spi_end_int(&self) -> SpiEndIntR {
        SpiEndIntR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TX FIFO ready interrupt status"]
    #[inline(always)]
    pub fn spi_txf_int(&self) -> SpiTxfIntR {
        SpiTxfIntR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RX FIFO ready interrupt status"]
    #[inline(always)]
    pub fn spi_rxf_int(&self) -> SpiRxfIntR {
        SpiRxfIntR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Slave-mode timeout interrupt status"]
    #[inline(always)]
    pub fn spi_sto_int(&self) -> SpiStoIntR {
        SpiStoIntR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - TX underflow interrupt status"]
    #[inline(always)]
    pub fn spi_txu_int(&self) -> SpiTxuIntR {
        SpiTxuIntR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - FIFO error interrupt status"]
    #[inline(always)]
    pub fn spi_fer_int(&self) -> SpiFerIntR {
        SpiFerIntR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - Mask transfer end interrupt"]
    #[inline(always)]
    pub fn cr_spi_end_mask(&self) -> CrSpiEndMaskR {
        CrSpiEndMaskR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Mask TX FIFO ready interrupt"]
    #[inline(always)]
    pub fn cr_spi_txf_mask(&self) -> CrSpiTxfMaskR {
        CrSpiTxfMaskR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Mask RX FIFO ready interrupt"]
    #[inline(always)]
    pub fn cr_spi_rxf_mask(&self) -> CrSpiRxfMaskR {
        CrSpiRxfMaskR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Mask slave timeout interrupt"]
    #[inline(always)]
    pub fn cr_spi_sto_mask(&self) -> CrSpiStoMaskR {
        CrSpiStoMaskR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Mask TX underflow interrupt"]
    #[inline(always)]
    pub fn cr_spi_txu_mask(&self) -> CrSpiTxuMaskR {
        CrSpiTxuMaskR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Mask FIFO error interrupt"]
    #[inline(always)]
    pub fn cr_spi_fer_mask(&self) -> CrSpiFerMaskR {
        CrSpiFerMaskR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 24 - Enable transfer end interrupt"]
    #[inline(always)]
    pub fn cr_spi_end_en(&self) -> CrSpiEndEnR {
        CrSpiEndEnR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Enable TX FIFO ready interrupt"]
    #[inline(always)]
    pub fn cr_spi_txf_en(&self) -> CrSpiTxfEnR {
        CrSpiTxfEnR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Enable RX FIFO ready interrupt"]
    #[inline(always)]
    pub fn cr_spi_rxf_en(&self) -> CrSpiRxfEnR {
        CrSpiRxfEnR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Enable slave timeout interrupt"]
    #[inline(always)]
    pub fn cr_spi_sto_en(&self) -> CrSpiStoEnR {
        CrSpiStoEnR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Enable TX underflow interrupt"]
    #[inline(always)]
    pub fn cr_spi_txu_en(&self) -> CrSpiTxuEnR {
        CrSpiTxuEnR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Enable FIFO error interrupt"]
    #[inline(always)]
    pub fn cr_spi_fer_en(&self) -> CrSpiFerEnR {
        CrSpiFerEnR::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - Mask transfer end interrupt"]
    #[inline(always)]
    pub fn cr_spi_end_mask(&mut self) -> CrSpiEndMaskW<'_, SpiIntStsSpec> {
        CrSpiEndMaskW::new(self, 8)
    }
    #[doc = "Bit 9 - Mask TX FIFO ready interrupt"]
    #[inline(always)]
    pub fn cr_spi_txf_mask(&mut self) -> CrSpiTxfMaskW<'_, SpiIntStsSpec> {
        CrSpiTxfMaskW::new(self, 9)
    }
    #[doc = "Bit 10 - Mask RX FIFO ready interrupt"]
    #[inline(always)]
    pub fn cr_spi_rxf_mask(&mut self) -> CrSpiRxfMaskW<'_, SpiIntStsSpec> {
        CrSpiRxfMaskW::new(self, 10)
    }
    #[doc = "Bit 11 - Mask slave timeout interrupt"]
    #[inline(always)]
    pub fn cr_spi_sto_mask(&mut self) -> CrSpiStoMaskW<'_, SpiIntStsSpec> {
        CrSpiStoMaskW::new(self, 11)
    }
    #[doc = "Bit 12 - Mask TX underflow interrupt"]
    #[inline(always)]
    pub fn cr_spi_txu_mask(&mut self) -> CrSpiTxuMaskW<'_, SpiIntStsSpec> {
        CrSpiTxuMaskW::new(self, 12)
    }
    #[doc = "Bit 13 - Mask FIFO error interrupt"]
    #[inline(always)]
    pub fn cr_spi_fer_mask(&mut self) -> CrSpiFerMaskW<'_, SpiIntStsSpec> {
        CrSpiFerMaskW::new(self, 13)
    }
    #[doc = "Bit 16 - Clear transfer end interrupt"]
    #[inline(always)]
    pub fn cr_spi_end_clr(&mut self) -> CrSpiEndClrW<'_, SpiIntStsSpec> {
        CrSpiEndClrW::new(self, 16)
    }
    #[doc = "Bit 19 - Clear slave timeout interrupt"]
    #[inline(always)]
    pub fn cr_spi_sto_clr(&mut self) -> CrSpiStoClrW<'_, SpiIntStsSpec> {
        CrSpiStoClrW::new(self, 19)
    }
    #[doc = "Bit 20 - Clear TX underflow interrupt"]
    #[inline(always)]
    pub fn cr_spi_txu_clr(&mut self) -> CrSpiTxuClrW<'_, SpiIntStsSpec> {
        CrSpiTxuClrW::new(self, 20)
    }
    #[doc = "Bit 24 - Enable transfer end interrupt"]
    #[inline(always)]
    pub fn cr_spi_end_en(&mut self) -> CrSpiEndEnW<'_, SpiIntStsSpec> {
        CrSpiEndEnW::new(self, 24)
    }
    #[doc = "Bit 25 - Enable TX FIFO ready interrupt"]
    #[inline(always)]
    pub fn cr_spi_txf_en(&mut self) -> CrSpiTxfEnW<'_, SpiIntStsSpec> {
        CrSpiTxfEnW::new(self, 25)
    }
    #[doc = "Bit 26 - Enable RX FIFO ready interrupt"]
    #[inline(always)]
    pub fn cr_spi_rxf_en(&mut self) -> CrSpiRxfEnW<'_, SpiIntStsSpec> {
        CrSpiRxfEnW::new(self, 26)
    }
    #[doc = "Bit 27 - Enable slave timeout interrupt"]
    #[inline(always)]
    pub fn cr_spi_sto_en(&mut self) -> CrSpiStoEnW<'_, SpiIntStsSpec> {
        CrSpiStoEnW::new(self, 27)
    }
    #[doc = "Bit 28 - Enable TX underflow interrupt"]
    #[inline(always)]
    pub fn cr_spi_txu_en(&mut self) -> CrSpiTxuEnW<'_, SpiIntStsSpec> {
        CrSpiTxuEnW::new(self, 28)
    }
    #[doc = "Bit 29 - Enable FIFO error interrupt"]
    #[inline(always)]
    pub fn cr_spi_fer_en(&mut self) -> CrSpiFerEnW<'_, SpiIntStsSpec> {
        CrSpiFerEnW::new(self, 29)
    }
}
#[doc = "SPI Interrupt Status and Control\n\nYou can [`read`](crate::Reg::read) this register and get [`spi_int_sts::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_int_sts::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SpiIntStsSpec;
impl crate::RegisterSpec for SpiIntStsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi_int_sts::R`](R) reader structure"]
impl crate::Readable for SpiIntStsSpec {}
#[doc = "`write(|w| ..)` method takes [`spi_int_sts::W`](W) writer structure"]
impl crate::Writable for SpiIntStsSpec {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x0019_0000;
}
#[doc = "`reset()` method sets SPI_INT_STS to value 0x3f00_3f02"]
impl crate::Resettable for SpiIntStsSpec {
    const RESET_VALUE: u32 = 0x3f00_3f02;
}
