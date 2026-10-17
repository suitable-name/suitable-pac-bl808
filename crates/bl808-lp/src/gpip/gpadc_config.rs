#[doc = "Register `GPADC_CONFIG` reader"]
pub type R = crate::R<GpadcConfigSpec>;
#[doc = "Register `GPADC_CONFIG` writer"]
pub type W = crate::W<GpadcConfigSpec>;
#[doc = "Field `gpadc_dma_en` reader - GPADC DMA Enable"]
pub type GpadcDmaEnR = crate::BitReader;
#[doc = "Field `gpadc_dma_en` writer - GPADC DMA Enable"]
pub type GpadcDmaEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `gpadc_fifo_clr` reader - FIFO Clear Signal"]
pub type GpadcFifoClrR = crate::BitReader;
#[doc = "Field `gpadc_fifo_clr` writer - FIFO Clear Signal"]
pub type GpadcFifoClrW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `gpadc_fifo_ne` reader - FIFO Not Empty Flag"]
pub type GpadcFifoNeR = crate::BitReader;
#[doc = "Field `gpadc_fifo_full` reader - FIFO Full Flag"]
pub type GpadcFifoFullR = crate::BitReader;
#[doc = "Field `gpadc_rdy` reader - Conversion Data Ready Interrupt Flag"]
pub type GpadcRdyR = crate::BitReader;
#[doc = "Field `gpadc_fifo_overrun` reader - FIFO Overrun Interrupt Flag"]
pub type GpadcFifoOverrunR = crate::BitReader;
#[doc = "Field `gpadc_fifo_underrun` reader - FIFO Underrun Interrupt Flag"]
pub type GpadcFifoUnderrunR = crate::BitReader;
#[doc = "Field `gpadc_rdy_clr` reader - Clear Data Ready Interrupt"]
pub type GpadcRdyClrR = crate::BitReader;
#[doc = "Field `gpadc_rdy_clr` writer - Clear Data Ready Interrupt"]
pub type GpadcRdyClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `gpadc_fifo_overrun_clr` reader - Clear Overrun Interrupt"]
pub type GpadcFifoOverrunClrR = crate::BitReader;
#[doc = "Field `gpadc_fifo_overrun_clr` writer - Clear Overrun Interrupt"]
pub type GpadcFifoOverrunClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `gpadc_fifo_underrun_clr` reader - Clear Underrun Interrupt"]
pub type GpadcFifoUnderrunClrR = crate::BitReader;
#[doc = "Field `gpadc_fifo_underrun_clr` writer - Clear Underrun Interrupt"]
pub type GpadcFifoUnderrunClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `gpadc_rdy_mask` reader - Mask Data Ready Interrupt"]
pub type GpadcRdyMaskR = crate::BitReader;
#[doc = "Field `gpadc_rdy_mask` writer - Mask Data Ready Interrupt"]
pub type GpadcRdyMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `gpadc_fifo_overrun_mask` reader - Mask Overrun Interrupt"]
pub type GpadcFifoOverrunMaskR = crate::BitReader;
#[doc = "Field `gpadc_fifo_overrun_mask` writer - Mask Overrun Interrupt"]
pub type GpadcFifoOverrunMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `gpadc_fifo_underrun_mask` reader - Mask Underrun Interrupt"]
pub type GpadcFifoUnderrunMaskR = crate::BitReader;
#[doc = "Field `gpadc_fifo_underrun_mask` writer - Mask Underrun Interrupt"]
pub type GpadcFifoUnderrunMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `gpadc_fifo_data_count` reader - Number of data points in FIFO"]
pub type GpadcFifoDataCountR = crate::FieldReader;
#[doc = "Field `gpadc_fifo_thl` reader - FIFO Threshold: 0:1, 1:4, 2:8, 3:16 data items"]
pub type GpadcFifoThlR = crate::FieldReader;
#[doc = "Field `gpadc_fifo_thl` writer - FIFO Threshold: 0:1, 1:4, 2:8, 3:16 data items"]
pub type GpadcFifoThlW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - GPADC DMA Enable"]
    #[inline(always)]
    pub fn gpadc_dma_en(&self) -> GpadcDmaEnR {
        GpadcDmaEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - FIFO Clear Signal"]
    #[inline(always)]
    pub fn gpadc_fifo_clr(&self) -> GpadcFifoClrR {
        GpadcFifoClrR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - FIFO Not Empty Flag"]
    #[inline(always)]
    pub fn gpadc_fifo_ne(&self) -> GpadcFifoNeR {
        GpadcFifoNeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - FIFO Full Flag"]
    #[inline(always)]
    pub fn gpadc_fifo_full(&self) -> GpadcFifoFullR {
        GpadcFifoFullR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Conversion Data Ready Interrupt Flag"]
    #[inline(always)]
    pub fn gpadc_rdy(&self) -> GpadcRdyR {
        GpadcRdyR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - FIFO Overrun Interrupt Flag"]
    #[inline(always)]
    pub fn gpadc_fifo_overrun(&self) -> GpadcFifoOverrunR {
        GpadcFifoOverrunR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - FIFO Underrun Interrupt Flag"]
    #[inline(always)]
    pub fn gpadc_fifo_underrun(&self) -> GpadcFifoUnderrunR {
        GpadcFifoUnderrunR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - Clear Data Ready Interrupt"]
    #[inline(always)]
    pub fn gpadc_rdy_clr(&self) -> GpadcRdyClrR {
        GpadcRdyClrR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Clear Overrun Interrupt"]
    #[inline(always)]
    pub fn gpadc_fifo_overrun_clr(&self) -> GpadcFifoOverrunClrR {
        GpadcFifoOverrunClrR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Clear Underrun Interrupt"]
    #[inline(always)]
    pub fn gpadc_fifo_underrun_clr(&self) -> GpadcFifoUnderrunClrR {
        GpadcFifoUnderrunClrR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 12 - Mask Data Ready Interrupt"]
    #[inline(always)]
    pub fn gpadc_rdy_mask(&self) -> GpadcRdyMaskR {
        GpadcRdyMaskR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Mask Overrun Interrupt"]
    #[inline(always)]
    pub fn gpadc_fifo_overrun_mask(&self) -> GpadcFifoOverrunMaskR {
        GpadcFifoOverrunMaskR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Mask Underrun Interrupt"]
    #[inline(always)]
    pub fn gpadc_fifo_underrun_mask(&self) -> GpadcFifoUnderrunMaskR {
        GpadcFifoUnderrunMaskR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 16:21 - Number of data points in FIFO"]
    #[inline(always)]
    pub fn gpadc_fifo_data_count(&self) -> GpadcFifoDataCountR {
        GpadcFifoDataCountR::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 22:23 - FIFO Threshold: 0:1, 1:4, 2:8, 3:16 data items"]
    #[inline(always)]
    pub fn gpadc_fifo_thl(&self) -> GpadcFifoThlR {
        GpadcFifoThlR::new(((self.bits >> 22) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - GPADC DMA Enable"]
    #[inline(always)]
    pub fn gpadc_dma_en(&mut self) -> GpadcDmaEnW<'_, GpadcConfigSpec> {
        GpadcDmaEnW::new(self, 0)
    }
    #[doc = "Bit 1 - FIFO Clear Signal"]
    #[inline(always)]
    pub fn gpadc_fifo_clr(&mut self) -> GpadcFifoClrW<'_, GpadcConfigSpec> {
        GpadcFifoClrW::new(self, 1)
    }
    #[doc = "Bit 8 - Clear Data Ready Interrupt"]
    #[inline(always)]
    pub fn gpadc_rdy_clr(&mut self) -> GpadcRdyClrW<'_, GpadcConfigSpec> {
        GpadcRdyClrW::new(self, 8)
    }
    #[doc = "Bit 9 - Clear Overrun Interrupt"]
    #[inline(always)]
    pub fn gpadc_fifo_overrun_clr(&mut self) -> GpadcFifoOverrunClrW<'_, GpadcConfigSpec> {
        GpadcFifoOverrunClrW::new(self, 9)
    }
    #[doc = "Bit 10 - Clear Underrun Interrupt"]
    #[inline(always)]
    pub fn gpadc_fifo_underrun_clr(&mut self) -> GpadcFifoUnderrunClrW<'_, GpadcConfigSpec> {
        GpadcFifoUnderrunClrW::new(self, 10)
    }
    #[doc = "Bit 12 - Mask Data Ready Interrupt"]
    #[inline(always)]
    pub fn gpadc_rdy_mask(&mut self) -> GpadcRdyMaskW<'_, GpadcConfigSpec> {
        GpadcRdyMaskW::new(self, 12)
    }
    #[doc = "Bit 13 - Mask Overrun Interrupt"]
    #[inline(always)]
    pub fn gpadc_fifo_overrun_mask(&mut self) -> GpadcFifoOverrunMaskW<'_, GpadcConfigSpec> {
        GpadcFifoOverrunMaskW::new(self, 13)
    }
    #[doc = "Bit 14 - Mask Underrun Interrupt"]
    #[inline(always)]
    pub fn gpadc_fifo_underrun_mask(&mut self) -> GpadcFifoUnderrunMaskW<'_, GpadcConfigSpec> {
        GpadcFifoUnderrunMaskW::new(self, 14)
    }
    #[doc = "Bits 22:23 - FIFO Threshold: 0:1, 1:4, 2:8, 3:16 data items"]
    #[inline(always)]
    pub fn gpadc_fifo_thl(&mut self) -> GpadcFifoThlW<'_, GpadcConfigSpec> {
        GpadcFifoThlW::new(self, 22)
    }
}
#[doc = "GPADC Configuration and Interrupt Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpadc_config::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpadc_config::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpadcConfigSpec;
impl crate::RegisterSpec for GpadcConfigSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpadc_config::R`](R) reader structure"]
impl crate::Readable for GpadcConfigSpec {}
#[doc = "`write(|w| ..)` method takes [`gpadc_config::W`](W) writer structure"]
impl crate::Writable for GpadcConfigSpec {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x02;
}
#[doc = "`reset()` method sets GPADC_CONFIG to value 0"]
impl crate::Resettable for GpadcConfigSpec {}
