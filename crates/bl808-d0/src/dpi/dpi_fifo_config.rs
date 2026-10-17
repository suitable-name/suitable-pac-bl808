#[doc = "Register `DPI_FIFO_CONFIG` reader"]
pub type R = crate::R<DpiFifoConfigSpec>;
#[doc = "Register `DPI_FIFO_CONFIG` writer"]
pub type W = crate::W<DpiFifoConfigSpec>;
#[doc = "Field `reg_dpi_fifo_th` reader - FIFO Threshold for DMA request generation"]
pub type RegDpiFifoThR = crate::FieldReader;
#[doc = "Field `reg_dpi_fifo_th` writer - FIFO Threshold for DMA request generation"]
pub type RegDpiFifoThW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_dpi_format` reader - Display Data Format: 0: RGB565, 1: RGB888, 2: RGBA8888, 3: YUV422"]
pub type RegDpiFormatR = crate::FieldReader;
#[doc = "Field `reg_dpi_format` writer - Display Data Format: 0: RGB565, 1: RGB888, 2: RGBA8888, 3: YUV422"]
pub type RegDpiFormatW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:7 - FIFO Threshold for DMA request generation"]
    #[inline(always)]
    pub fn reg_dpi_fifo_th(&self) -> RegDpiFifoThR {
        RegDpiFifoThR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:17 - Display Data Format: 0: RGB565, 1: RGB888, 2: RGBA8888, 3: YUV422"]
    #[inline(always)]
    pub fn reg_dpi_format(&self) -> RegDpiFormatR {
        RegDpiFormatR::new(((self.bits >> 16) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - FIFO Threshold for DMA request generation"]
    #[inline(always)]
    pub fn reg_dpi_fifo_th(&mut self) -> RegDpiFifoThW<'_, DpiFifoConfigSpec> {
        RegDpiFifoThW::new(self, 0)
    }
    #[doc = "Bits 16:17 - Display Data Format: 0: RGB565, 1: RGB888, 2: RGBA8888, 3: YUV422"]
    #[inline(always)]
    pub fn reg_dpi_format(&mut self) -> RegDpiFormatW<'_, DpiFifoConfigSpec> {
        RegDpiFormatW::new(self, 16)
    }
}
#[doc = "FIFO and Pixel Format Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`dpi_fifo_config::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dpi_fifo_config::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DpiFifoConfigSpec;
impl crate::RegisterSpec for DpiFifoConfigSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dpi_fifo_config::R`](R) reader structure"]
impl crate::Readable for DpiFifoConfigSpec {}
#[doc = "`write(|w| ..)` method takes [`dpi_fifo_config::W`](W) writer structure"]
impl crate::Writable for DpiFifoConfigSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DPI_FIFO_CONFIG to value 0"]
impl crate::Resettable for DpiFifoConfigSpec {}
