#[doc = "Register `DBI_FIFO_CONFIG_0` reader"]
pub type R = crate::R<DbiFifoConfig0Spec>;
#[doc = "Register `DBI_FIFO_CONFIG_0` writer"]
pub type W = crate::W<DbiFifoConfig0Spec>;
#[doc = "Field `dbi_dma_tx_en` reader - TX DMA enable"]
pub type DbiDmaTxEnR = crate::BitReader;
#[doc = "Field `dbi_dma_tx_en` writer - TX DMA enable"]
pub type DbiDmaTxEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tx_fifo_clr` writer - Clear TX FIFO"]
pub type TxFifoClrW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `tx_fifo_overflow` reader - TX FIFO overflow status"]
pub type TxFifoOverflowR = crate::BitReader;
#[doc = "Field `tx_fifo_underflow` reader - TX FIFO underflow status"]
pub type TxFifoUnderflowR = crate::BitReader;
#[doc = "Field `fifo_format` reader - FIFO data format selection"]
pub type FifoFormatR = crate::FieldReader;
#[doc = "Field `fifo_format` writer - FIFO data format selection"]
pub type FifoFormatW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bit 0 - TX DMA enable"]
    #[inline(always)]
    pub fn dbi_dma_tx_en(&self) -> DbiDmaTxEnR {
        DbiDmaTxEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - TX FIFO overflow status"]
    #[inline(always)]
    pub fn tx_fifo_overflow(&self) -> TxFifoOverflowR {
        TxFifoOverflowR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - TX FIFO underflow status"]
    #[inline(always)]
    pub fn tx_fifo_underflow(&self) -> TxFifoUnderflowR {
        TxFifoUnderflowR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 29:31 - FIFO data format selection"]
    #[inline(always)]
    pub fn fifo_format(&self) -> FifoFormatR {
        FifoFormatR::new(((self.bits >> 29) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - TX DMA enable"]
    #[inline(always)]
    pub fn dbi_dma_tx_en(&mut self) -> DbiDmaTxEnW<'_, DbiFifoConfig0Spec> {
        DbiDmaTxEnW::new(self, 0)
    }
    #[doc = "Bit 2 - Clear TX FIFO"]
    #[inline(always)]
    pub fn tx_fifo_clr(&mut self) -> TxFifoClrW<'_, DbiFifoConfig0Spec> {
        TxFifoClrW::new(self, 2)
    }
    #[doc = "Bits 29:31 - FIFO data format selection"]
    #[inline(always)]
    pub fn fifo_format(&mut self) -> FifoFormatW<'_, DbiFifoConfig0Spec> {
        FifoFormatW::new(self, 29)
    }
}
#[doc = "DBI FIFO Configuration 0\n\nYou can [`read`](crate::Reg::read) this register and get [`dbi_fifo_config_0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbi_fifo_config_0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DbiFifoConfig0Spec;
impl crate::RegisterSpec for DbiFifoConfig0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dbi_fifo_config_0::R`](R) reader structure"]
impl crate::Readable for DbiFifoConfig0Spec {}
#[doc = "`write(|w| ..)` method takes [`dbi_fifo_config_0::W`](W) writer structure"]
impl crate::Writable for DbiFifoConfig0Spec {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x04;
}
#[doc = "`reset()` method sets DBI_FIFO_CONFIG_0 to value 0"]
impl crate::Resettable for DbiFifoConfig0Spec {}
