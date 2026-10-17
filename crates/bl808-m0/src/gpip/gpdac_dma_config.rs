#[doc = "Register `GPDAC_DMA_CONFIG` reader"]
pub type R = crate::R<GpdacDmaConfigSpec>;
#[doc = "Register `GPDAC_DMA_CONFIG` writer"]
pub type W = crate::W<GpdacDmaConfigSpec>;
#[doc = "Field `gpdac_dma_tx_en` reader - GPDAC DMA TX enable"]
pub type GpdacDmaTxEnR = crate::BitReader;
#[doc = "Field `gpdac_dma_tx_en` writer - GPDAC DMA TX enable"]
pub type GpdacDmaTxEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `gpdac_dma_inv_msb` reader - GPDAC DMA Data Inverse MSB"]
pub type GpdacDmaInvMsbR = crate::BitReader;
#[doc = "Field `gpdac_dma_inv_msb` writer - GPDAC DMA Data Inverse MSB"]
pub type GpdacDmaInvMsbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `gpdac_dma_format` reader - DMA TX format (Data 10-bit). Modes 0-11."]
pub type GpdacDmaFormatR = crate::FieldReader;
#[doc = "Field `gpdac_dma_format` writer - DMA TX format (Data 10-bit). Modes 0-11."]
pub type GpdacDmaFormatW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0 - GPDAC DMA TX enable"]
    #[inline(always)]
    pub fn gpdac_dma_tx_en(&self) -> GpdacDmaTxEnR {
        GpdacDmaTxEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - GPDAC DMA Data Inverse MSB"]
    #[inline(always)]
    pub fn gpdac_dma_inv_msb(&self) -> GpdacDmaInvMsbR {
        GpdacDmaInvMsbR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 4:7 - DMA TX format (Data 10-bit). Modes 0-11."]
    #[inline(always)]
    pub fn gpdac_dma_format(&self) -> GpdacDmaFormatR {
        GpdacDmaFormatR::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - GPDAC DMA TX enable"]
    #[inline(always)]
    pub fn gpdac_dma_tx_en(&mut self) -> GpdacDmaTxEnW<'_, GpdacDmaConfigSpec> {
        GpdacDmaTxEnW::new(self, 0)
    }
    #[doc = "Bit 1 - GPDAC DMA Data Inverse MSB"]
    #[inline(always)]
    pub fn gpdac_dma_inv_msb(&mut self) -> GpdacDmaInvMsbW<'_, GpdacDmaConfigSpec> {
        GpdacDmaInvMsbW::new(self, 1)
    }
    #[doc = "Bits 4:7 - DMA TX format (Data 10-bit). Modes 0-11."]
    #[inline(always)]
    pub fn gpdac_dma_format(&mut self) -> GpdacDmaFormatW<'_, GpdacDmaConfigSpec> {
        GpdacDmaFormatW::new(self, 4)
    }
}
#[doc = "GPDAC DMA Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpdac_dma_config::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpdac_dma_config::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpdacDmaConfigSpec;
impl crate::RegisterSpec for GpdacDmaConfigSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpdac_dma_config::R`](R) reader structure"]
impl crate::Readable for GpdacDmaConfigSpec {}
#[doc = "`write(|w| ..)` method takes [`gpdac_dma_config::W`](W) writer structure"]
impl crate::Writable for GpdacDmaConfigSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPDAC_DMA_CONFIG to value 0"]
impl crate::Resettable for GpdacDmaConfigSpec {}
